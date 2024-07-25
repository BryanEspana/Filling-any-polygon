mod colors;
mod framebuffer;

use framebuffer::FrameBuffer;
use colors::Color;

fn main() {
    let width = 500;
    let height = 500;
    let mut fb = FrameBuffer::new(width, height);

    let polygon_points = vec![
        (165, 380), (185, 360), (180, 330), (207, 345),
        (233, 330), (230, 360), (250, 380), (220, 385),
        (205, 410), (193, 383),
    ];
    

    // Draw the polygon
    for i in 0..polygon_points.len() {
        let (x1, y1) = polygon_points[i];
        let (x2, y2) = if i == polygon_points.len() - 1 {
            polygon_points[0]
        } else {
            polygon_points[i + 1]
        };
        draw_line(&mut fb, x1, y1, x2, y2, Color::white());
    }

    // Save to file
    fb.save("out.bmp");
}

fn draw_line(fb: &mut FrameBuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        fb.draw_pixel(x as isize, y as isize, color);
        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            if x == x1 { break; }
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            if y == y1 { break; }
            err += dx;
            y += sy;
        }
    }
}
