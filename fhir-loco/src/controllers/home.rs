use loco_rs::prelude::*;

use crate::views::home::HomeResponse;

// `loco_rs::Error` is over 128 bytes and every loco handler returns it by
// convention; boxing it is the framework's decision, not this crate's.
#[allow(clippy::result_large_err)]
#[debug_handler]
async fn current() -> Result<Response> {
    format::json(HomeResponse::new("loco"))
}

pub fn routes() -> Routes {
    Routes::new().prefix("/api").add("/", get(current))
}
