use minifb::{Scale, Window, WindowOptions};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

mod functions;
use functions::line;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = match Window::new(
        "Mouse Draw",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    }; //error handling

    while window.is_open() {
        /*buffer[((200 /*y*/ as usize) * (WIDTH)) + 200 /*x*/ as usize] = 0x00ffffff;*/
        line(&mut buffer, 10, 10, 256, 256, 0x000fffff);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Oops!");
        buffer = vec![0; WIDTH * HEIGHT];
    }
}
