use gilrs::{Gilrs, Button, Event};

pub fn example() {
    println!("Hello, world!");
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);
        }

        // // You can also use cached gamepad state
        // if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
        //     if gamepad.is_pressed(Button::South) {
        //         println!("Button South is pressed (XBox - A, PS - X)");
        //     }
        //}
    }
}


pub fn connect(){
    let mut gilrs = Gilrs::new().unwrap();

    for (id, gamepad) in gilrs.gamepads() {
        assert!(gamepad.is_connected());
        println!("Gamepad with id {} and name {} is connected",
                 id, gamepad.name());
    }
}