use framework::math::*;
use framework::*;

use crate::obj::Obj;

pub struct MyApp
{
    obj: Obj
}

impl App for MyApp
{
    fn render(&self, frame: &mut Frame)
    {
        // reset frame
        frame.par_iter_pixels_mut().for_each(|(_, px)|
        {
            px.r = 0x00;
            px.g = 0x00;
            px.b = 0x00;
            px.a = 0xff;
        });

        // frame size as a float vector
        let size: Vec2<f32> = Self::SIZE.as_().into();

        for [mut v0, mut v1, mut v2] in self.obj.iter_faces()
        {
            // reframe
            v0 = v0 * 150.0 + size / 2.0;
            v1 = v1 * 150.0 + size / 2.0;
            v2 = v2 * 150.0 + size / 2.0;

            // draw wireframe
            frame.draw_line(v0.xy().as_(), v1.xy().as_(), [0xff, 0xff, 0xff, 0xff]);
            frame.draw_line(v0.xy().as_(), v2.xy().as_(), [0xff, 0xff, 0xff, 0xff]);
            frame.draw_line(v1.xy().as_(), v2.xy().as_(), [0xff, 0xff, 0xff, 0xff]);
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
            obj: Obj::load("res/cone.obj")
        }
    }
}