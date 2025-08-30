use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use parking_lot::Mutex;
use tokio::runtime::{Builder, Runtime};

type RuntimeMap = HashMap<String, Arc<Runtime>>;
type SharedRuntimeMap = Arc<Mutex<RuntimeMap>>;

static RUNTIME_MAP: OnceLock<SharedRuntimeMap> = OnceLock::new();

pub fn get_or_init_runtime(group: &str) -> Arc<Runtime> {
    const THREAD_PREFIX: &str = "syncyam-";
    let map = RUNTIME_MAP.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut map_guard = map.lock();
    match map_guard.get(group) {
        Some(rt) => rt.clone(),
        None => {
            let rt = Arc::new(
                Builder::new_multi_thread()
                    .enable_all()
                    .thread_name(format!("{THREAD_PREFIX}{group}"))
                    .build()
                    .unwrap(),
            );
            map_guard.insert(group.to_string(), rt.clone());
            rt
        }
    }
}

#[allow(dead_code)]
pub fn close_runtime(group: &str) {
    if let Some(map) = RUNTIME_MAP.get() {
        let mut map_guard = map.lock();
        map_guard.remove(group);
    }
}

#[cfg(test)]
mod tests_runtime {
    use std::{
        sync::Arc,
        time::{Duration, Instant},
    };

    use parking_lot::Mutex;

    use crate::utils::runtime::{close_runtime, get_or_init_runtime};

    #[test]
    fn can_return_same_runtime_for_same_group() {
        let rt1 = get_or_init_runtime("test_group");
        let rt2 = get_or_init_runtime("test_group");
        assert!(Arc::ptr_eq(&rt1, &rt2));
        close_runtime("test_group");
    }

    #[test]
    fn can_return_different_runtime_for_different_groups() {
        let rt1 = get_or_init_runtime("group1");
        let rt2 = get_or_init_runtime("group2");
        assert!(!Arc::ptr_eq(&rt1, &rt2));
        close_runtime("group1");
        close_runtime("group2");
    }

    #[test]
    fn can_execute_runtimes_concurrently() {
        let start = Instant::now();
        let sleep_duration = Duration::from_millis(100);
        let cnt = Arc::new(Mutex::new(0));

        {
            let rt = get_or_init_runtime("test_runtime1");
            let cnt = cnt.clone();
            rt.spawn(async move {
                for _i in 0..10 {
                    tokio::time::sleep(sleep_duration).await;
                    let mut cnt = cnt.lock();
                    *cnt += 1;
                }
            });
        }

        {
            let rt = get_or_init_runtime("test_runtime2");
            let cnt = cnt.clone();
            rt.spawn(async move {
                for _i in 0..20 {
                    tokio::time::sleep(sleep_duration).await;
                    let mut cnt = cnt.lock();
                    *cnt += 1;
                }
            });
        }

        let cnt = cnt.clone();
        awaitility::at_most(Duration::from_secs(5))
            .poll_interval(Duration::from_millis(100))
            .until(move || {
                let cnt = cnt.lock();
                *cnt >= 20 // meet this condition only in one sec;
            });
        close_runtime("test_runtime1");
        close_runtime("test_runtime2");

        assert!(start.elapsed().as_secs() < 2);
    }
}
