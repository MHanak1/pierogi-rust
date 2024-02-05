#![windows_subsystem = "windows"]

use rand::seq::SliceRandom;
use std::ffi::OsStr;
use std::num::NonZeroU32;
use std::path::Path;
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::{cmp, env, process};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use webbrowser;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, WindowLevel};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(
        WindowBuilder::new()
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_decorations(false)
            .with_visible(false)
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


	let mut refresh_rate: u32 = 60;

    //let move_delay = Duration::from_millis(30);
    let min_speed = 100.0;
    let max_speed = 300.0; 

    let mut move_ammount = 0.0;
    println!("{}", move_ammount);

    let _persistent_mode = false; //SPICY!!! NO RECOMMEND!!! wether the program should save itself to the autostart (not implemented)
    let max_instances: i32 = 30; //set to -1 to disable
    let clone_ammount = 3; //how many new pierogis to spawn when the app is closed
    let closable_window = true; //wether the app should close on alt + f4 or remain open
    let random_event_chance = 1.0; //chance for a random event to happen per second if i did the math correctly (see fn random_event())
    let open_browser = false; //enable browser opening random event
	let _vim_like_exit = false; //wether to stop the program when :q! is typed on a keyboard (not implemented)
	
    if (get_running_instances() as i32 > max_instances) && (max_instances > -1) {
        process::exit(0);
    }
    println!("{}", get_running_instances());

    window.set_resizable(false);
    //	window.set_always_on_top(true); //why you no exist???
    //window.set_min_inner_size(Some(LogicalSize::new(w, h)));
    //window.set_max_inner_size(Some(LogicalSize::new(w, h)));
    let _ = window.request_inner_size(LogicalSize::new(w as f64 / window.scale_factor(), h as f64 / window.scale_factor()));

    //event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.set_control_flow(ControlFlow::WaitUntil(
        Instant::now().checked_add(Duration::from_millis((1000/refresh_rate).into())).unwrap(),
    ));
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

					spawn_instances(clone_ammount, max_instances);
					
                    if closable_window {
                        elwt.exit();
                    }
                }
				Event::WindowEvent{
					event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() =>{
					let _ = window.request_inner_size(LogicalSize::new(w as f64 /  window.scale_factor(), h as f64 /  window.scale_factor()));
				}
				
                Event::NewEvents(cause) => {
                    match cause {
                        StartCause::ResumeTimeReached { .. } => {
                            elwt.set_control_flow(ControlFlow::WaitUntil(
                                Instant::now().checked_add(Duration::from_millis((950/refresh_rate).into())).unwrap(),
                            ));
                            let monitor = window.current_monitor();
                            if monitor.is_some() {
                                let screensize = monitor.clone().unwrap().size();

                                if !pos_set {
                                    xpos = (rand::random::<f32>()
                                        * (screensize.width as f32 / window.scale_factor() as f32 - window.outer_size().width as f32))
                                        .abs();
                                    ypos = (rand::random::<f32>()
                                        * (screensize.height as f32 / window.scale_factor() as f32 - window.outer_size().width as f32))
                                        .abs();
                                    window.set_visible(true);
                                    window.set_window_level(WindowLevel::AlwaysOnTop);
                                    pos_set = true;
									refresh_rate = monitor.unwrap().refresh_rate_millihertz().unwrap()/1000;
    								move_ammount = (min_speed + (rand::random::<f32>() * (max_speed - min_speed))) / refresh_rate as f32;
                                }

                                //println!("{}\t{}", xpos, ypos);
                                if (screensize.width - window.outer_size().width) as f32 / window.scale_factor() as f32 <= xpos {
                                    xup = false;
                                }
                                if (screensize.height - window.outer_size().height) as f32 / window.scale_factor() as f32 <= ypos {
                                    yup = false;
                                }
                                if xpos <= 1.0 {
                                    xup = true;
                                }
                                if ypos <= 1.0 {
                                    yup = true;
                                }

                                if xup {
                                    xpos += move_ammount;
                                } else {
                                    xpos -= move_ammount;
                                }
                                if yup {
                                    ypos += move_ammount;
                                } else {
                                    ypos -= move_ammount;
                                }

                                window.set_outer_position(LogicalPosition::new(xpos, ypos));

                                if random_event_chance / refresh_rate as f32  > rand::random::<f32>() * 100.0{
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
    let s = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );
    let mut i = 0;
    for process in s.processes_by_exact_name(&get_program_name().unwrap()) {
        println!("{}\t{:?}", process.name(), process.group_id());
        i += 1;
    }
    return i / 5;
}

fn random_event(open_browser: bool) {
    let sites = vec![
        "https://www.google.com/search?q=pierogi",
        "https://www.google.com/search?q=pierogi+z+mi%C4%99sem",
        "https://aniagotuje.pl/przepis/pierogi-ruskie",
        "https://www.google.com/search?q=pierogi+ruskie",
        "https://translate.google.com/?sl=pl&tl=en&text=Pierogi&op=translate",
    ];

	//println!("opened browser");
    if open_browser {
        let _ = webbrowser::open(sites.choose(&mut rand::thread_rng()).unwrap()).is_ok();
    }
}

fn get_program_name() -> Option<String> {
    env::args()
        .next()
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


fn spawn_instances(i: i32, max: i32){
	let remaining_instances = max - get_running_instances() as i32;
					
	let mut to_clone = i;
	if max > -1{
		to_clone = cmp::min(i, remaining_instances);
	}


	for _i in 0..to_clone {
		//println!("{}", i);
		let executable = &push_front(get_program_name().unwrap(), "./");
		let _ = process::Command::new(executable).spawn();
	}
}
