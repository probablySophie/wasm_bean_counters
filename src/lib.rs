// This is a library, it doesn't use all its own functions
#![allow(dead_code)]
// And stuff that wasm doesn't want / care about / allow
#![allow(clippy::new_without_default)]
#![allow(clippy::must_use_candidate)]

extern crate wasm_bindgen;
extern crate web_sys;

#[macro_use]
mod js; // Our binds to JavaScript functions!

mod canvas_handler;

mod world;
mod player;

mod game;

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 600;




// TODO:
// Create a load function that gets the website to load the assets we want to use


// TODO: https://stackoverflow.com/questions/22213555/display-image-in-canvas-with-javascript#22213591
// ????????????/

// https://stackoverflow.com/questions/22213555/display-image-in-canvas-with-javascript#22213591
