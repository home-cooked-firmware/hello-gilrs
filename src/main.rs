use gilrs::{Gilrs, Event};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
        while let Some(Event { id, event, time, .. }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
        }
    }
}
