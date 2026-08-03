//! Every environment's configuration must parse (**F-59**).
//!
//! `config/production.yaml` was **empty**, and Loco selects its configuration
//! by `LOCO_ENV` rather than merging over a default — so `LOCO_ENV=production`
//! did not start with sensible defaults, it refused to start:
//!
//! ```text
//! Error: YAMLFile(Error("missing field `logger`"), "config/production.yaml")
//! ```
//!
//! The one environment this service exists to run in was the one it could not
//! run in, and nothing noticed because the test suite runs as `test` and the
//! developer loop runs as `development`.
//!
//! These tests are deliberately cheap and need no database: a configuration
//! that cannot parse is a boot failure, and a boot failure found in CI costs
//! nothing while the same failure found at deploy time costs an outage.

use loco_rs::config::Config;
use loco_rs::environment::Environment;

/// The regression. `Environment::Production` must resolve to a config that
/// deserializes.
#[test]
fn every_environment_config_parses() {
    for env in [
        Environment::Development,
        Environment::Test,
        Environment::Production,
    ] {
        let cfg = Config::new(&env);
        assert!(
            cfg.is_ok(),
            "config for {env:?} does not parse, so `LOCO_ENV={env:?}` cannot \
             boot: {:?}",
            cfg.err()
        );
    }
}

/// `O10.8` asks for resource limits at the edge. Loco 1.0.1 gives us two of
/// them — a body limit and a request timeout — and production must set both.
///
/// Asserted on production specifically. Development deliberately leaves them
/// off so a local session is not fighting a timeout while stepping through a
/// debugger; that asymmetry is the reason to check the one that matters.
#[test]
fn production_enforces_the_request_limits_it_can() {
    let cfg = Config::new(&Environment::Production).expect("production config parses");
    let mw = &cfg.server.middlewares;

    // `LimitPayload` has no `enable` field — presence in the config *is* the
    // enablement, and the `enable: true` line in the YAML is consumed by
    // Loco's own middleware gate. What matters here is that a limit is set
    // rather than left to a default nobody chose.
    let payload = mw
        .limit_payload
        .as_ref()
        .expect("O10.8: production must bound the request body");
    assert!(
        !format!("{:?}", payload.body_limit).contains("Disable"),
        "the body limit is present but disabled: {:?}",
        payload.body_limit
    );

    let timeout = mw
        .timeout_request
        .as_ref()
        .expect("O10.8: production must bound request duration");
    assert!(timeout.enable, "the timeout is configured but disabled");
}

/// A backtrace can carry file paths and argument values, and this process
/// handles PHI. The audit chain and access log are the intended record
/// (`PR12.5`).
#[test]
fn production_does_not_print_backtraces() {
    let cfg = Config::new(&Environment::Production).expect("production config parses");
    let logger = &cfg.logger;
    assert!(
        !logger.pretty_backtrace,
        "pretty_backtrace in production can put paths and values in logs"
    );
}
