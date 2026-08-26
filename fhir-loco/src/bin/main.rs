// Nothing here has any business dereferencing a raw pointer: this code
// parses and reshapes untrusted clinical data, and memory safety is the
// property that keeps a malformed resource from becoming a vulnerability.
#![forbid(unsafe_code)]

use fhir_loco::app::App;
use loco_rs::cli;

// `loco_rs::Error` is a wide enum, which clippy flags here. It is the
// framework's return type on a generated entry point, not something this
// project chooses, and boxing it would only move the size somewhere less
// obvious.
#[allow(clippy::result_large_err)]
#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App>().await
}
