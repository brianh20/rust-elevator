use crate::gui;
use crate::person;

/// Configure an elevator constraints
pub fn config_elevator() -> Elevator {
    let max_floors = get_max_floors();
    let max_weight = get_max_weight();
    Elevator::new(&max_floors, &max_weight)
}

/// Get top floor from user input
pub fn get_max_floors() -> u8 {
    gui::get_input_by_type_from_user::<u8>(&true, &false, &"Config Elevator", &"Number of floors:")
}

/// Get maximum weight for the elevator
pub fn get_max_weight() -> u16 {
    gui::get_input_by_type_from_user::<u16>(&true, &false, &"Config Elevator", &"Maximum weight:")
}

/// Elevator Struct
#[derive(Clone, Debug)]
pub struct Elevator {
    pub max_floors: u8,
    pub max_weight: u16,
    pub current_weight: u16,
    pub journey: Vec<u8>,
}

/// Elevator Struct implementation
impl Elevator {
    fn new(max_floors: &u8, max_weight: &u16) -> Elevator {
        Elevator {
            max_floors: *max_floors,
            max_weight: *max_weight,
            current_weight: 0,
            journey: Vec::new(),
        }
    }
}

/// Travel Trait
pub trait Travel {
    fn get_stops(&self, queue: &Vec<person::Person>) -> u16;
}

/// Travel Trait for Elevator
impl Travel for Elevator {

    /// Calculate number of stops
    /// 
    /// This is the core algorithm for this kata. Values have been borrowed instead of owned because this way we keep the history of the initial values for the queue.
    fn get_stops(&self, _queue: &Vec<person::Person>) -> u16 {
        // create local values for processing by cloning
        let mut elevator: Elevator = self.clone();
        let mut queue: Vec<person::Person> = _queue.clone().into_iter().rev().collect();
        let mut stops = 0;

        // while people in queue
        while queue.len() > 0 {
            // load elevator until it is full
            while queue.len() > 0
                && elevator.current_weight + queue[0].weight as u16 <= elevator.max_weight
            {
                let person = queue.pop().unwrap();
                elevator.current_weight += person.weight as u16;
                if !elevator.journey.contains(&person.floor) {
                    elevator.journey.push(person.floor);
                }
            }
            // count stops
            stops += elevator.journey.len() + 1;
            // flush elevator journey and weight
            elevator.journey = Vec::new();
            elevator.current_weight = 0;
        }

        stops.try_into().unwrap()
    }
}

#[test]
fn test_get_stops() {
    let test_elevator = Elevator::new(&10, &100);
    let mut queue: Vec<person::Person> = vec![];
    let mut person = person::create_person(&50, &1);
    queue.push(person);
    person = person::create_person(&40, &2);
    queue.push(person);
    person = person::create_person(&30, &1);
    queue.push(person);
    let stops = test_elevator.get_stops(&queue);
    assert_eq!(stops, 5);
}



