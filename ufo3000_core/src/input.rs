use std::collections::HashMap;
use instant;

use winit::event as ev;

pub use ev::VirtualKeyCode as Key;
use winit::dpi::PhysicalPosition;

/// An enum for mouse and keyboard button states.
#[derive(Clone,Copy,Debug)]
pub enum InputState {
    Pressed(u128),
    Down(u128,u128),
    Released(u128, u128),
}

impl InputState {
    /// Updates the state depending on given ElementState and the current state.
    pub fn update(&mut self, state: &ev::ElementState, time_now: u128) -> InputState {
        match state {
            ev::ElementState::Pressed => {
                match std::mem::replace(self, InputState::Pressed(666)) {
                    InputState::Pressed(start_time) => {
                        // #[cfg(feature = "input_debug")]
                        // {
                        //     log::info!("Key is already pressed. (start_time == {:?}, time_now == {:?}", start_time, time_now);
                        // }
                        let state = InputState::Down(start_time,time_now);
                        *self = state; //InputState::Down(start_time,time_now)
                        state                        
                    }
                    // This is updated in InputCache::pre_update function for mouse buttons.
                    // This won't never happen for mouse events.
                    InputState::Down(start_time, _) => {
                        // #[cfg(feature = "input_debug")]
                        // {
                        //     log::info!("Key is down. (start_time == {:?}, time_now == {:?})", start_time, time_now);
                        // }
                        let state = InputState::Down(start_time,time_now);
                        *self = state; //InputState::Down(start_time,time_now);
                        state
                    }
                    InputState::Released(_,_) => {
                        // #[cfg(feature = "input_debug")]
                        // {
                        //     log::info!("Pressed => released {:?}", time_now);
                        // }
                        let state = InputState::Pressed(time_now);
                        *self = state; // InputState::Pressed(time_now)
                        state
                    }
                }
            }
            ev::ElementState::Released => {
                match std::mem::replace(self, InputState::Pressed(777)) {
                    InputState::Pressed(start_time) => {
                        // #[cfg(feature = "input_debug")]
                        // {
                        //     log::info!("Key is released. (start_time == {:?}, time_now == {:?}", start_time, time_now);
                        // }
                        let state = InputState::Released(start_time,time_now);
                        *self = state; //InputState::Released(start_time,time_now)
                        state
                    }
                    InputState::Down(start_time, _) => {
                        // #[cfg(feature = "input_debug")]
                        // {
                        //     log::info!("Key is released. (start_time == {:?}, time_now == {:?}", start_time, time_now);
                        // }
                        let state = InputState::Released(start_time,time_now);
                        *self = state; //InputState::Released(start_time,time_now)
                        state
                    }
                    InputState::Released(_,_) => {
                        panic!("Already released.")
                    }
                }
            }
        }
    }
}

/// A struct for a single mouse button.
#[derive(Clone)]
pub struct MouseButton {
    state: Option<InputState>,

    #[allow(dead_code)]
    tag: ev::MouseButton,
}

/// A struct for mouse buttons (left, middle, right).
#[derive(Clone)]
pub struct MouseButtons {
    left: MouseButton,
    middle: MouseButton,
    right: MouseButton,
}

impl MouseButtons {
    pub fn init() -> Self {
        Self {
            left: MouseButton   { state: None , tag: ev::MouseButton::Left},
            middle: MouseButton { state: None , tag: ev::MouseButton::Middle},
            right: MouseButton  { state: None , tag: ev::MouseButton::Right},
        }
    }

    /// Update mouse button.
    pub fn update(&mut self, button: &ev::MouseButton, state: &ev::ElementState, time_now: u128) {
        match button {
            ev::MouseButton::Left => {
                match &mut self.left.state {
                    Some(s) => {
                        s.update(&state, time_now);
                        //log::info!("Left mouse : {:?}", self.left.state.as_ref());
                    }
                    None => {
                        self.left.state = Some(InputState::Pressed(time_now));
                        //log::info!("Left mouse : {:?}", self.left.state.as_ref());
                    }
                }
            }
            ev::MouseButton::Middle => {
                match &mut self.middle.state {
                    Some(s) => {
                        s.update(&state, time_now);
                        //log::info!("Middle mouse : {:?}", self.middle.state.as_ref());
                    }
                    None => {
                        self.middle.state = Some(InputState::Pressed(time_now));
                        //log::info!("Middle mouse : {:?}", self.middle.state.as_ref());
                    }
                }
            }
            ev::MouseButton::Right => {
                match &mut self.right.state {
                    Some(s) => {
                        s.update(&state, time_now);
                        //log::info!("Right mouse : {:?}", self.right.state.as_ref());
                    }
                    None => {
                        self.right.state = Some(InputState::Pressed(time_now));
                        //log::info!("Right mouse : {:?}", self.right.state.as_ref());
                    }
                }
            }
            _ => { /* Some other mouse button clicked. */ }
        }
    }

    /// Get the state of the left mouse button.
    pub fn get_left(&self) -> Option<InputState> {
        self.left.state.clone()
    }

    /// Get the state of the middle mouse button.
    pub fn get_middle(&self) -> Option<InputState> {
        self.middle.state.clone()
    }
    
    /// Get the state of the right mouse button.
    pub fn get_right(&self) -> Option<InputState> {
        self.right.state.clone()
    }
}

/// A stuct for keep track on mouse cursor position.
#[derive(Clone, Copy)]
pub struct CursorPosition {
    pos: Option<PhysicalPosition<f64>>,
    inside: bool,
}

impl CursorPosition {

    /// Create Cursor Position.
    pub fn init() -> Self {
        Self {
            pos: None,
            inside: false,
        }
    }
}

/// Handles the keyboard, mouse and time information. The idea is derived from https:/github.com/MoleTrooper/starframe.
#[derive(Clone)]
pub struct InputCache {

    /// HashMap for keyboard, keys/states.
    keyboard: HashMap<Key, InputState>,

    /// Left, middle and right mouse buttons.
    mouse_buttons: MouseButtons,

    /// The current mouse_position.
    mouse_position: CursorPosition,

    /// The delta for the current and previous mouse position.
    mouse_delta: PhysicalPosition::<f64>,

    /// The delta for the mouse scroll.
    #[allow(dead_code)]
    scroll_delta: f32,

    /// Time now in micro seconds.
    time_now: u128,

    /// Delta for the current time and previous tick.
    time_delta: u128,

    /// The timer instance.
    timer: instant::Instant,

    /// Mouse move event happened.
    mouse_moved: bool,
}

impl InputCache {

    /// Initialize InputCache.
    pub fn init() -> Self {
        let keyboard = HashMap::<Key, InputState>::with_capacity(128);
        let mouse_buttons = MouseButtons::init();
        let mouse_position = CursorPosition::init();
        let timer = instant::Instant::now();

        Self {
            keyboard: keyboard,
            mouse_buttons: mouse_buttons,
            mouse_position: mouse_position,
            mouse_delta: PhysicalPosition::<f64>::new(0.0, 0.0),
            scroll_delta: 0.0,
            time_now: 0,
            time_delta: 0,
            timer: timer,
            mouse_moved: false,
        }
    }

    /// Get the current time.
    pub fn get_time(&self) -> u128 {
        self.time_now
    }

    /// Get the difference between the current time and previous tick.
    pub fn get_time_delta(&self) -> u128 {
        self.time_delta
    }

    /// Get the difference between the current and previous mouse position.
    pub fn get_mouse_delta(&self) -> PhysicalPosition::<f64> {
        if self.mouse_moved { self.mouse_delta }
        else { PhysicalPosition::<f64>::new(0.0, 0.0) }
    }

    /// This should be called before the actual update to ensure the all events takes effect even
    /// winit doesn't produce any events.
    pub fn pre_update(&mut self) {
        
        self.mouse_moved = false;

        // Update timer.
        let now = self.timer.elapsed().as_nanos();
        self.time_delta = now - self.time_now;
        self.time_now = now;

        // If mouse buttons were released previously, apply None to those states.
        // TODO: loop.
        if let Some(InputState::Released(_,_)) = self.mouse_buttons.left.state   { self.mouse_buttons.left.state = None }
        if let Some(InputState::Released(_,_)) = self.mouse_buttons.middle.state { self.mouse_buttons.middle.state = None }
        if let Some(InputState::Released(_,_)) = self.mouse_buttons.right.state  { self.mouse_buttons.right.state = None }

        // If left mouse button was pressed in previous tick, change the state to down.
        if let Some(InputState::Pressed(start_time)) = self.mouse_buttons.left.state {
            self.mouse_buttons.left.state = Some(InputState::Down(start_time,self.time_now));
        }
        if let Some(InputState::Pressed(start_time)) = self.mouse_buttons.middle.state {
            self.mouse_buttons.middle.state = Some(InputState::Down(start_time,self.time_now));
        }
        if let Some(InputState::Pressed(start_time)) = self.mouse_buttons.right.state {
            self.mouse_buttons.right.state = Some(InputState::Down(start_time,self.time_now));
        }

        // If the buttons are down, they stay down.
        // TODO: loop
        if let Some(InputState::Down(start_time, _)) = self.mouse_buttons.left.state {
            self.mouse_buttons.left.state = Some(InputState::Down(start_time,self.time_now));
        }
        if let Some(InputState::Down(start_time, _)) = self.mouse_buttons.middle.state {
            self.mouse_buttons.middle.state = Some(InputState::Down(start_time,self.time_now));
        }
        if let Some(InputState::Down(start_time, _)) = self.mouse_buttons.right.state {
            self.mouse_buttons.right.state = Some(InputState::Down(start_time,self.time_now));
        }

        // If key is pressed, change it to down. If it's down, update the value.
        for (_, val) in self.keyboard.iter_mut() {
            match *val {
                InputState::Pressed(v) => { *val = InputState::Down(v, self.time_now) },
                InputState::Down(s, e) => { *val = InputState::Down(s, e + self.time_now) },
                _ => { }
            }

        }

        // Remove key from hashmap if its previous state was 'released'.
        self.keyboard.retain(|_, state| match state { InputState::Released(_,_) => false, _ => true }); 
    }

    /// Process the new inputs.
    pub fn update(&mut self, event: &ev::WindowEvent) {
        use ev::WindowEvent::*;

        match event {
            KeyboardInput { input, ..} => self.track_keyboard(*input),
            MouseInput { button, state, ..} => self.track_mouse_button(*button, *state),
            MouseWheel { delta, ..} => self.track_mouse_wheel(*delta),
            CursorMoved { position, ..} => self.track_cursor_movement(*position),
            CursorEntered { ..} => self.track_cursor_enter(),
            CursorLeft { ..} => self.track_cursor_leave(),
            _ => (),
        }
    }
    /// Get the InputState of keyboard key.
    pub fn key_state(&self, key: &Key) -> Option<InputState> {
        if let Some(val) = self.keyboard.get(key) {
            Some(val.clone())
        }
        else { None }
    }

    /// Get the InputState of mouse button.
    pub fn mouse_button_state(&self, button: &ev::MouseButton) -> Option<InputState> {
        match button {
            ev::MouseButton::Left => { self.mouse_buttons.left.state.clone() } 
            ev::MouseButton::Middle => { self.mouse_buttons.middle.state.clone() } 
            ev::MouseButton::Right => { self.mouse_buttons.right.state.clone() } 
            _ => None
        }
    }
    /// Update the state of keyboard.
    fn track_keyboard(&mut self, evt: ev::KeyboardInput) {
        if let Some(key) = evt.virtual_keycode {
            match self.keyboard.get_mut(&key) {
                Some(state) => {
                    // Update the key time value.

                    let _debug_state = state.update(&evt.state, self.time_now);

                    #[cfg(feature = "input_debug")]
                    {
                        log::info!("Updating key {:?} :: {:?}", key, _debug_state);
                    }
                }
                None => {

                    // The key doesn't have any state. Add a new pressed state for this key.
                    #[cfg(feature = "input_debug")]
                    {
                        log::info!("The key {:?} is pressed at time {:?}", key, self.time_now);
                    }
                    let _ = self.keyboard.insert(key, InputState::Pressed(self.time_now));
                }
            }
            // TODO: implement these with lambda functions.
        }
    }
    /// Update the state of mouse buttons.
    fn track_mouse_button(&mut self, button: ev::MouseButton, state: ev::ElementState) {
        self.mouse_buttons.update(&button, &state, self.time_now);
    }
    /// Update the state of mouse wheel.
    fn track_mouse_wheel(&mut self, _delta: ev::MouseScrollDelta) {
        //log::info!("track_mouse_wheel");
    }
    /// Update the state of mouse movement.
    fn track_cursor_movement(&mut self, new_pos: PhysicalPosition<f64>) {
        self.mouse_moved = true;
        match self.mouse_position.pos {
            None => { self.mouse_position.pos = Some(new_pos);
                      //self.mouse_delta = PhysicalPosition::<f64>::new(0.0, 0.0);
                    }
            Some(old_position) => {
                self.mouse_delta = PhysicalPosition::<f64>::new(new_pos.x - old_position.x , new_pos.y - old_position.y);
                self.mouse_position.pos = Some(new_pos);
            }
        }
    }
    /// Handle the cursor enter event. TODO: implement.
    fn track_cursor_enter(&mut self) {
        self.mouse_position.inside = true;
        #[cfg(feature = "input_debug")]
        {
            log::info!("cursor enters");
        }
    }
    /// Handle the cursor leave event. TODO: implement.
    fn track_cursor_leave(&mut self) {
        self.mouse_delta = PhysicalPosition::<f64>::new(0.0, 0.0);
        self.mouse_position.inside = false;
        #[cfg(feature = "input_debug")]
        {
            log::info!("cursor leaves");
        }
    }
}

pub struct KeyboardManager {
    keys: HashMap<Key, (f64, f64)>,
}

impl KeyboardManager {
    pub fn init() -> Self {
        Self {
            keys: HashMap::<Key, (f64, f64)>::new(),
        }
    }

    pub fn register_key(&mut self, key: Key, threshold: f64) {
        self.keys.insert(key, (0.0, threshold)); 
    }

    pub fn test_key(&mut self, key: &Key, input: &InputCache) -> bool {
        
        let state_key = input.key_state(key);
        let mut result = false;

        if let Some(v) = self.keys.get_mut(key) {

            match state_key {
                Some(InputState::Pressed(_)) => {
                    let delta = (input.get_time_delta() as f64 / 1000000.0) as f64;
                    v.0 = delta;
                }
                Some(InputState::Down(_, _)) => {
                    let delta = (input.get_time_delta() as f64 / 1000000.0) as f64;
                    v.0 = v.0 + delta;
                    if v.0 > v.1 {
                        v.0 = v.0 - v.1;
                        result = true;
                    }
                },
                Some(InputState::Released(_, _)) => {
                    v.0 = 0.0; 
                }
                _ => { }
            }
        }
        return result;
    }
}
