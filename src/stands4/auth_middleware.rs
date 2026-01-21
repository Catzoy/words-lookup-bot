use crate::stands4::config::Stands4Config;
use axum::http::{Request, Response, Uri};
use reqwest::Url;
use rustify::errors::ClientError;
use rustify::{Endpoint, MiddleWare};
use std::str::FromStr;

impl MiddleWare for Stands4Config {
    fn request<E: Endpoint>(
        &self,
        _endpoint: &E,
        req: &mut Request<Vec<u8>>,
    ) -> Result<(), ClientError> {
        let mut url = Url::parse(req.uri().to_string().as_str())
            .map_err(|e| ClientError::UrlParseError { source: e })?;
        url.query_pairs_mut()
            .append_pair("uid", &self.user_id)
            .append_pair("tokenid", &self.token)
            .append_pair("format", &self.format);
        let uri =
            Uri::from_str(url.as_str()).map_err(|e| ClientError::UrlBuildError { source: e })?;
        *req.uri_mut() = uri;

        Ok(())
    }

    fn response<E: Endpoint>(
        &self,
        _endpoint: &E,
        _resp: &mut Response<Vec<u8>>,
    ) -> Result<(), ClientError> {
        Ok(())
    }
}
