use actix_web::{post, web};

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(post_login);
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
struct PostLogin {
    username: String,
    password: String,
}

#[utoipa::path(
    tag = "auth",
    request_body = PostLogin,
    responses(
        (status = 200, body = crate::types::User),
        (status = 400),
    ),
)]
#[post("/auth/login")]
async fn post_login(
    payload: web::Json<PostLogin>,
    session: actix_session::Session,
    pool: web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    let users = crate::db::users::get_by_username(&payload.username, &pool).await?;

    let mut user = None;

    for u in users {
        if argon2::verify_encoded(&u.password, payload.password.as_bytes()).unwrap_or_default() {
            user = Some(u);
        }
    }

    let Some(user) = user else {
        return Ok(actix_web::HttpResponse::BadRequest().finish());
    };

    session
        .insert("user_id", user.id)
        .map_err(anyhow::Error::new)?;

    let user: crate::types::User = user.into();

    Ok(actix_web::HttpResponse::Ok().json(user))
}
