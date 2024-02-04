#![windows_subsystem = "windows"]

use std::num::{NonZeroU32};
use std::rc::Rc;
use std::{process, env, cmp};
use std::time::{Duration, Instant};
use std::path::Path;
use std::ffi::OsStr;
use winit::event::{Event, StartCause, WindowEvent};
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, WindowLevel};
use sysinfo::{System, RefreshKind, ProcessRefreshKind};
use webbrowser;
use rand::seq::SliceRandom;

fn main() {
	let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(
        WindowBuilder::new()
			.with_window_level(WindowLevel::AlwaysOnTop)
			.with_decorations(false)
            .build(&event_loop)
            .unwrap(),
    );
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
	
	let imgbytes = include_bytes!("resources/pierogi.jpg");

   let img = image::load_from_memory(imgbytes).unwrap().to_rgb8();

	let mut pos_set = false;

    let h = img.dimensions().1;
    let w = img.dimensions().0;

    let mut xpos: f32 = 0.0;
    let mut ypos: f32 = 0.0;

	let mut xup = rand::random::<bool>();
	let mut yup = rand::random::<bool>();

    let move_delay = Duration::from_millis(30);
	let min_speed = 3.0;
	let max_speed = 8.0;

    let move_ammount = min_speed + (rand::random::<f32>() * (max_speed - min_speed));
	println!("{}", move_ammount);

	let _persistent_mode = false;
	let max_instances = 20;
	let clone_ammount = 3;
	let closable_window = true;
	let random_event_chance = 0.07;
	let open_browser = true;


	if get_running_instances() > max_instances{
		process::exit(0);
	}
	println!("{}", get_running_instances());

	window.set_resizable(false);
//	window.set_always_on_top(true); //why you no exist???
    //window.set_min_inner_size(Some(LogicalSize::new(w, h)));
    //window.set_max_inner_size(Some(LogicalSize::new(w, h)));
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
					let remaining_instances = max_instances - get_running_instances();



					for _i in 0..cmp::min(clone_ammount, remaining_instances){
						//println!("{}", i);
						let executable = &push_front(get_program_name().unwrap(), "./");
						let _ = process::Command::new(executable).spawn();
					}
	  				if closable_window {elwt.exit();}
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

								if !pos_set{
									xpos = (rand::random::<f32>() % (screensize.width - window.outer_size().width) as f32).abs();
    								ypos = (rand::random::<f32>() % (screensize.height - window.outer_size().width) as f32).abs();
									pos_set = true;
								}

								//println!("{}\t{}", xpos, ypos);
								if (screensize.width - window.outer_size().width) as f32 <= xpos {xup = false;}
								if (screensize.height - window.outer_size().height) as f32 <= ypos {yup = false;}
								if xpos <= 0.0 {xup = true;}
								if ypos <= 0.0 {yup = true;}

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

								
								if random_event_chance > rand::random::<f32>()*100.0 {
									random_event(open_browser);
								}

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


fn get_running_instances() -> u32 {
	let s = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));
	let mut i = 0;
	for process in s.processes_by_exact_name(&get_program_name().unwrap()) {
		println!("{}\t{:?}", process.name(), process.group_id());
    	i += 1;
	}
	return i/5;
}

fn random_event(open_browser: bool){

	let sites = vec![
		"https://www.google.com/search?q=pierogi",
		"https://www.google.com/search?q=pierogi+z+mi%C4%99sem",
		"https://aniagotuje.pl/przepis/pierogi-ruskie",
		"https://www.google.com/search?q=pierogi+ruskie",
		"https://translate.google.com/?sl=pl&tl=en&text=Pierogi&op=translate"
	];
	
	if open_browser{
		let _ = webbrowser::open(sites.choose(&mut rand::thread_rng()).unwrap()).is_ok();
	}
}

fn get_program_name() -> Option<String> {
    env::args().next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
}

fn push_front(mut s: String, prefix: &str) -> String {
  s.insert_str(0, prefix);
  s
}
