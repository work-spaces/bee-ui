use crate::layout::Spacing;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContainerKind {
    #[default]
    Fixed,
    Fluid,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContainerScroll {
    #[default]
    None,
    Horizontal,
    Vertical,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContainerProps {
    pub fluid: bool,
    pub max_width: Option<f32>,
    pub padding: Spacing,
    pub scroll: ContainerScroll,
    pub fill_height: bool,
}

impl ContainerProps {
    pub const fn fixed() -> Self {
        Self {
            fluid: false,
            max_width: None,
            padding: Spacing::new(0.0, 0.0),
            scroll: ContainerScroll::None,
            fill_height: false,
        }
    }

    pub const fn fluid() -> Self {
        Self {
            fluid: true,
            max_width: None,
            padding: Spacing::new(0.0, 0.0),
            scroll: ContainerScroll::None,
            fill_height: false,
        }
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn padding(mut self, padding: Spacing) -> Self {
        self.padding = padding;
        self
    }

    pub fn fill_height(mut self) -> Self {
        self.fill_height = true;
        self
    }

    pub fn scroll(mut self, scroll: ContainerScroll) -> Self {
        self.scroll = scroll;
        self
    }

    pub fn scroll_x(self) -> Self {
        self.scroll(ContainerScroll::Horizontal)
    }

    pub fn scroll_y(self) -> Self {
        self.scroll(ContainerScroll::Vertical)
    }

    pub fn scroll_both(self) -> Self {
        self.scroll(ContainerScroll::Both)
    }
}

impl Default for ContainerProps {
    fn default() -> Self {
        Self::fixed()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Container {
    pub kind: ContainerKind,
}

impl Container {
    pub const fn new() -> Self {
        Self::fixed()
    }

    pub const fn fixed() -> Self {
        Self {
            kind: ContainerKind::Fixed,
        }
    }

    pub const fn fluid() -> Self {
        Self {
            kind: ContainerKind::Fluid,
        }
    }

    pub fn props(self) -> ContainerProps {
        self.into()
    }
}

impl From<Container> for ContainerProps {
    fn from(value: Container) -> Self {
        match value.kind {
            ContainerKind::Fixed => Self::fixed(),
            ContainerKind::Fluid => Self::fluid(),
        }
    }
}
