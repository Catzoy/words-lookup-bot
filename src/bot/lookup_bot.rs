use crate::bloc::common::LookupError;
use crate::format::LookupFormatter;
use shuttle_runtime::async_trait;

#[async_trait]
pub trait LookupBot: Clone {
    type Request: Clone + Send + Sync;
    type Formatter: LookupFormatter + Default;
    type Response: Clone + Send + Sync + Default;

    fn formatter(&self) -> Self::Formatter {
        Self::Formatter::default()
    }

    fn error_response() -> Self::Response {
        Self::Response::default()
    }

    fn empty_response() -> Self::Response {
        Self::Response::default()
    }

    async fn answer(&self, response: Self::Response) -> anyhow::Result<()>;
    async fn answer_generic_err(&self) -> anyhow::Result<()> {
        self.answer(Self::error_response()).await?;
        Ok(())
    }

    async fn drop_empty(&self, phrase: String) -> bool {
        match phrase.as_str() {
            "" => {
                let empty = Self::empty_response();
                let _ = self.answer(empty).await;
                false
            }
            _ => true,
        }
    }

    async fn ensure_request_success<Entity>(
        &self,
        response: Result<Entity, LookupError>,
    ) -> Option<Entity>
    where
        Entity: Send,
    {
        match response {
            Ok(values) => Some(values),
            Err(_) => {
                let resp = &self.answer(Self::error_response()).await;
                if let Err(e) = resp {
                    log::error!("Couldn't send error-response: {:?}", e);
                    let _ = &self.answer_generic_err().await;
                }
                None
            }
        }
    }
    async fn retrieve_or_generic_err(
        &self,
        response: Result<Self::Response, LookupError>,
    ) -> Option<Self::Response> {
        match response {
            Ok(values) => Some(values),
            Err(err) => {
                log::error!("Failed to build response: {:?}", err);
                let result = self.answer_generic_err().await;
                if let Err(e) = result {
                    log::error!("Failed to respond generic err: {:?}", e);
                }
                None
            }
        }
    }

    async fn respond(&self, response: Self::Response) -> anyhow::Result<()> {
        let res = self.answer(response).await;
        if let Err(e) = res {
            log::error!("Couldn't send response: {:?}", e);
            let _ = self.answer_generic_err().await;
        }
        Ok(())
    }
}
