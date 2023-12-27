use egui::Align2;
use serde::de::Unexpected::Option;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
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

        egui::Window::new("Cache preference")
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(true)
            .open(&mut self.show_cache_preference)
            .resizable(true)
            .movable(true)
            .default_width(600.)
            .show(ctx, |ui| {

                egui::Grid::new("settings").num_columns(2).show(ui, |ui| {

                    ui
                        .checkbox(&mut true, "Enable vsync")
                        .on_hover_text(
                            "Vsync reduces tearing and saves CPU. Toggling it off will make some operations such as panning/zooming more snappy. This needs a restart to take effect.",
                        );
                    ui
                        .checkbox(&mut true, "Show index slider")
                        .on_hover_text(
                            "Enable an index slider to quickly scrub through lots of images",
                        );
                    ui.end_row();

                    if ui
                        .checkbox(&mut true, "Wrap images at folder boundary")
                        .on_hover_text(
                            "When you move past the first or last image in a folder, should oculante continue or stop?",
                        )
                        .changed()
                    {
                    }
                    ui.horizontal(|ui| {
                        ui.label("Number of image to cache");
                        if ui
                            .add(egui::DragValue::new(&mut 0).clamp_range(0..=10000))

                            .on_hover_text(
                                "Keep this many images in memory for faster opening.",
                            )
                            .changed()
                        {

                        }
                    });

                    ui.end_row();
                    ui
                        .checkbox(&mut true, "Do not reset image view")
                        .on_hover_text(
                            "When a new image is loaded, keep current zoom and offset",
                        );

                    ui
                        .checkbox(&mut true, "Keep image edits")
                        .on_hover_text(
                            "When a new image is loaded, keep current edits",
                        );
                    ui.end_row();
                    ui
                        .checkbox(&mut true, "Show checker background")
                        .on_hover_text(
                            "Show a checker pattern as backdrop.",
                        );

                    ui
                        .checkbox(&mut true, "Draw frame around image")
                        .on_hover_text(
                            "Draw a small frame around the image. It is centered on the outmost pixel. This can be helpful on images with lots of transparency.",
                        );
                    ui.end_row();

                }


                );

                ui.horizontal(|ui| {
                    ui.label("Configure window title");
                    if ui
                        .text_edit_singleline(&mut "")
                        .on_hover_text(
                            "Configure the title. Use {APP}, {VERSION}, {FULLPATH}, {FILENAME} and {RES} as placeholders.",
                        )
                        .changed()
                    {

                    }
                });


                ui.vertical_centered_justified(|ui| {
                    if ui.button("Reset all settings").clicked() {
                    }
                });

                ui.collapsing("Keybindings",|ui| {
                });

            });
    }


}