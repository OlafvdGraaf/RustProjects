extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::process;

// for window (pop-up thing) custamization
use piston::window::WindowSettings;

// customize event settings and events
use piston::event_loop::{EventSettings, Events};

// the inputs we need
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs,
    RenderEvent, UpdateArgs, UpdateEvent };

// openGl graphics
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct App {
    gl: GlGraphics,
    left_score: i32,
    left_pos: i32,
    left_vel: i32,
    right_score: i32,
    right_pos: i32,
    right_vel: i32
}

fn main() {
    println!("Hello world")
}   