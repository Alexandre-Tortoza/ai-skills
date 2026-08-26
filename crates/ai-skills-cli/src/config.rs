use std::{
    env, fmt, fs,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Clone, Eq, PartialEq)]
pub struct Secret(String);

impl Secret {
    fn new(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    pub version: u8,
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub search: SearchConfig,
    pub embedding: Option<EmbeddingConfig>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ServerConfig {
    pub bind: SocketAddr,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct StorageConfig {
    pub path: PathBuf,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct SearchConfig {
    pub max_results: usize,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct EmbeddingConfig {
    pub provider: String,
    pub model: String,
    pub dimensions: usize,
    pub api_key: Option<Secret>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    version: Option<u8>,
    server: Option<FileServer>,
    storage: Option<FileStorage>,
    search: Option<FileSearch>,
    embedding: Option<FileEmbedding>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileServer {
    bind: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileStorage {
    path: Option<PathBuf>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileSearch {
    max_results: Option<usize>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileEmbedding {
    provider: Option<String>,
    model: Option<String>,
    dimensions: Option<usize>,
    api_key: Option<String>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, String> {
        let file = if path.exists() {
            toml::from_str(
                &fs::read_to_string(path)
                    .map_err(|error| format!("cannot read {}: {error}", path.display()))?,
            )
            .map_err(|error| format!("invalid TOML in {}: {error}", path.display()))?
        } else {
            FileConfig::default()
        };
        Self::from_file_and_env(file, |key| env::var(key).ok())
    }

    fn from_file_and_env(
        file: FileConfig,
        environment: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, String> {
        let version = environment("AI_SKILLS_VERSION")
            .or_else(|| file.version.map(|value| value.to_string()))
            .unwrap_or_else(|| "1".to_owned())
            .parse::<u8>()
            .map_err(|_| "version must be an integer".to_owned())?;
        if version != 1 {
            return Err(format!(
                "unsupported configuration version {version}; expected 1"
            ));
        }
        let bind = environment("AI_SKILLS_SERVER__BIND")
            .or_else(|| file.server.and_then(|value| value.bind))
            .unwrap_or_else(|| "127.0.0.1:8787".to_owned())
            .parse()
            .map_err(|_| "server.bind must be a valid socket address".to_owned())?;
        let path = environment("AI_SKILLS_STORAGE__PATH")
            .map(PathBuf::from)
            .or_else(|| file.storage.and_then(|value| value.path))
            .unwrap_or_else(|| PathBuf::from(".ai-skills"));
        if path.as_os_str().is_empty() {
            return Err("storage.path must not be empty".to_owned());
        }
        let max_results = environment("AI_SKILLS_SEARCH__MAX_RESULTS")
            .or_else(|| {
                file.search
                    .and_then(|value| value.max_results.map(|value| value.to_string()))
            })
            .unwrap_or_else(|| "20".to_owned())
            .parse::<usize>()
            .map_err(|_| "search.max_results must be an integer".to_owned())?;
        if max_results == 0 {
            return Err("search.max_results must be greater than zero".to_owned());
        }
        let embedding = match file.embedding {
            Some(value) => {
                let provider = environment("AI_SKILLS_EMBEDDING__PROVIDER")
                    .or(value.provider)
                    .ok_or_else(|| {
                        "embedding.provider is required when embedding is configured".to_owned()
                    })?;
                let model = environment("AI_SKILLS_EMBEDDING__MODEL")
                    .or(value.model)
                    .ok_or_else(|| {
                        "embedding.model is required when embedding is configured".to_owned()
                    })?;
                let dimensions = environment("AI_SKILLS_EMBEDDING__DIMENSIONS")
                    .or_else(|| value.dimensions.map(|value| value.to_string()))
                    .ok_or_else(|| {
                        "embedding.dimensions is required when embedding is configured".to_owned()
                    })?
                    .parse::<usize>()
                    .map_err(|_| "embedding.dimensions must be an integer".to_owned())?;
                if dimensions == 0 {
                    return Err("embedding.dimensions must be greater than zero".to_owned());
                }
                Some(EmbeddingConfig {
                    provider,
                    model,
                    dimensions,
                    api_key: environment("AI_SKILLS_EMBEDDING__API_KEY")
                        .or(value.api_key)
                        .map(Secret::new),
                })
            }
            None => None,
        };
        Ok(Self {
            version,
            server: ServerConfig { bind },
            storage: StorageConfig { path },
            search: SearchConfig { max_results },
            embedding,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_overrides_file_and_secrets_are_redacted() {
        let file: FileConfig = toml::from_str("[embedding]\nprovider = 'file'\nmodel = 'file-model'\ndimensions = 4\napi_key = 'file-secret'").unwrap();
        let config = Config::from_file_and_env(file, |key| match key {
            "AI_SKILLS_EMBEDDING__MODEL" => Some("env-model".to_owned()),
            "AI_SKILLS_EMBEDDING__API_KEY" => Some("env-secret".to_owned()),
            _ => None,
        })
        .unwrap();
        assert_eq!(config.embedding.as_ref().unwrap().model, "env-model");
        assert!(!format!("{config:#?}").contains("env-secret"));
    }
}
