use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type F = dyn FnOnce() + Send + 'static;

enum Message {
    Execute(Box<F>),
    Terminate,
}

#[allow(drop_bounds)]
pub trait ThreadPool: Drop {
    fn execute(&self, f: Box<F>);
}

pub fn new(size: usize) -> impl ThreadPool {
    let (sender, receiver) = channel::<Message>();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers: Vec<Worker> = Vec::with_capacity(size);
    for i in 0..size {
        let worker = Worker::new(i, receiver.clone());
        workers.push(worker);
    }

    MyThreadPool {
        size,
        workers,
        sender: sender,
    }
}

struct MyThreadPool {
    size: usize,
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool for MyThreadPool {
    fn execute(&self, f: Box<F>) {
        self.sender
            .send(Message::Execute(f))
            .expect("send job fail");
    }
}

impl Drop for MyThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.size {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(handle) = worker.join_handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    join_handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let join_handle = thread::spawn(move || {
            loop {
                let lock = receiver.lock();
                match lock.unwrap().recv().unwrap() {
                    Message::Execute(job) => {
                        println!("execute a job");
                        job();
                        println!("execute complete");
                    }
                    Message::Terminate => {
                        println!("terminate the thread");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            join_handle: Some(join_handle),
        }
    }
}
