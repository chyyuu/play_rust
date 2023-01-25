use std::sync::{Condvar, Mutex};
// use std::sync::mpsc::channel;
// use std::sync::Arc;
// use std::thread;
pub struct Semaphore {
    condvar: Condvar,
    counter: Mutex<isize>,
}

impl Semaphore {
    pub fn new(var: isize) -> Semaphore {
        Semaphore {
            condvar: Condvar::new(),
            counter: Mutex::new(var),
        }
    }
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

use std::sync::Arc;
use std::thread;
fn main() {
    //let sem = Semaphore::new(1);
    // 创建信号量，并设置允许同时访问的线程数为 2。
    let semaphore = Arc::new(Semaphore::new(2));

    // 创建三个线程。
    let threads = (0..3)
        .map(|i| {
            let semaphore = semaphore.clone();
            thread::spawn(move || {
                // 在信号量上调用 acquire 方法获取信号量。
                semaphore.acquire();

                // 输出消息。
                println!("Thread {}: acquired semaphore", i);

                // 模拟执行耗时操作。
                thread::sleep(std::time::Duration::from_secs(1));

                // 在信号量上调用 release 方法释放信号量。
                println!("Thread {}: releasing semaphore", i);
                semaphore.release();
            })
        })
        .collect::<Vec<_>>();

    // 等待所有线程完成。
    for thread in threads {
        thread.join().unwrap();
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
