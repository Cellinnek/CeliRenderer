use std::ops::Index;
use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};

const WIDTH: usize = 128;
const HEIGHT: usize = 128;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = match Window::new(
        "Mouse Draw",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    let (mut width, mut height) = (WIDTH, HEIGHT);

    while window.is_open() {
        /*buffer[((200 /*y*/ as usize) * (width)) + 200 /*x*/ as usize] = 0x00ffffff;*/
        for i in 0..buffer.len(){
            buffer[i] = 0xffffffff;
        }

        window.update_with_buffer(&buffer, width, height);
    }
}