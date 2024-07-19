use docker_api::Docker;

use crate::error::Result;

use super::ContainerEngine;

pub struct DockerServerEngine {
    docker: Docker
}

impl DockerServerEngine {
    pub fn new() -> Self {
        Self {
            docker: Docker::new(
                std::env::var("DOCKER_SOCKET").unwrap_or("tcp://localhost:2375".to_string()),
            ).expect("Failed to connect to docker daemon.")
        }
    }
}

#[async_trait::async_trait]
impl ContainerEngine for DockerServerEngine {
    async fn healthcheck(&self) -> Result<bool> {
        Ok(self.docker.version().await?.version.is_some())
    }
}
