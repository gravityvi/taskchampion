use crate::server::ClientKey;
use crate::storage::Storage;
use actix_web::{error, http::StatusCode, web, HttpRequest, Result, Scope};
use std::sync::Arc;

mod add_version;
mod get_child_version;

/// The content-type for history segments (opaque blobs of bytes)
pub(crate) const HISTORY_SEGMENT_CONTENT_TYPE: &str =
    "application/vnd.taskchampion.history-segment";

/// The header name for version ID
pub(crate) const VERSION_ID_HEADER: &str = "X-Version-Id";

/// The header name for client key
pub(crate) const CLIENT_KEY_HEADER: &str = "X-Client-Key";

/// The header name for parent version ID
pub(crate) const PARENT_VERSION_ID_HEADER: &str = "X-Parent-Version-Id";

/// The type containing a reference to the Storage object in the Actix state.
pub(crate) type ServerState = Arc<Box<dyn Storage>>;

pub(crate) fn api_scope() -> Scope {
    web::scope("")
        .service(get_child_version::service)
        .service(add_version::service)
}

/// Convert a failure::Error to an Actix ISE
fn failure_to_ise(err: anyhow::Error) -> impl actix_web::ResponseError {
    error::InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR)
}

/// Get the client key
fn client_key_header(req: &HttpRequest) -> Result<ClientKey> {
    fn badrequest() -> error::Error {
        error::ErrorBadRequest("bad x-client-id")
    }
    if let Some(client_key_hdr) = req.headers().get(CLIENT_KEY_HEADER) {
        let client_key = client_key_hdr.to_str().map_err(|_| badrequest())?;
        let client_key = ClientKey::parse_str(client_key).map_err(|_| badrequest())?;
        Ok(client_key)
    } else {
        Err(badrequest())
    }
}