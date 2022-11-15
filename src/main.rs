use minifb::{Scale, Window, WindowOptions};
use minifb::Key::{Escape};
use minifb::ScaleMode::AspectRatioStretch;

const WIDTH: usize = 1024;
const HEIGHT: usize = 512;

mod functions;
use functions::*;

fn main() {
    let mut cam_pos:(f64,f64,f64) = (0.0,0.0,-4000.0);
    let mut fov = 1024.0;
    //cube
    let mut cube_ver= vec![
        [256.0,256.0,-256.0],//   0
        [-256.0,256.0,-256.0],//  1
        [256.0,256.0,256.0],//    2
        [-256.0,256.0,256.0],//   3
        [256.0,-256.0,-256.0],//  4
        [-256.0,-256.0,-256.0],// 5
        [256.0,-256.0,256.0],//   6
        [-256.0,-256.0,256.0]//   7
    ];

    let cube_edges = vec![
        [0,1],
        [1,3],
        [3,2],
        [2,0],
        [4,5],
        [5,7],
        [7,6],
        [6,4],
        [0,4],
        [1,5],
        [2,6],
        [3,7]
    ];

    let cube_faces:Vec<(_,_,_,u32)>= vec![
        (0,1,3,0xffafffff),
        (0,2,3,0xffafffff),
        (4,5,7,0xffffafff),
        (4,6,7,0xffffafff),
        (2,0,4,0xffffffaf),
        (2,6,4,0xffffffaf),
        (2,6,7,0xffafffff),
        (2,3,7,0xffafffff),
        (0,1,5,0xffffafff),
        (0,4,5,0xffffafff),
        (3,1,5,0xffffffaf),
        (3,7,5,0xffffffaf)
    ];
    //cube

    //piramida
    let mut pir_ver = vec![
        [1024.0,-256.0,-256.0],//0
        [512.0,-256.0,-256.0],// 1
        [1024.0,-256.0,256.0],// 2
        [512.0,-256.0,256.0],//  3
        [768.0,256.0,0.0]//      4
    ];

    let pir_edges = vec![
        [0,1],
        [1,3],
        [3,2],
        [2,0],
        [0,4],
        [1,4],
        [2,4],
        [3,4]
    ];

    let pir_faces:Vec<(_,_,_,u32)> = vec![
        (0,1,3,0xffafffff),
        (3,2,0,0xffafffff),
        (0,4,1,0xffffafff),
        (1,4,3,0xffffffaf),
        (3,4,2,0xffffafaf),
        (2,4,0,0xffafafff)
    ];
    //piramida

    let mut buffer:Vec<u32>= vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    ).unwrap();

    window.set_position(500, 175);

    /*window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));*/

    let mut cast_cube_ver = vec![[0,0];cube_ver.len()];
    let mut cast_pir_ver = vec![[0,0];cube_ver.len()];

    while window.is_open() && !window.is_key_down(Escape) {
        for (i,v) in cube_ver.iter().enumerate() {
            cast_cube_ver[i] = cast(v,cam_pos,fov)
        }
        for (i,v) in pir_ver.iter().enumerate() {
            cast_pir_ver[i] = cast(v,cam_pos,fov)
        }

        //Draw faces
        /*for i in &cube_faces{
            triangle(&mut buffer,
                     cast_cube_ver[i.0 as usize],
                     cast_cube_ver[i.1 as usize],
                     cast_cube_ver[i.2 as usize],
                     i.3);
        }*/

        for i in &cube_edges {
            line(&mut buffer,
                 cast_cube_ver[i[0] as usize],
                 cast_cube_ver[i[1] as usize],
                 0xff00ff00);
        }

        /*for i in &pir_faces{
            triangle(&mut buffer,
                     cast_pir_ver[i.0 as usize],
                     cast_pir_ver[i.1 as usize],
                     cast_pir_ver[i.2 as usize],
                     i.3);
        }*/

        for i in &pir_edges {
            line(&mut buffer,
            cast_pir_ver[i[0] as usize],
            cast_pir_ver[i[1] as usize],
            0xff00ff00);
        }

        rotate(&mut pir_ver,(768.0,0.0,0.0), 0.004,1);
        rotate(&mut cube_ver,(0.0,0.0,0.0), 0.004,0);
        rotate(&mut cube_ver,(0.0,0.0,0.0), 0.004,1);
        rotate(&mut cube_ver,(0.0,0.0,0.0), 0.004,2);

        /*buffer[((200 /*y*/ as usize) * (WIDTH)) + 200 /*x*/ as usize] = 0x00ffffff;*/

        /*buffer.sort();*/

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Oops!");

        buffer.clear();
        buffer.resize(WIDTH*HEIGHT,0);
    }
}
