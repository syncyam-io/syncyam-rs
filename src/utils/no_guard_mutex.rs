use parking_lot::{
    RawMutex,
    lock_api::{RawMutex as _, RawMutexFair},
};

pub struct NoGuardMutex {
    lock: RawMutex,
}

impl NoGuardMutex {
    pub fn new() -> Self {
        Self {
            lock: RawMutex::INIT,
        }
    }

    pub fn lock(&self) {
        self.lock.lock();
    }

    pub fn unlock(&self) {
        if self.lock.is_locked() {
            unsafe { self.lock.unlock_fair() }
        }
    }

    #[allow(dead_code)]
    pub fn is_locked(&self) -> bool {
        self.lock.is_locked()
    }
}

impl Default for NoGuardMutex {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for NoGuardMutex {
    fn drop(&mut self) {
        self.unlock()
    }
}

#[cfg(test)]
mod tests_no_guard_mutex {
    use std::{
        sync::{
            Arc,
            atomic::{AtomicI32, Ordering},
        },
        time::Duration,
    };

    use crate::utils::no_guard_mutex::NoGuardMutex;

    #[test]
    fn can_lock_and_unlock() {
        let ng_mutex = Arc::new(NoGuardMutex::default());
        {
            ng_mutex.lock();
            // the mutex is not dropped here
        }
        assert!(ng_mutex.is_locked());
        ng_mutex.unlock();
        assert!(!ng_mutex.is_locked());
        ng_mutex.unlock();
        assert!(!ng_mutex.is_locked());
        ng_mutex.lock();
        assert!(ng_mutex.is_locked());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn can_lock_and_unlock_in_threads() {
        let ng_mutex = Arc::new(NoGuardMutex::new());
        let cnt = Arc::new(AtomicI32::new(0));
        for _i in 0..10 {
            let ng_mutex_clone = ng_mutex.clone();
            let cnt_clone = cnt.clone();
            tokio::spawn(async move {
                ng_mutex_clone.lock();
                tokio::time::sleep(Duration::from_millis(100)).await;
                tokio::spawn(async move {
                    cnt_clone.fetch_add(1, Ordering::SeqCst);
                    // unlock in another thread
                    ng_mutex_clone.unlock();
                });
            });
        }
        awaitility::at_most(Duration::from_secs(3)).until(|| cnt.load(Ordering::SeqCst) >= 10);
    }
}
