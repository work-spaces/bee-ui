use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Span {
    #[default]
    Auto,
    N(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanRangeError {
    pub columns: u8,
}

impl fmt::Display for SpanRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "span columns must be in {}..={}, got {}",
            Span::MIN_COLUMNS,
            Span::MAX_COLUMNS,
            self.columns
        )
    }
}

impl std::error::Error for SpanRangeError {}

impl Span {
    pub const MIN_COLUMNS: u8 = 1;
    pub const MAX_COLUMNS: u8 = 12;

    pub fn from_columns(columns: u8) -> Self {
        debug_assert!(
            (Self::MIN_COLUMNS..=Self::MAX_COLUMNS).contains(&columns),
            "span columns must be in {}..={}, got {}",
            Self::MIN_COLUMNS,
            Self::MAX_COLUMNS,
            columns
        );

        Self::try_from_columns(columns).unwrap_or(Self::Auto)
    }

    pub fn try_from_columns(columns: u8) -> Result<Self, SpanRangeError> {
        if (Self::MIN_COLUMNS..=Self::MAX_COLUMNS).contains(&columns) {
            Ok(Self::N(columns))
        } else {
            Err(SpanRangeError { columns })
        }
    }

    pub const fn columns(self) -> Option<u8> {
        match self {
            Self::Auto => None,
            Self::N(columns) => Some(columns),
        }
    }

    pub const fn is_valid(self) -> bool {
        match self {
            Self::Auto => true,
            Self::N(columns) => columns >= Self::MIN_COLUMNS && columns <= Self::MAX_COLUMNS,
        }
    }

    pub fn validate(self) -> Result<(), SpanRangeError> {
        match self {
            Self::Auto => Ok(()),
            Self::N(columns) => {
                if (Self::MIN_COLUMNS..=Self::MAX_COLUMNS).contains(&columns) {
                    Ok(())
                } else {
                    Err(SpanRangeError { columns })
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridSpec {
    pub columns: u8,
}

impl GridSpec {
    pub fn validate(self) -> Result<(), GridLayoutError> {
        if self.columns == 0 {
            Err(GridLayoutError::InvalidGridColumns {
                columns: self.columns,
            })
        } else {
            Ok(())
        }
    }

    pub fn pack_lines(
        self,
        cols: &[GridColSpec],
        sort_by_order: bool,
    ) -> Result<Vec<PackedLine>, GridLayoutError> {
        self.validate()?;

        let mut ordered = Vec::with_capacity(cols.len());
        for (source_index, col) in cols.iter().copied().enumerate() {
            col.validate(source_index, self.columns)?;
            ordered.push(PackedCol {
                source_index,
                span: col.span,
                offset: col.offset,
                order: col.order,
            });
        }

        if sort_by_order {
            ordered.sort_by_key(|col| (col.order.unwrap_or(0), col.source_index));
        }

        let mut lines: Vec<PackedLine> = Vec::new();
        let mut line_cols: Vec<PackedCol> = Vec::new();
        let mut line_used = 0_u8;

        for col in ordered {
            let required = col.required_columns();

            if required > self.columns {
                return Err(GridLayoutError::BudgetOverflow {
                    index: col.source_index,
                    required,
                    columns: self.columns,
                });
            }

            let next_used = line_used.saturating_add(required);
            if !line_cols.is_empty() && next_used > self.columns {
                lines.push(PackedLine {
                    cols: line_cols,
                    used_columns: line_used,
                });
                line_cols = Vec::new();
                line_used = 0;
            }

            line_used = line_used.saturating_add(required);
            line_cols.push(col);
        }

        if !line_cols.is_empty() {
            lines.push(PackedLine {
                cols: line_cols,
                used_columns: line_used,
            });
        }

        Ok(lines)
    }

    pub fn compute_lines(
        self,
        cols: &[GridColSpec],
        row_width: f32,
        gutter: f32,
        sort_by_order: bool,
    ) -> Result<Vec<ComputedLine>, GridLayoutError> {
        let packed_lines = self.pack_lines(cols, sort_by_order)?;
        let row_width = normalize_non_negative(row_width);
        let gutter = normalize_non_negative(gutter);

        let mut computed_lines = Vec::with_capacity(packed_lines.len());
        for packed in packed_lines {
            let col_count = packed.cols.len();
            let gaps = col_count.saturating_sub(1) as f32;
            let usable_width = (row_width - gutter * gaps).max(0.0);
            let unit_width = if self.columns == 0 {
                0.0
            } else {
                usable_width / f32::from(self.columns)
            };

            let auto_count = packed
                .cols
                .iter()
                .filter(|col| matches!(col.span, Span::Auto))
                .count();

            let consumed_units: u16 = packed
                .cols
                .iter()
                .map(|col| {
                    let span_units = u16::from(col.span.columns().unwrap_or(0));
                    span_units + u16::from(col.offset)
                })
                .sum();
            let consumed_width = unit_width * consumed_units as f32;

            let remaining_width = (usable_width - consumed_width).max(0.0);
            let auto_width = if auto_count > 0 {
                remaining_width / auto_count as f32
            } else {
                0.0
            };

            let mut computed_cols = Vec::with_capacity(col_count);
            let mut cursor_x = 0.0_f32;

            for (index_in_line, col) in packed.cols.iter().enumerate() {
                cursor_x += unit_width * f32::from(col.offset);

                let width = match col.span {
                    Span::N(span) => unit_width * f32::from(span),
                    Span::Auto => auto_width,
                };

                computed_cols.push(ComputedCol {
                    source_index: col.source_index,
                    span: col.span,
                    offset: col.offset,
                    order: col.order,
                    x: cursor_x,
                    width,
                });

                cursor_x += width;
                if index_in_line + 1 < col_count {
                    cursor_x += gutter;
                }
            }

            computed_lines.push(ComputedLine {
                cols: computed_cols,
                used_columns: packed.used_columns,
                usable_width,
            });
        }

        Ok(computed_lines)
    }
}

impl From<u8> for Span {
    fn from(columns: u8) -> Self {
        Self::from_columns(columns)
    }
}

impl Default for GridSpec {
    fn default() -> Self {
        Self { columns: 12 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GridColSpec {
    pub span: Span,
    pub offset: u8,
    pub order: Option<i32>,
}

impl GridColSpec {
    pub const fn auto() -> Self {
        Self {
            span: Span::Auto,
            offset: 0,
            order: None,
        }
    }

    pub const fn new(span: Span) -> Self {
        Self {
            span,
            offset: 0,
            order: None,
        }
    }

    pub const fn offset(mut self, offset: u8) -> Self {
        self.offset = offset;
        self
    }

    pub const fn order(mut self, order: i32) -> Self {
        self.order = Some(order);
        self
    }

    fn validate(self, index: usize, grid_columns: u8) -> Result<(), GridLayoutError> {
        if let Err(source) = self.span.validate() {
            return Err(GridLayoutError::SpanOutOfRange { index, source });
        }

        if self.offset > grid_columns {
            return Err(GridLayoutError::InvalidOffset {
                index,
                offset: self.offset,
                columns: grid_columns,
            });
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackedCol {
    pub source_index: usize,
    pub span: Span,
    pub offset: u8,
    pub order: Option<i32>,
}

impl PackedCol {
    pub fn required_columns(self) -> u8 {
        self.offset + self.span.columns().unwrap_or(0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackedLine {
    pub cols: Vec<PackedCol>,
    pub used_columns: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedCol {
    pub source_index: usize,
    pub span: Span,
    pub offset: u8,
    pub order: Option<i32>,
    pub x: f32,
    pub width: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComputedLine {
    pub cols: Vec<ComputedCol>,
    pub used_columns: u8,
    pub usable_width: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridLayoutError {
    InvalidGridColumns {
        columns: u8,
    },
    SpanOutOfRange {
        index: usize,
        source: SpanRangeError,
    },
    InvalidOffset {
        index: usize,
        offset: u8,
        columns: u8,
    },
    BudgetOverflow {
        index: usize,
        required: u8,
        columns: u8,
    },
}

impl fmt::Display for GridLayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidGridColumns { columns } => {
                write!(f, "grid columns must be greater than zero, got {}", columns)
            }
            Self::SpanOutOfRange { index, source } => {
                write!(f, "invalid span for column {}: {}", index, source)
            }
            Self::InvalidOffset {
                index,
                offset,
                columns,
            } => write!(
                f,
                "invalid offset {} for column {}: offset must be in 0..={} for this grid",
                offset, index, columns
            ),
            Self::BudgetOverflow {
                index,
                required,
                columns,
            } => write!(
                f,
                "column {} requires {} grid columns but line budget is {}",
                index, required, columns
            ),
        }
    }
}

impl std::error::Error for GridLayoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SpanOutOfRange { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn pack_grid_lines(
    cols: &[GridColSpec],
    sort_by_order: bool,
) -> Result<Vec<PackedLine>, GridLayoutError> {
    GridSpec::default().pack_lines(cols, sort_by_order)
}

pub fn compute_grid_lines(
    cols: &[GridColSpec],
    row_width: f32,
    gutter: f32,
    sort_by_order: bool,
) -> Result<Vec<ComputedLine>, GridLayoutError> {
    GridSpec::default().compute_lines(cols, row_width, gutter, sort_by_order)
}

fn normalize_non_negative(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}
