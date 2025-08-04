#![deny(clippy::all)]

use anyhow::{Context, Result};
use atlas_local_sdk::Client as AtlasLocalClient;
use bollard::Docker;
use napi_derive::napi;

#[napi]
pub struct Client {
  client: AtlasLocalClient,
}

#[napi]
impl Client {
  #[napi(factory)]
  pub fn connect() -> Result<Client> {
    let docker = Docker::connect_with_defaults().context("connect to docker")?;

    let atlas_local_client = AtlasLocalClient::new(docker);

    Ok(Client {
      client: atlas_local_client,
    })
  }

  #[napi]
  pub async fn list_deployments(&self) -> Result<Vec<Deployment>> {
    self
      .client
      .list_deployments()
      .await
      .context("list deployments")
      .map(|deployments| deployments.into_iter().map(|d| d.into()).collect())
  }
}

#[napi]
#[derive(Debug)]
pub struct Deployment {
  pub container_id: String,
  pub creation_source: Option<CreationSource>,
}

impl From<atlas_local_sdk::Deployment> for Deployment {
  fn from(deployment: atlas_local_sdk::Deployment) -> Self {
    Deployment {
      container_id: deployment.container_id,
      creation_source: deployment.creation_source.map(|s| s.into()),
    }
  }
}

#[napi]
#[derive(Debug)]
pub enum CreationSource {
  AtlasCLI,
}

impl From<atlas_local_sdk::CreationSource> for CreationSource {
  fn from(source: atlas_local_sdk::CreationSource) -> Self {
    match source {
      atlas_local_sdk::CreationSource::AtlasCLI => CreationSource::AtlasCLI,
    }
  }
}
