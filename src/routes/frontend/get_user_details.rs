use axum::{response::Redirect, Form};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDetails{
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub location: String,
    pub police_station: String,
}
pub async fn get_details(form_input: Form<UserDetails>) -> Redirect{


    let client = reqwest::Client::new();
    client.post("http://localhost:3000/api/v1/upload")
        .json(&form_input.0)
        .send()
        .await
        .expect("Failed to send reqwest");

    Redirect::to("/")
}
