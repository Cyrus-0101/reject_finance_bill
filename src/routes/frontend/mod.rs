mod handlers;
use askama::Template;
use axum::{response::{Html, IntoResponse, Response}, routing::{get, post}, Router};
use hyper::StatusCode;

use self::handlers::{events, form, home};



pub fn frontend_routes() -> Router{
    
    Router::new()
        .route("/", get(home))
        .route("/form", get(form))
        .route("/events", get(events))
}


pub struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}




