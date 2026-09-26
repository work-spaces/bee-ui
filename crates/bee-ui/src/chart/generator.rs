use std::f64::consts::TAU;

use super::data::{PlotPoint, SeriesId, TimeSeries};

const DEFAULT_SAMPLE_RATE_HZ: f64 = 120.0;
const DEFAULT_NOISE_AMPLITUDE: f64 = 0.05;
const DEFAULT_DRIFT: f64 = 0.0;
const DEFAULT_SEED: u64 = 0x5eed_fade_cafe_beef;
const MIN_SAMPLE_RATE_HZ: f64 = 1e-6;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GeneratorProfile {
    Sine,
    #[default]
    MultiSine,
    Chirp,
    RandomWalk,
    BurstyWithSpikes,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MathGenerator {
    pub profile: GeneratorProfile,
    pub sample_rate_hz: f64,
    pub noise_amplitude: f64,
    pub drift: f64,
    seed: u64,
    rng_state: u64,
    sample_index: u64,
    random_walk_value: f64,
    burst_remaining: u32,
    burst_amplitude: f64,
    regime_bias: f64,
}

impl Default for MathGenerator {
    fn default() -> Self {
        Self::new(GeneratorProfile::default())
    }
}

impl MathGenerator {
    pub fn new(profile: GeneratorProfile) -> Self {
        let mut generator = Self {
            profile,
            sample_rate_hz: DEFAULT_SAMPLE_RATE_HZ,
            noise_amplitude: DEFAULT_NOISE_AMPLITUDE,
            drift: DEFAULT_DRIFT,
            seed: DEFAULT_SEED,
            rng_state: DEFAULT_SEED,
            sample_index: 0,
            random_walk_value: 0.0,
            burst_remaining: 0,
            burst_amplitude: 0.0,
            regime_bias: 0.0,
        };
        generator.reset();
        generator
    }

    pub fn seeded(profile: GeneratorProfile, seed: u64) -> Self {
        Self::new(profile).with_seed(seed)
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self.reset();
        self
    }

    pub fn sample_rate_hz(mut self, sample_rate_hz: f64) -> Self {
        self.sample_rate_hz = sample_rate_hz;
        self
    }

    pub fn noise_amplitude(mut self, noise_amplitude: f64) -> Self {
        self.noise_amplitude = noise_amplitude;
        self
    }

    pub fn drift(mut self, drift: f64) -> Self {
        self.drift = drift;
        self
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn reset(&mut self) {
        self.rng_state = if self.seed == 0 {
            DEFAULT_SEED
        } else {
            self.seed
        };
        self.sample_index = 0;
        self.random_walk_value = 0.0;
        self.burst_remaining = 0;
        self.burst_amplitude = 0.0;
        self.regime_bias = 0.0;
    }

    pub fn next_point(&mut self) -> PlotPoint {
        let sample_rate_hz = sanitize_sample_rate(self.sample_rate_hz);
        let x = self.sample_index as f64 / sample_rate_hz;
        let y = self.sample_value(x, sample_rate_hz);
        self.sample_index = self.sample_index.saturating_add(1);
        PlotPoint::new(x, y)
    }

    pub fn generate_next(&mut self, point_count: usize) -> Vec<PlotPoint> {
        (0..point_count).map(|_| self.next_point()).collect()
    }

    pub fn generate_bulk(&self, point_count: usize) -> Vec<PlotPoint> {
        let mut generator = self.clone();
        generator.reset();
        generator.generate_next(point_count)
    }

    pub fn generate_series(
        &self,
        id: impl Into<SeriesId>,
        label: impl Into<String>,
        point_count: usize,
    ) -> TimeSeries {
        TimeSeries::new(id, label).with_points(self.generate_bulk(point_count))
    }

    fn sample_value(&mut self, t: f64, sample_rate_hz: f64) -> f64 {
        match self.profile {
            GeneratorProfile::Sine => {
                let base = (TAU * 0.25 * t).sin();
                base + self.drift * t + self.sample_noise()
            }
            GeneratorProfile::MultiSine => {
                let base = (TAU * 0.08 * t).sin() * 0.85
                    + (TAU * 0.23 * t + 0.4).sin() * 0.35
                    + (TAU * 0.61 * t + 1.3).sin() * 0.15;
                base + self.drift * t + self.sample_noise()
            }
            GeneratorProfile::Chirp => {
                let chirp_rate = 0.01;
                let phase = TAU * ((0.02 * t) + 0.5 * chirp_rate * t * t);
                phase.sin() + self.drift * t + self.sample_noise()
            }
            GeneratorProfile::RandomWalk => {
                let step_noise = self.next_symmetric_unit() * self.noise_amplitude.max(0.02);
                let drift_step = self.drift / sample_rate_hz;
                self.random_walk_value += step_noise + drift_step;
                self.random_walk_value
            }
            GeneratorProfile::BurstyWithSpikes => {
                if self.next_unit_interval() < 0.0015 {
                    self.regime_bias += self.next_symmetric_unit() * 1.5;
                }

                if self.burst_remaining == 0 && self.next_unit_interval() < 0.002 {
                    self.burst_remaining = 18 + (self.next_unit_interval() * 72.0) as u32;
                    self.burst_amplitude = 1.5 + self.next_unit_interval() * 4.0;
                }

                let burst_multiplier = if self.burst_remaining > 0 {
                    let progress = self.burst_remaining as f64 / 90.0;
                    self.burst_remaining -= 1;
                    1.0 + self.burst_amplitude * progress
                } else {
                    1.0
                };

                let base = ((TAU * 0.07 * t).sin() * 0.7 + (TAU * 0.19 * t + 0.8).sin() * 0.4)
                    * burst_multiplier;

                let mut value =
                    base + self.regime_bias + self.drift * t + self.sample_noise() * 1.25;

                if self.next_unit_interval() < 0.003 {
                    value += self.next_symmetric_unit() * (3.0 + self.burst_amplitude * 1.5);
                }

                value
            }
        }
    }

    fn sample_noise(&mut self) -> f64 {
        let amplitude = if self.noise_amplitude.is_finite() {
            self.noise_amplitude.abs()
        } else {
            0.0
        };
        self.next_symmetric_unit() * amplitude
    }

    fn next_unit_interval(&mut self) -> f64 {
        let value = next_u64(&mut self.rng_state);
        let shifted = value >> 11;
        shifted as f64 / ((1_u64 << 53) as f64)
    }

    fn next_symmetric_unit(&mut self) -> f64 {
        self.next_unit_interval() * 2.0 - 1.0
    }
}

fn sanitize_sample_rate(sample_rate_hz: f64) -> f64 {
    if sample_rate_hz.is_finite() && sample_rate_hz >= MIN_SAMPLE_RATE_HZ {
        sample_rate_hz
    } else {
        DEFAULT_SAMPLE_RATE_HZ
    }
}

fn next_u64(state: &mut u64) -> u64 {
    let mut x = if *state == 0 { DEFAULT_SEED } else { *state };
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    *state = x;
    x.wrapping_mul(0x2545_F491_4F6C_DD1D)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bulk_generation_is_deterministic_for_same_seed() {
        let generator = MathGenerator::seeded(GeneratorProfile::MultiSine, 42)
            .sample_rate_hz(250.0)
            .noise_amplitude(0.15)
            .drift(0.001);

        let first = generator.generate_bulk(512);
        let second = generator.generate_bulk(512);

        assert_eq!(first, second);
    }

    #[test]
    fn incremental_generation_matches_bulk_after_reset() {
        let mut generator = MathGenerator::seeded(GeneratorProfile::Chirp, 7)
            .sample_rate_hz(120.0)
            .noise_amplitude(0.1);

        let incremental = generator.generate_next(256);
        generator.reset();
        let replay = generator.generate_next(256);

        assert_eq!(incremental, replay);
    }

    #[test]
    fn generate_series_uses_requested_point_count() {
        let generator = MathGenerator::seeded(GeneratorProfile::Sine, 99);
        let series = generator.generate_series("latency", "Latency", 1024);

        assert_eq!(series.id.as_str(), "latency");
        assert_eq!(series.label, "Latency");
        assert_eq!(series.points.len(), 1024);
        assert!(series.points[0].x <= series.points[1].x);
    }
}
