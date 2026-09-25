use crate::{
    components::CardProps,
    core::{Span, Variant},
};

use super::PlotArea;

#[derive(Clone, Debug, PartialEq)]
pub struct ChartCard {
    pub card: CardProps,
    pub plot: PlotArea,
}

impl ChartCard {
    pub fn new(title: impl Into<Span>, plot_id: impl Into<String>) -> Self {
        Self {
            card: CardProps::with_title(title),
            plot: PlotArea::new(plot_id),
        }
    }

    pub fn card(mut self, card: impl Into<CardProps>) -> Self {
        self.card = card.into();
        self
    }

    pub fn plot(mut self, plot: impl Into<PlotArea>) -> Self {
        self.plot = plot.into();
        self
    }

    pub fn title(mut self, title: impl Into<Span>) -> Self {
        self.card = self.card.title(title);
        self
    }

    pub fn footer(mut self, footer: impl Into<Span>) -> Self {
        self.card = self.card.footer(footer);
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.card = self.card.variant(variant);
        self
    }

    pub fn outlined(mut self, outlined: bool) -> Self {
        self.card = self.card.outlined(outlined);
        self
    }

    pub fn outline(self) -> Self {
        self.outlined(true)
    }

    pub fn plot_height(mut self, height: f32) -> Self {
        self.plot = self.plot.height(height);
        self
    }
}
