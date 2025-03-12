mod auth;
mod timeframes;
mod users;

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.configure(auth::configure)
        .configure(timeframes::configure)
        .configure(users::configure);
}

fn admin_guard(ctx: &actix_web::guard::GuardContext) -> bool {
    use actix_session::SessionExt as _;
    use actix_web::web;

    let session = ctx.get_session();

    let Ok(Some(user_id)) = session.get::<i32>("user_id") else {
        return false;
    };

    let Some(pool) = ctx.app_data::<web::Data<sqlx::PgPool>>() else {
        return false;
    };

    let Ok(user) =
        futures::executor::block_on(async { crate::db::users::get_by_id(user_id, pool).await })
    else {
        return false;
    };

    user.role as u8 == crate::types::Role::Admin.to_u8()
}

#[derive(Debug)]
struct ApiError(anyhow::Error);

impl actix_web::error::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
