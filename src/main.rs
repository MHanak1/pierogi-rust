#![windows_subsystem = "windows"]

use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{Event, StartCause, WindowEvent};
//use winit::event;
use std::time::{Duration, Instant};
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(
        WindowBuilder::new()
			.with_decorations(false)
            .build(&event_loop)
            .unwrap(),
    );
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let imgbytes = include_bytes!("resources/pierogi.jpg");

   let img = image::load_from_memory(imgbytes).unwrap().to_rgb8();

    let h = img.dimensions().1;
    let w = img.dimensions().0;

    let mut xpos = 0;
    let mut ypos = 0;

	let mut xup = false;
	let mut yup = false;

    let move_delay = Duration::from_millis(30);
    let move_ammount = 4;
    window.set_min_inner_size(Some(LogicalSize::new(w, h)));
    window.set_max_inner_size(Some(LogicalSize::new(w, h)));
    let _ = window.request_inner_size(LogicalSize::new(w, h));

	//event_loop.set_control_flow(ControlFlow::Wait);

	event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now().checked_add(move_delay).unwrap()));
    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
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
                        let px = img.get_pixel(x, y);

                        buffer[index as usize] =
                            (px[2] as u32) | ((px[1] as u32) << 8) | ((px[0] as u32) << 16);
                   }
                   buffer.present().unwrap();
               }
               Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
	  				elwt.exit();
                }
                Event::NewEvents(cause) => {
                    match cause {
                        StartCause::ResumeTimeReached {
                            ..
                        } => {
                           elwt.set_control_flow(ControlFlow::WaitUntil(Instant::now().checked_add(move_delay).unwrap()));
                            let monitor = window.current_monitor();
                            if monitor.is_some() {
                                let screensize = monitor.unwrap().size();

								if (screensize.width - window.outer_size().width) <= xpos {xup = false;}
								if (screensize.height - window.outer_size().height) <= ypos {yup = false;}
								if xpos <= 0 {xup = true;}
								if ypos <= 0 {yup = true;}

								if xup{
									xpos += move_ammount;
								}else{
									xpos -= move_ammount;
								}
								if yup{
									ypos += move_ammount;
								}else{
									ypos -= move_ammount;
								}
                                
								window.set_outer_position(LogicalPosition::new(xpos, ypos));
                           }
                        }
                      _ => {}
                    }
                }

                _ => {}
            }
        })
        .unwrap();
}
