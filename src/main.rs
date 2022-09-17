mod vec3;
use crate::vec3::{Vec3, Color};
extern crate glium;

fn main() {
    // 1. The **winit::EventsLoop** for handling events.
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    println!("P3\n{IMAGE_WIDTH}  {IMAGE_HEIGHT} \n255");
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("Scanlines remaining: {j}");
        for i in (0..IMAGE_WIDTH){
            let color = Color{
                x: i as f64 / (IMAGE_WIDTH - 1) as f64,
                y: j as f64 / (IMAGE_HEIGHT - 1) as f64,
                z: 0.25,
            };
            color.write_color();
        }
    }
    eprintln!("Done");
    let example = Vec3{
        x: 5.0, 
        y: 3.0,
        z: 2.0,
    };

}