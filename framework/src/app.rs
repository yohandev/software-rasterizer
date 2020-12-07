use crate::Frame;

/// represents an application that can be run by the framework
pub trait App
{
    /// the name of this app, which will be displayed as the window
    /// title
    const TITLE: &'static str = "Playground";

    /// render to the window
    fn render(&self, frame: Frame);

    /// update the state of the app
    fn update(&mut self);
}