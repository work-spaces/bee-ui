use std::hash::Hash;

/// Open/closed binding used by stateful semantic widgets (dropdowns, modals, toasts, etc.).
///
/// Prefer the controlled variant (`DisclosureState::controlled` or passing `&mut bool`) so the
/// caller owns visibility state explicitly. The uncontrolled variant stores state in `egui`
/// context memory behind an explicit ID.
#[derive(Debug)]
pub enum DisclosureState<'a> {
    Controlled(&'a mut bool),
    Uncontrolled { id: egui::Id, default_open: bool },
}

impl<'a> DisclosureState<'a> {
    pub fn controlled(open: &'a mut bool) -> Self {
        Self::Controlled(open)
    }

    pub fn uncontrolled(id_source: impl Hash, default_open: bool) -> Self {
        Self::Uncontrolled {
            id: egui::Id::new(id_source),
            default_open,
        }
    }

    pub const fn is_controlled(&self) -> bool {
        matches!(self, Self::Controlled(_))
    }

    pub fn current(&self, ctx: &egui::Context) -> bool {
        match self {
            Self::Controlled(open) => **open,
            Self::Uncontrolled { id, default_open } => ctx
                .data_mut(|data| data.get_persisted::<bool>(*id))
                .unwrap_or(*default_open),
        }
    }

    pub fn set_current(&mut self, ctx: &egui::Context, open: bool) {
        match self {
            Self::Controlled(value) => {
                **value = open;
            }
            Self::Uncontrolled { id, .. } => {
                ctx.data_mut(|data| data.insert_persisted(*id, open));
            }
        }
    }

    pub fn toggle(&mut self, ctx: &egui::Context) {
        let next = !self.current(ctx);
        self.set_current(ctx, next);
    }
}

impl<'a> From<&'a mut bool> for DisclosureState<'a> {
    fn from(value: &'a mut bool) -> Self {
        Self::controlled(value)
    }
}

/// Active-item index binding used by stateful semantic widgets (tabs/nav, accordion, carousel).
///
/// Prefer the controlled variant (`ActiveIndexState::controlled` or passing `&mut usize`) so the
/// caller owns selection state explicitly. The uncontrolled variant stores state in `egui`
/// context memory behind an explicit ID.
#[derive(Debug)]
pub enum ActiveIndexState<'a> {
    Controlled(&'a mut usize),
    Uncontrolled { id: egui::Id, default_index: usize },
}

impl<'a> ActiveIndexState<'a> {
    pub fn controlled(active_index: &'a mut usize) -> Self {
        Self::Controlled(active_index)
    }

    pub fn uncontrolled(id_source: impl Hash, default_index: usize) -> Self {
        Self::Uncontrolled {
            id: egui::Id::new(id_source),
            default_index,
        }
    }

    pub const fn is_controlled(&self) -> bool {
        matches!(self, Self::Controlled(_))
    }

    pub fn current(&self, ctx: &egui::Context) -> usize {
        match self {
            Self::Controlled(active_index) => **active_index,
            Self::Uncontrolled { id, default_index } => ctx
                .data_mut(|data| data.get_persisted::<usize>(*id))
                .unwrap_or(*default_index),
        }
    }

    pub fn set_current(&mut self, ctx: &egui::Context, active_index: usize) {
        match self {
            Self::Controlled(value) => {
                **value = active_index;
            }
            Self::Uncontrolled { id, .. } => {
                ctx.data_mut(|data| data.insert_persisted(*id, active_index));
            }
        }
    }
}

impl<'a> From<&'a mut usize> for ActiveIndexState<'a> {
    fn from(value: &'a mut usize) -> Self {
        Self::controlled(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controlled_disclosure_updates_referenced_flag() {
        let ctx = egui::Context::default();
        let mut open = false;

        {
            let mut state = DisclosureState::controlled(&mut open);
            assert!(state.is_controlled());
            assert!(!state.current(&ctx));

            state.toggle(&ctx);
            assert!(state.current(&ctx));
        }

        assert!(open);
    }

    #[test]
    fn uncontrolled_disclosure_persists_state_by_id() {
        let ctx = egui::Context::default();
        let mut state = DisclosureState::uncontrolled("runnerd-bs.test.disclosure", false);

        assert!(!state.current(&ctx));
        state.set_current(&ctx, true);
        assert!(state.current(&ctx));
    }

    #[test]
    fn controlled_active_index_updates_referenced_value() {
        let ctx = egui::Context::default();
        let mut active_index = 1;

        {
            let mut state = ActiveIndexState::controlled(&mut active_index);
            assert!(state.is_controlled());
            assert_eq!(state.current(&ctx), 1);

            state.set_current(&ctx, 3);
            assert_eq!(state.current(&ctx), 3);
        }

        assert_eq!(active_index, 3);
    }

    #[test]
    fn uncontrolled_active_index_persists_state_by_id() {
        let ctx = egui::Context::default();
        let mut state = ActiveIndexState::uncontrolled("runnerd-bs.test.active-index", 2);

        assert_eq!(state.current(&ctx), 2);
        state.set_current(&ctx, 5);
        assert_eq!(state.current(&ctx), 5);
    }
}
