use framework::*;

fn main()
{
    run::<MyApp>();
}

#[derive(Debug, Default)]
struct MyApp;

impl App for MyApp
{
    fn render(&self, frame: &mut Frame)
    {
        for (_, _, px) in frame.iter_pixels_mut()
        {
            px.copy_from_slice(&[0xff, 0x00, 0xff, 0xff]);
        }
    }

    fn update(&mut self, time: &Time)
    {
        println!("FPS: {:.1}", 1.0 / time.dt());
    }
}