pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub tz: String,
    pub role: i32,
}

pub struct InsertUser {
    pub username: String,
    pub password: String,
    pub name: String,
    pub tz: String,
    pub role: i32,
}

pub async fn insert(insert_user: &InsertUser, pool: &sqlx::PgPool) -> anyhow::Result<User> {
    Ok(sqlx::query_file_as!(
        User,
        "sql/users/insert.sql",
        insert_user.username,
        insert_user.password,
        insert_user.name,
        insert_user.tz,
        insert_user.role
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get(pool: &sqlx::PgPool) -> anyhow::Result<Vec<User>> {
    Ok(sqlx::query_file_as!(User, "sql/users/get.sql")
        .fetch_all(pool)
        .await?)
}

pub async fn get_by_id(id: i32, pool: &sqlx::PgPool) -> anyhow::Result<User> {
    Ok(sqlx::query_file_as!(User, "sql/users/get_by_id.sql", id)
        .fetch_one(pool)
        .await?)
}

pub async fn get_by_username(username: &str, pool: &sqlx::PgPool) -> anyhow::Result<Vec<User>> {
    Ok(
        sqlx::query_file_as!(User, "sql/users/get_by_username.sql", username)
            .fetch_all(pool)
            .await?,
    )
}

pub async fn delete_by_id(id: i32, pool: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query_file_as!(User, "sql/users/delete_by_id.sql", id)
        .execute(pool)
        .await?;

    Ok(())
}
