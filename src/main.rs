use winit::{
    event::{Event, WindowEvent, KeyEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowButtons},
    keyboard::Key
};

#[path = "util/fill.rs"]
mod fill;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rustpad")
        .with_decorations(true)
        .build(&event_loop)
        .unwrap();
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { 
                event: WindowEvent::CloseRequested,
                window_id
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            // Event::DeviceEvent { event: WindowEvent::KeyboardInput { device_id, event, is_synthetic } }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: key,
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => match key.as_ref() {
                Key::Character("F" | "f") => {
                    let buttons = window.enabled_buttons();
                    window.set_enabled_buttons(buttons ^ WindowButtons::CLOSE);
                }
                Key::Character("G" | "g") => {
                    let buttons = window.enabled_buttons();
                    window.set_enabled_buttons(buttons ^ WindowButtons::MAXIMIZE);
                }
                Key::Character("H" | "h") => {
                    let buttons = window.enabled_buttons();
                    window.set_enabled_buttons(buttons ^ WindowButtons::MINIMIZE);
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                fill::fill_window(&window, 0xe575b8);
            },
            _ => (),
        }
    });
}