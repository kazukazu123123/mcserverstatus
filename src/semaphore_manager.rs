use std::sync::Arc;
use tokio::sync::{Semaphore, SemaphorePermit, AcquireError};

pub struct SemaphoreManager {
    semaphore: Arc<Semaphore>,
}

impl SemaphoreManager {
    pub fn new(initial_size: usize) -> Self {
        SemaphoreManager {
            semaphore: Arc::new(Semaphore::new(initial_size)),
        }
    }

    pub fn semaphore(&self) -> Arc<Semaphore> {
        Arc::clone(&self.semaphore)
    }

    pub fn update_size(&mut self, new_size: usize) {
        self.semaphore = Arc::new(Semaphore::new(new_size));
    }

    pub async fn acquire(&self) -> Result<SemaphorePermit, AcquireError> {
        self.semaphore.acquire().await
    }
}
