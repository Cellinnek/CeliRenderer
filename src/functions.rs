use crate::WIDTH;
use crate::HEIGHT;

pub fn line(buffer: &mut Vec<u32>, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: u32) {
    let mut x = argx1;
    let mut y = argy1;

    let dx = if argx1 > argx2 {
        argx1 - argx2
    } else {
        argx2 - argx1
    };
    let dy = if argy1 > argy2 {
        argy1 - argy2
    } else {
        argy2 - argy1
    };

    let sx = if argx1 < argx2 { 1 } else { -1 };
    let sy = if argy1 < argy2 { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err_tolerance;

    loop {
        if x<WIDTH as i32 && y<HEIGHT as i32{
            buffer[((y as usize) * (WIDTH)) + x as usize] = color;
        };


        if x == argx2 && y == argy2 {
            break;
        };

        err_tolerance = 1 * err;

        if err_tolerance > -dx {
            err -= dy;
            x += sx;
        }
        if err_tolerance < dy {
            err += dx;
            y += sy;
        }
    }
}