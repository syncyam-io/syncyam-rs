#[must_use]
pub struct DeferGuard<'a> {
    defer_func_lifo: Vec<Box<dyn FnMut(bool) + 'a>>,
    committed: bool,
}

impl<'a> DeferGuard<'a> {
    pub fn new() -> Self {
        Self {
            defer_func_lifo: vec![],
            committed: false,
        }
    }

    pub fn add_defer_func(&mut self, defer_func: impl FnMut(bool) + 'a) {
        self.defer_func_lifo.push(Box::new(defer_func));
    }

    pub fn commit(&mut self) {
        self.committed = true;
    }
}

impl Drop for DeferGuard<'_> {
    fn drop(&mut self) {
        while let Some(mut drop_func) = self.defer_func_lifo.pop() {
            (drop_func)(self.committed)
        }
    }
}

#[cfg(test)]
mod tests_defer_guard {
    use std::sync::{Arc, Mutex};

    use tracing::info;

    use crate::utils::defer_guard::DeferGuard;

    #[test]
    fn can_drop_defer_guard() {
        let x = Arc::new(Mutex::new(10));
        {
            let mut guard = DeferGuard::new();
            let x1 = x.clone();
            guard.add_defer_func(move |committed| {
                let mut x = x1.lock().unwrap();
                assert_eq!(11, *x);
                *x += 1;
                assert!(committed);
                info!("x is {x}");
            });
            let x2 = x.clone();
            guard.add_defer_func(move |_c| {
                let mut x = x2.lock().unwrap();
                assert_eq!(10, *x);
                *x += 1;
            });
            guard.commit();
        }
        assert_eq!(12, *x.lock().unwrap());
    }
}
