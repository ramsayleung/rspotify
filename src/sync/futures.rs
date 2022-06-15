#[derive(Debug, Default)]
pub struct Mutex<T: ?Sized>(futures::lock::Mutex<T>);

#[derive(Debug)]
pub struct LockError;

impl<T> Mutex<T> {
    pub fn new(val: T) -> Self {
        Mutex(futures::lock::Mutex::new(val))
    }

    pub async fn lock(&self) -> Result<futures::lock::MutexGuard<'_, T>, LockError> {
        let val = self.0.lock().await;
        Ok(val)
    }
}
