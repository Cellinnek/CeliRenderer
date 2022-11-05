use crate::WIDTH;
use crate::HEIGHT;
use std::mem::swap;
use minifb::Key::W;

pub fn line(buffer: &mut [u32], (argx1,argy1): (i32,i32), (argx2,argy2): (i32,i32), color: u32) {
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

        err_tolerance = 2 * err;

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

pub fn triangle(buffer: &mut [u32],(mut x1,mut y1):(i32,i32),(mut x2,mut y2):(i32,i32),(mut x3,mut y3):(i32,i32), color:u32){
    let height = HEIGHT as i32;
    let width = WIDTH as i32;

    if y2 > y3
    {
        swap(&mut x2,&mut x3);
        swap(&mut y2,&mut y3);
    }
    if y1 > y2
    {
        swap(&mut x1,&mut x2);
        swap(&mut y1,&mut y2);
    }
    if y2 > y3
    {
        swap(&mut x2,&mut x3);
        swap(&mut y2,&mut y3);
    }

    let dx_far = (x3 - x1) as f32/ (y3 - y1 + 1) as f32;
    let dx_upper = (x2 - x1) as f32 / (y2 - y1 + 1) as f32;
    let dx_low = (x3 - x2) as f32 / (y3 - y2 + 1) as f32;
    let mut xf = x1 as f32;
    let mut xt = x1 as f32 + dx_upper;
        for y in y1..(if y3<height-1{y3} else{height-1}) {
            if y >= 0 {
                for x in (if xf>0.0{xf as i32} else{0})..(if xt < (width-1) as f32{xt as i32} else{width-1}){
                    buffer[(x+y*width) as usize] = color;
                }
                for x in (if xt > 0.0{xt as i32} else{0})..(if xf<width as f32{xf as i32} else{width-1}){
                    buffer[(x+y*width) as usize] = color;
                }
            }
            xf += dx_far;
            if y < y2{xt += dx_upper;}
            else{ xt += dx_low;}
        }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

impl Vertex {
    pub fn cast(self,(cx,cy,cz):(f64,f64,f64), foc:f64) -> (i32,i32){
        ((foc*(self.x+cx)/(foc+(self.z+cz))) as i32 + WIDTH as i32/2, (foc*(self.y+cy)/(foc+(self.z+cz))) as i32 + HEIGHT as i32/2)
    }
}

pub fn rotate(arr: &mut [Vertex], fi: f64, axis: u8) {
    match axis%3{
        0 => for i in arr {
            let (y,z) = (i.y,i.z);
            i.z = z*fi.cos()-y*fi.sin();
            i.y = z*fi.sin()+y*fi.cos();},
        1 => for i in arr {
            let (x,z) = (i.x,i.z);
            i.x = x*fi.cos()-z*fi.sin();
            i.z = x*fi.sin()+z*fi.cos();},
        2 => for i in arr {
            let (x,y) = (i.x,i.y);
            i.y = y*fi.cos()-x*fi.sin();
            i.x = y*fi.sin()+x*fi.cos();},
        _ => println!("Axis error!")
    }
}

