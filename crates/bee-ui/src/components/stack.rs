use crate::theme::bootstrap_tokens;

/// Lays children out left-to-right at their natural (content) width, vertically centered.
///
/// Unlike `Row`, an `HStack` does not divide the available width into grid columns, which
/// makes it the right fit for toolbars and inline control groups (Bootstrap's `.hstack`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HStack {
    pub gap: f32,
    pub wrap: bool,
}

impl HStack {
    pub fn new() -> Self {
        Self {
            gap: bootstrap_tokens::spacing().sm,
            wrap: false,
        }
    }

    /// Horizontal space between adjacent children.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Wrap children onto additional lines when they overflow the available width.
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn resolved_gap(self) -> f32 {
        self.gap.max(0.0)
    }
}

impl Default for HStack {
    fn default() -> Self {
        Self::new()
    }
}
