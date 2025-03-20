use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use std::thread;

trait Message {}

#[derive(Debug)]
struct PingMessage {
    pub message: String,
}

#[derive(Debug)]
struct PongMessage {
    pub message: String,
}

impl Message for PingMessage {}
impl Message for PongMessage {}

#[derive(Debug)]
struct Ping {
    pub sender: Sender<Arc<PingMessage>>,
    pub receiver: Receiver<Arc<PongMessage>>,
}

impl Ping {
    pub fn send(&self, num_send: i32) {
        for i in 0..num_send {
            let message = Arc::new(PingMessage {
                message: format!("Ping message i = {}", i),
            });
            println!("send Ping Message : message = {:?}", message);
            if let Err(e) = self.sender.send(message) {
                eprintln!("Ping message send is failed : e = {}", e);
            }
        }
    }

    pub fn check_message(&self, num_receive: i32) {
        for i in 0..num_receive {
            let message_opt = self.receiver.recv();
            match message_opt {
                Ok(_message) => {
                    println!("message receiving success : message = {:?}", _message);
                }
                Err(e) => {
                    println!("message receiving error : error = {}", e);
                }
            };
        }
    }
}

impl Pong {
    pub fn receive_and_respond(&self, num_message: i32) {
        for i in 0..num_message {
            let received = self.receiver.recv();
            match received {
                Ok(_message) => {
                    println!("Pong received message = {:?}", _message);
                }
                Err(e) => {
                    eprintln!("Pong : message reveiving was faild, e = {}", e);
                }
            }
            if let Err(e) = self.sender.send(Arc::new(PongMessage {
                message: format!("Pong message message id =  {}", i),
            })) {
                eprintln!("message seinding is afailed");
            };
        }
    }
}

struct Pong {
    pub sender: Sender<Arc<PongMessage>>,
    pub receiver: Receiver<Arc<PingMessage>>,
}

pub fn run() {
    let (ping_send_channel, ping_receive_channel) = channel::<Arc<PingMessage>>();
    let (pong_send_channel, pong_receive_channel) = channel::<Arc<PongMessage>>();

    let ping: Ping = Ping {
        sender: ping_send_channel,
        receiver: pong_receive_channel,
    };
    let pong: Pong = Pong {
        sender: pong_send_channel,
        receiver: ping_receive_channel,
    };

    let num_message: i32 = 10;

    // this implementation send all message at onece and then wait for receive message from Pong
    let ping_thread = thread::spawn(move || {
        ping.send(num_message);
        ping.check_message(num_message);
    });

    let pong_thread = thread::spawn(move || {
        pong.receive_and_respond(num_message);
    });

    let _ = ping_thread.join();
    let _ = pong_thread.join();
}
