use crate::bloc::common::LookupError;
use crate::format::LookupFormatter;
use shuttle_runtime::async_trait;

#[async_trait]
pub trait LookupBot: Clone {
    type Request: Clone + Send + Sync;
    type Formatter: LookupFormatter + Default;
    type Response: Clone + Send + Sync;
    async fn answer_generic_err(&self) -> anyhow::Result<()>;
    async fn answer(&self, response: Self::Response) -> anyhow::Result<()>;
    async fn drop_empty(&self, phrase: String) -> bool;
    async fn ensure_request_success<Entity>(
        &self,
        response: Result<Entity, LookupError>,
    ) -> Option<Entity>
    where
        Entity: Send;
    async fn retrieve_or_generic_err(
        &self,
        response: Result<Self::Response, LookupError>,
    ) -> Option<Self::Response>;

    async fn respond(&self, response: Self::Response) -> anyhow::Result<()>;
    fn formatter(&self) -> Self::Formatter {
        Self::Formatter::default()
    }
}
