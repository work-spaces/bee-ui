use crate::core::{Span, Variant};

pub type TableAutoSizeMode = egui_table::AutoSizeMode;

#[derive(Clone, Debug)]
pub struct TableRow {
    pub cells: Vec<Span>,
    pub active: bool,
    pub on_click: Option<fn(&egui::Context)>,
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            active: false,
            on_click: None,
        }
    }

    pub fn cell(mut self, cell: impl Into<Span>) -> Self {
        self.cells.push(cell.into());
        self
    }

    pub fn cells<I, S>(mut self, cells: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Span>,
    {
        self.cells.extend(cells.into_iter().map(Into::into));
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn current(self) -> Self {
        self.active(true)
    }

    pub fn on_click(mut self, on_click: fn(&egui::Context)) -> Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }
}

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> From<[&str; N]> for TableRow {
    fn from(value: [&str; N]) -> Self {
        Self::new().cells(value)
    }
}

impl From<Vec<Span>> for TableRow {
    fn from(value: Vec<Span>) -> Self {
        Self::new().cells(value)
    }
}

impl From<Vec<String>> for TableRow {
    fn from(value: Vec<String>) -> Self {
        Self::new().cells(value)
    }
}

#[derive(Clone, Debug)]
pub struct TableProps {
    pub headers: Vec<Span>,
    pub rows: Vec<TableRow>,
    pub variant: Variant,
    pub striped: bool,
    pub hover: bool,
    pub sticky_columns: usize,
    pub auto_size_mode: TableAutoSizeMode,
    pub resizable_columns: bool,
}

impl TableProps {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            rows: Vec::new(),
            variant: Variant::Primary,
            striped: false,
            hover: false,
            sticky_columns: 0,
            auto_size_mode: TableAutoSizeMode::OnParentResize,
            resizable_columns: false,
        }
    }

    pub fn header(mut self, header: impl Into<Span>) -> Self {
        self.headers.push(header.into());
        self
    }

    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Span>,
    {
        self.headers.extend(headers.into_iter().map(Into::into));
        self
    }

    pub fn row(mut self, row: impl Into<TableRow>) -> Self {
        self.rows.push(row.into());
        self
    }

    pub fn push_row(&mut self, row: impl Into<TableRow>) {
        self.rows.push(row.into());
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn striped_rows(self) -> Self {
        self.striped(true)
    }

    pub fn hover(mut self, hover: bool) -> Self {
        self.hover = hover;
        self
    }

    pub fn hoverable(self) -> Self {
        self.hover(true)
    }

    pub fn sticky_columns(mut self, sticky_columns: usize) -> Self {
        self.sticky_columns = sticky_columns;
        self
    }

    pub fn auto_size_mode(mut self, auto_size_mode: TableAutoSizeMode) -> Self {
        self.auto_size_mode = auto_size_mode;
        self
    }

    pub fn resizable_columns(mut self, resizable_columns: bool) -> Self {
        self.resizable_columns = resizable_columns;
        self
    }

    pub fn column_count(&self) -> usize {
        self.rows
            .iter()
            .map(TableRow::len)
            .max()
            .unwrap_or(0)
            .max(self.headers.len())
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.headers.is_empty() && self.rows.is_empty()
    }
}

impl Default for TableProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Table {
    pub headers: Vec<Span>,
    pub rows: Vec<TableRow>,
    pub variant: Variant,
    pub striped: bool,
    pub hover: bool,
    pub sticky_columns: usize,
    pub auto_size_mode: TableAutoSizeMode,
    pub resizable_columns: bool,
}

impl Table {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            rows: Vec::new(),
            variant: Variant::Primary,
            striped: false,
            hover: false,
            sticky_columns: 0,
            auto_size_mode: TableAutoSizeMode::OnParentResize,
            resizable_columns: false,
        }
    }

    pub fn header(mut self, header: impl Into<Span>) -> Self {
        self.headers.push(header.into());
        self
    }

    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Span>,
    {
        self.headers.extend(headers.into_iter().map(Into::into));
        self
    }

    pub fn row(mut self, row: impl Into<TableRow>) -> Self {
        self.rows.push(row.into());
        self
    }

    pub fn push_row(&mut self, row: impl Into<TableRow>) {
        self.rows.push(row.into());
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn striped_rows(self) -> Self {
        self.striped(true)
    }

    pub fn hover(mut self, hover: bool) -> Self {
        self.hover = hover;
        self
    }

    pub fn hoverable(self) -> Self {
        self.hover(true)
    }

    pub fn sticky_columns(mut self, sticky_columns: usize) -> Self {
        self.sticky_columns = sticky_columns;
        self
    }

    pub fn auto_size_mode(mut self, auto_size_mode: TableAutoSizeMode) -> Self {
        self.auto_size_mode = auto_size_mode;
        self
    }

    pub fn resizable_columns(mut self, resizable_columns: bool) -> Self {
        self.resizable_columns = resizable_columns;
        self
    }

    pub fn props(self) -> TableProps {
        self.into()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Table> for TableProps {
    fn from(value: Table) -> Self {
        Self {
            headers: value.headers,
            rows: value.rows,
            variant: value.variant,
            striped: value.striped,
            hover: value.hover,
            sticky_columns: value.sticky_columns,
            auto_size_mode: value.auto_size_mode,
            resizable_columns: value.resizable_columns,
        }
    }
}

impl From<TableProps> for Table {
    fn from(value: TableProps) -> Self {
        Self {
            headers: value.headers,
            rows: value.rows,
            variant: value.variant,
            striped: value.striped,
            hover: value.hover,
            sticky_columns: value.sticky_columns,
            auto_size_mode: value.auto_size_mode,
            resizable_columns: value.resizable_columns,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_row_builder_collects_cells_and_active_state() {
        let row = TableRow::new().cell("scheduler").cell("Healthy").current();

        assert_eq!(
            row.cells,
            vec![Span::from("scheduler"), Span::from("Healthy")]
        );
        assert!(row.active);
    }

    #[test]
    fn table_builder_collects_headers_rows_and_flags() {
        let table = Table::new()
            .headers(["Service", "Status", "P95"])
            .row(
                TableRow::new()
                    .cells(["scheduler", "Healthy", "42 ms"])
                    .current(),
            )
            .row(TableRow::new().cells(["api-gateway", "Degraded", "121 ms"]))
            .variant(Variant::Info)
            .striped_rows()
            .hoverable()
            .sticky_columns(1)
            .auto_size_mode(TableAutoSizeMode::Always)
            .resizable_columns(true);

        assert_eq!(
            table.headers,
            vec![
                Span::from("Service"),
                Span::from("Status"),
                Span::from("P95")
            ]
        );
        assert_eq!(table.rows.len(), 2);
        assert_eq!(
            table.rows[0].cells,
            vec![
                Span::from("scheduler"),
                Span::from("Healthy"),
                Span::from("42 ms")
            ]
        );
        assert!(table.rows[0].active);
        assert_eq!(table.variant, Variant::Info);
        assert!(table.striped);
        assert!(table.hover);
        assert_eq!(table.sticky_columns, 1);
        assert_eq!(table.auto_size_mode, TableAutoSizeMode::Always);
        assert!(table.resizable_columns);
        assert_eq!(table.props().column_count(), 3);
    }

    #[test]
    fn table_row_and_headers_accept_string_inputs() {
        let table = Table::new()
            .headers(vec![String::from("Node"), String::from("State")])
            .row(TableRow::from(vec![
                String::from("api-1"),
                String::from("Ready"),
            ]));

        assert_eq!(table.headers[0], Span::from("Node"));
        assert_eq!(table.headers[1], Span::from("State"));
        assert_eq!(table.rows[0].cells[0], Span::from("api-1"));
        assert_eq!(table.rows[0].cells[1], Span::from("Ready"));
    }
}
