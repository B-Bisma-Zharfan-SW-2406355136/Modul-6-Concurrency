use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, &'static str> {
        if size == 0 {
            return Err("Size must be greater than zero.");
        }    
       
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        Ok(ThreadPool {workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Err(_) = self.sender.send(job) {
            eprintln!("Failed to send job to thread pool.");
        }
    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let locked_receiver = match receiver.lock() {
                    Ok(receiver) => receiver,
                    Err(_) => {
                        eprintln!("Failed to acquire lock on receiver.");
                        break; // Keluar dari loop jika tidak bisa mendapatkan lock
                    }
                };
                let job = match locked_receiver.recv() {
                    Ok(job) => job,
                    Err(_) => {
                        eprintln!("Worker {id} disconnected; shutting down.");
                        break; // Keluar dari loop jika channel sudah ditutup
                    }
                };

                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
    }
}