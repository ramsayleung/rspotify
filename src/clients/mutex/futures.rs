use futures::lock::{Mutex, MutexGuard};

#[derive(Debug, Default)]
pub struct FuturesMutex<T: ?Sized>(Mutex<T>);

#[derive(Debug)]
pub struct LockError;

impl<T> FuturesMutex<T> {
    pub fn new(val: T) -> Self {
        FuturesMutex(Mutex::new(val))
    }

    pub async fn lock(&self) -> Result<MutexGuard<'_, T>, LockError> {
        let val = self.0.lock().await;
        Ok(val)
    }
}