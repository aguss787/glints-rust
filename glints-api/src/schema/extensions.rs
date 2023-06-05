use crate::errors::GraphQlError;
use async_graphql::extensions::{
    Extension, ExtensionContext, ExtensionFactory, NextResolve, ResolveInfo,
};
use async_graphql::ServerResult;
use std::sync::Arc;

pub struct Logger;

impl ExtensionFactory for Logger {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(Logger)
    }
}

#[async_trait::async_trait]
impl Extension for Logger {
    async fn resolve(
        &self,
        ctx: &ExtensionContext<'_>,
        info: ResolveInfo<'_>,
        next: NextResolve<'_>,
    ) -> ServerResult<Option<async_graphql::Value>> {
        let result = next.run(ctx, info).await;

        // Try to downcast the source error
        // TODO: breakdown this nested hell
        if let Err(err) = &result {
            if let Some(err) = &err.source {
                if let Some(err) = err.downcast_ref::<GraphQlError>() {
                    match err {
                        GraphQlError::InternalServerError { source } => {
                            log::error!("internal server error: {}", source)
                        }
                    }
                }
            }
        }

        result
    }
}
