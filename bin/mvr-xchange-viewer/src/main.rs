use maak_ui::gpui::prelude::*;
use maak_ui::gpui::{Entity, Window, div};
use maak_ui::{Tab, Tabs, TabsState, TabsVariant};

use crate::local::LocalView;

mod local;

fn main() {
    maak_ui::build_simple_app()
        .window_title("MVR-xchange Viewer")
        .run(|window, cx| cx.new(|cx| MainView::new(window, cx)));
}

struct MainView {
    tabs: Entity<TabsState>,

    local_view: Entity<LocalView>,
}

impl MainView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            tabs: cx.new(|_| TabsState::new().with_selected("local")),
            local_view: cx.new(|cx| LocalView::new(window, cx)),
        }
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let main_tabs = Tabs::new("main-tabs", self.tabs.clone(), TabsVariant::Sidebar).tabs([
            Tab::new("local", "Local", self.local_view.clone().into_any_element()),
            Tab::new("service", "Service", maak_ui::todo(cx).into_any_element()),
            Tab::new("files", "Files", maak_ui::todo(cx).into_any_element()),
        ]);

        div().size_full().child(main_tabs)
    }
}
