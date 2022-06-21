use std::time;

pub const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36";

// OSS closes idle connections after 60 seconds,
// so we can close idle connections ahead of time to prevent re-using them.
// See also https://github.com/hyperium/hyper/issues/2136
pub const REQUEST_POOL_IDLE_TIMEOUT: time::Duration = time::Duration::from_secs(50);

pub const REQUEST_CONNECT_TIMEOUT: time::Duration = time::Duration::from_secs(20);

pub const REQUEST_TIMEOUT: time::Duration = time::Duration::from_secs(30);
