use crate::layout::Spacing;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowProps {
    pub gutter_x: f32,
    pub gutter_y: f32,
    /// Explicit ID source for the row's cached layout. See [`Row::id_salt`].
    pub id_salt: Option<egui::Id>,
}

impl RowProps {
    pub const fn new() -> Self {
        Self {
            gutter_x: 0.0,
            gutter_y: 0.0,
            id_salt: None,
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

    /// See [`Row::id_salt`].
    pub fn id_salt(mut self, id_salt: impl std::hash::Hash) -> Self {
        self.id_salt = Some(egui::Id::new(id_salt));
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
    pub id_salt: Option<egui::Id>,
}

impl Row {
    pub const fn new() -> Self {
        Self {
            gutter: Spacing::new(0.0, 0.0),
            id_salt: None,
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

    /// Gives the row a stable ID for its cached column layout.
    ///
    /// By default a row's ID comes from its position among its siblings, like egui's automatic
    /// widget IDs. Rows whose position changes between frames (for example, rows inside a
    /// conditionally shown section) re-measure their columns, costing an extra pass. Set an
    /// `id_salt` that is unique among the row's siblings to avoid that.
    pub fn id_salt(mut self, id_salt: impl std::hash::Hash) -> Self {
        self.id_salt = Some(egui::Id::new(id_salt));
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
            id_salt: value.id_salt,
        }
    }
}
