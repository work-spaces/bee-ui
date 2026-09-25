#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spacing {
    pub x: f32,
    pub y: f32,
}

impl Spacing {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn all(value: f32) -> Self {
        Self { x: value, y: value }
    }

    pub fn normalized(self) -> Self {
        Self {
            x: normalize_non_negative(self.x),
            y: normalize_non_negative(self.y),
        }
    }

    /// Maps `Spacing` to symmetric margins (x => left/right, y => top/bottom).
    pub fn as_margin(self) -> EdgeInsets {
        let normalized = self.normalized();
        EdgeInsets::symmetric(normalized.x, normalized.y)
    }

    /// Maps `Spacing` to symmetric padding (x => left/right, y => top/bottom).
    pub fn as_padding(self) -> EdgeInsets {
        let normalized = self.normalized();
        EdgeInsets::symmetric(normalized.x, normalized.y)
    }

    pub fn gutter_x(self) -> f32 {
        normalize_non_negative(self.x)
    }

    pub fn gutter_y(self) -> f32 {
        normalize_non_negative(self.y)
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeInsets {
    pub const fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub const fn all(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub const fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    pub fn normalized(self) -> Self {
        Self {
            top: normalize_non_negative(self.top),
            right: normalize_non_negative(self.right),
            bottom: normalize_non_negative(self.bottom),
            left: normalize_non_negative(self.left),
        }
    }
}

impl Default for EdgeInsets {
    fn default() -> Self {
        Self::all(0.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SemanticSpacing {
    pub gutter: Spacing,
    pub margin: EdgeInsets,
    pub padding: EdgeInsets,
}

impl SemanticSpacing {
    pub const fn new(gutter: Spacing, margin: EdgeInsets, padding: EdgeInsets) -> Self {
        Self {
            gutter,
            margin,
            padding,
        }
    }

    pub fn from_row_and_box(gutter: Spacing, margin: Spacing, padding: Spacing) -> Self {
        Self {
            gutter,
            margin: margin.as_margin(),
            padding: padding.as_padding(),
        }
    }

    pub fn normalized(self) -> Self {
        Self {
            gutter: self.gutter.normalized(),
            margin: self.margin.normalized(),
            padding: self.padding.normalized(),
        }
    }
}

fn normalize_non_negative(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}
