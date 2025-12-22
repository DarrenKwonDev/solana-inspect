pub mod api_cache;
pub mod client;
pub mod dex_filter;
pub mod protocols;

pub const CACHE_TOKEN_FILE_NAME: &str = "tokens.json";

pub const GREEN: &str = "\x1b[32m";
pub const MAGENTA: &str = "\x1b[35m";
pub const YELLOW: &str = "\x1b[33m";
pub const ORANGE: &str = "\x1b[38;5;208m";
pub const RESET: &str = "\x1b[0m";
