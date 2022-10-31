use minifb::{Scale, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

mod functions;
use functions::{line,Vertex,rotate,clear};

fn main() {
    let mut cam_pos:(f32,f32,f32) = (0.0,0.0,1024.0);
    let mut fov:f32 = 512.0;

    let mut ver: [Vertex; 8] = [
        Vertex{x:256.0,y:256.0,z:-256.0},//   0
        Vertex{x:-256.0,y:256.0,z:-256.0},//  1
        Vertex{x:256.0,y:256.0,z:256.0},//    2
        Vertex{x:-256.0,y:256.0,z:256.0},//   3
        Vertex{x:256.0,y:-256.0,z:-256.0},//  4
        Vertex{x:-256.0,y:-256.0,z:-256.0},// 5
        Vertex{x:256.0,y:-256.0,z:256.0},//   6
        Vertex{x:-256.0,y:-256.0,z:256.0}//   7
    ];

    let cube_edges: [[u16;2]; 12] = [
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

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    ).unwrap();

    /*window.limit_update_rate(Some(std::time::Duration::from_micros(16600/2)));*/

    let mut now = Instant::now();
    let now2 = Instant::now();
    while window.is_open() {

        rotate(&mut ver,0.5*3.14/180.0);
        let cast_vet: [(i32,i32); 8] = [
            ver[0].cast(cam_pos,fov),
            ver[1].cast(cam_pos,fov),
            ver[2].cast(cam_pos,fov),
            ver[3].cast(cam_pos,fov),
            ver[4].cast(cam_pos,fov),
            ver[5].cast(cam_pos,fov),
            ver[6].cast(cam_pos,fov),
            ver[7].cast(cam_pos,fov)];

        for i in cube_edges {
            let x1 = cast_vet[i[0] as usize].0+WIDTH as i32/2;
            let y1 = cast_vet[i[0] as usize].1+HEIGHT as i32/2;
            let x2 = cast_vet[i[1] as usize].0+WIDTH as i32/2;
            let y2 = cast_vet[i[1] as usize].1+HEIGHT as i32/2;
            line(&mut buffer,x1,y1, x2,y2, 0xffffffff);
        }

        /*buffer[((200 /*y*/ as usize) * (WIDTH)) + 200 /*x*/ as usize] = 0x00ffffff;*/

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        clear(&mut buffer, 0);
        let elapsed_time = 1.0/now.elapsed().as_secs_f32();
        now = Instant::now();
        if now.duration_since(now2).as_millis()%30==0{
            window.set_title(&(elapsed_time as i32 *2).to_string());
        }

    }
}
