//! Immediate-mode placement for `UiRoot::row`.
//!
//! A row can't know every column's spec before drawing the first one, so it lays out from the
//! specs recorded on the previous pass (the same approach as `egui::Grid`). When this pass's specs
//! differ, they're stored and the pass is discarded so egui re-runs it with the correct layout.

use egui::{Pos2, Rect, pos2, vec2};

use crate::{
    components::{ColProps, RowProps},
    layout::{Breakpoint, ComputedCol, ComputedLine, GridColSpec, Span, compute_grid_lines},
};

/// Column specs seen on the previous pass, kept in egui temp memory under the row's ID.
#[derive(Clone)]
struct StoredRowSpecs(Vec<GridColSpec>);

/// Resolves a column's props into a grid spec for the active breakpoint.
///
/// Invalid values fall back to their defaults so layout never fails; strict mode reports them
/// separately.
pub(crate) fn col_spec(props: &ColProps, breakpoint: Breakpoint) -> GridColSpec {
    let span = props.resolve_span_unchecked(breakpoint);
    let span = if span.is_valid() { span } else { Span::Auto };
    let offset = if props.offset <= Span::MAX_COLUMNS {
        props.offset
    } else {
        0
    };
    let order = props
        .order
        .filter(|order| (ColProps::MIN_ORDER..=ColProps::MAX_ORDER).contains(order));

    GridColSpec {
        span,
        offset,
        order,
    }
}

struct OpenLine {
    index: usize,
    line: ComputedLine,
    top: f32,
    bottom: f32,
}

/// Tracks placement while a row's closure runs.
pub(crate) struct RowCursor {
    id: egui::Id,
    props: RowProps,
    breakpoint: Breakpoint,
    origin: Pos2,
    width: f32,
    max_bottom: f32,
    line_gap: f32,
    /// Specs from the previous pass, or `None` the first time this row is seen.
    stored: Option<Vec<GridColSpec>>,
    /// Lines computed from `stored`.
    lines: Vec<ComputedLine>,
    /// Line index and placement for each stored column, indexed by source order.
    slots: Vec<(usize, ComputedCol)>,
    /// Specs recorded on this pass.
    specs: Vec<GridColSpec>,
    open_line: Option<OpenLine>,
    next_line_top: f32,
    used_rect: Option<Rect>,
}

impl RowCursor {
    /// Starts a row at the top-left of `available`, laid out from the specs stored for `id`.
    ///
    /// `line_gap` is the vertical space inserted between wrapped lines.
    pub(crate) fn new(
        ctx: &egui::Context,
        id: egui::Id,
        props: RowProps,
        breakpoint: Breakpoint,
        available: Rect,
        line_gap: f32,
    ) -> Self {
        let stored = ctx.data(|data| data.get_temp::<StoredRowSpecs>(id).map(|stored| stored.0));
        let width = available.width();
        let gutter_x = props.spacing().gutter_x();

        let lines = stored
            .as_deref()
            .map(|specs| {
                compute_grid_lines(specs, width, gutter_x, false)
                    .or_else(|_| {
                        let fallback = vec![GridColSpec::auto(); specs.len()];
                        compute_grid_lines(&fallback, width, gutter_x, false)
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let mut slots = Vec::with_capacity(stored.as_ref().map_or(0, Vec::len));
        for (line_index, line) in lines.iter().enumerate() {
            for col in &line.cols {
                slots.push((line_index, *col));
            }
        }
        slots.sort_by_key(|(_, col)| col.source_index);

        Self {
            id,
            props,
            breakpoint,
            origin: available.min,
            width,
            max_bottom: available.max.y,
            line_gap: line_gap.max(0.0),
            stored,
            lines,
            slots,
            specs: Vec::new(),
            open_line: None,
            next_line_top: available.min.y,
            used_rect: None,
        }
    }

    pub(crate) const fn props(&self) -> &RowProps {
        &self.props
    }

    /// `true` when there's no stored layout yet, so this pass's placement is only a guess.
    pub(crate) const fn is_first_appearance(&self) -> bool {
        self.stored.is_none()
    }

    /// Records the next column's spec and returns its line index and placement.
    ///
    /// Columns without a stored slot (more columns than last pass) each get their own
    /// full-width line after the stored ones. That pass is discarded, so it only needs to be
    /// reasonable.
    pub(crate) fn next_placement(&mut self, props: &ColProps) -> (usize, ComputedCol) {
        let source_index = self.specs.len();
        let spec = col_spec(props, self.breakpoint);
        self.specs.push(spec);

        if let Some(&(line_index, computed)) = self.slots.get(source_index) {
            return (line_index, computed);
        }

        let line_index = self.lines.len() + (source_index - self.slots.len());
        let computed = ComputedCol {
            source_index,
            span: Span::Auto,
            offset: 0,
            order: spec.order,
            x: 0.0,
            width: self.width.max(0.0),
        };
        (line_index, computed)
    }

    pub(crate) fn open_line_index(&self) -> Option<usize> {
        self.open_line.as_ref().map(|open| open.index)
    }

    /// Starts line `index` below the previous one and returns it for the backend's row hook.
    ///
    /// `computed` is the first column placed on the line; it's used to build a single-column
    /// line when `index` has no stored line.
    pub(crate) fn open_line(&mut self, index: usize, computed: &ComputedCol) -> &ComputedLine {
        debug_assert!(self.open_line.is_none(), "previous row line was not closed");

        let line = self
            .lines
            .get(index)
            .cloned()
            .unwrap_or_else(|| ComputedLine {
                cols: vec![*computed],
                used_columns: 0,
                usable_width: self.width.max(0.0),
            });
        let top = self.next_line_top;

        &self
            .open_line
            .insert(OpenLine {
                index,
                line,
                top,
                bottom: top,
            })
            .line
    }

    /// Ends the open line, if any, and returns it for the backend's row hook.
    pub(crate) fn close_line(&mut self) -> Option<ComputedLine> {
        let open = self.open_line.take()?;
        self.next_line_top = open.bottom + self.line_gap;
        Some(open.line)
    }

    /// The rectangle a column may draw into on the open line.
    pub(crate) fn slot_rect(&self, computed: &ComputedCol) -> Rect {
        let top = self
            .open_line
            .as_ref()
            .map_or(self.next_line_top, |open| open.top);
        let height = (self.max_bottom - top).max(0.0);

        Rect::from_min_size(
            pos2(self.origin.x + computed.x, top),
            vec2(computed.width.max(0.0), height),
        )
    }

    /// Records the space a column used so the open line and the row grow to fit it.
    pub(crate) fn record_col(&mut self, slot: Rect, content: Rect) {
        let Some(open) = self.open_line.as_mut() else {
            debug_assert!(false, "row column recorded without an open line");
            return;
        };

        open.bottom = open.bottom.max(content.bottom());
        let used = Rect::from_min_max(slot.min, pos2(slot.right(), open.bottom)).union(content);
        self.used_rect = Some(self.used_rect.map_or(used, |rect| rect.union(used)));
    }

    /// Stores this pass's specs and requests a discard if they changed since the last pass.
    ///
    /// Returns the rectangle to allocate in the parent, or `None` if no column was drawn.
    pub(crate) fn finish(self, ctx: &egui::Context) -> Option<Rect> {
        debug_assert!(self.open_line.is_none(), "last row line was not closed");

        let layout_changed = self.stored.as_deref().unwrap_or_default() != self.specs.as_slice();
        if self.stored.as_deref() != Some(self.specs.as_slice()) {
            ctx.data_mut(|data| data.insert_temp(self.id, StoredRowSpecs(self.specs)));
        }

        if layout_changed {
            ctx.request_discard("bee-ui row layout changed");
            // The discard is denied once egui runs out of passes; make sure the next frame
            // comes soon so the corrected layout shows up without waiting for input.
            if !ctx.will_discard() {
                ctx.request_repaint();
            }
        }

        self.used_rect
            .map(|used| Rect::from_min_max(self.origin, used.max.max(self.origin)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn col_spec_sanitizes_invalid_values() {
        let props = ColProps::new().xs(Span::N(0)).offset(13).order(99);
        assert_eq!(col_spec(&props, Breakpoint::Xs), GridColSpec::auto());
    }

    #[test]
    fn col_spec_resolves_the_active_breakpoint() {
        let props = ColProps::new().xs(12).md(4).offset(2).order(1);
        assert_eq!(
            col_spec(&props, Breakpoint::Lg),
            GridColSpec::new(Span::N(4)).offset(2).order(1)
        );
        assert_eq!(
            col_spec(&props, Breakpoint::Sm),
            GridColSpec::new(Span::N(12)).offset(2).order(1)
        );
    }
}
