use glints_infra::errors::InfraError;

#[derive(thiserror::Error, Debug)]
pub enum HubberError {
    #[error("database error: {source}")]
    DatabaseError {
        #[from]
        source: diesel::result::Error,
    },

    #[error("infra error: {source}")]
    InfraError {
        #[from]
        source: InfraError,
    },
}
