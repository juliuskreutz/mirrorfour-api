mod api;
mod db;
mod types;

use actix_web::web;
use utoipa::OpenApi as _;
use utoipa_actix_web::AppExt as _;

#[macro_use]
extern crate tracing;

#[derive(utoipa::OpenApi)]
#[openapi()]
struct ApiDoc;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting api!");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(100)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    let pool_data = web::Data::new(pool.clone());

    let session_key = session_key()?;

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(pool_data.clone())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(
                actix_session::SessionMiddleware::builder(
                    actix_session::storage::CookieSessionStore::default(),
                    session_key.clone(),
                )
                .session_lifecycle(
                    actix_session::config::PersistentSession::default()
                        .session_ttl(actix_web::cookie::time::Duration::weeks(4)),
                )
                .cookie_secure(!cfg!(debug_assertions))
                .build(),
            )
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(utoipa_actix_web::scope("/api").configure(api::configure))
            .openapi_service(|api| {
                utoipa_swagger_ui::SwaggerUi::new("/doc/swagger-ui/{_:.*}")
                    .url("/doc/openapi.json", api)
            })
            .into_app()
    })
    .bind(("localhost", 8000))?
    .run()
    .await?;

    Ok(())
}

fn session_key() -> anyhow::Result<actix_web::cookie::Key> {
    use actix_web::cookie::Key;
    use base64::{Engine, prelude::BASE64_STANDARD};

    let key_bytes = BASE64_STANDARD.decode(std::env::var("SESSION_KEY")?)?;

    let key = Key::from(&key_bytes);

    Ok(key)
}
