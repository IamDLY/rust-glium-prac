extern crate glium;

use glium::glutin;
use glium::Surface;

fn main() {
    let mut events_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut closed = false;

    while !closed {
        events_loop.run(move |ev, _, control_flow| {});
    }
}
