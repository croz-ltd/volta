use crate::config::Settings;
use crate::error::{ErrorKind, VoltaError};
use log::debug;
use std::io::Error;
use url::{ParseError, Url};

pub struct NodeSource {
    base: Url,
}

pub struct RegistrySource {
    base: Url,
}

impl NodeSource {
    pub fn from_settings(settings: &Settings) -> Result<Self, VoltaError> {
        let default_base = "https://nodejs.org/dist";
        let base_str = settings.mirrors.node.as_deref().unwrap_or(default_base);

        let mut base = Url::parse(base_str).map_err(|err| {
            VoltaError::from(ErrorKind::ConfigError {
                message: format!("Invalid VOLTA_NODE_DIST_BASE '{}': {}", base_str, err),
            })
        })?;

        // normalize trailing slash for safe join
        if !base.as_str().ends_with('/') {
            base = Url::parse(&(base.as_str().to_owned() + "/")).map_err(|err| {
                VoltaError::from(ErrorKind::ConfigError {
                    message: format!("Invalid VOLTA_NODE_DIST_BASE '{}': {}", base_str, err),
                })
            })?;
        }
        debug!("Using base {}", base);
        Ok(Self { base })
    }

    pub fn base_url(&self) -> Result<Url, Error> {
        Ok(self.base.clone())
    }

    pub fn index_url(&self) -> Result<Url, ParseError> {
        self.base.join("index.json")
    }
}

impl RegistrySource {
    pub fn from_settings(settings: &Settings) -> Result<Self, VoltaError> {
        let default_base = "https://registry.npmjs.org/";
        let base_str = settings.mirrors.registry.as_deref().unwrap_or(default_base);

        let mut base = Url::parse(base_str).map_err(|err| {
            VoltaError::from(ErrorKind::ConfigError {
                message: format!("Invalid VOLTA_REGISTRY_BASE '{}': {}", base_str, err),
            })
        })?;
        // normalize trailing slash for safe join
        if !base.as_str().ends_with('/') {
            base = Url::parse(&(base.as_str().to_owned() + "/")).map_err(|err| {
                VoltaError::from(ErrorKind::ConfigError {
                    message: format!("Invalid VOLTA_REGISTRY_BASE '{}': {}", base_str, err),
                })
            })?;
        }
        debug!("Using base {}", base);
        Ok(Self { base })
    }

    pub fn base_url(&self) -> Result<Url, Error> {
        Ok(self.base.clone())
    }
}

#[test]
fn node_source_default() {
    let s = Settings::default();
    let src = NodeSource::from_settings(&s).unwrap();
    assert_eq!(
        src.index_url().unwrap().as_str(),
        "https://nodejs.org/dist/index.json"
    );
}

#[test]
fn node_source_env_override() {
    std::env::set_var("VOLTA_NODE_DIST_BASE", "https://mirror.local/nodejs/dist");
    let s = crate::config::load::load_settings().unwrap();
    let src = NodeSource::from_settings(&s).unwrap();
    assert_eq!(
        src.index_url().unwrap().as_str(),
        "https://mirror.local/nodejs/dist/index.json"
    );
}

#[test]
fn registry_source_default() {
    let s = Settings::default();
    let src = RegistrySource::from_settings(&s).unwrap();
    assert_eq!(
        src.base_url().unwrap().as_str(),
        "https://registry.npmjs.org/"
    );
}

#[test]
fn registry_source_env_override() {
    std::env::set_var("VOLTA_REGISTRY_BASE", "https://registry2.npmjs.org/");
    let s = crate::config::load::load_settings().unwrap();
    let src = RegistrySource::from_settings(&s).unwrap();
    assert_eq!(
        src.base_url().unwrap().as_str(),
        "https://registry2.npmjs.org/"
    );
}
