use framework::math::*;
use framework::*;

pub struct MyApp
{
    rect: Bitmap<Vec<u8>>,
    x: isize,
    y: f32,
}

impl App for MyApp
{
    fn render(&self, frame: &mut Frame)
    {
        frame.par_iter_pixels_mut().for_each(|(_, px)|
        {
            px.r = 0xff;
            px.g = 0x0f;
            px.b = 0xff;
            px.a = 0xff;
        });

        frame.draw_bitmap(&self.rect, (self.x, self.y as isize));
        frame.draw_line((200, -10), (self.x, self.y as isize), [0x0f, 0xf0, 0xff, 0xff]);
    }

    fn update(&mut self, time: &Time)
    {
        self.x = (f32::sin(time.elapsed().as_secs_f32()) * 400.0 + 210.0) as isize;
        self.y += time.dt() * 50.0;

        println!("FPS: {:.1}", 1.0 / time.dt());
    }
}

impl Default for MyApp
{
    fn default() -> Self
    {
        const RECT_SIZE: Extent2<usize> = Extent2::new(300, 120);

        let rect = std::iter::repeat(0xff).take(RECT_SIZE.w * RECT_SIZE.h * 4).collect();

        Self
        {
            rect: Bitmap::new(rect, RECT_SIZE),
            x: 0,
            y: 0.0,
        }
    }
}