use crate::stands4::config::Stands4Config;
use axum::http::{Request, Response, Uri};
use reqwest::Url;
use rustify::errors::ClientError;
use rustify::{Endpoint, MiddleWare};
use std::str::FromStr;

impl MiddleWare for Stands4Config {
    /// Adds Stands4 authentication and response-format query parameters to the request URI.
    ///
    /// This method parses the request URI, appends the `uid`, `tokenid`, and `format` query
    /// parameters taken from the `Stands4Config`, and replaces the request's URI with the
    /// updated value.
    ///
    /// Returns `Ok(())` on success; returns `ClientError::UrlParseError` if the original URI
    /// cannot be parsed as a `Url`, or `ClientError::UrlBuildError` if the modified URL
    /// cannot be converted back into a `Uri`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustify::Request;
    /// use http::Uri;
    ///
    /// // Assuming `cfg` is a Stands4Config with fields `user_id`, `token`, and `format`.
    /// let mut req = Request::builder()
    ///     .uri(Uri::from_static("https://api.example.com/lookup"))
    ///     .body(Vec::new())
    ///     .unwrap();
    ///
    /// // cfg.request(&endpoint, &mut req).unwrap();
    /// // After calling, req.uri() will include `?uid=...&tokenid=...&format=...`
    /// ```
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

    /// Ensures the response has a non-empty body by replacing an empty body with the JSON object `{}`.
    ///
    /// If the response body is empty, it is replaced with the bytes of the string `"{}"`. Otherwise
    /// the response is left unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustify::response::Response;
    /// use stands4::Stands4Config;
    ///
    /// let config = Stands4Config::default();
    /// let mut resp = Response::new(Vec::new());
    ///
    /// // After middleware runs, empty body becomes "{}"
    /// config.response(&(), &mut resp).unwrap();
    /// assert_eq!(resp.body(), b"{}");
    /// ```
    fn response<E: Endpoint>(
        &self,
        _endpoint: &E,
        resp: &mut Response<Vec<u8>>,
    ) -> Result<(), ClientError> {
        if resp.body().is_empty() {
            let empty_json = "{}".to_string();
            resp.body_mut().clone_from_slice(empty_json.as_bytes());
        }
        Ok(())
    }
}