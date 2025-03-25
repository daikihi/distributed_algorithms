struct Node {
    id: i32,
    logical_clock: Vec<LogicalClock>
}

impl Node {
    pub fn  new(node_id: i32) -> Self {
        Self{
            id: node_id,
            logical_clock: Vec::<LogicalClock>::new()
        }
    }

    pub fn update_logical_clock(&mut self, received_logical_clock: Vec<LogicalClock>) -> Vec<LogicalClock>{
        let mut new_clock = Vec::<LogicalClock>::new();
        for lc in received_logical_clock {
            new_clock.push(self.update_time_stamp(lc));
        }
        self.logical_clock = new_clock.clone();
        new_clock
    }

    pub fn update_time_stamp(&self, logical_clock: LogicalClock) -> LogicalClock {
        if self.id == logical_clock.id {
            let mut max: LogicalClock = logical_clock.get_max(logical_clock.clone());
            max.clock += 1;
            max
        }else {
            logical_clock.get_max(logical_clock.clone()) 
        }
    }

    fn print_current_lc_state(&self) {
        for lc in self.logical_clock.clone() {
            println!("{:?}", lc);
        }
    }
}

#[cfg(test)]
mod tests_for_node {
    use super::*;

    #[test]
    fn test_update_logical_clock() {
        let mut node = Node::new(1);

        node.logical_clock = vec![
            LogicalClock{id: 1, clock: 0},
            LogicalClock{id: 2, clock: 0},
        ];

        let received_logical_clock = vec![
            LogicalClock{id: 1, clock: 0},
            LogicalClock{id: 2, clock: 3},
        ];

        let updated: Vec<LogicalClock> = node.update_logical_clock(received_logical_clock);
        
        assert_eq!(updated.len(), 2);
        assert_eq!(updated[0].id, 1);
        assert_eq!(updated[0].clock, 1);
        assert_eq!(updated[1].id, 2);
        assert_eq!(updated[1].clock, 3);

        assert_eq!(node.logical_clock, updated);
    }
}

/// A structure representing a logical clock in a distributed system.
/// 
/// This clock is identified by a unique `id` and maintains a `clock` value
/// to track the logical time of the associated process or entity.
/// 
/// # Fields
/// 
/// * `id` - A unique identifier for the logical clock.
/// * `clock` - The current logical time value, represented as a 128-bit integer.
/// 
/// # Example
/// 
/// ```rust
/// let clock = LogicalClock { id: 1, clock: 0 };
/// println!("Clock ID: {}, Time: {}", clock.id, clock.clock);
/// ```
#[derive(Debug, Clone, PartialEq)]
struct LogicalClock {
    pub id: i32,
    pub clock: i128
}

impl LogicalClock {
    pub fn get_max(&self, received_logical_clock: LogicalClock) -> LogicalClock {
        LogicalClock {
            id: self.id,
            clock: self.clock.max(received_logical_clock.clock),
        }
    }
}

struct Message {
    sender_id: i32,
    vector_clock: Vec<LogicalClock>
}

struct Envieonment {

}

pub fn run() {

}