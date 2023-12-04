#![allow(unused, dead_code, deprecated)]
use std::collections::HashMap;

use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::*,
    window::WindowBuilder,
};
// To have a trait object it must be object safe
// - i can not create a trait object for a trait with generics (also if a method uses generics i
// cant use it in trait objects like dyn obj.method<T>()
// - must have a self in the methods args or just anotate some methods with where Self: Sized (But this
// means that I won't be able to call the methods on a trait object)
//  actually self is not valid but &self, &mut self, Box<Self> Arc<Self> Rc<Self> Pin<P> where P is
//  one of the types above
// - the trait cant have a type that returns self
/*  ex: fn clone(v; &dyn Clone){
 *         let x: dyn Clone = v.clone();    <--- Clone has a method that returns Self [fn clone(&self) -> Self]
 *                                              but when it returns Self, Self: dyn Clone 
 *                                              dyn Clone is not Sized and every returned type
 *                                              from a function MUST be Sized
     *  }
 * The trait Clone can't be turned into a trait object.
 *
 * */


/* 
 * WORKS because next doesn't have Self: Sized
 * fn it(v: &mut dyn Iterator<Item = bool>) {
 *  let _ = v.next()
 * }
 *
 * NOT WORK because collect has Self: Sized
 * fn it(v: &mut dyn Iterator<Item = bool>) {
 *  let _ = v.collect()
 * }
 *  
 *  see Rust Reference
 *  Object Safety
 *
 *
 *
 * */

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct MyVirtualKeyCode(VirtualKeyCode);

trait Listener {
    fn on_event(&self, event: MyVirtualKeyCode);
}

struct EventSystem {
    // instead of Vec<Box<dyn listener>> I should probably have a Set so I can make sure the same
    // listener is not mapped to the same key twice...
    events: HashMap<MyVirtualKeyCode, Vec<Box<dyn Listener>>>,
}

impl EventSystem {
    pub fn new() -> Self {
        let events = HashMap::new();

        Self { events }
    }

    pub fn fire_event(&self, event: MyVirtualKeyCode) {
        if self.events.contains_key(&event) {
            for i in 0..self.events[&event].len() {
                self.events[&event][i].on_event(event);
            }
        }
    }

    // Are these two equivalent?

    pub fn register_event_impl(
        &mut self,
        listener: impl Listener + 'static,
        event: MyVirtualKeyCode,
    ) {
        if self.events.contains_key(&event) {
            let x = self.events.get_mut(&event).unwrap();
            x.push(Box::new(listener));
        } else {
            self.events.insert(event, vec![Box::new(listener)]);
        }
    }

    pub fn register_event_boxed(&mut self, listener: Box<dyn Listener>, event: MyVirtualKeyCode) {
        if self.events.contains_key(&event) {
            let x = self.events.get_mut(&event).unwrap();
            x.push(listener);
        } else {
            self.events.insert(event, vec![listener]);
        }
    }

    // pub fn deregister_event(&mut self, listener: impl Listener + 'static, event: MyVirtualKeyCode) {
    //     unimplemented!()
    // }
}

#[derive(Clone)]
struct TestObject1 {
    x: i32,
    y: i32,
}

impl Listener for TestObject1 {
    fn on_event(&self, event: MyVirtualKeyCode) {
        println!("Event: {:?} on TestObject1", event);
    }
}

#[derive(Clone)]
struct TestObject2 {
    name: String,
}

impl Listener for TestObject2 {
    fn on_event(&self, event: MyVirtualKeyCode) {
        println!("Event: {:?} on TestObject2", event);
    }
}

fn main() {
    let mut event_system = EventSystem::new();
    let event_loop = EventLoop::new();
    let main_window = WindowBuilder::new()
        .with_title("Game")
        .build(&event_loop)
        .unwrap();

    let test_object_1 = TestObject1 { x: 1, y: 2 };
    let test_object_2 = TestObject2 {
        name: String::from("Object 1"),
    };

    // using impl
    event_system.register_event_impl(test_object_1.clone(), MyVirtualKeyCode(VirtualKeyCode::A));
    event_system.register_event_impl(test_object_2.clone(), MyVirtualKeyCode(VirtualKeyCode::D));

    // using box
    event_system.register_event_boxed(Box::new(test_object_1), MyVirtualKeyCode(VirtualKeyCode::W));
    event_system.register_event_boxed(Box::new(test_object_2), MyVirtualKeyCode(VirtualKeyCode::S));

    event_loop.run(move |event, _, control_flow| {
        control_flow
            .set_wait_until(std::time::Instant::now() + std::time::Duration::from_millis(1000));

        match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            scancode,
                            state,
                            virtual_keycode,
                            modifiers,
                        },
                    ..
                } => {
                    event_system.fire_event(MyVirtualKeyCode(virtual_keycode.unwrap()));
                }
                _ => (),
            },
            _ => (),
        }
    })
}
