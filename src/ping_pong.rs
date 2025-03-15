use std::sync::{mpsc::{channel, Sender}, Arc, Mutex};
use std::thread;

trait Message: std::fmt::Debug + Send {
    fn say_message(sender: Arc<Mutex<Sender<Box<dyn Message>>>>) where Self: Sized;
}

#[derive(Debug)]
struct Ping {}

impl Message for Ping {
    fn say_message(sender: Arc<Mutex<Sender<Box<dyn Message>>>>) {
        sender.lock().unwrap().send(Box::new(Ping {})).unwrap();
    }
}

#[derive(Debug)]
struct Pong {}

impl Message for Pong {
    fn say_message(sender: Arc<Mutex<Sender<Box<dyn Message>>>>) {
        sender.lock().unwrap().send(Box::new(Pong {})).unwrap();
    }

}

pub fn run() {
    let (tx, rx) = channel::<Box<dyn Message>>();
    let tx_arc = Arc::new(Mutex::new(tx));

    // Spawn a thread for Ping
    {
        let tx_clone = Arc::clone(&tx_arc);
        thread::spawn(move || {
            Ping::say_message(tx_clone);
        });
    }

    // Spawn a thread for Pong
    {
        let tx_clone = Arc::clone(&tx_arc);
        thread::spawn(move || {
            Pong::say_message(tx_clone);
        });
    }

    for _ in 0..2 {
        let msg = rx.recv().unwrap();
        println!("{:?}", msg);
    }
}
