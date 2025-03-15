use std::thread;
use std::sync::mpsc::channel;

#[derive(Debug)]
struct Message{
    pub value: String
}

pub fn run(){
    let (ping_sender, ping_receiver) = channel::<Message>();
    let (pong_sender, pong_receiver) = channel::<Message>();
    let mut handles = vec![];

    // Ping
    let ping_handle = thread::spawn(move||{
        let message = Message{value: String::from("PING")};
        let _ = ping_sender.send(message);
        let pong = pong_receiver.recv();
        match pong {
            Ok(p) => {
                println!("simple mission compleated message = {:?}", p);
            }
            Err(e) => {
                println!("ERROR!  {:?}", e);
            }
        }
    });
    handles.push(ping_handle);

    // Pong 
    let pong_hundle = thread::spawn(move||{
        let received_message = ping_receiver.recv();
        match  received_message {
            Ok(m) =>{
                println!("receive.... {:?}", m);
                let _ = pong_sender.send(Message{value: String::from("PONG")});
            }
            Err(_e) => {
                println!("PONG ERROR");
            }
        }
    });
    handles.push(pong_hundle);

    for h in handles{
        let _ = h.join();
    }
}
