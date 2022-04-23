fn main() {
    // image

    let image_width = 256;
    let image_height = 256; 

    // render

    println!("P3\n{} {}\n255", image_width, image_height);

    for i in (0..256).rev() {
        eprintln!("\rScanlines remaing: {} ", i);
        for j in 0..256 {

            let pixel_color: Vec3 = Vec3::new(
                (j as f32 / (image_width) as f32) as i32,
                (j as f32 / (image_width) as f32) as i32,
                (j as f32 / (image_width) as f32) as i32, 
            );

            write_color(pixel_color);

        }
    }

}

fn write_color(pixel_color:Vec3) -> () {
    // Write the translated [0,255] value of each color component
    println!(
        "{} {} {}",
        255.999 * pixel_color.x as f32,
        255.999 * pixel_color.y as f32,
        255.999 * pixel_color.z as f32
    )
}

struct Vec3 {
    x:i32,
    y:i32,
    z:i32,
}

impl Vec3 {

    fn new(x:i32, y:i32, z:i32) -> Self {
        return Self {
            x : x,
            y : y,
            z : z, 
        }
    }

}
