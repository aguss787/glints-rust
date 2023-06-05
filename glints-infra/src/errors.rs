#[derive(thiserror::Error, Debug)]
pub enum InfraError {
    #[error("database connection pool error: {source}")]
    DatabaseConnectionPoolError { source: anyhow::Error },
}
