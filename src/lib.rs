use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
pub struct ThreadPool {
    _handles: Vec<JoinHandle<()>>,
    _sender: Sender<Box<dyn Fn() + Send>>,
}

impl ThreadPool {
    pub fn new(size: u8) -> Self {
        let (sender, reciever) = channel::<Box<dyn Fn() + Send>>();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut _handles = vec![];
        for _ in 0..size {
            let reciever = Arc::clone(&reciever);
            let handle = thread::spawn(move || loop {
                let work;
                match reciever.lock().unwrap().recv() {
                    Ok(w) => work = w,
                    Err(_) => break,
                }
                println!("Starting working");
                work();
                println!("Ending working");
            });
            _handles.push(handle);
        }
        Self {
            _handles,
            _sender: sender,
        }
    }

    pub fn execute<F: Fn() + Send + 'static>(&self, work: F) {
        self._sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(5);
        pool.execute(|| {
            println!("start Hello, World!1 ");
            thread::sleep(Duration::from_secs(1));
            println!("end Hello, World!1 ");
        });
        pool.execute(|| println!("Hello, World! 2"));
        pool.execute(|| println!("Hello, World! 3"));
        thread::sleep(std::time::Duration::from_secs(3));
    }
}
