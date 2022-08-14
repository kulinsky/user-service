use std::sync::Arc;

use serde::Serialize;
use uuid::Uuid;
use warp::reject::reject;
use warp::reply::json;
use warp::{Rejection, Reply};

use crate::application::services::user::service::UserService;
use crate::implementation::postgres::repository::UserRepositoryPG;

#[derive(Serialize)]
pub struct UserResponse<'a> {
    id: Uuid,
    email: String,
    first_name: &'a str,
    last_name: &'a str,
}

pub async fn get_by_id(
    user_service: Arc<UserService<UserRepositoryPG>>,
) -> Result<impl Reply, Rejection> {
    let id = "062ee66d-bf14-71a6-8000-26806ef51b28"
        .to_string()
        .try_into()
        .unwrap();

    let u = user_service.get_by_id(&id).await.unwrap();

    let resp = UserResponse {
        id: u.id().value,
        email: u.email().value,
        first_name: u.first_name(),
        last_name: u.last_name(),
    };

    Ok(json(&resp))
}
