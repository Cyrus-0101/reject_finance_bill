use axum::{extract::State, Json};
use hyper::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::database::userdetails as User;

use crate::utils::app_error::AppError;

pub async fn send_data(
    State(database): State<DatabaseConnection>,
    Json(request_user): Json<GetUserDetails>
    ) -> Result<Json<RespondUserDetails>, AppError>{
    
    let new_data = User::ActiveModel{
        first_name: Set(request_user.first_name),
        last_name: Set(request_user.last_name),
        location: Set(request_user.location),
        police_station: Set(request_user.police_station),
        username: if let Some(username) = request_user.username{
            Set(Some(username))
            }else{
                Set(None)
            }
        ,
        ..Default::default()


    }.save(&database)
    .await
    .map_err(|error|{
        eprintln!("something went wrong saving the details: {}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something Went wrong")
    })?;
   let details = new_data.try_into_model()
        .map_err(|error|{
            eprintln!("Error, could not convert user details into model: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?;


   Ok(Json(RespondUserDetails { username: details.username, first_name: details.first_name, location: details.location, police_station: details.police_station }))
}

#[derive(Serialize, Deserialize)]
pub struct GetUserDetails{
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub location: String,
    pub police_station: String,
}

#[derive(Serialize, Deserialize)]
pub struct RespondUserDetails{
    pub username: Option<String>,
    pub first_name: String,
    pub location: String,
    pub police_station: String

}
