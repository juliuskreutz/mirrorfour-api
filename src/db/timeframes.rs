pub struct Timeframe {
    pub id: i32,
    pub user_id: i32,
    pub day: i32,
    pub start: chrono::NaiveTime,
    pub duration: i32,
}

pub struct InsertTimeframe {
    pub user_id: i32,
    pub day: i32,
    pub start: chrono::NaiveTime,
    pub duration: i32,
}

pub async fn insert(
    insert_timeframe: &InsertTimeframe,
    pool: &sqlx::PgPool,
) -> anyhow::Result<Timeframe> {
    Ok(sqlx::query_file_as!(
        Timeframe,
        "sql/timeframes/insert.sql",
        insert_timeframe.user_id,
        insert_timeframe.day,
        insert_timeframe.start,
        insert_timeframe.duration,
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get(pool: &sqlx::PgPool) -> anyhow::Result<Vec<Timeframe>> {
    Ok(sqlx::query_file_as!(Timeframe, "sql/timeframes/get.sql")
        .fetch_all(pool)
        .await?)
}

pub async fn get_by_id(id: i32, pool: &sqlx::PgPool) -> anyhow::Result<Timeframe> {
    Ok(
        sqlx::query_file_as!(Timeframe, "sql/timeframes/get_by_id.sql", id)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_by_user_id(user_id: i32, pool: &sqlx::PgPool) -> anyhow::Result<Vec<Timeframe>> {
    Ok(
        sqlx::query_file_as!(Timeframe, "sql/timeframes/get_by_user_id.sql", user_id)
            .fetch_all(pool)
            .await?,
    )
}

pub async fn delete_by_id(id: i32, pool: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query_file_as!(Timeframe, "sql/timeframes/delete_by_id.sql", id)
        .execute(pool)
        .await?;

    Ok(())
}
