use crate::elevator::Elevator;
use crate::gui;
use crate::person;

pub fn fill_queue(elevator: &Elevator) -> Vec<person::Person> {
    let mut queue: Vec<person::Person> = vec![];
    'running: loop {
        let option = gui::show_fill_or_process_menu();
        match option {
            0 => break 'running,
            1 => queue.push(person::get_person(
                &elevator.max_floors,
                &elevator.max_weight,
            )),
            2 => {
                let mut add = 0;
                while add < 10 {
                    queue.push(person::get_random_person(
                        &elevator.max_floors,
                        &elevator.max_weight,
                    ));
                    add += 1;
                }
                continue;
            }
            _ => continue,
        }
    }
    println!("\nPeople in Queue: {:?}", queue);
    queue
}
