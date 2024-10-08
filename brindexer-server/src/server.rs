use crate::{
    proto::{health_actix::route_health, health_server::HealthServer},
    services::HealthService,
    settings::Settings,
};
use blockscout_service_launcher::{launcher, launcher::LaunchSettings, tracing};
use brindexer_logic::{IndexerRuntime, RpcClient};
use sea_orm::{ConnectOptions, Database};

use std::sync::Arc;

const SERVICE_NAME: &str = "brindexer";

#[derive(Clone)]
struct Router {
    // TODO: add services here
    health: Arc<HealthService>,
}

impl Router {
    pub fn grpc_router(&self) -> tonic::transport::server::Router {
        tonic::transport::Server::builder().add_service(HealthServer::from_arc(self.health.clone()))
    }
}

impl launcher::HttpRouter for Router {
    fn register_routes(&self, service_config: &mut actix_web::web::ServiceConfig) {
        service_config.configure(|config| route_health(config, self.health.clone()));
    }
}

pub async fn run(settings: Settings) -> Result<(), anyhow::Error> {
    tracing::init_logs(SERVICE_NAME, &settings.tracing, &settings.jaeger)?;

    let health = Arc::new(HealthService::default());

    let create_database_options = ConnectOptions::new(settings.database.connect.url())
        .sqlx_logging(false)
        .to_owned();
    let db = Arc::new(Database::connect(create_database_options).await?);

    let rpc = Arc::new(RpcClient::from_url(settings.rpc.url.parse().unwrap()));

    let mut indexer_runtime = IndexerRuntime::init(db.clone(), rpc.clone()).await?;
    indexer_runtime.add_all_jobs(&settings.indexers).await?;
    indexer_runtime.run_background().await?;

    let router = Router { health };

    let grpc_router = router.grpc_router();
    let http_router = router;

    let launch_settings = LaunchSettings {
        service_name: SERVICE_NAME.to_string(),
        server: settings.server,
        metrics: settings.metrics,
    };

    launcher::launch(&launch_settings, http_router, grpc_router).await
}
