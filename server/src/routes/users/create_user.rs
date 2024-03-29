use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::{
    database::users,
    utilities::{
        app_error::AppError, hash::hash_password, jwt::create_token,
        token_wrapper::TokenWrapper,
    },
};

use super::{
    convert_active_to_model, RequestCreateUser, ResponseDataUser, ResponseUser,
};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username.clone());
    new_user.password = Set(hash_password(&request_user.password)?);
    new_user.token =
        Set(Some(create_token(&jwt_secret.0, request_user.username)?));
    let user = new_user.save(&db).await.map_err(|error| {
        eprintln!("Error creating user: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again",
        )
    })?;
    let user = convert_active_to_model(user)?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap(),
        },
    }))
}
