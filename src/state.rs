use std::sync::RwLock;

pub struct State {
    lock:RwLock<()>,
}

impl State {
    pub fn new() -> Self {
        Self {
            lock: RwLock::new(()),
        }
    }

    pub fn get_lock(&self) -> std::sync::RwLockReadGuard<()> {
        self.lock.read().unwrap()
    }

    pub fn get_lock_mut(&self) -> std::sync::RwLockWriteGuard<()> {
        self.lock.write().unwrap()
    }

}