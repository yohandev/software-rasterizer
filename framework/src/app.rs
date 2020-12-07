use crate::{ Frame, Time };

/// represents an application that can be run by the framework
pub trait App: 'static
{
    /// the name of this app, which will be displayed as the window
    /// title
    const TITLE: &'static str = "Playground";

    /// the default width(in pixels) for the window and framebuffer
    const WIDTH: usize = 600;
    /// the default height(in pixels) for the window and framebuffer
    const HEIGHT: usize = 400;

    /// render to the window
    fn render(&self, frame: &mut Frame);

    /// update the state of the app
    fn update(&mut self, time: &Time);
}