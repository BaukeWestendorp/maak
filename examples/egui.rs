#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release.

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Maak egui",
        options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    )
}

struct Application {
    maak: maak::app::App,
}

impl Application {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let mut maak = maak::app::App::new();
        maak.add_plugin(maak::feat::egui::EguiPlugin::new(cc));
        Self { maak }
    }
}

impl eframe::App for Application {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        self.maak.update_schedule.run(&mut self.maak.world);

        eframe::egui::CentralPanel::default().show(ui, |ui| {
            if let Some(target) = self
                .maak
                .world
                .get_resource::<maak::feat::egui::EguiTextureTarget>()
            {
                ui.image(eframe::egui::load::SizedTexture::new(
                    target.egui_tex_id,
                    [1080.0, 720.0],
                ));
            }
        });

        ui.request_repaint();
    }
}
