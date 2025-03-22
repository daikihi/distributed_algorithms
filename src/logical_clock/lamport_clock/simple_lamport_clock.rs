use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
extern crate rand;
use rand::Rng;

struct Node {
    pub id: i32,
    pub current_clock: i128,
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
    pub num_nodes: i32
}

impl Node {
    pub fn new(_id: i32, sender: Sender<Message>, receiver: Receiver<Message>, num_nodes: i32) -> Self {
        Self {
            id: _id,
            current_clock: 0,
            sender: sender,
            receiver: receiver,
            num_nodes: num_nodes
        }
    }

    pub fn send_message(&mut self, target: i32) {
        self.current_clock += 1;

        let send_message = Message {
            sender: self.id,
            target,
            sender_clock: self.current_clock,
        };

        if let Err(e) = self.sender.send(send_message.clone()) {
            eprintln!("Error : Node {} failed to send a message: {}", self.id, e);
        } else {
            println!("Node {}  send message: {:?}", self.id, send_message);
        }
    }

    pub fn receive_messages(&mut self) {
        while let Ok(message) = self.receiver.try_recv() {
            println!("Node {} receives a message {:?}", self.id, message);
            self.current_clock = self.current_clock.max(message.sender_clock) + 1;
            println!("Node {} updated clock to {}", self.id, self.current_clock);
            let target: i32 = rand::thread_rng().gen_range(0..self.num_nodes);
            self.current_clock += 1;
            if self.current_clock <= 100000 {
                self.send_message(target);
            }else{
                println!("my_id = {}, DONE!", self.id);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Message {
    pub sender: i32,
    pub target: i32,
    pub sender_clock: i128,
}

struct Envieonment {
    pub nodes: Vec<Node>,
}

impl Envieonment {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node_id: i32, num_nodes: i32) {
        let (sender, receiver) = channel::<Message>();
        let node = Node::new(node_id, sender, receiver, num_nodes);
        self.nodes.push(node);
    }

    pub fn get_sender(&self, node_id: i32) -> Option<Sender<Message>> {
        self.nodes
            .iter()
            .find(|node| node.id == node_id)
            .map(|node| node.sender.clone())
    }
}

pub fn run() {
    println!("start to run lamport clock.");
    let mut env = Envieonment::new();

    let num_nodes = 10;
    for i in 0..num_nodes {
        env.add_node(i, num_nodes);
    }


    // 各ノードをスレッドで動作させる
    let mut handles = vec![];
    let mut nodes = std::mem::take(&mut env.nodes); 
    for mut node in nodes {
        handles.push(thread::spawn(move || {
            for _ in 0..num_nodes {
                node.receive_messages();
                let target: i32 = rand::thread_rng().gen_range(0..node.num_nodes);
                node.send_message(target);
                thread::sleep(std::time::Duration::from_millis(500));
            }
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}
