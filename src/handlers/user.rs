use actix_web::{post, web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;

use crate::db::establish_connection;
use crate::errors::CustomError;
use crate::models::user::{Column, Entity as User, Model as UserModel};
use crate::models::user::{LoginUser, UserData};
use crate::utils::{generate_jwt, verify_password};

#[post("/login")]
pub async fn login(login_data: web::Json<LoginUser>) -> Result<impl Responder, CustomError> {
    let db = establish_connection().await?;

    let user: Option<UserModel> = User::find()
        .filter(Column::Username.eq(&login_data.username))
        .one(&db)
        .await?;

    match user {
        Some(user) => match verify_password(&login_data.password, &user.password) {
            Ok(valid) => {
                if valid {
                    match generate_jwt(user.id) {
                        Ok(token) => {
                            let user_data = UserData {
                                username: user.username,
                            };
                            Ok(HttpResponse::Ok()
                                .json(json!({ "token": token, "user": user_data })))
                        }
                        Err(err) => Err(CustomError::Other(format!(
                            "Failed to generate JWT: {}",
                            err
                        ))),
                    }
                } else {
                    Err(CustomError::InvalidCredentials)
                }
            }
            Err(_) => Err(CustomError::Other("Failed to verify password".to_string())),
        },
        None => Err(CustomError::InvalidCredentials),
    }
}
