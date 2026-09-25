use std::fmt;

use super::grid::{Span, SpanRangeError};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Breakpoint {
    #[default]
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Breakpoints {
    pub sm: u16,
    pub md: u16,
    pub lg: u16,
    pub xl: u16,
    pub xxl: u16,
}

impl Breakpoints {
    pub const fn bootstrap() -> Self {
        Self {
            sm: 576,
            md: 768,
            lg: 992,
            xl: 1200,
            xxl: 1400,
        }
    }

    pub const fn is_monotonic(self) -> bool {
        self.sm <= self.md && self.md <= self.lg && self.lg <= self.xl && self.xl <= self.xxl
    }
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self::bootstrap()
    }
}

pub fn resolve_active_breakpoint(viewport_width: f32, breakpoints: Breakpoints) -> Breakpoint {
    debug_assert!(
        breakpoints.is_monotonic(),
        "breakpoint thresholds must be monotonic"
    );

    let width = if viewport_width.is_finite() {
        viewport_width.max(0.0)
    } else {
        0.0
    };

    if width >= f32::from(breakpoints.xxl) {
        Breakpoint::Xxl
    } else if width >= f32::from(breakpoints.xl) {
        Breakpoint::Xl
    } else if width >= f32::from(breakpoints.lg) {
        Breakpoint::Lg
    } else if width >= f32::from(breakpoints.md) {
        Breakpoint::Md
    } else if width >= f32::from(breakpoints.sm) {
        Breakpoint::Sm
    } else {
        Breakpoint::Xs
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResponsiveSpan {
    pub xs: Span,
    pub sm: Option<Span>,
    pub md: Option<Span>,
    pub lg: Option<Span>,
    pub xl: Option<Span>,
    pub xxl: Option<Span>,
}

impl ResponsiveSpan {
    pub const fn new(xs: Span) -> Self {
        Self {
            xs,
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xxl: None,
        }
    }

    pub const fn sm(mut self, span: Span) -> Self {
        self.sm = Some(span);
        self
    }

    pub const fn md(mut self, span: Span) -> Self {
        self.md = Some(span);
        self
    }

    pub const fn lg(mut self, span: Span) -> Self {
        self.lg = Some(span);
        self
    }

    pub const fn xl(mut self, span: Span) -> Self {
        self.xl = Some(span);
        self
    }

    pub const fn xxl(mut self, span: Span) -> Self {
        self.xxl = Some(span);
        self
    }

    pub fn validate(&self) -> Result<(), ResponsiveSpanError> {
        validate_span(Breakpoint::Xs, self.xs)?;

        if let Some(span) = self.sm {
            validate_span(Breakpoint::Sm, span)?;
        }

        if let Some(span) = self.md {
            validate_span(Breakpoint::Md, span)?;
        }

        if let Some(span) = self.lg {
            validate_span(Breakpoint::Lg, span)?;
        }

        if let Some(span) = self.xl {
            validate_span(Breakpoint::Xl, span)?;
        }

        if let Some(span) = self.xxl {
            validate_span(Breakpoint::Xxl, span)?;
        }

        Ok(())
    }

    /// Resolves the effective span for the active breakpoint.
    ///
    /// Fallback rule (Bootstrap-like): if the active breakpoint does not have
    /// an explicit declaration, the nearest lower breakpoint declaration is
    /// used. Example: at `Lg`, use `lg`, else `md`, else `sm`, else `xs`.
    pub fn resolve_for(&self, breakpoint: Breakpoint) -> Result<Span, ResponsiveSpanError> {
        self.validate()?;
        Ok(self.resolve_for_unchecked(breakpoint))
    }

    /// Resolves the effective span using fallback semantics without validating
    /// the declared values.
    pub fn resolve_for_unchecked(&self, breakpoint: Breakpoint) -> Span {
        match breakpoint {
            Breakpoint::Xxl => self
                .xxl
                .or(self.xl)
                .or(self.lg)
                .or(self.md)
                .or(self.sm)
                .unwrap_or(self.xs),
            Breakpoint::Xl => self
                .xl
                .or(self.lg)
                .or(self.md)
                .or(self.sm)
                .unwrap_or(self.xs),
            Breakpoint::Lg => self.lg.or(self.md).or(self.sm).unwrap_or(self.xs),
            Breakpoint::Md => self.md.or(self.sm).unwrap_or(self.xs),
            Breakpoint::Sm => self.sm.unwrap_or(self.xs),
            Breakpoint::Xs => self.xs,
        }
    }
}

impl Default for ResponsiveSpan {
    fn default() -> Self {
        Self::new(Span::Auto)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResponsiveSpanError {
    pub breakpoint: Breakpoint,
    pub source: SpanRangeError,
}

impl fmt::Display for ResponsiveSpanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid span at {:?}: {}", self.breakpoint, self.source)
    }
}

impl std::error::Error for ResponsiveSpanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

fn validate_span(breakpoint: Breakpoint, span: Span) -> Result<(), ResponsiveSpanError> {
    span.validate()
        .map_err(|source| ResponsiveSpanError { breakpoint, source })
}

pub fn resolve_breakpoint_span(
    viewport_width: f32,
    breakpoints: Breakpoints,
    spans: &ResponsiveSpan,
) -> Result<(Breakpoint, Span), ResponsiveSpanError> {
    let active = resolve_active_breakpoint(viewport_width, breakpoints);
    let span = spans.resolve_for(active)?;
    Ok((active, span))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_viewport_width_to_breakpoint() {
        let bps = Breakpoints::default();

        assert_eq!(resolve_active_breakpoint(0.0, bps), Breakpoint::Xs);
        assert_eq!(resolve_active_breakpoint(575.9, bps), Breakpoint::Xs);

        assert_eq!(resolve_active_breakpoint(576.0, bps), Breakpoint::Sm);
        assert_eq!(resolve_active_breakpoint(767.9, bps), Breakpoint::Sm);

        assert_eq!(resolve_active_breakpoint(768.0, bps), Breakpoint::Md);
        assert_eq!(resolve_active_breakpoint(991.9, bps), Breakpoint::Md);

        assert_eq!(resolve_active_breakpoint(992.0, bps), Breakpoint::Lg);
        assert_eq!(resolve_active_breakpoint(1199.9, bps), Breakpoint::Lg);

        assert_eq!(resolve_active_breakpoint(1200.0, bps), Breakpoint::Xl);
        assert_eq!(resolve_active_breakpoint(1399.9, bps), Breakpoint::Xl);

        assert_eq!(resolve_active_breakpoint(1400.0, bps), Breakpoint::Xxl);
    }

    #[test]
    fn resolves_responsive_span_with_bootstrap_fallback() {
        let spans = ResponsiveSpan::new(Span::from_columns(2))
            .md(Span::from_columns(6))
            .xl(Span::from_columns(8));

        assert_eq!(spans.resolve_for(Breakpoint::Xs), Ok(Span::N(2)));
        assert_eq!(spans.resolve_for(Breakpoint::Sm), Ok(Span::N(2)));
        assert_eq!(spans.resolve_for(Breakpoint::Md), Ok(Span::N(6)));
        assert_eq!(spans.resolve_for(Breakpoint::Lg), Ok(Span::N(6)));
        assert_eq!(spans.resolve_for(Breakpoint::Xl), Ok(Span::N(8)));
        assert_eq!(spans.resolve_for(Breakpoint::Xxl), Ok(Span::N(8)));
    }

    #[test]
    fn rejects_invalid_span_values() {
        assert_eq!(
            Span::try_from_columns(0),
            Err(SpanRangeError { columns: 0 })
        );
        assert_eq!(
            Span::try_from_columns(13),
            Err(SpanRangeError { columns: 13 })
        );

        assert_eq!(Span::N(0).validate(), Err(SpanRangeError { columns: 0 }));

        let invalid = ResponsiveSpan::new(Span::N(4)).lg(Span::N(0));
        assert_eq!(
            invalid.resolve_for(Breakpoint::Lg),
            Err(ResponsiveSpanError {
                breakpoint: Breakpoint::Lg,
                source: SpanRangeError { columns: 0 },
            })
        );
    }
}
