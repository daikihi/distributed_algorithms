use std::collections::HashMap;

enum Event {
    Send,
    Receive,
    Local
}

#[derive(Debug, Clone)]
struct VectorClock{
    pub my_id:i128,
    pub my_clock:i128,
    pub all_clocks: HashMap<i128, i128>,
}

impl VectorClock {
    fn new(num_nodes: i128, my_id: i128) -> Self {
        let mut all_clocks = HashMap::new();
        for i in 0..num_nodes {
            all_clocks.insert(i, 0);
        }
        VectorClock {
            my_id: my_id,
            my_clock: 0,
            all_clocks: all_clocks,
        }
    }

    // include send event
    fn local_event(&mut self) {
        self.my_clock += 1;
        self.all_clocks.insert(self.my_id, self.my_clock);
    }

    fn receive_event(&mut self, sender_id: i128, sender_clocks: HashMap<i128, i128>) {
        // self.all_clocks.insert(self.my_id, self.my_clock + 1);
        for (id, clock) in sender_clocks.iter() {
            if self.my_id == *id {
                let self_clock = self.my_clock;
                let max_clock = std::cmp::max(*clock, self_clock) + 1;
                self.my_clock = max_clock;
                self.all_clocks.insert(*id, max_clock);
            }else{
                let self_clock = self.all_clocks.get(id).unwrap();
                let max_clock = std::cmp::max(*clock, *self_clock);
                self.all_clocks.insert(*id, max_clock);
            }
        }
    }
    
}


struct EventData {
    pub event: Event,
    pub sender_id: i128,
    pub receiver_id: Option<i128>,
}

fn seinario() -> Vec<EventData>{
    let mut sienario_map: Vec< EventData> = Vec::new();
    sienario_map.push(EventData{event: Event::Local, sender_id: 0, receiver_id: None});
    sienario_map.push(EventData{event: Event::Send, sender_id: 0, receiver_id: Some(1)});
    sienario_map.push(EventData{event: Event::Receive, sender_id: 1, receiver_id: Some(0)});
    sienario_map.push(EventData{event: Event::Local, sender_id: 1, receiver_id: None});
    sienario_map
}


pub fn run(){
    let num_nodes = 5;
    let mut all_nodes: HashMap<i128, VectorClock> = HashMap::new();
    for id in 0..num_nodes {
        let vc: VectorClock = VectorClock::new(num_nodes, id);
        println!("{:?}", vc);
        all_nodes.insert(id, vc);
    }

    let event_map = seinario();
    for event in event_map.iter() {
        match event.event {
            Event::Local => {
                let mut vc = all_nodes.get_mut(&event.sender_id).unwrap();
                vc.local_event();
                println!("Local event: {:?}", vc);
            },
            Event::Send => {
                let mut vc = all_nodes.get_mut(&event.sender_id).unwrap();
                vc.local_event();
                println!("Send event: {:?}", vc);
            },
            Event::Receive => {
                let sender_vc: &mut VectorClock = all_nodes.get_mut(&event.sender_id).unwrap();
                let _tmp_sender_vc = sender_vc.clone();
                sender_vc.receive_event(event.sender_id, _tmp_sender_vc.all_clocks.clone());
                let mut vc = all_nodes.get_mut(&event.sender_id).unwrap();
                println!("Receive event: {:?}", vc);
            }
        }
    }
    let mut _for_print: Vec<(&i128, &VectorClock)> = all_nodes.iter().clone().collect();
    _for_print.sort_by(|x,y| x.0.cmp(&y.0));
    for (id, vc) in _for_print {
        println!("Final state of node {}: {:?}", id, vc);
    }

}   