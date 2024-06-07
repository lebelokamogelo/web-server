use std::thread;

pub struct ThreadPool {
    // Creating Space to Store the threads
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
}
// our ThreadPool have a new and execute methods
//
// TODO: A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread
// Define a Worker struct that holds an id and a JoinHandle<()>.
//Change ThreadPool to hold a vector of Worker instances.
//efine a Worker::new function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure.
//In ThreadPool::new, use the for loop counter to generate an id, create a new Worker with that id, and store the worker in the vector.

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // validatinf the number of threads in new
        // using the assert;

        assert!(size > 0);
        // let mut threads = Vec::with_capacity(size);
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    // the execute method should me the same as the thread spawn
    // but must not return the joinHandle to avoid it runnung instantly

    pub fn execute<F>(&self, f: F)
    where
        // inherit the thread::spawn properties
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
