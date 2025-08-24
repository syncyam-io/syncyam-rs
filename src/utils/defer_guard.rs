#[must_use]
pub struct DeferGuard<'a> {
    drop_func: Box<dyn FnMut(bool) + 'a>,
    committed: bool,
}

impl<'a> DeferGuard<'a> {
    pub fn new(drop_func: impl FnMut(bool) + 'a) -> Self {
        Self {
            drop_func: Box::new(drop_func),
            committed: false,
        }
    }

    pub fn commit(&mut self) {
        self.committed = true;
    }
}

impl Drop for DeferGuard<'_> {
    fn drop(&mut self) {
        (self.drop_func)(self.committed)
    }
}

#[cfg(test)]
mod tests_defer_guard {
    use tracing::info;

    use crate::utils::defer_guard::DeferGuard;

    #[test]
    fn can_drop_defer_guard() {
        let mut x = 10;
        {
            let mut guard = DeferGuard::new(|committed| {
                x += 1;
                assert!(committed);
                info!("x is {x}");
            });
            guard.commit();
        }
        assert_eq!(x, 11);
    }
}
