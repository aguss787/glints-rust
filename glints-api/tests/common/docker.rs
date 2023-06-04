use crate::common::HealthConfig;
use bollard::container::{Config, RemoveContainerOptions, StartContainerOptions};
use bollard::models::{HealthStatusEnum, HostConfig, PortBinding};
use bollard::Docker;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum ContainerInitializationError {
    #[error("error from docker engine: {0}")]
    DockerError(bollard::errors::Error),
    #[error("container is not ready in time")]
    HealthCheckTimeout,
}

impl From<bollard::errors::Error> for ContainerInitializationError {
    fn from(value: bollard::errors::Error) -> Self {
        ContainerInitializationError::DockerError(value)
    }
}

struct Container {
    docker: Arc<Docker>,
    id: String,
}

impl Drop for Container {
    fn drop(&mut self) {
        log::debug!("dropping a container wrapper");

        let docker = self.docker.clone();
        let id = self.id.clone();

        std::thread::spawn(|| {
            let runtime = actix_web::rt::Runtime::new()
                .expect("unable to build runtime for container destructor");

            runtime.block_on((|| async move {
                log::debug!("removing container {}", id);
                let resp = docker
                    .remove_container(
                        &id,
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
        })
        .join()
        .expect("unable to join destructor thread");
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
    async fn new(
        docker: Arc<Docker>,
        opts: NewContainerOpts,
    ) -> Result<Container, ContainerInitializationError> {
        log::debug!("creating container");
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
            .await?;

        log::debug!("container created");
        let container = Container {
            docker,
            id: response.id,
        };

        log::debug!("starting container");
        container
            .docker
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await?;

        log::debug!("waiting for container to be ready...");
        container
            .wait_until_ready(std::time::Duration::from_secs(30))
            .await?;

        log::debug!("container ready!");

        Ok(container)
    }

    async fn wait_until_ready(
        &self,
        time_limit: std::time::Duration,
    ) -> Result<(), ContainerInitializationError> {
        let start_time = std::time::Instant::now();
        while std::time::Instant::now() - start_time <= time_limit {
            let resp = self.docker.inspect_container(&self.id, None).await?;

            let health = resp.state.and_then(|o| o.health).and_then(|o| o.status);
            match health {
                None => {
                    log::warn!("missing health info for container {}", self.id);
                    return Ok(());
                }
                Some(health) => match health {
                    HealthStatusEnum::HEALTHY => return Ok(()),
                    _ => actix_web::rt::time::sleep(std::time::Duration::from_millis(100)).await,
                },
            }
        }

        Err(ContainerInitializationError::HealthCheckTimeout)
    }
}

pub struct DockerEnv {
    docker: Arc<Docker>,
    containers: Vec<Container>,
}

#[derive(thiserror::Error, Debug)]
pub enum DockerEnvInitializationError {
    #[error("error from docker engine: {0}")]
    DockerError(bollard::errors::Error),
}

impl From<bollard::errors::Error> for DockerEnvInitializationError {
    fn from(value: bollard::errors::Error) -> Self {
        DockerEnvInitializationError::DockerError(value)
    }
}

impl DockerEnv {
    pub fn new() -> Result<Self, DockerEnvInitializationError> {
        log::debug!("starting testing docker env");
        let docker = Docker::connect_with_socket_defaults()?;

        Ok(DockerEnv {
            docker: Arc::new(docker),
            containers: vec![],
        })
    }

    pub async fn add_container(
        &mut self,
        opts: NewContainerOpts,
    ) -> Result<(), ContainerInitializationError> {
        Ok(self
            .containers
            .push(Container::new(self.docker.clone(), opts).await?))
    }
}
