#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum ContainerEngineKind {
    #[default] Docker,
    Kubernetes
}
