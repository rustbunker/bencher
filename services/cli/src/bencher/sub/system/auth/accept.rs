use std::convert::TryFrom;

use async_trait::async_trait;
use bencher_client::types::JsonAccept;
use bencher_json::Jwt;

use crate::{
    bencher::{backend::Backend, sub::SubCmd},
    parser::system::auth::CliAuthAccept,
    CliError,
};

#[derive(Debug, Clone)]
pub struct Accept {
    pub invite: Jwt,
    pub backend: Backend,
}

impl TryFrom<CliAuthAccept> for Accept {
    type Error = CliError;

    fn try_from(accept: CliAuthAccept) -> Result<Self, Self::Error> {
        let CliAuthAccept { invite, backend } = accept;
        Ok(Self {
            invite,
            backend: backend.try_into()?,
        })
    }
}

impl From<Accept> for JsonAccept {
    fn from(accept: Accept) -> Self {
        let Accept { invite, .. } = accept;
        Self {
            invite: invite.into(),
        }
    }
}

#[async_trait]
impl SubCmd for Accept {
    async fn exec(&self) -> Result<(), CliError> {
        let _json = self
            .backend
            .send(|client| async move { client.auth_accept_post().body(self.clone()).send().await })
            .await?;
        Ok(())
    }
}