use std::{env, error::Error};
use sqlx::{prelude::FromRow, PgPool};
use tokio::time::{self, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&url).await?;

    let born_ymd = sqlx::types::chrono::NaiveDate::from_ymd_opt(2003, 7, 16).unwrap();

    let payload = CreateUser {
        name: "りょう".to_string(),
        date: born_ymd,
    };

    let res1 = insert(&pool, payload).await?;
    println!("{:#?}", res1);

    sleep(time::Duration::from_secs(4)).await;

    let res2 = query(&pool, 1).await?;
    println!("{:#?}", res2);

    Ok(())
}

#[derive(Debug, FromRow)]
pub struct User {
    id: i32,
    name: String,
    date: sqlx::types::chrono::NaiveDate,
}

pub struct CreateUser {
    name: String,
    date: sqlx::types::chrono::NaiveDate,
}

async fn insert(pool: &PgPool, payload: CreateUser) -> Result<User, Box<dyn Error>> {
    let user = sqlx::query_as::<_, User>(
        r#"
INSERT INTO user_info (name, date)
VALUES ($1, $2)
RETURNING *
    "#,
    )
    .bind(payload.name)
    .bind(payload.date)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

async fn query(pool: &PgPool, id: i32) -> Result<User, Box<dyn Error>> {
    let user = sqlx::query_as::<_, User>(
        r#"
SELECT * FROM user_info
WHERE id = $1
    "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}
