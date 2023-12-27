use egui::Align2;
use serde::de::Unexpected::Option;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Deserialize, Serialize)]
pub struct MenuBar {
    pub show_cache_preference: bool,
}

impl MenuBar{
    pub fn new() -> Self {
        MenuBar {
            show_cache_preference: false,
        }
    }
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.menu_button("File", |ui| unsafe {
                        if ui.button("Open").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {

                            }                        }
                        if ui.button("Save").clicked() {
                        }
                        if ui.button("Quit").clicked() {
                        }
                    });
                }
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }


}