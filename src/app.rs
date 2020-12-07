use framework::*;

#[derive(Debug, Default)]
pub struct MyApp
{
    rect: DummyBitmap,
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

        frame.paste(&self.rect, self.x, self.y as isize);
    }

    fn update(&mut self, time: &Time)
    {
        self.x = (f32::sin(time.elapsed().as_secs_f32()) * 400.0 + 200.0) as isize;
        self.y += time.dt() * 10.0;

        println!("FPS: {:.1}", 1.0 / time.dt());
    }
}

#[derive(Debug)]
struct DummyBitmap(Vec<u8>);

impl DummyBitmap
{
    const WIDTH: usize = 300;
    const HEIGHT: usize = 120;
}

impl Default for DummyBitmap
{
    fn default() -> Self
    {
        Self(std::iter::repeat(0xff).take(Self::WIDTH * Self::HEIGHT * 4).collect())
    }
}

impl Bitmap for DummyBitmap
{
    fn width(&self) -> usize
    {
        Self::WIDTH
    }

    fn height(&self) -> usize
    {
        Self::HEIGHT
    }

    fn pixels(&self) -> &[u8]
    {
        &self.0
    }

    fn pixels_mut(&mut self) -> &mut [u8]
    {
        &mut self.0
    }
}