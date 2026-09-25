use runnerd_bs::layout::{
    GridColSpec, GridLayoutError, GridSpec, Span, compute_grid_lines, pack_grid_lines,
};

fn approx_eq(left: f32, right: f32) {
    let diff = (left - right).abs();
    assert!(
        diff <= 1e-4,
        "expected {left} to be within 1e-4 of {right} (diff={diff})"
    );
}

#[test]
fn grid_packing_wraps_when_budget_would_overflow() {
    let cols = vec![
        GridColSpec::new(Span::N(4)),
        GridColSpec::new(Span::N(4)).offset(4),
        GridColSpec::new(Span::N(2)),
    ];

    let lines = pack_grid_lines(&cols, false).expect("packing should succeed");

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].used_columns, 12);
    assert_eq!(lines[1].used_columns, 2);

    let line0_indices: Vec<usize> = lines[0].cols.iter().map(|col| col.source_index).collect();
    let line1_indices: Vec<usize> = lines[1].cols.iter().map(|col| col.source_index).collect();

    assert_eq!(line0_indices, vec![0, 1]);
    assert_eq!(line1_indices, vec![2]);
}

#[test]
fn grid_packing_can_sort_by_order_before_packing() {
    let cols = vec![
        GridColSpec::auto().order(2),
        GridColSpec::new(Span::N(6)),
        GridColSpec::new(Span::N(4)).order(-1),
        GridColSpec::new(Span::N(2)).order(2),
    ];

    let lines = pack_grid_lines(&cols, true).expect("packing should succeed");
    assert_eq!(lines.len(), 1);

    let sorted_indices: Vec<usize> = lines[0].cols.iter().map(|col| col.source_index).collect();
    assert_eq!(sorted_indices, vec![2, 1, 0, 3]);
    assert_eq!(lines[0].used_columns, 12);
}

#[test]
fn grid_width_math_handles_mixed_fixed_and_auto() {
    let cols = vec![
        GridColSpec::new(Span::N(6)),
        GridColSpec::auto(),
        GridColSpec::auto(),
    ];

    let lines = compute_grid_lines(&cols, 1200.0, 24.0, false).expect("layout should succeed");
    assert_eq!(lines.len(), 1);

    let line = &lines[0];
    assert_eq!(line.cols.len(), 3);
    approx_eq(line.usable_width, 1152.0);

    approx_eq(line.cols[0].x, 0.0);
    approx_eq(line.cols[0].width, 576.0);

    approx_eq(line.cols[1].x, 600.0);
    approx_eq(line.cols[1].width, 288.0);

    approx_eq(line.cols[2].x, 912.0);
    approx_eq(line.cols[2].width, 288.0);
}

#[test]
fn grid_width_math_handles_all_auto_columns() {
    let cols = vec![
        GridColSpec::auto(),
        GridColSpec::auto(),
        GridColSpec::auto(),
    ];

    let lines = compute_grid_lines(&cols, 900.0, 30.0, false).expect("layout should succeed");
    assert_eq!(lines.len(), 1);

    let line = &lines[0];
    assert_eq!(line.used_columns, 0);
    approx_eq(line.usable_width, 840.0);

    approx_eq(line.cols[0].x, 0.0);
    approx_eq(line.cols[0].width, 280.0);

    approx_eq(line.cols[1].x, 310.0);
    approx_eq(line.cols[1].width, 280.0);

    approx_eq(line.cols[2].x, 620.0);
    approx_eq(line.cols[2].width, 280.0);
}

#[test]
fn grid_width_math_accounts_for_offsets() {
    let cols = vec![GridColSpec::new(Span::N(3)).offset(3), GridColSpec::auto()];

    let lines = compute_grid_lines(&cols, 1200.0, 0.0, false).expect("layout should succeed");
    assert_eq!(lines.len(), 1);

    let line = &lines[0];
    assert_eq!(line.used_columns, 6);
    approx_eq(line.usable_width, 1200.0);

    approx_eq(line.cols[0].x, 300.0);
    approx_eq(line.cols[0].width, 300.0);

    approx_eq(line.cols[1].x, 600.0);
    approx_eq(line.cols[1].width, 600.0);
}

#[test]
fn grid_rejects_invalid_or_unplaceable_column_specs() {
    let invalid_offset = vec![GridColSpec::auto().offset(13)];
    assert_eq!(
        pack_grid_lines(&invalid_offset, false),
        Err(GridLayoutError::InvalidOffset {
            index: 0,
            offset: 13,
            columns: 12,
        })
    );

    let overflow = vec![GridColSpec::new(Span::N(12)).offset(1)];
    assert_eq!(
        pack_grid_lines(&overflow, false),
        Err(GridLayoutError::BudgetOverflow {
            index: 0,
            required: 13,
            columns: 12,
        })
    );
}

#[test]
fn grid_packing_exact_budget_stays_on_single_line() {
    let cols = vec![
        GridColSpec::new(Span::N(4)).offset(2),
        GridColSpec::new(Span::N(6)),
    ];

    let lines = pack_grid_lines(&cols, false).expect("packing should succeed");
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].used_columns, 12);
}

#[test]
fn grid_packing_order_ties_keep_source_order_stable() {
    let cols = vec![
        GridColSpec::auto().order(1),
        GridColSpec::new(Span::N(4)).order(1),
        GridColSpec::new(Span::N(4)).order(1),
        GridColSpec::new(Span::N(4)).order(0),
    ];

    let lines = pack_grid_lines(&cols, true).expect("packing should succeed");
    assert_eq!(lines.len(), 1);

    let sorted_indices: Vec<usize> = lines[0].cols.iter().map(|col| col.source_index).collect();
    assert_eq!(sorted_indices, vec![3, 0, 1, 2]);
}

#[test]
fn grid_spec_with_custom_column_count_wraps_consistently() {
    let spec = GridSpec { columns: 4 };
    let cols = vec![
        GridColSpec::new(Span::N(2)),
        GridColSpec::new(Span::N(3)),
        GridColSpec::auto(),
    ];

    let lines = spec
        .pack_lines(&cols, false)
        .expect("packing should succeed");

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].used_columns, 2);
    assert_eq!(lines[1].used_columns, 3);

    let line0_indices: Vec<usize> = lines[0].cols.iter().map(|col| col.source_index).collect();
    let line1_indices: Vec<usize> = lines[1].cols.iter().map(|col| col.source_index).collect();

    assert_eq!(line0_indices, vec![0]);
    assert_eq!(line1_indices, vec![1, 2]);
}

#[test]
fn grid_packing_and_math_handle_empty_inputs() {
    let packed = pack_grid_lines(&[], false).expect("packing should succeed");
    assert!(packed.is_empty());

    let computed =
        compute_grid_lines(&[], 800.0, 24.0, false).expect("empty layout should succeed");
    assert!(computed.is_empty());
}

#[test]
fn grid_width_math_normalizes_non_finite_and_negative_inputs() {
    let cols = vec![GridColSpec::new(Span::N(6)), GridColSpec::auto()];

    let lines =
        compute_grid_lines(&cols, f32::NAN, -8.0, false).expect("layout should normalize inputs");
    assert_eq!(lines.len(), 1);

    let line = &lines[0];
    approx_eq(line.usable_width, 0.0);
    approx_eq(line.cols[0].width, 0.0);
    approx_eq(line.cols[1].width, 0.0);
}

#[test]
fn grid_width_math_with_large_gutter_never_produces_negative_widths() {
    let cols = vec![GridColSpec::auto(), GridColSpec::auto()];

    let lines = compute_grid_lines(&cols, 100.0, 120.0, false).expect("layout should succeed");
    assert_eq!(lines.len(), 1);

    let line = &lines[0];
    approx_eq(line.usable_width, 0.0);
    assert!(line.cols.iter().all(|col| col.width >= 0.0));
}
