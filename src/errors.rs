use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("cache must be larger than 0")]
    InvalidCacheCapacity,
}
