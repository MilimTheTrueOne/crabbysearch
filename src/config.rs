//! Config module

use figment::{providers::Serialized, Figment};
use serde::{Deserialize, Serialize};

/// Struct holding config Options
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// It stores the parsed port number option on which the server should launch.
    pub port: u16,
    /// It stores the parsed ip address option on which the server should launch
    pub binding_ip: String,
    /// Memory cache invalidation time
    pub cache_expiry_time: u64,
    /// It stores the option to whether enable or disable logs.
    pub logging: bool,
    /// It stores the option to whether enable or disable debug mode.
    pub debug: bool,
    /// It toggles whether to use adaptive HTTP windows
    pub adaptive_window: bool,
    /// It stores all the engine names that were enabled by the user.
    pub upstream_search_engines: crate::engines::Engines,
    /// It stores the time (secs) which controls the server request timeout.
    pub request_timeout: u8,
    /// Set the keep-alive time for client connections to the HTTP server
    //pub client_connection_keep_alive: u8,
    /// It stores the TCP connection keepalive duration in seconds.
    pub tcp_connection_keep_alive: u8,
    /// It stores the pool idle connection timeout in seconds.
    pub pool_idle_connection_timeout: u8,
}

/// Configuration options for the rate limiter middleware.
pub struct RateLimiter {
    /// The number of request that are allowed within a provided time limit.
    pub number_of_requests: u8,
    /// The time limit in which the quantity of requests that should be accepted.
    pub time_limit: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            binding_ip: "127.0.0.1".into(),
            cache_expiry_time: 600,
            logging: true,
            debug: false,
            adaptive_window: false,
            upstream_search_engines: Default::default(),
            request_timeout: 2,
            tcp_connection_keep_alive: 10,
            pool_idle_connection_timeout: 30,
        }
    }
}

impl Config {
    /// Creates a new config based on the environment variables.
    pub fn parse() -> Self {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(figment::providers::Env::prefixed("SEARCH"))
            .extract()
            .unwrap()
    }
}
