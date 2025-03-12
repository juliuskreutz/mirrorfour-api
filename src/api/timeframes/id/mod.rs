use actix_web::{delete, get, web};

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_timeframe).service(delete_timeframe);
}

#[derive(serde::Deserialize)]
struct GetTimeframe {
    tz: Option<chrono_tz::Tz>,
}

#[utoipa::path(
    tag = "timeframes",
    responses(
        (status = 200, body = crate::types::Timeframe),
    ),
    params(
        ("tz" = Option<String>, Query),
    ),
)]
#[get("/timeframes/{id}", guard = "crate::api::admin_guard")]
async fn get_timeframe(
    path: web::Path<i32>,
    query: web::Query<GetTimeframe>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    let timeframe_id = path.into_inner();

    let mut timeframe: crate::types::Timeframe =
        crate::db::timeframes::get_by_id(timeframe_id, &pool)
            .await?
            .into();

    let Some(tz) = query.tz else {
        return Ok(actix_web::HttpResponse::Ok().json(timeframe));
    };

    use chrono::Datelike as _;

    let week_start = chrono::Utc::now().with_timezone(&tz).date_naive();
    let week_start =
        week_start - chrono::Duration::days(week_start.weekday().num_days_from_monday() as i64);

    let local_start = (week_start + chrono::Duration::days(timeframe.day as i64))
        .and_time(timeframe.start)
        .and_utc()
        .with_timezone(&tz);

    timeframe.day = local_start.weekday().num_days_from_monday() as i32;
    timeframe.start = local_start.time();

    Ok(actix_web::HttpResponse::Ok().json(timeframe))
}

#[utoipa::path(
    tag = "timeframes",
    responses(
        (status = 200),
    ),
)]
#[delete("/timeframes/{id}", guard = "crate::api::admin_guard")]
async fn delete_timeframe(
    path: web::Path<i32>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    let timeframe_id = path.into_inner();

    crate::db::timeframes::delete_by_id(timeframe_id, &pool).await?;

    Ok(actix_web::HttpResponse::Ok().finish())
}
