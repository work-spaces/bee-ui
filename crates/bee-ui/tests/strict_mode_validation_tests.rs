use std::{cell::RefCell, rc::Rc};

use bee_ui::{
    core::UiRoot,
    layout::Span,
    prelude::{Col, Row},
    show_root,
};

/// Runs one egui pass and builds `f` into a fresh root, returning the last pass's value.
fn run_frame<R>(mut f: impl FnMut(&mut UiRoot<'_>) -> R) -> R {
    let ctx = egui::Context::default();
    let mut result = None;
    let _ = ctx.run(egui::RawInput::default(), |ctx| {
        result = Some(show_root(ctx, &mut f));
    });
    result.expect("egui pass should run the build closure")
}

#[test]
fn strict_mode_is_opt_in() {
    let errors = run_frame(|ui| {
        ui.row(Row::new(), |ui| {
            ui.col(Col::new().xs(Span::N(0)).offset(13), |_ui| {});
        });
        ui.take_strict_mode_errors()
    });

    assert!(errors.is_empty());
}

#[test]
fn strict_mode_collects_actionable_prop_errors() {
    let messages: Vec<String> = run_frame(|ui| {
        ui.set_strict_mode(true);

        ui.row(Row::new(), |ui| {
            ui.col(Col::new().xs(Span::N(0)).offset(13).order(99), |_ui| {});
        });

        ui.strict_mode_errors()
            .iter()
            .map(ToString::to_string)
            .collect()
    });

    assert_eq!(messages.len(), 3);
    assert!(
        messages
            .iter()
            .any(|message| message.contains("ColProps.xs") && message.contains("value `0`"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("ColProps.offset") && message.contains("value `13`"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("ColProps.order") && message.contains("value `99`"))
    );
}

#[test]
fn strict_mode_optional_reporter_receives_errors() {
    let captured = Rc::new(RefCell::new(Vec::<String>::new()));

    run_frame(|ui| {
        captured.borrow_mut().clear();
        ui.set_strict_mode(true);

        let sink = Rc::clone(&captured);
        ui.set_strict_mode_error_reporter(move |error| sink.borrow_mut().push(error.to_string()));

        ui.col(Col::new().xs(Span::N(0)), |_ui| {});
    });

    assert_eq!(captured.borrow().len(), 1);
    assert!(captured.borrow()[0].contains("ColProps.xs"));
}
