use crate::core::{Span, Variant};

#[derive(Clone, Debug, PartialEq)]
pub struct CardProps {
    pub variant: Variant,
    pub outlined: bool,
    pub fill_height: bool,
    pub header: Option<Span>,
    pub footer: Option<Span>,
}

impl CardProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(title: impl Into<Span>) -> Self {
        Self::new().title(title)
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn outlined(mut self, outlined: bool) -> Self {
        self.outlined = outlined;
        self
    }

    pub fn outline(self) -> Self {
        self.outlined(true)
    }

    pub fn header(mut self, header: impl Into<Span>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<Span>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub fn title(self, title: impl Into<Span>) -> Self {
        self.header(title)
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    pub fn has_footer(&self) -> bool {
        self.footer.is_some()
    }

    pub fn fill_height(mut self) -> Self {
        self.fill_height = true;
        self
    }
}

impl Default for CardProps {
    fn default() -> Self {
        Self {
            variant: Variant::Primary,
            outlined: false,
            fill_height: false,
            header: None,
            footer: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub variant: Variant,
    pub outlined: bool,
    pub fill_height: bool,
    pub header: Option<Span>,
    pub footer: Option<Span>,
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(title: impl Into<Span>) -> Self {
        Self::new().title(title)
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn outlined(mut self, outlined: bool) -> Self {
        self.outlined = outlined;
        self
    }

    pub fn outline(self) -> Self {
        self.outlined(true)
    }

    pub fn header(mut self, header: impl Into<Span>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<Span>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub fn title(self, title: impl Into<Span>) -> Self {
        self.header(title)
    }

    pub fn fill_height(mut self) -> Self {
        self.fill_height = true;
        self
    }

    pub fn props(self) -> CardProps {
        self.into()
    }
}

impl Default for Card {
    fn default() -> Self {
        Self {
            variant: Variant::Primary,
            outlined: false,
            fill_height: false,
            header: None,
            footer: None,
        }
    }
}

impl From<Card> for CardProps {
    fn from(value: Card) -> Self {
        Self {
            variant: value.variant,
            outlined: value.outlined,
            fill_height: value.fill_height,
            header: value.header,
            footer: value.footer,
        }
    }
}

impl From<CardProps> for Card {
    fn from(value: CardProps) -> Self {
        Self {
            variant: value.variant,
            outlined: value.outlined,
            fill_height: value.fill_height,
            header: value.header,
            footer: value.footer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_builders_populate_header_and_footer() {
        let props = Card::with_title("Status")
            .footer("Updated just now")
            .outline()
            .fill_height()
            .props();

        assert_eq!(props.header, Some(Span::from("Status")));
        assert_eq!(props.footer, Some(Span::from("Updated just now")));
        assert!(props.outlined);
        assert!(props.fill_height);
        assert!(props.has_header());
        assert!(props.has_footer());
    }

    #[test]
    fn card_accepts_string_header_and_footer_inputs() {
        let card = Card::new()
            .header(String::from("Summary"))
            .footer(String::from("Updated"));

        assert_eq!(card.header, Some(Span::from("Summary")));
        assert_eq!(card.footer, Some(Span::from("Updated")));
    }
}
