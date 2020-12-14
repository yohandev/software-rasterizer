use framework::math::*;
use framework::*;

use crate::obj::Obj;
use crate::draw;

pub struct MyApp
{
    light: Vec3<f32>,
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

        // lighting
        let light_dir = self.light.normalized();

        // iterate object's triangle faces
        for [mut v0, mut v1, mut v2] in self.obj.iter_faces()
        {
            // transformation matrix
            let t = Mat3::identity()
                .scaled_3d([150.0, -150.0, 150.0]);

            // reframe
            v0 = v0 * t + size / 2.0;
            v1 = v1 * t + size / 2.0;
            v2 = v2 * t + size / 2.0;

            // lighting
            let n = (v2 - v0).cross(v1 - v0).normalized();
            let l = (n.dot(light_dir)).clamped_minus1_1().powi(2);

            // visible face
            if l > 0.0
            {
                // lighting color
                let col = Rgb::broadcast((l * 255.0) as u8).into();

                // draw mesh
                draw::triangle(frame, &mut depth, [v0, v1, v2], col);

                // // prepare wireframe
                // let pts = [v0.xy().as_(), v1.xy().as_(), v2.xy().as_()];
                // let wht = Rgba::white();

                // // draw wireframe
                // draw::line(frame, pts[0], pts[1], wht);
                // draw::line(frame, pts[0], pts[2], wht);
                // draw::line(frame, pts[1], pts[2], wht);
            }
        }
    }

    fn update(&mut self, time: &Time)
    {
        println!("FPS: {:.1}", 1.0 / time.dt());

        let (s, c) = time.elapsed().as_secs_f32().sin_cos();

        self.light = Vec3::new(c, s, c * s);
    }
}

impl Default for MyApp
{
    fn default() -> Self
    {
        Self
        {
            light: Vec3::zero(),
            obj: Obj::load("res/head.obj"),
        }
    }
}