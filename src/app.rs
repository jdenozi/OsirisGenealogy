use std::cell::RefCell;
use std::rc::Rc;
use eframe::epaint::FontFamily::Proportional;
use eframe::epaint::FontId;
use egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::gui::{fonts};
use crate::gui::main_frame::MainFrame;
use crate::gui::menu_bar::MenuBar;

#[derive(Deserialize, Serialize)]
pub struct App {
    value: f32,
    state: Rc<RefCell<AppState>>,
    menu_bar: MenuBar,
    main_frame: MainFrame,
}

impl Default for App {
    fn default() -> Self {
        let state: Rc<RefCell<AppState>> = Rc::new(RefCell::new(AppState::default()));
        Self {
            value: 0.7,
            state: state.clone(),
            menu_bar: MenuBar::new(state.clone()),
            main_frame: MainFrame::new(state.clone()),
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
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
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
