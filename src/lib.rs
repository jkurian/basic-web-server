use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct PoolCreationError;
pub struct WorkerCreationError;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>
}

struct Job;

impl Worker {
  fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, ()> {
    let thread = thread::spawn(|| {
      receiver;
    });

    Ok(Worker{
      id,
      thread,
    })
  }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
      if(size <= 0) {
        return Err(PoolCreationError)
      }

      let (sender, receiver) = mpsc::channel();

      let receiver = Arc::new(Mutex::new(receiver));

      let mut workers = Vec::with_capacity(size);

      for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)).unwrap())
      }

      Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
      where
          F: FnOnce() + Send + 'static
    {
      
    }
}
