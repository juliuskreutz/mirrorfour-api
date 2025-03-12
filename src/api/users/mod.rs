mod id;

use actix_web::{get, post, web};
use rand::Rng as _;

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.configure(id::configure)
        .service(get_users)
        .service(post_users);
}

#[utoipa::path(
    tag = "users",
    responses(
        (status = 200, body = Vec<crate::types::User>),
    ),
)]
#[get("/users", guard = "crate::api::admin_guard")]
async fn get_users(
    pool: web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    let users: Vec<crate::types::User> = crate::db::users::get(&pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(actix_web::HttpResponse::Ok().json(users))
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
struct PostUser {
    username: String,
    password: String,
    name: String,
    #[schema(value_type = String)]
    tz: chrono_tz::Tz,
    role: crate::types::Role,
}

#[utoipa::path(
    tag = "users",
    request_body = PostUser,
    responses(
        (status = 201),
    ),
)]
#[post("/users", guard = "crate::api::admin_guard")]
async fn post_users(
    payload: web::Json<PostUser>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    let salt = rand::rng().random::<[u8; 32]>();

    let password = argon2::hash_encoded(
        payload.password.as_bytes(),
        &salt,
        &argon2::Config::rfc9106_low_mem(),
    )
    .map_err(anyhow::Error::new)?;

    let insert_user = crate::db::users::InsertUser {
        username: payload.username.clone(),
        password,
        name: payload.name.clone(),
        tz: payload.tz.to_string(),
        role: payload.role.to_u8() as i32,
    };
    let user: crate::types::User = crate::db::users::insert(&insert_user, &pool).await?.into();

    Ok(actix_web::HttpResponse::Created().json(user))
}
