use crate::layout::Spacing;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowProps {
    pub gutter_x: f32,
    pub gutter_y: f32,
}

impl RowProps {
    pub const fn new() -> Self {
        Self {
            gutter_x: 0.0,
            gutter_y: 0.0,
        }
    }

    pub fn gutter(mut self, gutter: Spacing) -> Self {
        self.gutter_x = gutter.x;
        self.gutter_y = gutter.y;
        self
    }

    pub fn gutter_x(mut self, gutter_x: f32) -> Self {
        self.gutter_x = gutter_x;
        self
    }

    pub fn gutter_y(mut self, gutter_y: f32) -> Self {
        self.gutter_y = gutter_y;
        self
    }

    pub const fn spacing(self) -> Spacing {
        Spacing::new(self.gutter_x, self.gutter_y)
    }
}

impl Default for RowProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Row {
    pub gutter: Spacing,
}

impl Row {
    pub const fn new() -> Self {
        Self {
            gutter: Spacing::new(0.0, 0.0),
        }
    }

    pub fn gutter(mut self, gutter: Spacing) -> Self {
        self.gutter = gutter;
        self
    }

    pub fn gutter_x(mut self, gutter_x: f32) -> Self {
        self.gutter.x = gutter_x;
        self
    }

    pub fn gutter_y(mut self, gutter_y: f32) -> Self {
        self.gutter.y = gutter_y;
        self
    }

    pub fn props(self) -> RowProps {
        self.into()
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Row> for RowProps {
    fn from(value: Row) -> Self {
        Self {
            gutter_x: value.gutter.x,
            gutter_y: value.gutter.y,
        }
    }
}
