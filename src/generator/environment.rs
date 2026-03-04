use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EnvConfOpts {
    Docker,
    Compose,
    #[serde(untagged)]
    Manual(EnvConfig)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvConfig {
    pub targets: TargetPaths,
    pub target_refs: TargetPaths,
    pub binds: ListenConfig,
    pub endpoints: EndpointConfig
}

/// The target directories for generated configs / other files.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TargetPaths {
    pub nginx: PathBuf,
    pub anubis: PathBuf,
    pub iocaine: PathBuf,
    pub prometheus: PathBuf,
    pub supervisord: PathBuf
}

/// The endpoints each service should listen on. Since the syntax is different
/// between services, this will look different than the targets.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListenConfig {
    pub external: String,
    pub internal: String,
    pub anubis: String,
    pub anubis_type: String,
    pub iocaine: String,
    pub prometheus: String,
    pub metrics: MetricsListenConfig
}

/// The endpoints that metrics services should listen on.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricsListenConfig {
    pub anubis: String,
    pub iocaine: String,
    pub anubis_type: String
}

/// The endpoints each service should target.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndpointConfig {
    pub iocaine: String,
    pub anubis: String,
    pub internal: String,
    #[serde(default)]
    pub use_docker_resolver: bool,
    pub metrics: MetricsEndpointConfig
}

/// targets for Prometheus to use in serving metrics.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricsEndpointConfig {
    pub anubis: String,
    pub iocaine: String
}

impl EnvConfOpts {
    pub fn config(&self) -> EnvConfig {
        match self {
            EnvConfOpts::Docker => {
                EnvConfig { 
                    targets: TargetPaths {
                        nginx: PathBuf::from("/etc/nginx"),
                        anubis: PathBuf::from("/etc/anubis"),
                        iocaine: PathBuf::from("/etc/iocaine"),
                        prometheus: PathBuf::from("/etc/prometheus"),
                        supervisord: PathBuf::from("/etc/supervisord")
                    },
                    target_refs: TargetPaths {
                        nginx: PathBuf::from("/etc/nginx"),
                        anubis: PathBuf::from("/etc/anubis"),
                        iocaine: PathBuf::from("/etc/iocaine"),
                        prometheus: PathBuf::from("/etc/prometheus"),
                        supervisord: PathBuf::from("/etc/supervisord")
                    },
                    binds: ListenConfig {
                        external: "80".to_string(),
                        iocaine: "/run/iocaine.sock".to_string(),
                        anubis: "/run/anubis.sock".to_string(),
                        anubis_type: "unix".to_string(),
                        internal: "unix:/run/internal.sock".to_string(),
                        prometheus: ":9090".to_string(),
                        metrics: MetricsListenConfig {
                            anubis: ":9902".to_string(),
                            iocaine: "127.0.0.1:9901".to_string(),
                            anubis_type: "tcp".to_string()
                        }
                    }, endpoints: EndpointConfig {
                        iocaine: "http://unix:/run/iocaine.sock".to_string(),
                        anubis: "http://unix:/run/anubis.sock".to_string(),
                        internal: "unix:///run/internal.sock".to_string(),
                        use_docker_resolver: false,
                        metrics: MetricsEndpointConfig {
                            anubis: "localhost:9901".to_string(),
                            iocaine: "localhost:9902".to_string()
                        }
                    }
                }
            },
            EnvConfOpts::Compose => {
                EnvConfig { 
                    targets: TargetPaths {
                        nginx: PathBuf::from("./docker-include/nginx"),
                        anubis: PathBuf::from("./docker-include/anubis"),
                        iocaine: PathBuf::from("./docker-include/iocaine"),
                        prometheus: PathBuf::from("./docker-include/prometheus"),
                        supervisord: PathBuf::from("./docker-include/supervisord")
                    },
                    target_refs: TargetPaths {
                        nginx: PathBuf::from("/etc/nginx"),
                        anubis: PathBuf::from("/etc/anubis"),
                        iocaine: PathBuf::from("/etc/iocaine"),
                        prometheus: PathBuf::from("/etc/prometheus"),
                        supervisord: PathBuf::from("/etc/supervisord")
                    },
                    binds: ListenConfig {
                        external: "80".to_string(),
                        iocaine: "0.0.0.0:42069".to_string(),
                        anubis: ":8080".to_string(),
                        anubis_type: "tcp".to_string(),
                        internal: "8081".to_string(),
                        prometheus: ":9090".to_string(),
                        metrics: MetricsListenConfig {
                            anubis: ":9091".to_string(),
                            iocaine: "0.0.0.0:42042".to_string(),
                            anubis_type: "tcp".to_string()
                        }
                    }, endpoints: EndpointConfig {
                        iocaine: "http://iocaine:42069".to_string(),
                        anubis: "http://anubis:8080".to_string(),
                        internal: "http://proxy:8081".to_string(),
                        use_docker_resolver: false,
                        metrics: MetricsEndpointConfig {
                            anubis: "anubis:9091".to_string(),
                            iocaine: "iocaine:42042".to_string()
                        }
                    }
                }
            },
            EnvConfOpts::Manual(cfg) => cfg.clone()
        }
    }
}