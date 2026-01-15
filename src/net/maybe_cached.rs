use std::ops::Deref;
use std::ops::DerefMut;
use std::time::Duration;

pub enum MaybeCached<T> {
    FromCache(T),
    FromNetwork(T),
}
impl<T> MaybeCached<T> {
    pub fn was_cached(&self) -> bool {
        matches!(self, MaybeCached::FromCache(_))
    }
    pub fn respectful_sleep(&self, delay: Duration) -> impl Future<Output = ()> + 'static {
        let should_sleep = !self.was_cached();
        async move {
            if should_sleep {
                tokio::time::sleep(delay).await;
            }
        }
    }

    pub fn take(self) -> T {
        match self {
            MaybeCached::FromCache(t) => t,
            MaybeCached::FromNetwork(t) => t,
        }
    }
}

impl<T> Deref for MaybeCached<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeCached::FromCache(t) => t,
            MaybeCached::FromNetwork(t) => t,
        }
    }
}
impl<T> DerefMut for MaybeCached<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MaybeCached::FromCache(t) => t,
            MaybeCached::FromNetwork(t) => t,
        }
    }
}
impl<T> AsRef<T> for MaybeCached<T> {
    fn as_ref(&self) -> &T {
        match self {
            MaybeCached::FromCache(t) => t,
            MaybeCached::FromNetwork(t) => t,
        }
    }
}
impl<T> AsMut<T> for MaybeCached<T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            MaybeCached::FromCache(t) => t,
            MaybeCached::FromNetwork(t) => t,
        }
    }
}
