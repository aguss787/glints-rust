use crate::common::docker::{DockerEnv, NewContainerOpts, PortMapping};
use crate::common::ports::allocate_port;
pub use bollard::models::HealthConfig;
use diesel::migration::MigrationSource;
use log::LevelFilter;
use serde::Serialize;
use std::future::Future;

mod docker;
mod ports;

#[derive(Serialize)]
pub struct GraphQLQuery {
    pub query: String,
}

static INITIALIZE_LOGGER: std::sync::Once = std::sync::Once::new();

struct Ports {
    postgres: usize,
}

impl Ports {
    fn new() -> Self {
        Ports {
            postgres: allocate_port(),
        }
    }
}

async fn setup_docker(mut docker_env: DockerEnv, ports: &Ports) -> DockerEnv {
    docker_env
        .add_container(NewContainerOpts {
            image: "postgres:14".to_string(),
            env: vec![
                "POSTGRES_DB=managed_talent".to_string(),
                "POSTGRES_USER=glints".to_string(),
                "POSTGRES_PASSWORD=glints".to_string(),
            ],
            ports: vec![PortMapping {
                host_port: ports.postgres.to_string(),
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
        .await
        .expect("failed to prepare docker container");

    docker_env
}

pub fn docker_test<T, Fut>(f: T)
where
    T: FnOnce() -> Fut,
    Fut: Future<Output = ()> + 'static,
{
    INITIALIZE_LOGGER.call_once(|| {
        env_logger::builder().filter_level(LevelFilter::Info).init();
    });

    let ports = Ports::new();
    let runtime = actix_web::rt::Runtime::new().expect("failed to initialize async runtime");

    glints_config::tests::overwrite_config_for_current_thread(|config| {
        config.postgres.database_url = format!(
            "postgresql://glints:glints@localhost:{}/managed_talent",
            ports.postgres,
        );
    });

    let docker_env = DockerEnv::new().expect("failed to initialize docker env");
    let _docker_env = runtime.block_on(setup_docker(docker_env, &ports));

    {
        log::debug!("initializing database");
        // TODO: Remove the hard coded path
        let migrations =
            diesel_migrations::FileBasedMigrations::from_path("../glints-infra/migrations")
                .expect("failed to initialize diesel migration");

        use diesel::pg::PgConnection;
        use diesel::prelude::*;
        let mut conn = PgConnection::establish(&format!(
            "postgresql://glints:glints@localhost:{}/managed_talent",
            ports.postgres
        ))
        .expect("failed to connect to database for migration");

        let migrations = migrations
            .migrations()
            .expect("failed to get diesel migrations");
        for m in migrations {
            m.run(&mut conn).expect("failed to run migration");
        }

        // TODO: seed data
    }

    log::debug!("running test");
    let result = runtime.block_on((|| async move { actix_web::rt::spawn(f()).await })());

    match result {
        Ok(_) => {}
        Err(err) => {
            if err.is_panic() {
                std::panic::resume_unwind(err.into_panic());
            }
        }
    }
}
