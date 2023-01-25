use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    condvar: Condvar,
    counter: Mutex<isize>,
}

impl Semaphore {
    pub fn acquire(&self) {
        // gain access to the atomic integer
        let mut count = self.counter.lock().unwrap();

        // wait so long as the value of the integer <= 0
        while *count <= 0 {
            count = self.condvar.wait(count).unwrap();
        }

        // decrement our count to indicate that we acquired
        // one of the resources
        *count -= 1;
    }
    pub fn release(&self) {
        // gain access to the atomic integer
        let mut count = self.counter.lock().unwrap();

        // increment its value
        *count += 1;

        // notify one of the waiting threads
        self.condvar.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use super::Semaphore;
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_sem_acquire_release() {
        let sem = Semaphore::new(1);

        sem.acquire();
        sem.release();
        sem.acquire();
    }

    #[test]
    fn test_child_waits_parent_signals() {
        let s1 = Arc::new(Semaphore::new(0));
        let s2 = s1.clone();

        let (tx, rx) = channel();

        let _t = thread::spawn(move || {
            s2.acquire();
            tx.send(()).unwrap();
        });

        s1.release();
        let _ = rx.recv();
    }

    #[test]
    fn test_parent_waits_child_signals() {
        let s1 = Arc::new(Semaphore::new(0));
        let s2 = s1.clone();

        let (tx, rx) = channel();

        let _t = thread::spawn(move || {
            s2.release();
            let _ = rx.recv();
        });

        s1.acquire();
        tx.send(()).unwrap();
    }
}
