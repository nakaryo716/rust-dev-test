use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .envファイルは一番上のディレクトリに置かないと読み込めなかった
    dotenvy::dotenv()?;

    let response = env::var("DATABASE_URL")?;
    let response2 = env::var("API_KEY")?;

    // 出力: postgres://localhost:5432/mydb
    println!("{}", &response);

    // 出力: your_api_key
    println!("{}", &response2);

    Ok(())
}
