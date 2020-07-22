// use diesel::pg::PgConnection;
    // use diesel::r2d2::ConnectionManager;
    // use r2d2::{Pool, PooledConnection};

use sqlx::MySqlPool;
/// A database "repository", for running database workloads.
#[derive(Clone, Debug)]
pub struct Repo {
    pub connection_pool: MySqlPool,
    // pg_connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl Repo {
    /// Creates a repo using default configuration for the underlying connection pool.
    pub async fn new(database_url: &str) -> Self {
        Self::from_pool_builder(&database_url).await
    }

    // Creates a repo with a pool builder, allowing you to customize
    // any connection pool configuration.
    pub async fn from_pool_builder(
        database_url: &str,
    ) -> Self {
        let connection_pool = MySqlPool::builder().max_size(5).min_size(1).build(&database_url).await.expect("init database error");
        
        //todo: 删除
        // let manager = ConnectionManager::new(database_url);
        // let pg_connection_pool = r2d2::Builder::default()
        //     .build(manager)
        //     .expect("could not initiate test db pool");
        Repo { connection_pool/*, pg_connection_pool*/ }
    }

    // todo: 删除
    // pub fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
    //     self.pg_connection_pool.get().unwrap()
    // }

}
