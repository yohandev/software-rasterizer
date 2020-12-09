use rayon::prelude::*;

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
    pub fn iter_pixels(&self) -> impl Iterator<Item = (Vec2<usize>, &Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels()
            .iter()
            .enumerate()
            .map(move |(i, px)| (Vec2::new(i % w, i / h), px))
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
    pub fn iter_pixels_mut(&mut self) -> impl Iterator<Item = (Vec2<usize>, &mut Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels_mut()
            .iter_mut()
            .enumerate()
            .map(move |(i, px)| (Vec2::new(i % w, i / h), px))
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
    pub fn par_iter_pixels(&self) -> impl ParallelIterator<Item = (Vec2<usize>, &Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels()
            .par_iter()
            .enumerate()
            .map(move |(i, px)| (Vec2::new(i % w, i / h), px))
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
    pub fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = (Vec2<usize>, &mut Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels_mut()
            .par_iter_mut()
            .enumerate()
            .map(move |(i, px)| (Vec2::new(i % w, i / h), px))
    }

    /// paste another bitmap on top of this one, clipping any invisible
    /// pixels and (optionally) translating it
    ///
    /// the source bitmap isn't affected
    pub fn draw_bitmap(&mut self, src: &Bitmap<impl Buf>, pos: impl Into<Vec2<isize>>)
    {
        // convert
        let pos = pos.into();

        // givens
        let dst_size: Vec2<isize> = self.size().as_::<isize>().into();
        let src_size: Vec2<isize> = src.size().as_::<isize>().into();

        let src_buf = src.raw_pixels();
        let dst_buf = self.raw_pixels_mut();

        // as you iterate src's pixels; [0, src_width] and [0, src_height]
        let src_min = pos.map2(src_size, |p, s| (if p < 0 { -p } else { 0 }).min(s));
        let src_max = pos.map3(src_size, dst_size, |p, ss, ds| if p + ss > ds { ds - p } else { ss });
  
        // as you copy to dst's pixels; [0, dst_width] and [0, dst_height]
        let dst_min_x = if pos.x < 0 { 0 } else { pos.x };
        let dst_max_x = dst_min_x + (src_max.x - src_min.x);

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

    /// draw a single pixel to this bitmap. nothing is drawn
    /// if the pixel is out of bounds
    pub fn draw_pixel(&mut self, pos: impl Into<Vec2<isize>>, col: impl Into<Rgba<u8>>)
    {
        // convert
        let pos = pos.into();

        if pos.x >= self.width() as isize
        || pos.y >= self.height() as isize
        || pos.x < 0
        || pos.y < 0
        {
            return;
        }
        let i = (pos.y as usize * self.width() + pos.x as usize) * 4;

        self
            .raw_pixels_mut()[i..i + 4]
            .copy_from_slice(&col.into());
    }

    /// draws a line on top of this bitmap. the line is clipped
    /// if some(or all) of its pixels are out of bounds
    pub fn draw_line(&mut self, a: impl Into<Vec2<isize>>, b: impl Into<Vec2<isize>>, col: impl Into<Rgba<u8>>)
    {
        // convert
        let mut a = a.into();
        let mut b = b.into();
        let col = col.into();

        // if steep, reverse the coords
        let steep = if (a.x - b.x).abs() < (a.y - b.y).abs()
        {
            std::mem::swap(&mut a.x, &mut a.y);
            std::mem::swap(&mut b.x, &mut b.y);

            true
        }
        else { false };

        // flip the x so that we always start with the lowest x
        if a.x > b.x
        {
            std::mem::swap(&mut a.x, &mut b.x); 
            std::mem::swap(&mut a.y, &mut b.y); 
        }

        let d = b - a;          // delta
        let de = d.y.abs() * 2; // slope error increment(0.5)

        let mut e = 0;          // slope error(0.5)
        let mut y = a.y;        // starting y
        
        // begin drawing
        for x in a.x..=b.x
        {
            // set the color
            self.draw_pixel(if steep { (y, x) } else { (x, y) }, col); 
            
            // increment slope error
            e += de; 
            if e > d.x
            {
                y += if b.y > a.y { 1 } else { -1 };
                e -= d.x * 2;
            }
        }
    }
}

/// blanket implementation
impl<T: AsRef<[u8]> + AsMut<[u8]>> Buf for T { }