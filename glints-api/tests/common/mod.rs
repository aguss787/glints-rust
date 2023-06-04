use bollard::container::{Config, RemoveContainerOptions, StartContainerOptions};
use bollard::models::{HealthStatusEnum, PortBinding};
use bollard::secret::HostConfig;
use bollard::Docker;
use serde::Serialize;
use std::collections::HashMap;
use std::rc::Rc;

pub use bollard::models::HealthConfig;

struct Container {
    docker: Rc<Docker>,
    id: String,
}

impl Drop for Container {
    fn drop(&mut self) {
        let runtime = actix_web::rt::Runtime::new().expect("unable to build runtime");

        runtime.block_on((|| async move {
            let _y = self.docker.inspect_container(&self.id, None).await;
            log::info!("{:?}", _y.unwrap().state.unwrap());

            log::info!("removing container {}", self.id);
            let resp = self
                .docker
                .remove_container(
                    &self.id,
                    Some(RemoveContainerOptions {
                        v: true,
                        force: true,
                        link: false,
                    }),
                )
                .await;

            if let Err(err) = resp {
                log::error!("{}", err.to_string());
            }
        })());
        // self.docker.stop_container()
    }
}

pub struct NewContainerOpts {
    pub image: String,
    pub env: Vec<String>,
    pub ports: Vec<PortMapping>,
    pub health_check: Option<HealthConfig>,
}

pub struct PortMapping {
    pub container_port: String,
    pub container_protocol: String,
    pub host_port: String,
}

impl Container {
    async fn new(docker: Rc<Docker>, opts: NewContainerOpts) -> Container {
        log::info!("starting container");
        let response = docker
            .create_container::<&str, String>(
                None,
                Config {
                    image: Some(opts.image),
                    env: Some(opts.env),
                    exposed_ports: Some(HashMap::from_iter(opts.ports.iter().map(|p| {
                        (
                            format!("{}/{}", p.container_port, p.container_protocol),
                            HashMap::new(),
                        )
                    }))),
                    host_config: Some(HostConfig {
                        port_bindings: Some(HashMap::from_iter(opts.ports.into_iter().map(|p| {
                            (
                                format!("{}/{}", p.container_port, p.container_protocol),
                                Some(vec![PortBinding {
                                    host_port: Some(p.host_port),
                                    host_ip: None,
                                }]),
                            )
                        }))),
                        ..Default::default()
                    }),
                    healthcheck: opts.health_check,
                    ..Default::default()
                },
            )
            .await
            .expect("todo: unable to create container");

        docker
            .start_container(&response.id, None::<StartContainerOptions<String>>)
            .await
            .expect("unable to start container");

        let container = Container {
            docker,
            id: response.id,
        };

        container
            .wait_until_ready(std::time::Duration::from_secs(30))
            .await;
        container
    }

    async fn wait_until_ready(&self, time_limit: std::time::Duration) {
        let start_time = std::time::Instant::now();
        while std::time::Instant::now() - start_time <= time_limit {
            let resp = self
                .docker
                .inspect_container(&self.id, None)
                .await
                .expect("failed to get status");

            let health = resp.state.and_then(|o| o.health).and_then(|o| o.status);
            log::info!("{:?}", health);
            match health {
                None => {
                    log::warn!("missing health info for container {}", self.id);
                    return;
                }
                Some(health) => match health {
                    HealthStatusEnum::HEALTHY => return,
                    _ => actix_web::rt::time::sleep(std::time::Duration::from_millis(100)).await,
                },
            }
        }
    }
}

pub struct DockerEnv {
    docker: Rc<Docker>,
    containers: Vec<Container>,
}

impl DockerEnv {
    pub fn new() -> Self {
        log::info!("starting testing docker env");
        let docker = Docker::connect_with_socket_defaults().expect("unable to build docker client");

        DockerEnv {
            docker: Rc::new(docker),
            containers: vec![],
        }
    }

    pub async fn add_container(&mut self, opts: NewContainerOpts) {
        self.containers
            .push(Container::new(self.docker.clone(), opts).await);
    }
}

#[derive(Serialize)]
pub struct GraphQLQuery {
    pub query: String,
}
