use framework::*;

#[derive(Debug, Default)]
pub struct MyApp
{
    rect: DummyBitmap
}

impl App for MyApp
{
    fn render(&self, frame: &mut Frame)
    {
        frame.par_iter_pixels_mut().for_each(|(_, _, px)|
        {
            px.copy_from_slice(&[0xff, 0x00, 0xff, 0xff]);
        });

        frame.paste(&self.rect, 80, 40);
    }

    fn update(&mut self, time: &Time)
    {
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