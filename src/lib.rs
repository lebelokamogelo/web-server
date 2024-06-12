use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    // Creating Space to Store the threads
    // threads: Vec<thread::JoinHandle<()>>,
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
// our ThreadPool have a new and execute methods
//
// TODO: A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread
// Define a Worker struct that holds an id and a JoinHandle<()>.
//Change ThreadPool to hold a vector of Worker instances.
//efine a Worker::new function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure.
//In ThreadPool::new, use the for loop counter to generate an id, create a new Worker with that id, and store the worker in the vector.

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // validatinf the number of threads in new
        // using the assert;
        // TODO: Sending Requests to Threads via Channels
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // we use the Arc to make use of the same reciever mult
        let reciever = Arc::new(Mutex::new(receiver));

        // let mut threads = Vec::with_capacity(size);
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // pass each reciever to each worker
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool {
            _workers: workers,
            sender,
        }
    }

    // the execute method should me the same as the thread spawn
    // but must not return the joinHandle to avoid it runnung instantly

    pub fn execute<F>(&self, f: F)
    where
        // inherit the thread::spawn properties
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    _id: usize,
    _thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // loop the thread to run infinite looking for jobs
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker {
            _id: id,
            _thread: thread,
        }
    }
}
