use std::cell::RefCell;
use std::option::Option;
use std::rc::Rc;
use egui::{CentralPanel, Color32, PointerButton, Pos2, Slider, SliderOrientation};
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::gui::charts::sun_burst;
use egui_extras::{Column, TableBuilder};


#[derive(Deserialize, Serialize)]
enum Mode {
    Graph,
    SunBurst,
}

impl Default for Mode {
    fn default() -> Self {
        Self::SunBurst
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct MainFrame {
    state: Rc<RefCell<AppState>>,
    mode: Mode,
    layout_color: Color32,
    sun_burst: sun_burst::SunburstChart,
    sun_burst_depth: i8,
    sun_burst_zoom: i8,
    sun_burst_pos: Option<Pos2>,
    sun_burst_drag: bool,
    sun_burst_ctrl: bool

}

impl MainFrame {
    pub  fn new(global_state: Rc<RefCell<AppState>>) -> Self {
        MainFrame {
            state: global_state,
            mode: Mode::Graph,
            layout_color: Default::default(),
            sun_burst: sun_burst::SunburstChart::new(),
            sun_burst_depth: 1,
            sun_burst_zoom: 0,
            sun_burst_pos:Option::default(),
            sun_burst_drag: false,
            sun_burst_ctrl: false,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Graph").clicked() {
                    self.mode = Mode::Graph;
                }
                if ui.button("Sunburst Chart").clicked() {
                    self.mode = Mode::SunBurst;
                }
            });

            match self.mode {
                Mode::Graph => {
                    if let Some(gedcomx_data) = &self.state.borrow_mut().gedcomx_data {

                        let table = TableBuilder::new(ui)
                            .striped(false)
                            .resizable(false)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .column(Column::auto())
                            .column(Column::initial(100.0).range(40.0..=300.0))
                            .column(Column::initial(100.0).at_least(40.0).clip(true))
                            .column(Column::remainder())
                            .min_scrolled_height(0.0);

                        table.header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("Firstname");
                            });
                            header.col(|ui| {
                                ui.strong("Lastname");
                            });
                            header.col(|ui| {
                                ui.strong("Birthday");
                            });
                            header.col(|ui| {
                                ui.strong("Death day");
                            });
                            header.col(|ui| {
                                ui.strong("Birth place");
                            });
                        });

                    table.

                    } else {
                        ui.label("No data available.");
                    }



                }
                Mode::SunBurst =>  {

                    // Handle zoom event
                    if ctx.input(|i| i.scroll_delta.y.abs() > 0.0) {
                        let scroll_delta = ctx.input(|i| i.scroll_delta.y) as i8;
                        let new_zoom = self.sun_burst_zoom + (scroll_delta / 10);
                        self.sun_burst_zoom = new_zoom.clamp(-5, 20);
                    }


                    // Handle sun burst dragging
                    if ctx.input(|i| i.pointer.button_pressed(PointerButton::Primary)) {
                        self.sun_burst_drag = true;
                    }
                    if ctx.input(|i| i.pointer.button_released(PointerButton::Primary)) {
                        self.sun_burst_drag = false;
                    }

                    if self.sun_burst_drag {
                        // Dragging is active, update position
                        let delta = ctx.input(|i| i.pointer.delta());
                        self.sun_burst_pos = Some(self.sun_burst_pos.unwrap_or_default() + delta);
                    }



                    egui::Window::new("Options").show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.group(|ui| {
                                ui.vertical(|ui| {
                                    if ui.button("Center").clicked() {
                                        self.sun_burst_pos = None;
                                    }
                                    ui.checkbox(&mut true, "collapsible");
                                    ui.checkbox(&mut true, "resizable");
                                });
                            });
                            ui.vertical(|ui| {
                                ui.add(
                                    Slider::new(&mut self.sun_burst_depth, (1i8)..=(10i8))
                                        .orientation(SliderOrientation::Horizontal)
                                        .text("depth")
                                        .step_by(1.0)
                                );

                                ui.add(
                                    Slider::new(&mut self.sun_burst_zoom, (-5i8)..=(20i8))
                                        .orientation(SliderOrientation::Horizontal)
                                        .text("zoom")
                                        .step_by(1.0)
                                );
                            });
                        });
                    });

                    if self.sun_burst_pos == Option::None {
                        self.sun_burst_pos = Option::from(ui.available_rect_before_wrap().center());
                    }

                    self.sun_burst.draw(ui.painter(), &ui.available_rect_before_wrap(), self.sun_burst_depth, self.sun_burst_zoom, self.sun_burst_pos);
                }
            }
        });
    }
}
