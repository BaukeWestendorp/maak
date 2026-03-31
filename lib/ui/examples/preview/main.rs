mod binding;
mod button;
mod grid;
mod icon;
mod org;
mod settings;
mod table;
mod tabs;
mod theme;
mod tiles;
mod title_bar;
mod typo;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Debug).init();

    app::run()?;

    Ok(())
}

mod app {
    use gpui::prelude::*;
    use gpui::{Entity, Window, div};
    use maak_ui::{ConfigAppExt as _, Tab, Tabs, TabsState, TabsVariant};

    use crate::binding::BindingPreview;
    use crate::button::ButtonPreview;
    use crate::grid::GridPreview;
    use crate::icon::IconPreview;
    use crate::org::OrgPreview;
    use crate::settings::SettingsPreview;
    use crate::table::TablePreview;
    use crate::tabs::TabsPreview;
    use crate::theme::ThemePreview;
    use crate::tiles::TilesPreview;
    use crate::title_bar::TitleBarPreview;
    use crate::typo::TypoPreview;

    pub fn run() -> anyhow::Result<()> {
        maak_ui::build_simple_app()
            .window_title("MaakUI Preview")
            .config(
                maak_ui::config::Config::builder()
                    .add_source(maak_ui::config::File::from(
                        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                            .join("examples")
                            .join("preview")
                            .join("config.toml"),
                    ))
                    .build()?,
            )
            .run(|window, cx| cx.new(|cx| PreviewApp::new(window, cx)));

        Ok(())
    }

    struct PreviewApp {
        tabs: Entity<TabsState>,

        tab_binding: Entity<BindingPreview>,
        tab_button: Entity<ButtonPreview>,
        tab_grid: Entity<GridPreview>,
        tab_icon: Entity<IconPreview>,
        tab_org: Entity<OrgPreview>,
        tab_settings: Entity<SettingsPreview>,
        tab_tabs: Entity<TabsPreview>,
        tab_table: Entity<TablePreview>,
        tab_theme: Entity<ThemePreview>,
        tab_tiles: Entity<TilesPreview>,
        tab_title_bar: Entity<TitleBarPreview>,
        tab_typo: Entity<TypoPreview>,
    }

    impl PreviewApp {
        fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
            Self {
                tabs: cx.new(|cx| {
                    let selected =
                        cx.config().get_string("current_tab").unwrap_or("button".to_string());
                    TabsState::new().with_selected(selected)
                }),

                tab_binding: cx.new(|cx| BindingPreview::new(window, cx)),
                tab_button: cx.new(|cx| ButtonPreview::new(window, cx)),
                tab_grid: cx.new(|cx| GridPreview::new(window, cx)),
                tab_icon: cx.new(|cx| IconPreview::new(window, cx)),
                tab_org: cx.new(|cx| OrgPreview::new(window, cx)),
                tab_settings: cx.new(|cx| SettingsPreview::new(window, cx)),
                tab_tabs: cx.new(|cx| TabsPreview::new(window, cx)),
                tab_table: cx.new(|cx| TablePreview::new(window, cx)),
                tab_theme: cx.new(|cx| ThemePreview::new(window, cx)),
                tab_tiles: cx.new(|cx| TilesPreview::new(window, cx)),
                tab_title_bar: cx.new(|cx| TitleBarPreview::new(window, cx)),
                tab_typo: cx.new(|cx| TypoPreview::new(window, cx)),
            }
        }
    }

    impl Render for PreviewApp {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                Tabs::new("preview-pages", self.tabs.clone(), TabsVariant::Sidebar).tabs([
                    Tab::new("binding", "Bindings", self.tab_binding.clone().into_any_element()),
                    Tab::new("button", "Buttons", self.tab_button.clone().into_any_element()),
                    Tab::new("grid", "Grid", self.tab_grid.clone().into_any_element()),
                    Tab::new("icon", "Icon", self.tab_icon.clone().into_any_element()),
                    Tab::new("org", "Organization", self.tab_org.clone().into_any_element()),
                    Tab::new("settings", "Settings", self.tab_settings.clone().into_any_element()),
                    Tab::new("table", "Table", self.tab_table.clone().into_any_element()),
                    Tab::new("tabs", "Tabs", self.tab_tabs.clone().into_any_element()),
                    Tab::new("tiles", "Tiles", self.tab_tiles.clone().into_any_element()),
                    Tab::new("theme", "Themes", self.tab_theme.clone().into_any_element()),
                    Tab::new(
                        "title_bar",
                        "Title Bar",
                        self.tab_title_bar.clone().into_any_element(),
                    ),
                    Tab::new("typo", "Typography", self.tab_typo.clone().into_any_element()),
                ]),
            )
        }
    }
}

pub fn alpha_content() -> gpui::Div {
    use gpui::{ParentElement as _, Styled as _};

    gpui::div()
        .size_full()
        .p_2()
        .flex()
        .justify_center()
        .items_center()
        .border_1()
        .border_color(gpui::red())
        .bg(gpui::red().opacity(0.2))
        .child("Alpha")
        .font_weight(gpui::FontWeight::BOLD)
}

pub fn beta_content() -> gpui::Div {
    use gpui::{ParentElement as _, Styled as _};

    gpui::div()
        .size_full()
        .p_2()
        .flex()
        .justify_center()
        .items_center()
        .border_1()
        .border_color(gpui::green())
        .bg(gpui::green().opacity(0.2))
        .child("Beta")
        .font_weight(gpui::FontWeight::BOLD)
}

pub fn gamma_content() -> gpui::Div {
    use gpui::{ParentElement as _, Styled as _};

    gpui::div()
        .size_full()
        .p_2()
        .flex()
        .justify_center()
        .items_center()
        .border_1()
        .border_color(gpui::blue())
        .bg(gpui::blue().opacity(0.2))
        .child("Gamma")
        .font_weight(gpui::FontWeight::BOLD)
}
