pub mod instruction;
pub mod oracle_account;
pub mod processor;
pub mod request;
pub mod response;

const PUBLIC_KEY_LEN: usize = 32;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
