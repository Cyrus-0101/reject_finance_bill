use axum::{middleware, routing::{get, post}, Router};

use crate::utils::app_state::AppState;

pub fn backend_routes(app_state: AppState) -> Router{
    Router::new()
        .with_state(app_state)

}
