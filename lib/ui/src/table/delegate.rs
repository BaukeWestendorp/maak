use std::hash::Hash;

use gpui::prelude::*;
use gpui::{App, Window};

use crate::{Column, TableState};

pub trait TableDelegate {
    type RowId: Clone + Eq + Hash;

    fn columns(&self, cx: &App) -> &[Column]
    where
        Self: Sized;

    fn root_row_ids(&self, cx: &App) -> Vec<Self::RowId>
    where
        Self: Sized;

    fn row_children(&self, _row_id: &Self::RowId, _cx: &App) -> Vec<Self::RowId>
    where
        Self: Sized,
    {
        Vec::new()
    }

    fn edit_rows(&self, _row_ids: &[Self::RowId], _cx: &mut App)
    where
        Self: Sized,
    {
    }

    fn delete_rows(&self, _row_ids: &[Self::RowId], _cx: &mut App)
    where
        Self: Sized,
    {
    }

    fn render_cell(
        &self,
        cell: Cell<Self>,
        window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;
}

pub struct Cell<'a, D: TableDelegate> {
    pub(crate) row_id: &'a D::RowId,
    pub(crate) column: &'a Column,
}

impl<'a, D: TableDelegate> Cell<'a, D> {
    pub fn row_id(&self) -> &'a D::RowId {
        self.row_id
    }

    pub fn column(&self) -> &'a Column {
        self.column
    }
}
