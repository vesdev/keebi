use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use enigo::{Button, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use hebi4::prelude::*;
use rand::Rng;

pub struct State {
    enigo: Enigo,
    args: Vec<String>,
}
impl State {
    fn new(args: Vec<String>) -> Self {
        Self {
            args,
            enigo: Enigo::new(&Settings::default()).unwrap(),
        }
    }
}

pub fn module(args: Vec<String>) -> NativeModule {
    let state = Arc::new(Mutex::new(State::new(args)));

    NativeModule::builder("keebi")
        .function(f!("arg", {
            let state = state.clone();
            move |cx: Context, i: i64| {
                let mut state = state.lock().expect("poisonded");
                keebi_arg(cx, i, &mut state)
            }
        }))
        .function(f!("sleep", keebi_sleep))
        .function(f!("exec", keebi_exec))
        .function(f!("rand_range", keebi_rand_range))
        .function(f!("rand_char", keebi_rand_char))
        .function(f!("text", {
            let state = state.clone();
            move |cx: Context, text: Param<Str>| {
                let mut state = state.lock().expect("poisonded");
                keebi_text(cx, text, &mut state);
            }
        }))
        .function(f!("button", {
            let state = state.clone();
            move |cx: Context, button: Param<Str>, direction: Param<Str>| {
                let mut state = state.lock().expect("poisonded");
                keebi_button(cx, button, direction, &mut state);
            }
        }))
        .function(f!("key", {
            let state = state.clone();
            move |cx: Context, key: Param<Str>, direction: Param<Str>| {
                let mut state = state.lock().expect("poisonded");
                keebi_key(cx, key, direction, &mut state);
            }
        }))
        .finish()
}

pub fn keebi_sleep(_cx: Context, seconds: f64) {
    std::thread::sleep(Duration::from_secs_f64(seconds));
}

pub fn keebi_arg(_cx: Context, i: i64, state: &mut State) -> HebiResult<String> {
    state
        .args
        .get(i as usize)
        .ok_or(error("Argument index out of bounds"))
        .cloned()
}

pub fn keebi_exec(cx: Context, cmd: Param<Str>) -> HebiResult<String> {
    let (shell, shell_arg) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let output = std::process::Command::new(shell)
        .arg(shell_arg)
        .arg(cmd.as_ref(cx.heap()).as_str())
        .output()
        .map_err(|_| error("Failed to exectue command"))?;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn keebi_rand_range(_cx: Context, min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn keebi_rand_char(_cx: Context) -> String {
    let mut rng = rand::rng();
    rng.random::<char>().to_string()
}

pub fn keebi_text(cx: Context, text: Param<Str>, state: &mut State) {
    state.enigo.text(text.as_ref(cx.heap()).as_str()).unwrap();
}

pub fn keebi_button(cx: Context, button: Param<Str>, direction: Param<Str>, state: &mut State) {
    state
        .enigo
        .button(
            parse_button(button.as_ref(cx.heap()).as_str()).unwrap(),
            parse_direction(direction.as_ref(cx.heap()).as_str()).unwrap_or(Direction::Click),
        )
        .unwrap();
}

pub fn keebi_key(cx: Context, key: Param<Str>, direction: Param<Str>, state: &mut State) {
    state
        .enigo
        .key(
            parse_key(key.as_ref(cx.heap()).as_str()).unwrap(),
            parse_direction(direction.as_ref(cx.heap()).as_str()).unwrap_or(Direction::Click),
        )
        .unwrap();
}

fn parse_key(button: &str) -> Option<enigo::Key> {
    if button.len() == 1 {
        return Some(Key::Unicode(button.chars().next()?));
    }

    match button {
        "alt" => Some(Key::Alt),
        "control" => Some(Key::Control),
        "backspace" => Some(Key::Backspace),
        "escape" => Some(Key::Escape),
        "enter" => Some(Key::Return),
        _ => None,
    }
}

fn parse_button(button: &str) -> Option<enigo::Button> {
    match button {
        "left" => Some(Button::Left),
        "right" => Some(Button::Right),
        "middle" => Some(Button::Middle),
        _ => None,
    }
}

fn parse_direction(button: &str) -> Option<enigo::Direction> {
    match button {
        "press" => Some(Direction::Press),
        "release" => Some(Direction::Release),
        "click" => Some(Direction::Click),
        _ => None,
    }
}
