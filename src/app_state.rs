use gedcomx::Gedcomx;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct AppState {
    pub is_file_opened: bool,
    pub gedcomx_data: Option<Gedcomx>,
}

impl AppState{
    pub fn new() -> Self {
        AppState { is_file_opened: false, gedcomx_data: None }
    }
}
impl Default for AppState {
    fn default() -> Self {
        AppState::new()
    }
}
