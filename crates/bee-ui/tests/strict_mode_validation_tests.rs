use std::{cell::RefCell, rc::Rc};

use runnerd_bs::{
    core::UiRoot,
    layout::Span,
    prelude::{Col, Row},
};

#[test]
fn strict_mode_is_opt_in() {
    let ctx = egui::Context::default();
    let mut ui = UiRoot::new(&ctx);

    ui.row(Row::new(), |ui| {
        ui.col(Col::new().xs(Span::N(0)).offset(13), |_ui| {});
    });

    assert!(ui.strict_mode_errors().is_empty());
}

#[test]
fn strict_mode_collects_actionable_prop_errors() {
    let ctx = egui::Context::default();
    let mut ui = UiRoot::new(&ctx);
    ui.set_strict_mode(true);

    ui.row(Row::new(), |ui| {
        ui.col(Col::new().xs(Span::N(0)).offset(13).order(99), |_ui| {});
    });

    let messages: Vec<String> = ui
        .strict_mode_errors()
        .iter()
        .map(ToString::to_string)
        .collect();

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

    let ctx = egui::Context::default();
    let mut ui = UiRoot::new(&ctx);
    ui.set_strict_mode(true);

    let sink = Rc::clone(&captured);
    ui.set_strict_mode_error_reporter(move |error| sink.borrow_mut().push(error.to_string()));

    ui.col(Col::new().xs(Span::N(0)), |_ui| {});

    assert_eq!(captured.borrow().len(), 1);
    assert!(captured.borrow()[0].contains("ColProps.xs"));
}
