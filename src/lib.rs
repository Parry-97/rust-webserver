use std::{
    fmt::Display,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

#[derive(Debug)]
struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

//NOTE: We’ll change Job from a struct to a type alias for a trait object that holds the type of closure that execute receives.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing");
            job();
        });
        Worker { id, handle }
    }
}

#[derive(Debug)]
pub struct PoolCreationError(String);

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occured: {}", self.0)
    }
    // add code here
}

impl ThreadPool {
    // fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //     assert!(size > 0);
    //     Ok(ThreadPool {
    //         workers: Vec::with_capacity(size),
    //     })
    // }

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for n in 0..size {
            workers.push(Worker::new(n, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    //NOTE: we decided our thread pool should have an interface similar to thread::spawn.
    //In addition, we’ll implement the execute function so it takes the closure it’s given
    //and gives it to an idle thread in the pool to run.
    //We’ll define the execute method on ThreadPool to take a closure as a parameter.
    pub fn execute<F>(&self, f: F) -> ()
    where
        //NOTE: The F type parameter also has the trait bound Send and the lifetime bound 'static,
        //which are useful in our situation: we need Send to transfer the closure from one thread
        //to another and 'static because we don’t know how long the thread will take to execute.
        //WARN: even if we have no parameters, we still need the parentheses.
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
