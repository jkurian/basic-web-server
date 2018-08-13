use std::thread;

pub struct PoolCreationError;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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
      let mut threads = Vec::with_capacity(size);

      for _ in 0..size {
        //Create threads here
      }

      Ok(ThreadPool { threads })
    }

    pub fn execute<F>(&self, f: F)
      where
          F: FnOnce() + Send + 'static
    {
      
    }
}