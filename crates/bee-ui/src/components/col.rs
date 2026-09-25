use std::fmt;

use crate::layout::{Breakpoint, ResponsiveSpan, ResponsiveSpanError, Span};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColValidationIssue {
    SpanOutOfRange { prop: &'static str, columns: u8 },
    OffsetOutOfRange { offset: u8, max: u8 },
    OrderOutOfRange { order: i32, min: i32, max: i32 },
}

impl fmt::Display for ColValidationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SpanOutOfRange { prop, columns } => write!(
                f,
                "invalid semantic prop `ColProps.{}` value `{}`: span must be in {}..={} or `Span::Auto`",
                prop,
                columns,
                Span::MIN_COLUMNS,
                Span::MAX_COLUMNS
            ),
            Self::OffsetOutOfRange { offset, max } => write!(
                f,
                "invalid semantic prop `ColProps.offset` value `{}`: offset must be in 0..={} for a 12-column grid",
                offset, max
            ),
            Self::OrderOutOfRange { order, min, max } => write!(
                f,
                "invalid semantic prop `ColProps.order` value `{}`: order must be in {}..={} for semantic Bootstrap-like ordering",
                order, min, max
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColProps {
    pub xs: Span,
    pub sm: Option<Span>,
    pub md: Option<Span>,
    pub lg: Option<Span>,
    pub xl: Option<Span>,
    pub xxl: Option<Span>,
    pub offset: u8,
    pub order: Option<i32>,
}

impl ColProps {
    pub const MIN_ORDER: i32 = -12;
    pub const MAX_ORDER: i32 = 12;

    pub fn new() -> Self {
        Self::default()
    }

    pub const fn responsive_span(&self) -> ResponsiveSpan {
        ResponsiveSpan {
            xs: self.xs,
            sm: self.sm,
            md: self.md,
            lg: self.lg,
            xl: self.xl,
            xxl: self.xxl,
        }
    }

    pub fn resolve_span(&self, active_breakpoint: Breakpoint) -> Result<Span, ResponsiveSpanError> {
        self.responsive_span().resolve_for(active_breakpoint)
    }

    pub fn resolve_span_unchecked(&self, active_breakpoint: Breakpoint) -> Span {
        self.responsive_span()
            .resolve_for_unchecked(active_breakpoint)
    }

    pub fn xs(mut self, span: impl Into<Span>) -> Self {
        self.xs = span.into();
        self
    }

    pub fn sm(mut self, span: impl Into<Span>) -> Self {
        self.sm = Some(span.into());
        self
    }

    pub fn md(mut self, span: impl Into<Span>) -> Self {
        self.md = Some(span.into());
        self
    }

    pub fn lg(mut self, span: impl Into<Span>) -> Self {
        self.lg = Some(span.into());
        self
    }

    pub fn xl(mut self, span: impl Into<Span>) -> Self {
        self.xl = Some(span.into());
        self
    }

    pub fn xxl(mut self, span: impl Into<Span>) -> Self {
        self.xxl = Some(span.into());
        self
    }

    pub fn offset(mut self, offset: u8) -> Self {
        self.offset = offset;
        self
    }

    pub fn order(mut self, order: i32) -> Self {
        self.order = Some(order);
        self
    }

    pub fn validation_issues(&self) -> Vec<ColValidationIssue> {
        let mut issues = Vec::new();
        Self::push_span_issue("xs", self.xs, &mut issues);

        if let Some(span) = self.sm {
            Self::push_span_issue("sm", span, &mut issues);
        }

        if let Some(span) = self.md {
            Self::push_span_issue("md", span, &mut issues);
        }

        if let Some(span) = self.lg {
            Self::push_span_issue("lg", span, &mut issues);
        }

        if let Some(span) = self.xl {
            Self::push_span_issue("xl", span, &mut issues);
        }

        if let Some(span) = self.xxl {
            Self::push_span_issue("xxl", span, &mut issues);
        }

        if self.offset > Span::MAX_COLUMNS {
            issues.push(ColValidationIssue::OffsetOutOfRange {
                offset: self.offset,
                max: Span::MAX_COLUMNS,
            });
        }

        if let Some(order) = self.order
            && !(Self::MIN_ORDER..=Self::MAX_ORDER).contains(&order)
        {
            issues.push(ColValidationIssue::OrderOutOfRange {
                order,
                min: Self::MIN_ORDER,
                max: Self::MAX_ORDER,
            });
        }

        issues
    }

    pub fn debug_assert_valid(&self) {
        debug_assert!(
            self.xs.is_valid(),
            "invalid semantic prop `ColProps.xs` value `{:?}`: span must be in {}..={} or `Span::Auto`",
            self.xs,
            Span::MIN_COLUMNS,
            Span::MAX_COLUMNS
        );

        if let Some(span) = self.sm {
            debug_assert!(
                span.is_valid(),
                "invalid semantic prop `ColProps.sm` value `{:?}`: span must be in {}..={} or `Span::Auto`",
                span,
                Span::MIN_COLUMNS,
                Span::MAX_COLUMNS
            );
        }

        if let Some(span) = self.md {
            debug_assert!(
                span.is_valid(),
                "invalid semantic prop `ColProps.md` value `{:?}`: span must be in {}..={} or `Span::Auto`",
                span,
                Span::MIN_COLUMNS,
                Span::MAX_COLUMNS
            );
        }

        if let Some(span) = self.lg {
            debug_assert!(
                span.is_valid(),
                "invalid semantic prop `ColProps.lg` value `{:?}`: span must be in {}..={} or `Span::Auto`",
                span,
                Span::MIN_COLUMNS,
                Span::MAX_COLUMNS
            );
        }

        if let Some(span) = self.xl {
            debug_assert!(
                span.is_valid(),
                "invalid semantic prop `ColProps.xl` value `{:?}`: span must be in {}..={} or `Span::Auto`",
                span,
                Span::MIN_COLUMNS,
                Span::MAX_COLUMNS
            );
        }

        if let Some(span) = self.xxl {
            debug_assert!(
                span.is_valid(),
                "invalid semantic prop `ColProps.xxl` value `{:?}`: span must be in {}..={} or `Span::Auto`",
                span,
                Span::MIN_COLUMNS,
                Span::MAX_COLUMNS
            );
        }

        debug_assert!(
            self.offset <= Span::MAX_COLUMNS,
            "invalid semantic prop `ColProps.offset` value `{}`: offset must be in 0..={} for a 12-column grid",
            self.offset,
            Span::MAX_COLUMNS
        );

        if let Some(order) = self.order {
            debug_assert!(
                (Self::MIN_ORDER..=Self::MAX_ORDER).contains(&order),
                "invalid semantic prop `ColProps.order` value `{}`: order must be in {}..={} for semantic Bootstrap-like ordering",
                order,
                Self::MIN_ORDER,
                Self::MAX_ORDER
            );
        }
    }

    fn push_span_issue(prop: &'static str, span: Span, issues: &mut Vec<ColValidationIssue>) {
        if let Span::N(columns) = span
            && !(Span::MIN_COLUMNS..=Span::MAX_COLUMNS).contains(&columns)
        {
            issues.push(ColValidationIssue::SpanOutOfRange { prop, columns });
        }
    }
}

impl Default for ColProps {
    fn default() -> Self {
        Self {
            xs: Span::Auto,
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xxl: None,
            offset: 0,
            order: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Col {
    pub xs: Span,
    pub sm: Option<Span>,
    pub md: Option<Span>,
    pub lg: Option<Span>,
    pub xl: Option<Span>,
    pub xxl: Option<Span>,
    pub offset: u8,
    pub order: Option<i32>,
}

impl Col {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn responsive_span(&self) -> ResponsiveSpan {
        ResponsiveSpan {
            xs: self.xs,
            sm: self.sm,
            md: self.md,
            lg: self.lg,
            xl: self.xl,
            xxl: self.xxl,
        }
    }

    pub fn resolve_span(&self, active_breakpoint: Breakpoint) -> Result<Span, ResponsiveSpanError> {
        self.responsive_span().resolve_for(active_breakpoint)
    }

    pub fn xs(mut self, span: impl Into<Span>) -> Self {
        self.xs = span.into();
        self
    }

    pub fn sm(mut self, span: impl Into<Span>) -> Self {
        self.sm = Some(span.into());
        self
    }

    pub fn md(mut self, span: impl Into<Span>) -> Self {
        self.md = Some(span.into());
        self
    }

    pub fn lg(mut self, span: impl Into<Span>) -> Self {
        self.lg = Some(span.into());
        self
    }

    pub fn xl(mut self, span: impl Into<Span>) -> Self {
        self.xl = Some(span.into());
        self
    }

    pub fn xxl(mut self, span: impl Into<Span>) -> Self {
        self.xxl = Some(span.into());
        self
    }

    pub fn offset(mut self, offset: u8) -> Self {
        self.offset = offset;
        self
    }

    pub fn order(mut self, order: i32) -> Self {
        self.order = Some(order);
        self
    }

    pub fn props(self) -> ColProps {
        self.into()
    }
}

impl Default for Col {
    fn default() -> Self {
        Self {
            xs: Span::Auto,
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xxl: None,
            offset: 0,
            order: None,
        }
    }
}

impl From<Col> for ColProps {
    fn from(value: Col) -> Self {
        Self {
            xs: value.xs,
            sm: value.sm,
            md: value.md,
            lg: value.lg,
            xl: value.xl,
            xxl: value.xxl,
            offset: value.offset,
            order: value.order,
        }
    }
}

impl From<ColProps> for Col {
    fn from(value: ColProps) -> Self {
        Self {
            xs: value.xs,
            sm: value.sm,
            md: value.md,
            lg: value.lg,
            xl: value.xl,
            xxl: value.xxl,
            offset: value.offset,
            order: value.order,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn col_validation_flags_invalid_semantic_values() {
        let props = ColProps::new().xs(Span::N(0)).offset(13).order(99);
        let issues = props.validation_issues();

        assert_eq!(issues.len(), 3);
        assert!(issues.iter().any(|issue| matches!(
            issue,
            ColValidationIssue::SpanOutOfRange {
                prop: "xs",
                columns: 0
            }
        )));
        assert!(issues.iter().any(|issue| matches!(
            issue,
            ColValidationIssue::OffsetOutOfRange {
                offset: 13,
                max: 12
            }
        )));
        assert!(issues.iter().any(|issue| matches!(
            issue,
            ColValidationIssue::OrderOutOfRange {
                order: 99,
                min: ColProps::MIN_ORDER,
                max: ColProps::MAX_ORDER,
            }
        )));
    }
}
