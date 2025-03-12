mod timeframes;

use actix_web::{delete, get, web};

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.configure(timeframes::configure)
        .service(get_user)
        .service(delete_user);
}

#[utoipa::path(
    tag = "users",
    responses(
        (status = 200, body = crate::types::User),
    ),
)]
#[get("/users/{id}", guard = "crate::api::admin_guard")]
async fn get_user(
    path: web::Path<i32>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    let user_id = path.into_inner();

    let user: crate::types::User = crate::db::users::get_by_id(user_id, &pool).await?.into();

    Ok(actix_web::HttpResponse::Ok().json(user))
}

#[utoipa::path(
    tag = "users",
    responses(
        (status = 200),
    ),
)]
#[delete("/users/{id}", guard = "crate::api::admin_guard")]
async fn delete_user(
    path: web::Path<i32>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    let user_id = path.into_inner();

    crate::db::users::delete_by_id(user_id, &pool).await?;

    Ok(actix_web::HttpResponse::Ok().finish())
}
