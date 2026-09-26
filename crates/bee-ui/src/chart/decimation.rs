use super::data::PlotPoint;

pub const DEFAULT_DECIMATION_POINTS_PER_PIXEL: f32 = 2.0;
const MIN_POINT_BUDGET: usize = 128;
const MAX_POINT_BUDGET: usize = 8192;

pub fn viewport_point_budget(plot_width_points: f32, points_per_pixel: f32) -> usize {
    let width = if plot_width_points.is_finite() {
        plot_width_points.max(1.0)
    } else {
        1.0
    };

    let density = if points_per_pixel.is_finite() && points_per_pixel > 0.0 {
        points_per_pixel
    } else {
        DEFAULT_DECIMATION_POINTS_PER_PIXEL
    };

    ((width * density).round() as usize).clamp(MIN_POINT_BUDGET, MAX_POINT_BUDGET)
}

pub fn select_visible_points(points: &[PlotPoint], x_min: f64, x_max: f64) -> Vec<PlotPoint> {
    if points.is_empty() {
        return Vec::new();
    }

    if !x_min.is_finite() || !x_max.is_finite() {
        return points
            .iter()
            .copied()
            .filter(|point| point.x.is_finite() && point.y.is_finite())
            .collect();
    }

    let (window_min, window_max) = if x_min <= x_max {
        (x_min, x_max)
    } else {
        (x_max, x_min)
    };

    if can_use_partition_search(points) {
        let start = points.partition_point(|point| point.x < window_min);
        let end = points.partition_point(|point| point.x <= window_max);

        if start == end {
            let before = start.saturating_sub(1);
            let after = start.min(points.len().saturating_sub(1));
            let range_end = (after + 1).min(points.len());
            return points[before..range_end]
                .iter()
                .copied()
                .filter(|point| point.x.is_finite() && point.y.is_finite())
                .collect();
        }

        let range_start = start.saturating_sub(1);
        let range_end = (end + 1).min(points.len());

        return points[range_start..range_end]
            .iter()
            .copied()
            .filter(|point| point.x.is_finite() && point.y.is_finite())
            .collect();
    }

    points
        .iter()
        .copied()
        .filter(|point| {
            point.x.is_finite()
                && point.y.is_finite()
                && point.x >= window_min
                && point.x <= window_max
        })
        .collect()
}

pub fn downsample_min_max(points: &[PlotPoint], max_points: usize) -> Vec<PlotPoint> {
    if points.is_empty() || max_points == 0 {
        return Vec::new();
    }

    if points.len() <= max_points {
        return points.to_vec();
    }

    if max_points == 1 {
        return vec![points[0]];
    }

    if max_points == 2 {
        return vec![points[0], points[points.len() - 1]];
    }

    if max_points == 3 {
        return vec![
            points[0],
            points[points.len() / 2],
            points[points.len() - 1],
        ];
    }

    let interior = &points[1..points.len() - 1];
    if interior.is_empty() {
        return vec![points[0], points[points.len() - 1]];
    }

    let bucket_count = ((max_points.saturating_sub(2)) / 2)
        .max(1)
        .min(interior.len());
    let mut reduced = Vec::with_capacity((bucket_count * 2 + 2).min(points.len()));
    reduced.push(points[0]);

    for bucket_index in 0..bucket_count {
        let bucket_start = bucket_index * interior.len() / bucket_count;
        let mut bucket_end = (bucket_index + 1) * interior.len() / bucket_count;
        bucket_end = bucket_end.max(bucket_start + 1).min(interior.len());
        let bucket = &interior[bucket_start..bucket_end];

        let mut min_idx = 0_usize;
        let mut max_idx = 0_usize;

        for (idx, point) in bucket.iter().enumerate().skip(1) {
            if point.y < bucket[min_idx].y {
                min_idx = idx;
            }
            if point.y > bucket[max_idx].y {
                max_idx = idx;
            }
        }

        if min_idx <= max_idx {
            reduced.push(bucket[min_idx]);
            if max_idx != min_idx {
                reduced.push(bucket[max_idx]);
            }
        } else {
            reduced.push(bucket[max_idx]);
            reduced.push(bucket[min_idx]);
        }
    }

    reduced.push(points[points.len() - 1]);

    dedupe_consecutive_points(&mut reduced);

    while reduced.len() > max_points {
        let removal_index = reduced.len().saturating_sub(2);
        reduced.remove(removal_index);
    }

    reduced
}

pub fn prepare_line_points(
    points: &[PlotPoint],
    x_min: f64,
    x_max: f64,
    max_points: usize,
) -> Vec<[f64; 2]> {
    let visible = select_visible_points(points, x_min, x_max);
    let reduced = downsample_min_max(&visible, max_points);
    reduced
        .into_iter()
        .map(|point| [point.x, point.y])
        .collect()
}

fn can_use_partition_search(points: &[PlotPoint]) -> bool {
    let Some(first) = points.first() else {
        return false;
    };
    let Some(last) = points.last() else {
        return false;
    };

    first.x.is_finite() && last.x.is_finite() && first.x <= last.x
}

fn dedupe_consecutive_points(points: &mut Vec<PlotPoint>) {
    points.dedup_by(|a, b| (a.x - b.x).abs() <= f64::EPSILON && (a.y - b.y).abs() <= f64::EPSILON);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_points() -> Vec<PlotPoint> {
        (0..100)
            .map(|index| {
                let x = index as f64;
                let y = if index == 50 { 10.0 } else { (x / 9.0).sin() };
                PlotPoint::new(x, y)
            })
            .collect()
    }

    #[test]
    fn viewport_budget_is_clamped() {
        assert_eq!(viewport_point_budget(0.0, 2.0), MIN_POINT_BUDGET);
        assert_eq!(viewport_point_budget(20_000.0, 4.0), MAX_POINT_BUDGET);
    }

    #[test]
    fn visible_point_selection_keeps_edge_context() {
        let points = sample_points();
        let visible = select_visible_points(&points, 20.0, 24.0);

        assert!(visible.first().is_some_and(|point| point.x <= 20.0));
        assert!(visible.last().is_some_and(|point| point.x >= 24.0));
        assert!(visible.len() >= 5);
    }

    #[test]
    fn min_max_downsampling_keeps_spikes_and_bounds() {
        let points = sample_points();
        let reduced = downsample_min_max(&points, 24);

        assert!(reduced.len() <= 24);
        assert_eq!(reduced.first(), points.first());
        assert_eq!(reduced.last(), points.last());
        assert!(reduced.iter().any(|point| point.y >= 10.0));
    }

    #[test]
    fn prepare_line_points_combines_filtering_and_decimation() {
        let points = sample_points();
        let line = prepare_line_points(&points, 10.0, 90.0, 32);

        assert!(line.len() <= 32);
        assert!(line.iter().all(|[x, y]| x.is_finite() && y.is_finite()));
    }
}
