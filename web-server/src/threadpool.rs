use std::sync::mpsc::*;
use std::sync::{ Arc, Mutex };
use std::thread::{ self, JoinHandle };

type F = FnOnce() + Send + 'static;

pub trait ThreadPool {
    fn execute(&self, f: Box<F>);
}

pub fn new(size: usize) -> impl ThreadPool {
    let (sender, receiver) = channel::<Box<F>>();
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
    sender: Sender<Box<F>>
}

impl ThreadPool for MyThreadPool {

    fn execute(&self, f: Box<F>) {
        self.sender.send(f).expect("send job fail");
    }

}

struct Worker {
    id: usize,
    join_handle: JoinHandle<()>
}

impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<Receiver<Box<F>>>>) -> Self {
        let join_handle = thread::spawn(move || {
            loop {
                let lock = receiver.lock();
                lock.unwrap().recv().unwrap()();
            }
        });

        Worker {
            id,
            join_handle
        }
    }

}