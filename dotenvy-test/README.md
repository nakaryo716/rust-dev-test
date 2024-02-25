# .envファイルを読み取る
## 用途
axumなどデータベースに接続する際にデータベースURLを取得するのに使う

## 使用したクレート
**dotenvy**  
これは**dotenv**を整備したクレート  https://crates.io/crates/dotenvy
### Usage
```rust
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    Ok(())
}
```

## 注意事項
.envファイルはプロジェクトのルート(最上位)に置かないといけない  
そうしないとnotfoundとエラーになる