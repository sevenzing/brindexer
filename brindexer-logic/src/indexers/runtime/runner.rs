use super::{IndexerJob, IndexerJobContext};
use crate::{indexers::token::TokenDataJob, rpc::RpcClient};
use derive_new::new;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

#[derive(new)]
pub struct IndexerRuntime {
    scheduler: JobScheduler,
    db: Arc<DatabaseConnection>,
    rpc: Arc<RpcClient>,
}

impl IndexerRuntime {
    pub async fn init(
        db: Arc<DatabaseConnection>,
        rpc: Arc<RpcClient>,
    ) -> Result<Self, anyhow::Error> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self::new(scheduler, db, rpc))
    }

    pub async fn add_job(&self, job: Arc<dyn IndexerJob>) -> Result<(), JobSchedulerError> {
        let schedule = job.schedule();
        let db = self.db.clone();
        let rpc = self.rpc.clone();
        let mutex = Arc::new(Mutex::new(false));
        let job = Job::new_async(schedule, move |uuid, mut _l| {
            let job = job.clone();
            let mut ctx = IndexerJobContext::from_db_rpc(db.clone(), rpc.clone());
            let mutex = mutex.clone();
            Box::pin(async move {
                tracing::debug!("uuid: {}", uuid);
                let _guard = match mutex.try_lock() {
                    Ok(guard) => guard,
                    Err(_) => {
                        return;
                    }
                };
                for attempt in 0..job.retries() {
                    ctx.with_retries(attempt);
                    match job.execute(&ctx).await {
                        Ok(_) => break,
                        Err(e) => {
                            tracing::error!("job failed: {:?}", e);
                            tokio::time::sleep(job.retry_interval()).await;
                        }
                    }
                }
            })
        })?;
        self.scheduler.add(job).await?;

        Ok(())
    }

    pub async fn add_jobs(&self, jobs: Vec<Arc<dyn IndexerJob>>) -> Result<(), JobSchedulerError> {
        for job in jobs {
            self.add_job(job).await?;
        }
        Ok(())
    }

    pub async fn add_all_jobs(&self) -> Result<(), JobSchedulerError> {
        let jobs = vec![Arc::new(TokenDataJob::new(50)) as Arc<dyn IndexerJob>];
        self.add_jobs(jobs).await
    }

    pub async fn run_background(&mut self) -> Result<(), JobSchedulerError> {
        // Feature 'signal' must be enabled
        self.scheduler.shutdown_on_ctrl_c();

        // Add code to be run during/after shutdown
        self.scheduler.set_shutdown_handler(Box::new(|| {
            Box::pin(async move {
                println!("Shut down done");
            })
        }));

        // Start the scheduler
        self.scheduler.start().await?;

        Ok(())
    }
}

// mod tests {
//     use crate::IndexerJobError;

//     use super::*;

//     struct MockJob;

//     #[async_trait::async_trait]
//     impl IndexerJob for MockJob {
//         fn name(&self) -> &'static str {
//             "mock_job"
//         }

//         fn schedule(&self) -> &'static str {
//             "every 1 second"
//         }

//         async fn execute(&self, ctx: &IndexerJobContext) -> Result<(), IndexerJobError> {
//             if ctx.retries == 0 {
//                 Err(anyhow::anyhow!("failed"))?
//             } else {
//                 Ok(())
//             }
//         }

//     }
//     #[tokio::test]
//     async fn test_add_job() {
//         let db = Arc::new(DatabaseConnection::new());
//         let mut runner = IndexerRuntime::init(db).await.unwrap();

//     }
// }
