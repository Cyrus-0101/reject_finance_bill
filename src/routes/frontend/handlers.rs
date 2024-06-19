use askama::Template;
use axum::{response::IntoResponse, Form};
use serde::{Deserialize, Serialize};

use super:: HtmlTemplate;

#[derive(Template)]
#[template(path="index.html")]
pub struct HomeTemplate{}

pub async fn home() -> impl IntoResponse{
    let temp = HomeTemplate{};
    HtmlTemplate(temp)

}

#[derive(Template)]
#[template(path="forms.html")]
pub struct FormTemplate{}

pub async fn form() -> impl IntoResponse{
    let temp = FormTemplate{};
    HtmlTemplate(temp)

}

#[derive(Template)]
#[template(path="cards.html")]
pub struct EventTemplate{}

pub async fn events() -> impl IntoResponse{
    let temp = EventTemplate{};
    HtmlTemplate(temp)

}
