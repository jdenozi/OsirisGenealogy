use std::collections::HashMap;
use std::f32::consts::PI;
use egui::{Color32, Mesh, Painter, Pos2, Rect, TextureId, TextStyle, Align, FontDefinitions, FontFamily, Align2, Stroke, LayerId, FontId, Vec2};
use egui::epaint::{PathShape, Vertex};
use egui::ImageData::Color;
use rand::Rng;

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
}

impl SunburstChart {
    pub fn new() -> Self {
        SunburstChart {
            data: Vec::new(),
            areas: HashMap::new(),
            hovered_index: None,
        }
    }

    pub fn update_data(&mut self, new_data: Vec<f32>) {
        self.data = new_data;
    }

    pub fn draw(&mut self, painter: &Painter, rect: &Rect, depth: i8, zoom: i8, pos: Option<Pos2>) {
        self.areas.clear();
        let pos = pos.unwrap_or_else(|| rect.center());

        let min_dim = rect.width().min(rect.height());
        let initial_radius = min_dim / 2.0;

        for d in 0..depth {
            let scaled_radius = initial_radius * (1.0 - d as f32 / depth as f32);
            let scaled_zoom = zoom as f32 / 10.0;

            let adjusted_radius = scaled_radius * (1.0 + scaled_zoom);

            let circle_color = Color32::from_rgb(200, 200, 200);
            let stroke_color = Color32::GRAY;

            // Draw segments between the circles (excluding the first circle)
            if d > 0 {
                let num_segments = 2u32.pow((depth - d) as u32);
                let mut position_previous_segment: Option<Pos2> = None;
                let mut position_first_segment: Option<Pos2> = None;


                let prev_scaled_radius = initial_radius * (1.0 - (d - 1) as f32 / depth as f32);
                let prev_adjusted_radius = prev_scaled_radius * (1.0 + scaled_zoom);

                for i in 0..num_segments {

                    let angle = i as f32 / num_segments as f32 * std::f32::consts::PI * 2.0;
                    let start_x = pos.x + adjusted_radius * angle.cos();
                    let start_y = pos.y + adjusted_radius * angle.sin();

                    let end_x = pos.x + prev_adjusted_radius * angle.cos();
                    let end_y = pos.y + prev_adjusted_radius * angle.sin();

                    painter.line_segment(
                        [Pos2::new(start_x, start_y), Pos2::new(end_x, end_y)],
                        Stroke::new(2.0, Color32::BLACK),
                    );

                    self.create_area(
                        pos,
                        position_previous_segment,
                        end_x,
                        end_y,
                        adjusted_radius,
                        num_segments,
                        d,
                        depth,
                        i,
                    );

                    // Store the current position as the previous segment's position
                    position_previous_segment = Some(Pos2::new(end_x, end_y));
                    if i==0{
                        position_first_segment= Some(Pos2::new(end_x, end_y));

                    }
                }

                self.create_area(
                    pos,
                    position_previous_segment,
                    position_first_segment.unwrap().x,
                    position_first_segment.unwrap().y,
                    adjusted_radius,
                    num_segments,
                    d,
                    depth,
                    num_segments,
                );

            }
            // Draw the circle
            painter.circle(
                pos,
                adjusted_radius,
                circle_color,
                Stroke::new(3.0, stroke_color),
            );
        }
        self.draw_areas(painter);
    }

    fn create_area(
        &mut self,
        pos: Pos2,
        position_previous_segment: Option<Pos2>,
        end_x: f32,
        end_y: f32,
        adjusted_radius: f32,
        num_segments: u32,
        d: i8,
        depth: i8,
        i: u32,
    ) {
        // Calculate the index and area
        let mut index = i + (2u32.pow((depth - d - 1) as u32) * d as u32);
        let area = (PI * adjusted_radius) / num_segments as f32;

        // Check if we have both the current and previous segment's positions
        if let Some(position_previous_segment) = position_previous_segment {
            // Create the Area object once we have both positions
            let area_obj = Area {
                position1: position_previous_segment,
                position2: Pos2::new(end_x, end_y),
                index,
                area,
            };

            // Create the PathShape directly
            let vertex1 = pos;
            let vertex2 = area_obj.position1;
            let vertex3 = area_obj.position2;

            // Compute the midpoint of the base BC
            let m = 0.5 * (Vec2::new(vertex2.x, vertex2.y) + Vec2::new(vertex3.x, vertex3.y));

            // Compute the normalized direction vector along BC
            let bc = Vec2::new(vertex3.x, vertex3.y) - Vec2::new(vertex2.x, vertex2.y);
            let u = bc.normalized();

            // Interpolate points along the circular arc between vertex1 and vertex2
            let mut interpolated_points = Vec::new();
            let num_interpolated_points = 10;

            for i in 0..=num_interpolated_points {
                let t = i as f32 / num_interpolated_points as f32;
                let angle = t * PI;

                // Calculate the position on the circular arc
                let x_on_arc = vertex2.x + adjusted_radius * angle.cos();
                let y_on_arc = vertex2.y + adjusted_radius * angle.sin();

                interpolated_points.push(Vec2::new(x_on_arc, y_on_arc));
            }

            // Choose a height factor
            let height_factor = 1.0;

            // Create PathShape for each pair of consecutive points
            for i in 0..interpolated_points.len() - 1 {
                let point1 = interpolated_points[i];
                let point2 = interpolated_points[i + 1];

                // Compute the position of point D based on the interpolated points
                let d1 = m + height_factor * (point1 - m);
                let d2 = m + height_factor * (point2 - m);

                // Create PathShape for the pair of interpolated points
                let path_shape = PathShape::convex_polygon(
                    vec![vertex1, Pos2::new(d1.x, d1.y), Pos2::new(d2.x, d2.y), vertex2],
                    Color32::RED,
                    Stroke::new(2.0, Color32::TRANSPARENT),
                );

                // Store the PathShape object in the map
                self.areas.insert(index, path_shape);

                // Update the index for the next iteration
                index += 1;
            }
        }
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
            let mut area_path = area.clone();
            painter.add(area_path);

            // Calculate the center position of the area using center_of_mass
            let x: Vec<f32> = area.points.iter().skip(1).map(|point| point.x).collect();
            let x_center: f32 = x.iter().sum::<f32>() / x.len() as f32;
            let y: Vec<f32> = area.points.iter().skip(1).map(|point| point.y).collect();
            let y_center: f32 = y.iter().sum::<f32>() / y.len() as f32;
            let area_center = Pos2::new(x_center, y_center);

            // Draw the text in the middle of the area
            painter.text(
                area_center,
                Align2::CENTER_CENTER,
                format!("{}", index),
                FontId::monospace(10.0),
                Color32::BLACK
            );
        }
    }


    pub fn get_hovered_index(&self) -> Option<u32> {
        self.hovered_index
    }
}
