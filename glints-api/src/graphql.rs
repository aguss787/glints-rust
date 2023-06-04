use crate::schema::Query;
use actix_web::web::Data;

#[cfg(feature = "graphql-playground")]
use actix_web::HttpResponse;
#[cfg(feature = "graphql-playground")]
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

#[allow(dead_code)]
pub async fn index(
    schema: Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[allow(dead_code)]
#[cfg(feature = "graphql-playground")]
pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}
