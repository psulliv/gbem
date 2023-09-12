#![allow(unused)]

use crate::debug_utils;
use crate::sm83::ProcessorInterrupt;
use crate::{MemMap, ProcessorState};
use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

pub struct MachineState {
    pub processor_state: ProcessorState,
    pub mem_map: MemMap,
    pub last_interrupt: ProcessorInterrupt,
}

impl MachineState {
    pub fn new() -> Self {
        MachineState {
            processor_state: ProcessorState::new(),
            mem_map: MemMap::new(),
            last_interrupt: ProcessorInterrupt::ScanLine96,
        }
    }
}

impl Default for MachineState {
    fn default() -> Self {
        Self::new()
    }
}

// fn map_keyboard_to_button(key: &str) -> Option<Button> {
//     match key {
//         "ArrowLeft" => Some(Button::P1Left),
//         "ArrowRight" => Some(Button::P1Right),
//         "KeyA" => Some(Button::P2Left),
//         "KeyD" => Some(Button::P2Right),
//         "Space" => Some(Button::P1Shoot),
//         "Enter" => Some(Button::P2Shoot),
//         "Digit1" => Some(Button::P1Start),
//         "Digit2" => Some(Button::P2Start),
//         "KeyC" => Some(Button::Coin),
//         _ => None,
//     }
// }

// #[cfg(target_arch = "wasm32")]
// pub fn start_keyboard_listeners(m: &MachineState) {
//     let window = web_sys::window().expect("no global `window` exists");
//     let down_m = m.port_state.clone();
//     let keydown_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
//         if let Some(button) = map_keyboard_to_button(&(event.code())) {
//             let mut l_portstate = down_m.lock().unwrap();
//             l_portstate.button_down(button);
//         }
//     }) as Box<dyn FnMut(_)>);
//     window
//         .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
//         .unwrap();
//     keydown_closure.forget();

//     let up_m = m.port_state.clone();
//     let keyup_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
//         if let Some(button) = map_keyboard_to_button(&(event.code())) {
//             let mut l_portstate = up_m.lock().unwrap();
//             l_portstate.button_up(button);
//         }
//     }) as Box<dyn FnMut(_)>);
//     window
//         .add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())
//         .unwrap();
//     keyup_closure.forget();
// }
