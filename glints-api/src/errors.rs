use anyhow::anyhow;
use glints_managed_talent::hubber::errors::HubberError;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone)]
pub enum GraphQlError {
    #[error("internal server error")]
    InternalServerError {
        #[from]
        source: Arc<anyhow::Error>,
    },
}

impl From<HubberError> for GraphQlError {
    fn from(value: HubberError) -> Self {
        GraphQlError::InternalServerError {
            source: Arc::new(anyhow!(value)),
        }
    }
}

#[derive(Clone)]
pub struct GraphQlErrorWrapper(GraphQlError);

impl<T: Into<GraphQlError>> From<T> for GraphQlErrorWrapper {
    fn from(value: T) -> Self {
        GraphQlErrorWrapper(value.into())
    }
}

impl From<GraphQlErrorWrapper> for async_graphql::Error {
    fn from(value: GraphQlErrorWrapper) -> Self {
        async_graphql::Error::new_with_source(value.0)
    }
}
