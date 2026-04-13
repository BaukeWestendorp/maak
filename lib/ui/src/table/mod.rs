use gpui::prelude::*;
use gpui::{App, ElementId, Entity, Window, div, px};

mod column;
mod delegate;
mod state;

pub use column::*;
pub use delegate::*;
pub use state::*;

use crate::ActiveTheme as _;

pub(crate) mod action {
    use gpui::{App, KeyBinding, actions};

    actions!(
        root,
        [
            ClearSelection,
            EditSelection,
            DeleteSelection,
            ToggleExpandSelection,
            NextColumn,
            PrevColumn,
            NextRow,
            PrevRow,
            ExtendSelectionNext,
            ExtendSelectionPrev,
            SelectAll,
        ]
    );

    pub const KEY_CONTEXT: &str = "Table";

    pub fn init(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("escape", ClearSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("enter", EditSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("delete", DeleteSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("backspace", DeleteSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("tab", ToggleExpandSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("right", NextColumn, Some(KEY_CONTEXT)),
            KeyBinding::new("left", PrevColumn, Some(KEY_CONTEXT)),
            KeyBinding::new("down", NextRow, Some(KEY_CONTEXT)),
            KeyBinding::new("up", PrevRow, Some(KEY_CONTEXT)),
            KeyBinding::new("secondary-down", ExtendSelectionNext, Some(KEY_CONTEXT)),
            KeyBinding::new("secondary-up", ExtendSelectionPrev, Some(KEY_CONTEXT)),
            KeyBinding::new("secondary-a", SelectAll, Some(KEY_CONTEXT)),
        ]);
    }
}

#[derive(IntoElement)]
pub struct Table<D: TableDelegate + 'static> {
    id: ElementId,
    state: Entity<TableState<D>>,
}

impl<D: TableDelegate + 'static> Table<D> {
    pub fn new(id: impl Into<ElementId>, state: Entity<TableState<D>>) -> Self {
        Self { id: id.into(), state }
    }

    fn render_header(&self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let cells = self.state.read(cx).delegate().columns(cx).iter().map(|col| {
            div()
                .bg(cx.theme().bg_tertiary)
                .w(col.width())
                .min_w(col.min_width())
                .child(col.label().to_owned())
        });

        div().flex().children(cells)
    }

    fn render_body(&self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .w(px(400.0))
            .child("Row 1")
            .child("Row 2")
            .child("Row 3")
            .child("Row 4")
            .child("Row 5")
            .child("Row 6")
    }
}

impl<D: TableDelegate> RenderOnce for Table<D> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let table = div().child(self.render_header(window, cx)).child(self.render_body(window, cx));

        div().id(self.id.clone()).size_full().overflow_scroll().child(table)
    }
}
