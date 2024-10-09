use sea_orm::ConnectOptions;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let connect_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let connect_options = ConnectOptions::new(connect_url);
    let create_database = true;
    let run_migrations = true;
    println!("⏲️ Initializing postgres...");
    blockscout_service_launcher::database::initialize_postgres::<blockscout_db::migration::Migrator>(connect_options, create_database, run_migrations).await?;
    println!("🚀 Postgres initialized");
    Ok(())
}
