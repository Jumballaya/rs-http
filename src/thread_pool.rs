use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug)]
pub enum ThreadPoolError {
    ZeroSize,
    ThreadExecution(String),
}

impl std::fmt::Display for ThreadPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroSize => write!(f, "thread pool size must be larger than zero"),
            Self::ThreadExecution(details) => write!(f, "{}", details),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} terminated", id);
                    break;
                }
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolError> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::<Worker>::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            Ok(ThreadPool { workers, sender })
        } else {
            Err(ThreadPoolError::ZeroSize)
        }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Err(e) = self.sender.send(Message::NewJob(job)) {
            Err(ThreadPoolError::ThreadExecution(e.to_string()))
        } else {
            Ok(())
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                println!("Shutting down worker {}", worker.id);
                thread.join().unwrap();
            }
        }
    }
}
