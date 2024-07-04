#![warn(clippy::all, clippy::pedantic)]

use std::{env, fs};

use chrono::{Local, Utc};
use rdev::{grab, Event, EventType, GrabError, Key};
use screenshots::Screen;

const TARGET_DIR: &str = "rust_screens";

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let screen_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path = env::current_dir().unwrap();
    path.push(&screen_dir);
    let res = fs::create_dir_all(path.clone());

    if let Err(error) = grab(move |e| callback(e, &screen_dir)) {
        println!("Error: {error:?}");
    }
}

fn callback(e: Event, screens_dir: &String) -> Option<Event> {
    match e.event_type {
        EventType::KeyPress(Key::ShiftRight) => {
            make_screen(screens_dir);
            None
        }
        _ => Some(e),
    }
}

fn make_screen(screens_dir: &String) {
    let screens = Screen::all().unwrap();
    for screen in screens {
        let image = screen.capture().unwrap();

        let now = Local::now();
        let normal_path = format!("{}/{}.png", screens_dir, now.format("%d-%m-%Y_%H_%M_%S_%f"));
        let res = image.save(&normal_path);
        if let Err(error) = res {
            println!("Ошибка: {error:?}");
        } else {
            println!("Скриншот сделан по адресу  {}", &normal_path);
        }
    }
}
