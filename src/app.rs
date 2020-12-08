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
        frame.par_iter_pixels_mut().for_each(|(_, _, px)|
        {
            px.copy_from_slice(&[0xff, 0x00, 0xff, 0xff]);
        });

        frame.draw_bitmap(&self.rect, self.x, self.y as isize);
        frame.draw_line(20, 10, self.x, self.y as isize, &[0x0f, 0xf0, 0xff, 0xff]);
    }

    fn update(&mut self, time: &Time)
    {
        self.x = (f32::sin(time.elapsed().as_secs_f32()) * 400.0 + 200.0) as isize;
        self.y += time.dt() * 10.0;

        println!("FPS: {:.1}", 1.0 / time.dt());
    }
}

impl Default for MyApp
{
    fn default() -> Self
    {
        const WIDTH: usize = 300;
        const HEIGHT: usize = 120;

        let rect = std::iter::repeat(0xff).take(WIDTH * HEIGHT * 4).collect();

        Self
        {
            rect: Bitmap::new(rect, WIDTH, HEIGHT),
            x: 0,
            y: 0.0,
        }
    }
}