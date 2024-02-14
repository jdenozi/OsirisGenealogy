use std::cell::RefCell;
use std::rc::Rc;
use serde::Deserialize;
use serde::Serialize;
use crate::{model};
use crate::app_state::AppState;

#[derive(Deserialize, Serialize)]
pub struct MenuBar {
    pub state: Rc<RefCell<AppState>>,
    pub show_cache_preference: bool,
}

impl MenuBar {
    pub fn new(global_state:Rc<RefCell<AppState>>) -> Self {
        MenuBar {
            state: global_state,
            show_cache_preference: false,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.menu_button("File", |ui|  {
                        if ui.button("Open").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                let file = model::file::File::new(path.to_str().unwrap().to_string().clone());
                                self.state.borrow_mut().gedcomx_data = Option::from(file.read().unwrap());
                            }
                        }
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