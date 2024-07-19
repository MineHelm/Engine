use std::sync::Arc;

use self::docker::DockerServerEngine;
use crate::error::Result;

mod docker;

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum ContainerEngineKind {
    #[default] Docker,
    Kubernetes
}

#[derive(Clone, derive_more::Deref)]
pub struct ServerEngine {
    #[deref]
    pub(crate) engine: Arc<dyn ContainerEngine + Send + Sync>
}

impl ServerEngine {
    pub fn new(kind: ContainerEngineKind) -> Self {
        Self {
            engine: Arc::new(match kind {
                ContainerEngineKind::Docker => DockerServerEngine::new(),
                ContainerEngineKind::Kubernetes => unimplemented!("Kubernetes is not supported yet")
            })
        }
    }
}

#[async_trait::async_trait]
pub trait ContainerEngine {
    async fn healthcheck(&self) -> Result<bool>;
}
