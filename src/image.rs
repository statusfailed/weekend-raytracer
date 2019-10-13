//use std::fs::File;

pub struct P3 {
    pub width: usize,
    pub height: usize,
    pub data: Vec<(u8, u8, u8)>
}

pub fn write_ascii(img: P3) {
    // write b"P3\n"
    println!("P3\n{} {}\n255\n", img.width, img.height);

    let mut col: usize = 0;
    for (r, g, b) in img.data {
        print!("{} {} {}\t", r, g, b);
        if col == img.width - 1 {
            print!("\n");
        }
        col += 1;
    }

    return;
}

pub fn test_image() -> P3 {
    let nx = 200;
    let ny = 100;
    let sz = nx * ny;
    let mut data = Vec::with_capacity(sz);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.2_f64;
            let ir = (255.99 * r).round() as u8;
            let ig = (255.99 * g).round() as u8;
            let ib = (255.99 * b).round() as u8;
            data.push((ir, ig, ib));
        }
    }

    return P3 { width: nx, height: ny, data: data }
}
