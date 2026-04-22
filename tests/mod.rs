mod common;

#[path = "common/default_data.rs"]
mod default_data;

#[path ="common/db.rs"]
mod db;

mod users;

pub use football_group_manager::{
    create_app_lazy,
    dto::{
        user::{UserResponse, CreateUserRequest},
    },
};

pub use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
        header::CONTENT_TYPE,
    },
    Router,
};
pub use tower::ServiceExt;
pub use serde::Deserialize;
