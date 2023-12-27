use std::collections::HashMap;
use egui::{Color32, Mesh, Painter, Pos2, Rect, TextureId, TextStyle, Align, FontDefinitions, FontFamily, Align2, Stroke};
use egui::epaint::Vertex;

#[derive(Default, serde::Deserialize, serde::Serialize, Debug)]
struct Area {
    position: Pos2,
    index: u32,
    area: f32,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct SunburstChart {
    data: Vec<f32>,
    areas: HashMap<u32, Area>,
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
        let pos = pos.unwrap_or_else(|| rect.center());

        let min_dim = rect.width().min(rect.height());
        let initial_radius = min_dim / 2.0;

        for d in 0..depth {
            let scaled_radius = initial_radius * (1.0 - d as f32 / depth as f32);
            let scaled_zoom = zoom as f32 / 10.0;

            let adjusted_radius = scaled_radius * (1.0 + scaled_zoom);

            let circle_color = Color32::from_rgb(200, 200, 200);
            let stroke_color = Color32::GRAY;

            // Draw the circle
            painter.circle(
                pos,
                adjusted_radius,
                circle_color,
                Stroke::new(3.0, stroke_color),
            );

            // Draw segments between the circles (excluding the first circle)
            if d > 0 {
                let num_segments = 2u32.pow((depth - d) as u32);
                for i in 0..num_segments {
                    let angle = i as f32 / num_segments as f32 * std::f32::consts::PI * 2.0;
                    let start_x = pos.x + adjusted_radius * angle.cos();
                    let start_y = pos.y + adjusted_radius * angle.sin();

                    let prev_scaled_radius = initial_radius * (1.0 - (d - 1) as f32 / depth as f32);
                    let prev_adjusted_radius = prev_scaled_radius * (1.0 + scaled_zoom);

                    let end_x = pos.x + prev_adjusted_radius * angle.cos();
                    let end_y = pos.y + prev_adjusted_radius * angle.sin();

                    painter.line_segment(
                        [Pos2::new(start_x, start_y), Pos2::new(end_x, end_y)],
                        Stroke::new(2.0, Color32::BLACK),
                    );

                    // Calculate the index and area
                    let index = i + (2u32.pow((depth - d - 1) as u32) * d as u32);
                    let area = (std::f32::consts::PI * adjusted_radius) / num_segments as f32;

                    // Store the Area object in the map
                    let area_obj = Area {
                        position: Pos2::new((start_x + end_x) / 2.0, (start_y + end_y) / 2.0),
                        index,
                        area,
                    };

                    self.areas.insert(index, area_obj);
                }
            }
        }
        self.draw_areas(painter);
        self.handle_hover(painter);
    }

    fn draw_areas(&mut self, painter: &Painter) {
        for (_, area) in &self.areas {
            // Calculate the triangle points
            let start = Pos2::new(area.position.x, area.position.y - 10.0);
            let end1 = Pos2::new(area.position.x - 5.0, area.position.y + 5.0);
            let end2 = Pos2::new(area.position.x + 5.0, area.position.y + 5.0);

            // Create a mesh for the filled triangle
            let mut mesh = Mesh {
                indices: Vec::new(),
                vertices: Vec::new(),
                texture_id: TextureId::default()
            };

            // Define the vertices and indices for the triangle
            let vertex_start = mesh.vertices.len() as u32;
            mesh.vertices.push(Vertex::new(start, Pos2::ZERO, Color32::WHITE));
            mesh.vertices.push(Vertex::new(end1, Pos2::ZERO, Color32::WHITE));
            mesh.vertices.push(Vertex::new(end2, Pos2::ZERO, Color32::WHITE));
            mesh.indices.push(vertex_start);
            mesh.indices.push(vertex_start + 1);
            mesh.indices.push(vertex_start + 2);

            // Draw the mesh
            painter.mesh(mesh);

            // Draw the text
            painter.text(
                area.position,
                Align2::CENTER_CENTER,
                format!("Index: {}, Area: {:.2}", area.index, area.area),
                Default::default(),
                Default::default(),
            );
        }
    }

    fn handle_hover(&mut self, painter: &Painter) {
        for (_, area) in &self.areas {

        }
    }

    pub fn get_hovered_index(&self) -> Option<u32> {
        self.hovered_index
    }
}


