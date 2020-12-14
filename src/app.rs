use framework::math::*;
use framework::*;

use crate::obj::Obj;
use crate::draw;

pub struct MyApp
{
    obj: Obj,
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
        // reset the Z buffer
        let mut depth = [f32::MIN; Self::SIZE.w * Self::SIZE.h];

        // frame size as a float vector
        let size: Vec2<f32> = Self::SIZE.as_().into();

        // iterate object's triangle faces
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

            // draw mesh
            draw::triangle(frame, &mut depth, [v0, v1, v2], col);

            // draw wireframe
            draw::line(frame, pts[0], pts[1], wht);
            draw::line(frame, pts[0], pts[2], wht);
            draw::line(frame, pts[1], pts[2], wht);
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
        }
    }
}