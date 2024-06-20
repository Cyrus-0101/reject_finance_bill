use axum::{extract::State, Json};
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::utils::app_error::AppError;
use crate::database::userdetails::Entity as Users_det;

#[derive(Serialize, Deserialize)]
pub struct Items{
    pub username: Option<String>,
    pub first_name: String,
    pub location: String,
    pub police_station: String

}

pub async fn get_all_items(
    State(database): State<DatabaseConnection>
    ) -> Result<Json<Vec<Items>>, AppError>{
    
    let items: Vec<Items> = Users_det::find()
        .all(&database)
        .await
        .map_err(|error|{
            eprintln!("Error getting all tasks, {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?
    .into_iter()
    .map(|db_item| Items{
        username: db_item.username,
        first_name: db_item.first_name,
        location: db_item.location,
        police_station: db_item.police_station
    })
    .collect();

    Ok(Json(items))
}

