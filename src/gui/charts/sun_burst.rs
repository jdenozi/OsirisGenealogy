use std::collections::HashMap;
use std::f32::consts::PI;
use egui::{Color32, Painter, Pos2, Rect,Align2, Stroke, FontId};
use egui::epaint::{PathShape};

#[derive(Default, serde::Deserialize, serde::Serialize, Debug)]
struct Area {
    position1: Pos2,
    position2: Pos2,
    index: u32,
    area: f32,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct SunburstChart {
    data: Vec<f32>,
    areas: HashMap<u32, PathShape>,
    hovered_index: Option<u32>,
    node_number: u32,
}

impl SunburstChart {
    pub fn new() -> Self {
        SunburstChart {
            data: Vec::new(),
            areas: HashMap::new(),
            hovered_index: None,
            node_number: 0,

        }
    }

    pub fn draw(&mut self, painter: &Painter, rect: &Rect, depth: i8, zoom: i8, pos: Option<Pos2>) {
        self.areas.clear();
        let center = pos.unwrap_or_else(|| rect.center());
        let min_dim = rect.width().min(rect.height());
        let initial_radius = min_dim / 2.0;

        for d in 0..depth {
            let scaled_radius = initial_radius * (1.0 - d as f32 / depth as f32);
            let scaled_zoom = zoom as f32 / 10.0;
            let adjusted_radius = scaled_radius * (1.0 + scaled_zoom);
            let circle_color = Color32::from_rgb(200, 200, 200);

            // Draw the circle
            painter.circle(center, adjusted_radius, circle_color, Stroke::new(3.0, Color32::BLACK));

            // Drawing segments
            if d < depth - 1 {
                let num_segments = 2u32.pow((depth - d - 1) as u32);
                let segment_angle = 2.0 * PI / num_segments as f32;

                for i in 0..num_segments {
                    let angle = i as f32 * segment_angle;
                    let end_x = center.x + adjusted_radius * angle.cos();
                    let end_y = center.y + adjusted_radius * angle.sin();

                    painter.line_segment(
                        [center, Pos2::new(end_x, end_y)],
                        Stroke::new(2.0, Color32::BLACK),
                    );

                    // Creating areas
                    let start_angle = angle;
                    let end_angle = start_angle + segment_angle;

                    let area = self.create_area(center, start_angle, end_angle, adjusted_radius, Color32::TRANSPARENT);
                    self.node_number += 1;
                    self.areas.insert(self.node_number, area.clone());
                    painter.add(area);
                }
            }
        }

        self.draw_areas(painter);

        // Draw the text in the middle of the area
        painter.text(
            center,
            Align2::CENTER_CENTER,
            "Sylvie BOISSON".to_string(),
            FontId::monospace(10.0 + zoom as f32),
            Color32::BLACK,
        );
    }

    /// Creates a filled area (segment) for a sunburst chart.
    ///
    /// This function generates a PathShape representing a segment of a circle (or ring) in a sunburst chart.
    /// It interpolates points along the arc between two angles and connects them to form a filled shape.
    ///
    fn create_area(
        &mut self,
        center: Pos2,
        start_angle: f32,
        end_angle: f32,
        radius: f32,
        area_color: Color32,
    ) -> PathShape {
        let num_interpolated_points = 10;
        let mut area_points = Vec::new();
        area_points.push(center);

        for i in 0..=num_interpolated_points {
            let t = i as f32 / num_interpolated_points as f32;
            let angle = start_angle + t * (end_angle - start_angle);

            let x_on_arc = center.x + radius * angle.cos();
            let y_on_arc = center.y + radius * angle.sin();

            area_points.push(Pos2::new(x_on_arc, y_on_arc));
        }

        area_points.push(center);

        PathShape::convex_polygon(
            area_points,
            area_color,
            Stroke::new(2.0, Color32::TRANSPARENT),
        )
    }

    /// Draws the areas on the canvas using the provided painter.
    ///
    /// This function iterates over the areas in the map and adds their
    /// representations to the painter. It also calculates the center of each
    /// area and draws the corresponding index as text in the middle of the area.
    ///
    /// # Arguments
    ///
    /// - `self`: A mutable reference to the struct containing the areas.
    /// - `painter`: A reference to the painter used for rendering.
    /// `
    fn draw_areas(&mut self, painter: &Painter) {
        for (index, area) in &self.areas {
            // Extract the points from the area
            let points = &area.points;

            // Skip the first point (assuming it's vertex1 or 'a')
            if points.len() > 1 {
                // Calculate the center position of the area using center_of_mass
                let x_center: f32 = points.iter().skip(1).map(|point| point.x).sum::<f32>() / (points.len() - 1) as f32;
                let y_center: f32 = points.iter().skip(1).map(|point| point.y).sum::<f32>() / (points.len() - 1) as f32;
                let area_center = Pos2::new(x_center, y_center);

                // Draw the text in the middle of the area
                painter.text(
                    area_center,
                    Align2::CENTER_CENTER,
                    format!("{}", self.node_number + 1 - index),
                    FontId::monospace(10.0),
                    Color32::BLACK,
                );
            }
        }
    }
}
