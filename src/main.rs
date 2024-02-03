#![windows_subsystem = "windows"]

use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
//use image;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().with_decorations(false).build(&event_loop).unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
	
	let imgbytes = include_bytes!("resources/pierogi.jpg");

	//let img = image::open(imgbytes).unwrap();
	let img =  image::load_from_memory(imgbytes).unwrap().to_rgb8();

	let h = img.dimensions().1;
	let w = img.dimensions().0;


	window.set_min_inner_size(Some(LogicalSize::new(w, h)));
	window.set_max_inner_size(Some(LogicalSize::new(w, h)));
	let _ = window.request_inner_size(LogicalSize::new(w, h));
	//window.set_decorations(false);


	event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(w * h) {
                    let y = index / w;
                    let x = index % w;
                    //let red = x % 255;
                    //let green = y % 255;
                    //let blue = (x * y) % 255;

					let px = img.get_pixel(x, y);
                   
                    buffer[index as usize] = (px[2] as u32) | ((px[1] as u32) << 8) | ((px[0] as u32) << 16);
 

                    //buffer[index as usize] = blue | (green << 8) | (red << 16);
                }

                buffer.present().unwrap();
			
			}
			
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
//				elwt.exit();
            }
            _ => {}
        }
    }).unwrap();
}
