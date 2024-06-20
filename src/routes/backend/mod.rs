pub mod form_inputs;
pub mod get_all_details;

use axum::{routing::{get, post}, Router};

use crate::utils::app_state::AppState;

use self::{form_inputs::send_data, get_all_details::get_all_items};

pub fn backend_routes(app_state: AppState) -> Router{
    Router::new()
        .route("/upload", post(send_data))
        .route("/get", get(get_all_items))
        .with_state(app_state)

}
