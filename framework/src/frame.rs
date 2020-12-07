use rayon::prelude::*;

use crate::Bitmap;

/// represents a framebuffer, which can be iterated and
/// drawn to
#[derive(Debug)]
pub struct Frame<'a>
{
    inner: &'a mut [u8],

    width: usize,
    height: usize,
}

impl<'a> Frame<'a>
{
    /// create a new framebuffer
    pub(crate) fn new(inner: &'a mut [u8], width: usize, height: usize) -> Self
    {
        debug_assert_eq!(inner.len() % 4, 0);
        debug_assert_eq!(inner.len() / 4, width * height);

        Self { inner, width, height }
    }

    /// returns an iterator over the pixels in this framebuffer
    ///
    /// ```
    /// for (x, y, pixel) in frame.iter_pixels()
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

    /// returns an parallel iterator over the pixels in this framebuffer
    ///
    /// ```
    /// frame.par_iter_pixels().for_each(|(x, y, pixel)|
    /// {
    ///     if (*pixel[0] > 0)
    ///     {
    ///         println!("round some red!");
    ///     }
    /// });
    ///```
    pub fn par_iter_pixels(&self) -> impl ParallelIterator<Item = (usize, usize, &[u8])> + '_
    {
        let w = self.width;
        let h = self.height;

        self.inner
            .par_chunks_exact(4)
            .enumerate()
            .map(move |(i, px)| (i % w, i / h, px))
    }

    /// returns a parallel, mutable iterator over the pixels in this framebuffer
    ///
    /// ```
    /// frame.par_iter_pixels_mut().for_each(|(x, y, pixel)|
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
    /// });
    ///```
    pub fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = (usize, usize, &mut [u8])> + '_
    {
        let w = self.width;
        let h = self.height;

        self.inner
            .par_chunks_exact_mut(4)
            .enumerate()
            .map(move |(i, px)| (i % w, i / h, px))
    }
}

impl<'a> Bitmap for Frame<'a>
{
    /// get this framebuffer's width, in pixels
    fn width(&self) -> usize
    {
        self.width
    }

    /// get this framebuffer's height, in pixels
    fn height(&self) -> usize
    {
        self.height
    }

    /// get the raw pixel bytes in this frame
    fn pixels(&self) -> &[u8]
    {
        self.inner
    }

    /// get the raw pixel bytes in this frame, mutably
    fn pixels_mut(&mut self) -> &mut [u8]
    {
        self.inner
    }
}