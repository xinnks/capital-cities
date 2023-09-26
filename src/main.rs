extern crate dotenv;

use dotenv::dotenv;
use dotenv_codegen::dotenv;

use libsql::{params, Database};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_path = dotenv!("DB_PATH");
    let sync_url = dotenv!("TURSO_SYNC_URL");
    let auth_token = dotenv!("TURSO_AUTH_TOKEN");

    println!(
        "Here are the env vars: db_path: {}, auth_token: {}, sync_url: {}",
        db_path, auth_token, sync_url
    );

    let db = Database::open_with_remote_sync(db_path, sync_url, auth_token)
        .await
        .unwrap();
    db.sync().await.unwrap();

    let conn = db.connect().unwrap();

    conn.execute(
        "CREATE TABLE capital_cities (country VARCHAR, capital VARCHAR)",
        (),
    )
    .await
    .unwrap();

    conn.execute(
        "INSERT INTO capital_cities VALUES (?, ?)",
        params!["foo", "bar"],
    )
    .await
    .unwrap();

    let rows = conn
        .execute(
            "SELECT * FROM capital_cities where capibal = ?",
            params!["bar"],
        )
        .await
        .unwrap();
    println!("rows: {}", rows);
}
