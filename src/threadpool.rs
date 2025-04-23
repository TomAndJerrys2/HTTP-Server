// Developer: Kyle Brady

// For handlin multi threaded jobs (requests) and the assignment of
// different worker threads based on the amount of requests

use std::{
    sync::{mpsc, Arc, Mutex}, // for use of smart pointers
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

// Worker Message Instruction signals
enum Message {
    SpawnJob(Job),
    TerminateJob,
}

// Main Worker Structs - not accessible outside of module
struct Worker {
    wkr_id: usize,
    wkr_thread: Option<thread::JoinHandle<()>>,
}

// Main Thread Pool Struct
pub struct ThreadPool {
    wkr_threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// -- Thread Pool --
impl ThreadPool {
    // Creates a new Thread pool
    pub fn new(pool_size: usize) -> ThreadPool {
        assert!(pool_size > 0); // # Panics

        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(pool_size);

        for wkr_id in 0..size {
            workers.push(Worker::new(wkr_id, Arc::clone(&reciever)));
        }

        // Returns the workers and sender sig to the thread pool
        ThreadPool { workers, sender }
    }

    // Executes a Generic Thread
    pub fn execute<Type>(&self, closure: Type)
    where
        Type: FnOnce() + Send + 'static,
    {
        let job = Box::new(closure);
        self.sender.send(Message::SpawnJob(job)).unwrap();
    }
}

// For Exiting
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("[-] Dropping all Workers");

        for _ in &self.workers {
            self.sender.send(Message::TerminateJob).unwrap();
        }

        for worker in &mut self.workers {
            println!("[!] Shutting Worker Down {}", wkr_id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// -- Worker --
impl Worker {
    pub fn new(wkr_id: usize, reciever: Arc<Mutex<mpsc::Reciever<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = reciever.lock().unwrap().recv().unwrap();

            match message {
                Message::SpawnJob(job) => {
                    println!("[+] Worker Executing {}", wkr_id);
                    job();
                }

                Message::TerminateJob => {
                    println!("[!] Worker Terminating {}", wkr_id);
                    break;
                }
            }
        });

        // The worker gets assigned an ID and
        // a JOB => New Thread
        Worker {
            wkr_id,
            thread: Some(thread),
        }
    }
}

// Automation Tests for builds

#[cfg(test)]
mod ThreadTests {

    // No Running Threads
    // Will Cause the compiler to panic
    #[test]
    fn test_no_threads() {
        let tester = ThreadPool::new(0);
    }

    // Size of pool must = workers in a pool
    #[test]
    fn test_size_of_pool(mut n: i32) {
        let tester = ThreadPool::new(n);
        assert_eq!(tester.wkr_threads.len(), n);

        // Testing if the threadpool is the same size as our worker amount
    }
}
