use framework::util::{ Bresenham, Triangle };
use framework::math::*;
use framework::*;

use crate::obj::Obj;

pub struct MyApp
{
    obj: Obj,
    z_buf: [f32; AREA],
}

impl App for MyApp
{
    fn render(&mut self, frame: &mut Frame)
    {
        // reset frame
        frame.par_iter_pixels_mut().for_each(|(_, px)|
        {
            px.r = 0x00;
            px.g = 0x00;
            px.b = 0x00;
            px.a = 0xff;
        });
        // reset zbuf
        let mut z_buf = [f32::MIN; AREA];

        // frame size as a float vector
        let size: Vec2<f32> = Self::SIZE.as_().into();

        for [mut v0, mut v1, mut v2] in self.obj.iter_faces()
        {
            // transformation matrix
            let t = Mat3::identity()
                .scaled_3d([150.0, -150.0, 150.0]);

            // color
            let r = (((v0.x + v1.x + v2.x) / 3.0).clamped01() * 255.0) as u8;
            let g = (((v0.y + v1.y + v2.y) / 3.0).clamped01() * 255.0) as u8;
            let b = (((v0.z + v1.z + v2.z) / 3.0).clamped01() * 255.0) as u8;

            // reframe
            v0 = v0 * t + size / 2.0;
            v1 = v1 * t + size / 2.0;
            v2 = v2 * t + size / 2.0;

            // convert
            let pts = [v0.xy().as_(), v1.xy().as_(), v2.xy().as_()];
            let col = [r, g, b, 0xff].into();
            let wht = Rgba::white();

            // draw wireframe
            draw_triangle(frame, pts, col);
            draw_line(frame, pts[0], pts[1], wht);
            draw_line(frame, pts[0], pts[2], wht);
            draw_line(frame, pts[1], pts[2], wht);
        }
    }

    fn update(&mut self, time: &Time)
    {
        println!("FPS: {:.1}", 1.0 / time.dt());
    }
}

impl Default for MyApp
{
    fn default() -> Self
    {
        Self
        {
            obj: Obj::load("res/head.obj"),
            z_buf: [f32::MIN; AREA],
        }
    }
}

/// width * height, in pixels
const AREA: usize = MyApp::SIZE.w * MyApp::SIZE.h;

/// draw a pixel to the frame at the given position. panics if
/// out of bound
fn draw_pixel(frame: &mut Frame, zbuf: &[f32; AREA], pos: Vec2<i32>, col: Rgba<u8>)
{
    frame.set(pos, col);
}

/// draws a line to the frame. the line is clipped if some(or all)
/// of its pixels are out of bounds
fn draw_line(frame: &mut Frame,  zbuf: &[f32; AREA], a: Vec2<i32>, b: Vec2<i32>, col: Rgba<u8>)
{
    // convert
    let max = frame.size().as_();

    for p in Bresenham::new_bounded(a, b, max)
    {
        // draw line
        draw_pixel(frame, p, col);
    }
}

/// draws a triangle on top of the frame. the triangle is
/// clipped if some(or all) of its pixels are out of bounds
fn draw_triangle(frame: &mut Frame, zbuf: &[f32; AREA], pts: [Vec2<i32>; 3], col: Rgba<u8>)
{
    // convert
    let max = frame.size().as_();

    for p in Triangle::new_bounded(pts, max)
    {
        // draw triangle
        draw_pixel(frame, p, col);
    }
}