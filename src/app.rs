use eframe::epaint::FontFamily::Proportional;
use eframe::epaint::FontId;
use egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use serde::{Deserialize, Serialize};
use crate::gui::{fonts, menu_bar, main_frame, charts};
#[derive( Deserialize, Serialize)]
pub struct App {
    value: f32,
    menu_bar: menu_bar::MenuBar,
    main_frame: main_frame::MainFrame
}

impl Default for App {
    fn default() -> Self {
            Self {
                value: 0.7,
                menu_bar: menu_bar::MenuBar::new(),
                main_frame: main_frame::MainFrame::new()

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

        self.menu_bar.update(ctx, _frame);
        self.main_frame.update(ctx, _frame);

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(17.0, Proportional)),
            (Body, FontId::new(15.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(15.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
            .into();
        ctx.set_style(style);
    }

}
