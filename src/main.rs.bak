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
        WindowBuilder::new() /*.with_decorations(false)*/
            .build(&event_loop)
            .unwrap(),
    );
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let imgbytes = include_bytes!("resources/pierogi.jpg");

    //let img = image::open(imgbytes).unwrap();
    let img = image::load_from_memory(imgbytes).unwrap().to_rgb8();

    let h = img.dimensions().1;
    let w = img.dimensions().0;

    //let monitor = window.primary_monitor();
    //let screensize = monitor.unwrap().size();

    let mut xpos = 0;
    let mut ypos = 0;

    let move_delay = Duration::from_millis(50);
    let move_ammount = 3;
    window.set_min_inner_size(Some(LogicalSize::new(w, h)));
    window.set_max_inner_size(Some(LogicalSize::new(w, h)));
    let _ = window.request_inner_size(LogicalSize::new(w, h));
    //window.set_decorations(false);

    //event_loop.set_control_flow(ControlFlow::Wait);

	event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now().checked_add(move_delay).unwrap()));
    event_loop
        .run(move |event, elwt| {
            let mut redrawn = false;
            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    redrawn = true;
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

                        buffer[index as usize] =
                            (px[2] as u32) | ((px[1] as u32) << 8) | ((px[0] as u32) << 16);

                        //buffer[index as usize] = blue | (green << 8) | (red << 16);
                    }

                    buffer.present().unwrap();

                    //ControlFlow::wait_duration(move_delay);
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
                    //				elwt.exit();
                }
                Event::NewEvents(cause) => {
                    match cause {
                        StartCause::ResumeTimeReached {
                            start,
                            requested_resume,
                        } => {
                            redrawn = true;
                            println!("yay:");

                            elwt.set_control_flow(ControlFlow::WaitUntil(Instant::now().checked_add(move_delay).unwrap()));
                            let monitor = window.current_monitor();
                            if monitor.is_some() {
                                let screensize = monitor.unwrap().size();

                                println!("{}", xpos);

                                xpos = (xpos + move_ammount) % screensize.width;
                                ypos = (ypos + move_ammount) % screensize.height;

                                window.set_outer_position(LogicalPosition::new(xpos, ypos));
                                //window.request_redraw();
                                //ControlFlow::wait_duration(elwt.control_flow(), move_delay);
                            }
                        }
                        StartCause::WaitCancelled {
                            start,
                            requested_resume,
                        } => {
							redrawn = true;
							println!("req res");
							if requested_resume.is_some(){
                            	elwt.set_control_flow(ControlFlow::WaitUntil(
                            	    requested_resume.unwrap()
                            	));
                            	//return;
							}
                        }
                        StartCause::Poll => {
                            println!("maybe?");
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        })
        .unwrap();
}
