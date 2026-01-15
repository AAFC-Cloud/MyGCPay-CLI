use crate::paths::CacheHome;
use eyre::Context;
use std::ops::Deref;

/// Newtype wrapper for the cookie string used by MyGCPay
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MyGcPayCookie(pub String);

impl Deref for MyGcPayCookie {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl MyGcPayCookie {
    const COOKIE_FILE_NAME: &'static str = "cookie.txt";

    /// Read the cookie from the provided cache home.
    ///
    /// # Errors
    ///
    /// Returns an error if reading the cookie file fails.
    pub async fn read(cache: &CacheHome) -> eyre::Result<Option<MyGcPayCookie>> {
        let path = cache.join(Self::COOKIE_FILE_NAME);
        if !path.exists() {
            return Ok(None);
        }
        let s = tokio::fs::read_to_string(&path)
            .await
            .wrap_err_with(|| format!("failed to read cookie file: {}", path.display()))?;
        Ok(Some(MyGcPayCookie(s.trim().to_string())))
    }

    /// Write the cookie value into the provided cache home (creates parent dirs if needed).
    ///
    /// # Errors
    ///
    /// Returns an error if creating directories or writing the file fails.
    pub async fn write(&self, cache: &CacheHome) -> eyre::Result<()> {
        let path = cache.join(Self::COOKIE_FILE_NAME);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&path, self.0.as_bytes())
            .await
            .wrap_err_with(|| format!("failed to write cookie file: {}", path.display()))?;
        Ok(())
    }

    /// Delete the cookie file (if it exists).
    ///
    /// # Errors
    ///
    /// Returns an error if removing the file fails for reasons other than not existing.
    pub async fn delete(cache: &CacheHome) -> eyre::Result<()> {
        let path = cache.join(Self::COOKIE_FILE_NAME);
        match tokio::fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => {
                Err(e).wrap_err_with(|| format!("failed to remove cookie file: {}", path.display()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::CacheHome;
    use tempfile::TempDir;

    #[tokio::test]
    async fn write_and_read_cookie() {
        let tmp = TempDir::new().expect("tempdir");
        let cache = CacheHome(tmp.path().to_path_buf());

        MyGcPayCookie("abc123".to_string())
            .write(&cache)
            .await
            .expect("write");
        let read = MyGcPayCookie::read(&cache).await.expect("read");
        assert_eq!(read, Some(MyGcPayCookie("abc123".to_string())));
    }

    #[tokio::test]
    async fn delete_cookie() {
        let tmp = TempDir::new().expect("tempdir");
        let cache = CacheHome(tmp.path().to_path_buf());

        MyGcPayCookie("to-delete".to_string())
            .write(&cache)
            .await
            .expect("write");
        MyGcPayCookie::delete(&cache).await.expect("delete");
        assert!(MyGcPayCookie::read(&cache).await.expect("read").is_none());
    }

    #[tokio::test]
    async fn read_missing_is_none() {
        let tmp = TempDir::new().expect("tempdir");
        let cache = CacheHome(tmp.path().to_path_buf());
        assert!(MyGcPayCookie::read(&cache).await.expect("read").is_none());
    }

    #[tokio::test]
    async fn write_creates_parent_dir() {
        let tmp = TempDir::new().expect("tempdir");
        let nested = tmp.path().join("nested/dir");
        let cache = CacheHome(nested.clone());

        MyGcPayCookie("zz".to_string())
            .write(&cache)
            .await
            .expect("write");
        let cookie_path = nested.join(MyGcPayCookie::COOKIE_FILE_NAME);
        assert!(cookie_path.exists());
        let contents = tokio::fs::read_to_string(&cookie_path)
            .await
            .expect("read file");
        assert_eq!(contents, "zz");
    }
}
