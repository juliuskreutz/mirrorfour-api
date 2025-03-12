#[repr(u8)]
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Admin,
    User,
}

impl Role {
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(value: u8) -> anyhow::Result<Self> {
        match value {
            0 => Ok(Role::Admin),
            1 => Ok(Role::User),
            _ => Err(anyhow::anyhow!("Invalid value")),
        }
    }
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    #[schema(value_type = String)]
    pub tz: chrono_tz::Tz,
    pub role: Role,
}

impl From<crate::db::users::User> for User {
    fn from(user: crate::db::users::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            name: user.name,
            tz: user.tz.parse().unwrap(),
            role: Role::from_u8(user.role as u8).unwrap(),
        }
    }
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct Timeframe {
    pub id: i32,
    pub user_id: i32,
    pub day: i32,
    pub start: chrono::NaiveTime,
    pub duration: i32,
}

impl From<crate::db::timeframes::Timeframe> for Timeframe {
    fn from(timeframe: crate::db::timeframes::Timeframe) -> Self {
        Self {
            id: timeframe.id,
            user_id: timeframe.user_id,
            day: timeframe.day,
            start: timeframe.start,
            duration: timeframe.duration,
        }
    }
}
