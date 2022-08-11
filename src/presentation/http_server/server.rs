use std::sync::Arc;

use warp::Filter;

use crate::{
    application::services::user::service::UserService,
    implementation::postgres::repository::UserRepositoryPG, infrastructure::database::DB,
};

use super::{handler, middleware};

pub struct Server {
    port: u16,
    db_pool: DB,
}

impl Server {
    pub fn new(port: u16, db_pool: DB) -> Self {
        Self { port, db_pool }
    }

    pub async fn run(&self) {
        let repository = UserRepositoryPG::new();
        let user_service = Arc::new(UserService::new(self.db_pool.clone(), repository));

        let api = warp::path("api");
        let api_v1 = api.and(warp::path("v1"));
        let users = api_v1.and(warp::path("users"));

        let user_get_by_id = users
            .and(warp::get())
            .and(middleware::service::with_service(user_service.clone()))
            .and_then(handler::get_by_id);

        warp::serve(user_get_by_id)
            .run(([0, 0, 0, 0], self.port))
            .await
    }
}
