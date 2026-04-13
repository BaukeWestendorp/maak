use std::path::PathBuf;

use maak_ui::gpui::prelude::*;
use maak_ui::gpui::{App, Context, Entity, Window, div, px};
use maak_ui::{Column, Table, TableDelegate, TableState};

pub struct LocalView {
    table: Entity<TableState<LocalTable>>,
}

impl LocalView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let selection = cx.new(|_| Vec::new());

        Self { table: cx.new(|cx| TableState::new(LocalTable::new(), selection, window, cx)) }
    }
}

impl Render for LocalView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(Table::new(self.table.clone()))
    }
}

struct LocalTable {
    columns: Vec<Column>,
}

impl LocalTable {
    fn new() -> Self {
        Self { columns: vec![Column::new("path", "Path").with_min_width(px(100.0))] }
    }
}

impl TableDelegate for LocalTable {
    type RowId = PathBuf;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn root_row_ids(&self, _cx: &App) -> Vec<Self::RowId> {
        vec![PathBuf::from("/abc/def")]
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let path = row_id.as_path();

        match self.column(col_ix, cx).id().as_str() {
            "path" => path.display().to_string(),
            _ => "".to_string(),
        }
    }
}
