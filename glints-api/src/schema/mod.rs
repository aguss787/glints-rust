mod common;
mod extensions;
mod hubber;
mod query;

use crate::APIModule;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use extensions::Logger;
use query::Query;

pub type GlintsSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[allow(dead_code)]
pub fn build(api_module: APIModule) -> GlintsSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(api_module)
        .extension(Logger)
        .finish()
}
