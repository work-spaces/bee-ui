fn main() {
    let css = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../bootstrap/dist/css/bootstrap.css"
    ));

    let report = bee_ui::theme::bootstrap_inventory::render_phase1_bootstrap_inventory_report(css);
    println!("{report}");
}
