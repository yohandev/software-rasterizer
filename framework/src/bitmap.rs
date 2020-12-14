use rayon::prelude::*;

use crate::util::{ Bresenham, Triangle };
use crate::math::*;

/// represents a bitmap, which can be iterated and
/// drawn to
pub struct Bitmap<T: Buf>
{
    /// inner byte array representing this bitmap
    inner: T,
    /// width and height, in pixels, of this bitmap
    size: Extent2<usize>,
}

/// restrictions for a type that can be used as a bitmap
/// pixel buffer
pub trait Buf: AsRef<[u8]> + AsMut<[u8]> { }

impl<T: Buf> Bitmap<T>
{
    /// create a new bitmap from its raw parts
    pub fn new(inner: T, size: impl Into<Extent2<usize>>) -> Self
    {
        // convert
        let size = size.into();

        debug_assert_eq!(inner.as_ref().len() % 4, 0);
        debug_assert_eq!(inner.as_ref().len() / 4, size.w * size.h);

        Self { inner, size }
    }

    /// get this bitmap's width and height, in pixels
    #[inline]
    pub fn size(&self) -> Extent2<usize>
    {
        self.size
    }

    /// get this bitmap's width, in pixels
    #[inline]
    pub fn width(&self) -> usize
    {
        self.size.w
    }

    /// get this bitmap's height, in pixels
    #[inline]
    pub fn height(&self) -> usize
    {
        self.size.h
    }

    /// get this bitmap's area(width * height), in pixels
    #[inline]
    pub fn area(&self) -> usize
    {
        self.width() * self.height()
    }

    /// get the raw pixel bytes in this bitmap
    ///
    /// returns a slice of size width * height * 4
    #[inline]
    pub fn raw_pixels(&self) -> &[u8]
    {
        self.inner.as_ref()
    }

    /// get the raw pixel bytes in this bitmap, mutably
    ///
    /// returns a slice of size width * height * 4
    #[inline]
    pub fn raw_pixels_mut(&mut self) -> &mut [u8]
    {
        self.inner.as_mut()
    }

    /// get the pixels in this bitmap
    ///
    /// returns a slice of size width * height
    #[inline]
    pub fn pixels(&self) -> &[Rgba<u8>]
    {
        use std::slice::from_raw_parts as slice;
        unsafe
        {
            slice(self.raw_pixels().as_ptr() as *const Rgba<u8>, self.area())
        }
    }

    /// get the pixels in this bitmap, mutably
    ///
    /// returns a slice of size width * height
    #[inline]
    pub fn pixels_mut(&mut self) -> &mut [Rgba<u8>]
    {
        use std::slice::from_raw_parts_mut as slice;
        unsafe
        {
            slice(self.raw_pixels_mut().as_ptr() as *mut Rgba<u8>, self.area())
        }
    }

    /// returns an iterator over the pixels in this bitmap
    ///
    /// ```
    /// for (pos, pixel) in frame.iter_pixels()
    /// {
    ///     if (*pixel[0] > 0)
    ///     {
    ///         println!("round some red!");
    ///     }
    /// }
    ///```
    pub fn iter_pixels(&self) -> impl Iterator<Item = (Vec2<i32>, &Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels()
            .iter()
            .enumerate()
            .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
    }

    /// returns a mutable iterator over the pixels in this bitmap
    ///
    /// ```
    /// for (pos, pixel) in frame.iter_pixels_mut()
    /// {
    ///     // creates a black and white stripe pattern
    ///     if pos.x % 2 == 0
    ///     {
    ///         pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
    ///     }
    ///     else
    ///     {
    ///         pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
    ///     }
    /// }
    ///```
    pub fn iter_pixels_mut(&mut self) -> impl Iterator<Item = (Vec2<i32>, &mut Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels_mut()
            .iter_mut()
            .enumerate()
            .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
    }

    /// returns an parallel iterator over the pixels in this bitmap
    ///
    /// ```
    /// frame.par_iter_pixels().for_each(|(pos, pixel)|
    /// {
    ///     if (*pixel[0] > 0)
    ///     {
    ///         println!("round some red!");
    ///     }
    /// });
    ///```
    pub fn par_iter_pixels(&self) -> impl ParallelIterator<Item = (Vec2<i32>, &Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels()
            .par_iter()
            .enumerate()
            .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
    }

    /// returns a parallel, mutable iterator over the pixels in this bitmap
    ///
    /// ```
    /// frame.par_iter_pixels_mut().for_each(|(pos, pixel)|
    /// {
    ///     // creates a black and white stripe pattern
    ///     if pos.x % 2 == 0
    ///     {
    ///         pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
    ///     }
    ///     else
    ///     {
    ///         pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
    ///     }
    /// });
    ///```
    pub fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = (Vec2<i32>, &mut Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels_mut()
            .par_iter_mut()
            .enumerate()
            .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
    }

    /// draw a single pixel to this bitmap. panics if out of bounds
    #[inline]
    pub fn set(&mut self, pos: Vec2<i32>, col: Rgba<u8>)
    {
        // index
        let ind = (pos.y as usize * self.width() + pos.x as usize) * 4;

        self
            .raw_pixels_mut()[ind..ind + 4]
            .copy_from_slice(&col);
    }

    /// paste another bitmap on top of this one, clipping any invisible
    /// pixels and (optionally) translating it
    ///
    /// the source bitmap isn't affected
    pub fn paste(&mut self, pos: Vec2<i32>, src: &Bitmap<impl Buf>)
    {
        // givens
        let dst_size: Vec2<i32> = self.size().as_::<i32>().into();
        let src_size: Vec2<i32> = src.size().as_::<i32>().into();

        let src_buf = src.raw_pixels();
        let dst_buf = self.raw_pixels_mut();

        // as you iterate src's pixels; [0, src_width] and [0, src_height]
        let src_min = pos.map2(src_size, |p, s| (if p < 0 { -p } else { 0 }).min(s));
        let src_max = pos.map3(src_size, dst_size, |p, ss, ds| if p + ss > ds { ds - p } else { ss });
  
        // as you copy to dst's pixels; [0, dst_width] and [0, dst_height]
        let dst_min_x = if pos.x < 0 { 0 } else { pos.x };
        let dst_max_x = dst_min_x + (src_max.x - src_min.x);

        // nothing to copy
        if dst_max_x < dst_min_x
        {
            return;
        }

        // iterate vertically
        for y in src_min.y..src_max.y
        {
            let src_str = ((y * src_size.x * 4) + (src_min.x * 4)) as usize;
            let src_end = ((y * src_size.x * 4) + (src_max.x * 4)) as usize;

            let dst_str = (((y + pos.y) * dst_size.x * 4) + (dst_min_x * 4)) as usize;
            let dst_end = (((y + pos.y) * dst_size.x * 4) + (dst_max_x * 4)) as usize;

            // copy entire horizontal segments at once
            dst_buf[dst_str..dst_end].copy_from_slice(&src_buf[src_str..src_end]);
        }
    }

    /// draws a line on top of this bitmap. the line is clipped
    /// if some(or all) of its pixels are out of bounds
    pub fn draw_line(&mut self, a: Vec2<i32>, b: Vec2<i32>, col: Rgba<u8>)
    {
        // convert
        let max = self.size().as_::<i32>();

        for p in Bresenham::new_bounded(a, b, max)
        {
            // draw line
            self.set(p, col);
        }
    }

    /// draws a triangle on top of this bitmap. the triangle is
    /// clipped if some(or all) of its pixels are out of bounds
    pub fn draw_triangle(&mut self, pts: [Vec2<i32>; 3], col: Rgba<u8>)
    {
        // convert
        let max = self.size().as_();

        for p in Triangle::new_bounded(pts, max)
        {
            // draw triangle
            self.set(p, col);
        }
    }
}

/// blanket implementation
impl<T: AsRef<[u8]> + AsMut<[u8]>> Buf for T { }