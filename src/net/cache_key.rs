use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

/// A part that is joined to the [`crate::paths::CacheHome`] to form a complete cache path.
pub struct CacheKey(pub PathBuf);

impl CacheKey {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        CacheKey(path.into())
    }
    pub fn join(&self, path: impl AsRef<Path>) -> Self {
        CacheKey(self.0.join(path))
    }
}
impl AsRef<Path> for CacheKey {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Deref for CacheKey {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait HasCacheKey {
    /// Get the cache key for this type.
    fn cache_key(&self) -> CacheKey;
}
