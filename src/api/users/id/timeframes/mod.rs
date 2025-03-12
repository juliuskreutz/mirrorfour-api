use actix_web::{get, post, web};

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_user_timeframes)
        .service(post_user_timeframes);
}

#[derive(serde::Deserialize)]
struct GetTimeframe {
    tz: Option<chrono_tz::Tz>,
}

#[utoipa::path(
    tag = "users",
    responses(
        (status = 200, body = Vec<crate::types::Timeframe>),
    ),
    params(
        ("tz" = Option<String>, Query),
    ),
)]
#[get("/users/{id}/timeframes", guard = "crate::api::admin_guard")]
async fn get_user_timeframes(
    path: web::Path<i32>,
    query: web::Query<GetTimeframe>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    let user_id = path.into_inner();

    let mut timeframes: Vec<crate::types::Timeframe> =
        crate::db::timeframes::get_by_user_id(user_id, &pool)
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

#[derive(serde::Deserialize, utoipa::ToSchema)]
struct PostTimeframe {
    day: i32,
    start: chrono::NaiveTime,
    duration: i32,
    #[schema(value_type = String)]
    tz: chrono_tz::Tz,
}

#[utoipa::path(
    tag = "users",
    request_body = PostTimeframe,
    responses(
        (status = 200),
    ),
)]
#[post("/users/{id}/timeframes", guard = "crate::api::admin_guard")]
async fn post_user_timeframes(
    path: web::Path<i32>,
    payload: web::Json<PostTimeframe>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<actix_web::HttpResponse, crate::api::ApiError> {
    use chrono::Datelike as _;

    let user_id = path.into_inner();

    let week_start = chrono::Utc::now().with_timezone(&payload.tz).date_naive();
    let week_start =
        week_start - chrono::Duration::days(week_start.weekday().num_days_from_monday() as i64);

    let utc_start = (week_start + chrono::Duration::days(payload.day as i64))
        .and_time(payload.start)
        .and_local_timezone(payload.tz)
        .unwrap()
        .to_utc();

    let insert_timeframe = crate::db::timeframes::InsertTimeframe {
        user_id,
        day: utc_start.weekday().num_days_from_monday() as i32,
        start: utc_start.time(),
        duration: payload.duration,
    };
    let mut timeframe: crate::types::Timeframe =
        crate::db::timeframes::insert(&insert_timeframe, &pool)
            .await?
            .into();

    timeframe.day = payload.day;
    timeframe.start = payload.start;

    Ok(actix_web::HttpResponse::Ok().json(timeframe))
}
