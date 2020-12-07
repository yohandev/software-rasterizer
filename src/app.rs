use framework::*;

#[derive(Debug, Default)]
pub struct MyApp;

impl App for MyApp
{
    fn render(&self, frame: &mut Frame)
    {
        frame.par_iter_pixels_mut().for_each(|(_, _, px)|
        {
            px.copy_from_slice(&[0xff, 0x00, 0xff, 0xff]);
        });
    }

    fn update(&mut self, time: &Time)
    {
        println!("FPS: {:.1}", 1.0 / time.dt());
    }
}