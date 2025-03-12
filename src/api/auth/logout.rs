use actix_web::post;

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(post_logout);
}

#[utoipa::path(
    tag = "auth",
    responses(
        (status = 200),
    ),
)]
#[post("/auth/logout")]
async fn post_logout(
    session: actix_session::Session,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    session.clear();

    Ok(actix_web::HttpResponse::Ok().finish())
}
