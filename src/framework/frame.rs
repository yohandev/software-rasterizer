/// represents a framebuffer, which can be iterated and
/// drawn to
pub struct Frame<'a>
{
    inner: &'a mut [u8],

    width: usize,
    height: usize,
}

impl<'a> Frame<'a>
{
    /// get this framebuffer's width, in pixels
    pub fn width(&self) -> usize
    {
        self.width
    }

    /// get this framebuffer's height, in pixels
    pub fn height(&self) -> usize
    {
        self.height
    }

    /// get the raw pixel bytes in this frame
    pub fn pixels(&self) -> &[u8]
    {
        self.inner
    }

    /// get the raw pixel bytes in this frame, mutably
    pub fn pixels_mut(&mut self) -> &mut [u8]
    {
        self.inner
    }

    /// returns an iterator over the pixels in this framebuffer
    ///
    /// ```
    /// for (x, y, pixel) in frame.iter_pixels_mut()
    /// {
    ///     if (*pixel[0] > 0)
    ///     {
    ///         println!("round some red!");
    ///     }
    /// }
    ///```
    pub fn iter_pixels(&self) -> impl Iterator<Item = (usize, usize, &[u8])> + '_
    {
        let w = self.width;
        let h = self.height;

        self.inner
            .chunks_exact(4)
            .enumerate()
            .map(move |(i, px)| (i % w, i / h, px))
    }

    /// returns a mutable iterator over the pixels in this framebuffer
    ///
    /// ```
    /// for (x, y, pixel) in frame.iter_pixels_mut()
    /// {
    ///     // creates a black and white stripe pattern
    ///     if x % 2 == 0
    ///     {
    ///         pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
    ///     }
    ///     else
    ///     {
    ///         pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
    ///     }
    /// }
    ///```
    pub fn iter_pixels_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut [u8])> + '_
    {
        let w = self.width;
        let h = self.height;

        self.inner
            .chunks_exact_mut(4)
            .enumerate()
            .map(move |(i, px)| (i % w, i / h, px))
    }
}