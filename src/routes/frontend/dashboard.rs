use askama::Template;
use axum::{extract::{Query, State}, response::IntoResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};


use super::HtmlTemplate;
const ITEMS_PER_PAGE: usize = 5;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDetails {
    pub username: Option<String>,
    pub first_name: String,
    pub location: String,
    pub police_station: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTemplate {
    pub username: String,
    pub first_name: String,
    pub location: String,
    pub police_station: String,
}

#[derive(Template)]
#[template(path="index.html")]
pub struct HomeTemplate{
    users: Vec<UserTemplate>,
    current_page: usize,
    total_pages: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams{
    page: Option<usize>

}

pub async fn home(
    State(client): State<Client>,
    Query(pagination): Query<PaginationParams>
    ) -> impl IntoResponse{
    let url = "http://localhost:3000/api/v1/get";
    let response = client.get(url)
        .send()
        .await
        .expect("Failed to send to backend")
        .json::<Vec<UserDetails>>()
        .await
        .expect("Failed to deserialize response");

    let page = pagination.page.unwrap_or(1);
    let start_page = (page - 1) * ITEMS_PER_PAGE;
    let end = start_page + ITEMS_PER_PAGE;
    let paginated_response = response[start_page..end.min(response.len())].to_vec();
    let total_pages = (response.len() as f64 / ITEMS_PER_PAGE as f64).ceil() as usize;


    let user_templates: Vec<UserTemplate> = paginated_response.into_iter().map(|user| {
        UserTemplate {
            username: user.username.unwrap_or_else(|| "N/A".to_string()),
            first_name: user.first_name,
            location: user.location,
            police_station: user.police_station
        }
    }).collect();

    let temp = HomeTemplate{
        users: user_templates,
        current_page: page,
        total_pages
    };
    HtmlTemplate(temp)

}


