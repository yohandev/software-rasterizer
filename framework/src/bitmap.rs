use rayon::prelude::*;

/// represents a bitmap, which can be iterated and
/// drawn to
pub struct Bitmap<T: Buf>
{
    /// inner byte array representing this bitmap
    inner: T,

    /// width, in pixels, of this bitmap
    width: usize,
    /// height, in pixels, of this bitmap
    height: usize,
}

/// restrictions for a type that can be used as a bitmap
/// pixel buffer
pub trait Buf: AsRef<[u8]> + AsMut<[u8]> { }

impl<T: Buf> Bitmap<T>
{
    /// create a new bitmap from its raw parts
    pub fn new(inner: T, width: usize, height: usize) -> Self
    {
        debug_assert_eq!(inner.as_ref().len() % 4, 0);
        debug_assert_eq!(inner.as_ref().len() / 4, width * height);

        Self { inner, width, height }
    }

    /// get this bitmap's width, in pixels
    pub fn width(&self) -> usize
    {
        self.width
    }

    /// get this bitmap's height, in pixels
    pub fn height(&self) -> usize
    {
        self.height
    }

    /// get the raw pixel bytes in this bitmap
    pub fn pixels(&self) -> &[u8]
    {
        self.inner.as_ref()
    }

    /// get the raw pixel bytes in this bitmap, mutably
    pub fn pixels_mut(&mut self) -> &mut [u8]
    {
        self.inner.as_mut()
    }

    /// returns an iterator over the pixels in this bitmap
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
        let w = self.width();
        let h = self.height();

        self.pixels()
            .chunks_exact(4)
            .enumerate()
            .map(move |(i, px)| (i % w, i / h, px))
    }

    /// returns a mutable iterator over the pixels in this bitmap
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
        let w = self.width();
        let h = self.height();

        self.pixels_mut()
            .chunks_exact_mut(4)
            .enumerate()
            .map(move |(i, px)| (i % w, i / h, px))
    }

    /// returns an parallel iterator over the pixels in this bitmap
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
        let w = self.width();
        let h = self.height();

        self.pixels()
            .par_chunks_exact(4)
            .enumerate()
            .map(move |(i, px)| (i % w, i / h, px))
    }

    /// returns a parallel, mutable iterator over the pixels in this bitmap
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
        let w = self.width();
        let h = self.height();

        self.pixels_mut()
            .par_chunks_exact_mut(4)
            .enumerate()
            .map(move |(i, px)| (i % w, i / h, px))
    }

    /// paste another bitmap on top of this one, clipping any invisible
    /// pixels and (optionally) translating it
    ///
    /// the source bitmap isn't affected
    pub fn draw_bitmap(&mut self, src: &Bitmap<impl Buf>, dx: isize, dy: isize)
    {
        // givens
        let dst_width = self.width() as isize;
        let dst_height = self.height() as isize;

        let src_width = src.width() as isize;
        let src_height = src.height() as isize;

        let src_buf = src.pixels();
        let dst_buf = self.pixels_mut();

        // as you iterate src's pixels; [0, src_width] and [0, src_height]
        let src_x_min = (if dx < 0 { -dx } else { 0 }).min(src_width);
        let src_y_min = (if dy < 0 { -dy } else { 0 }).min(src_height);
        let src_x_max = if dx + src_width > dst_width { dst_width - dx } else { src_width };
        let src_y_max = if dy + src_height > dst_height { dst_height - dy } else { src_height };
  
        // as you copy to dst's pixels; [0, dst_width] and [0, dst_height]
        let dst_x_min = if dx < 0 { 0 } else { dx };
        let dst_x_max = dst_x_min + (src_x_max - src_x_min);

        // iterate vertically
        for y in src_y_min..src_y_max
        {
            let src_str = ((y * src_width * 4) + (src_x_min * 4)) as usize;
            let src_end = ((y * src_width * 4) + (src_x_max * 4)) as usize;

            let dst_str = (((y + dy) * dst_width * 4) + (dst_x_min * 4)) as usize;
            let dst_end = (((y + dy) * dst_width * 4) + (dst_x_max * 4)) as usize;

            // copy entire horizontal segments at once
            dst_buf[dst_str..dst_end].copy_from_slice(&src_buf[src_str..src_end]);
        }
    }

    /// draw a single pixel to this bitmap. nothing is drawn
    /// if the pixel is out of bounds
    pub fn draw_pixel(&mut self, x: isize, y: isize, col: &[u8; 4])
    {
        if x >= self.width() as isize
        || y >= self.height() as isize
        || x < 0
        || y < 0
        {
            return;
        }
        let i = (y as usize * self.width() + x as usize) * 4;

        self
            .pixels_mut()[i..i + 4]
            .copy_from_slice(col);
    }

    /// draws a line on top of this bitmap. the line is clipped
    /// if some(or all) of its pixels are out of bounds
    pub fn draw_line(&mut self, mut ax: isize, mut ay: isize, mut bx: isize, mut by: isize, col: &[u8; 4])
    {
        // if steep, reverse the coords
        let steep = if (ax - bx).abs() < (ay - by).abs()
        {
            std::mem::swap(&mut ax, &mut ay);
            std::mem::swap(&mut bx, &mut by);

            true
        }
        else { false };

        // flip the x so that we always start with the lowest x
        if ax > bx
        {
            std::mem::swap(&mut ax, &mut bx); 
            std::mem::swap(&mut ay, &mut by); 
        }

        let dx = bx - ax;
        let dy = by - ay;

        let derror2 = dy.abs() * 2;

        let mut error2 = 0;
        let mut y = ay;
        
        for x in ax..=bx
        {
            if steep { self.draw_pixel(y, x, col); } else { self.draw_pixel(x, y, col); }
            
            error2 += derror2; 
            if error2 > dx
            {
                y += if by > ay {1 } else { -1 };
                error2 -= dx * 2;
            }
        }
    }
}

/// blanket implementation
impl<T: AsRef<[u8]> + AsMut<[u8]>> Buf for T { }