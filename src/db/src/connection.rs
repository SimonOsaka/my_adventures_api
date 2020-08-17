use sqlx::MySqlPool;
/// A database "repository", for running database workloads.
#[derive(Clone, Debug)]
pub struct Repo {
    pub connection_pool: MySqlPool,
}

impl Repo {
    /// Creates a repo using default configuration for the underlying connection pool.
    pub async fn new(database_url: &str) -> Self {
        Self::from_pool_builder(&database_url).await
    }

    // Creates a repo with a pool builder, allowing you to customize
    // any connection pool configuration.
    pub async fn from_pool_builder(database_url: &str) -> Self {
        let connection_pool = MySqlPool::builder()
            .max_size(5)
            .min_size(1)
            .build(&database_url)
            .await
            .expect("init database error");
        Repo { connection_pool }
    }
}
