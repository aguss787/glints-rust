use crate::common::{DockerEnv, GraphQLQuery, HealthConfig, NewContainerOpts, PortMapping};
use actix_web::test::TestRequest;
use log::LevelFilter::Info;
use std::future::Future;
use std::panic;

mod common;

#[test]
fn test_async_with_time() {
    fn wrapper<T, Fut>(f: T)
    where
        T: FnOnce() -> Fut,
        Fut: Future<Output = ()> + 'static,
    {
        let runtime = actix_web::rt::Runtime::new().unwrap();

        let mut docker_env = DockerEnv::new();
        let _docker_env = runtime.block_on((|| async move {
            docker_env
                .add_container(NewContainerOpts {
                    image: "postgres:14".to_string(),
                    env: vec![
                        "POSTGRES_DB=managed_talent".to_string(),
                        "POSTGRES_USER=glints".to_string(),
                        "POSTGRES_PASSWORD=glints".to_string(),
                    ],
                    ports: vec![PortMapping {
                        host_port: "5432".to_string(),
                        container_port: "5432".to_string(),
                        container_protocol: "tcp".to_string(),
                    }],
                    health_check: Some(HealthConfig {
                        test: Some(vec![
                            "CMD-SHELL".to_string(),
                            "/usr/bin/pg_isready".to_string(),
                        ]),
                        interval: Some(500 * 1000000),
                        timeout: Some(1000 * 1000000),
                        retries: Some(10),
                        start_period: Some(5000 * 1000000),
                    }),
                })
                .await;
            docker_env
        })());

        log::info!("running test");
        let result = runtime.block_on((|| async move { actix_web::rt::spawn(f()).await })());

        match result {
            Ok(_) => {}
            Err(err) => {
                if err.is_panic() {
                    panic::resume_unwind(err.into_panic());
                }
            }
        }
    }

    env_logger::builder().filter_level(Info).init();

    wrapper(|| async {
        let api_module = glints_api::APIModule::default();
        let schema = glints_api::schema::build(api_module);

        let app = actix_web::test::init_service(
            actix_web::App::new().configure(glints_api::configure_actix(schema.clone())),
        )
        .await;

        let req = TestRequest::post()
            .set_json(GraphQLQuery {
                query: "{
                    hubbers {
                        id
                        name
                        computedString
                    }
                }"
                .to_string(),
            })
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;

        log::info!("{:?}", resp);

        assert!(resp.status().is_success());
    })
}
