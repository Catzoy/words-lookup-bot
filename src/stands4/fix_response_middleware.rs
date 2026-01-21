use axum::http::{Request, Response};
use rustify::errors::ClientError;
use rustify::{Endpoint, MiddleWare};

pub struct FixEmptyResponseMiddleware;

impl MiddleWare for FixEmptyResponseMiddleware {
    fn request<E: Endpoint>(
        &self,
        _endpoint: &E,
        _req: &mut Request<Vec<u8>>,
    ) -> Result<(), ClientError> {
        Ok(())
    }

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
