mod bmp;
mod framebuffer;
mod line;

use raylib::prelude::*;

use framebuffer::Framebuffer;
use line::{dda, draw_line, linea_ecuacion};

fn main() {
    let mut fb = Framebuffer::new(500, 600, Color::BLACK);

    linea_ecuacion(&mut fb, 50, 30, 330, 233, Color::WHITE);
    dda(&mut fb, 50, 90, 330, 293, Color::WHITE);
    draw_line(&mut fb, 50, 150, 330, 353, Color::WHITE);

    let output_file_name = "out.bmp";
    bmp::export(&fb, output_file_name);

    println!("Image saved successfully as '{}'!", output_file_name);
}
