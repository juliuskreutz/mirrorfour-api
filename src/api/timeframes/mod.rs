mod id;

use actix_web::{get, web};

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.configure(id::configure).service(get_timeframes);
}

#[derive(serde::Deserialize)]
struct GetTimeframes {
    tz: Option<chrono_tz::Tz>,
}

#[utoipa::path(
    tag = "timeframes",
    responses(
        (status = 200, body = Vec<crate::types::Timeframe>),
    ),
    params(
        ("tz" = Option<String>, Query),
    ),
)]
#[get("/timeframes", guard = "crate::api::admin_guard")]
async fn get_timeframes(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<GetTimeframes>,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    let mut timeframes: Vec<crate::types::Timeframe> = crate::db::timeframes::get(&pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    let Some(tz) = query.tz else {
        return Ok(actix_web::HttpResponse::Ok().json(timeframes));
    };

    use chrono::Datelike as _;

    let week_start = chrono::Utc::now().with_timezone(&tz).date_naive();
    let week_start =
        week_start - chrono::Duration::days(week_start.weekday().num_days_from_monday() as i64);

    for timeframe in timeframes.iter_mut() {
        let local_start = (week_start + chrono::Duration::days(timeframe.day as i64))
            .and_time(timeframe.start)
            .and_utc()
            .with_timezone(&tz);

        timeframe.day = local_start.weekday().num_days_from_monday() as i32;
        timeframe.start = local_start.time();
    }

    Ok(actix_web::HttpResponse::Ok().json(timeframes))
}
