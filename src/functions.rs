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
        if (x as usize)<WIDTH && (y as usize)<HEIGHT{
            buffer[(y*(WIDTH as i32) + x) as usize] = color;
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

pub fn clear(buffer: &mut Vec<u32>,color:u32) {for i in 0..buffer.len(){buffer[i]=color;}}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub x:f32,
    pub y:f32,
    pub z:f32,
}

impl Vertex {
    pub fn cast(&self,(cx,cy,cz):(f32,f32,f32), foc:f32) -> (i32,i32){
        ((foc*(&self.x+cx)/(foc+(&self.z+cz))) as i32, (foc*(&self.y+cy)/(foc+(&self.z+cz))) as i32)
    }
}

pub fn rotate(arr: &mut [Vertex], fi: f32) {
    for i in arr {
        i.x = i.x*fi.cos()-i.z*fi.sin();
        i.z = i.x*fi.sin()+i.z*fi.cos();

        i.z = i.z*fi.cos()-i.y*fi.sin();
        i.y = i.z*fi.sin()+i.y*fi.cos();

        i.y = i.y*fi.cos()-i.x*fi.sin();
        i.x = i.y*fi.sin()+i.x*fi.cos();
    }
}

