use crate::common::GraphQLQuery;
use actix_web::body::to_bytes;
use actix_web::test::TestRequest;
use serde::Deserialize;

mod common;

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct GraphQLResponse<T> {
    data: T,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct HubberResponse {
    hubbers: Vec<Hubber>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Hubber {
    code: String,
    name: String,
}

#[test]
fn test_get_all_hubbers() {
    common::docker_test(|| async {
        let api_module = glints_api::APIModule::default();
        let schema = glints_api::schema::build(api_module);

        let app = actix_web::test::init_service(
            actix_web::App::new().configure(glints_api::graphql::configure_actix(schema.clone())),
        )
        .await;

        let req = TestRequest::post()
            .set_json(GraphQLQuery {
                query: "{
                    hubbers {
                        name
                        code
                    }
                }"
                .to_string(),
            })
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        let resp: GraphQLResponse<HubberResponse> =
            serde_json::from_slice(&body_bytes[..]).unwrap();

        assert_eq!(
            resp,
            GraphQLResponse {
                data: HubberResponse {
                    hubbers: vec![
                        Hubber {
                            code: "GLID-EX-1".to_string(),
                            name: "CAT".to_string(),
                        },
                        Hubber {
                            code: "GLID-EX-2".to_string(),
                            name: "DOG".to_string(),
                        }
                    ]
                }
            }
        );
    })
}

#[test]
fn test_2() {
    test_get_all_hubbers();
}

#[test]
fn test_3() {
    test_get_all_hubbers();
}

#[test]
fn test_4() {
    test_get_all_hubbers();
}

#[test]
fn test_5() {
    test_get_all_hubbers();
}
