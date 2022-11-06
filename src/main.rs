use minifb::{Scale, Window, WindowOptions};
use minifb::Key::{Escape};
use minifb::ScaleMode::AspectRatioStretch;
use fps_counter::FPSCounter;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

mod functions;
use functions::*;

fn main() {
    let mut cam_pos:(f64,f64,f64) = (0.0,0.0,-1800.0);
    let mut fov = WIDTH as f64;

    let mut ver= vec![
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

    let mut cast_ver = vec![(0,0);ver.len()];

    let mut fps = FPSCounter::new();

    while window.is_open() && !window.is_key_down(Escape) {
        for (i,v) in ver.iter().enumerate() {
            cast_ver[i] = cast(v,cam_pos,fov)
        }

        //Draw faces
        for i in &cube_faces{
            triangle(&mut buffer,
                     cast_ver[i.0 as usize],
                     cast_ver[i.1 as usize],
                     cast_ver[i.2 as usize],
                     i.3);
        }

        //Drawing edges
        /*for i in &cube_edges {
            line(&mut buffer,
            cast_ver[i[0] as usize],
            cast_ver[i[1] as usize],
            0xff00ff00);
        }
*/
        rotate(&mut ver, 1.0/fps.tick() as f64,0);
        rotate(&mut ver, 1.0/fps.tick() as f64,1);
        rotate(&mut ver, 1.0/fps.tick() as f64,2);

        /*buffer[((200 /*y*/ as usize) * (WIDTH)) + 200 /*x*/ as usize] = 0x00ffffff;*/

        /*buffer.sort();*/

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Oops!");


        buffer.clear();
        buffer.resize(WIDTH*HEIGHT,0);
        println!("{}",fps.tick());
    }
}
