use crate::schema::GlintsSchema;
use actix_web::guard;
use actix_web::web::{resource, Data, ServiceConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

#[cfg(feature = "graphql-playground")]
use actix_web::HttpResponse;
#[cfg(feature = "graphql-playground")]
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

#[allow(dead_code)]
async fn index(schema: Data<GlintsSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[allow(dead_code)]
#[cfg(feature = "graphql-playground")]
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[allow(dead_code)]
pub fn configure_actix(schema: GlintsSchema) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .app_data(Data::new(schema))
            .service(resource("/").guard(guard::Post()).to(index));

        #[cfg(feature = "graphql-playground")]
        config.service(resource("/").guard(guard::Get()).to(graphql_playground));
    }
}
