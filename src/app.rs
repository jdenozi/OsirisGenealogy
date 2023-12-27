use eframe::epaint::FontFamily::Proportional;
use eframe::epaint::FontId;
use egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use crate::gui::{fonts, menu_bar};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    label: String,
    value: f32,
    menu_bar: menu_bar::MenuBar,
}

impl Default for App {
    fn default() -> Self {
        unsafe {
            Self {
                label: "Hello World!".to_owned(),
                value: 0.7,
                menu_bar: menu_bar::MenuBar::new(),
            }
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::setup_custom_fonts(&cc.egui_ctx);
        Default::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        //catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
        // self.image_layout.update(ctx, _frame);

        self.menu_bar.update(ctx, _frame);

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Body, FontId::new(15.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(18.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
            .into();
        ctx.set_style(style);
    }

}
