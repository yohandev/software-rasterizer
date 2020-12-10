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

    /// draw a single pixel to this bitmap. panics if out of bounds
    #[inline]
    pub fn draw_pixel(&mut self, pos: impl Into<Vec2<isize>>, col: impl Into<Rgba<u8>>)
    {
        // convert
        let pos = pos.into();
        // index
        let ind = (pos.y as usize * self.width() + pos.x as usize) * 4;

        self
            .raw_pixels_mut()[ind..ind + 4]
            .copy_from_slice(&col.into());
    }

    /// paste another bitmap on top of this one, clipping any invisible
    /// pixels and (optionally) translating it
    ///
    /// the source bitmap isn't affected
    pub fn draw_bitmap(&mut self, src: &Bitmap<impl Buf>, pos: impl Into<Vec2<isize>>)
    {
        // convert
        let pos: Vec2<isize> = pos.into();

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
    ///
    /// algorithm from http://stackoverflow.com/a/40902741/432509
    pub fn draw_line(&mut self, a: impl Into<Vec2<isize>>, b: impl Into<Vec2<isize>>, col: impl Into<Rgba<u8>>)
    {
        // convert
        let mut a: Vec2<isize> = a.into();
        let mut b: Vec2<isize> = b.into();
        let col = col.into();

        // bounds
        let mut max: Vec2<isize> = self.size().as_::<isize>().into();
        let mut min: Vec2<isize> = Vec2::zero();

        // Vertical line
        if a.x == b.x
        {
            if a.x < min.x || a.x >= max.x
            {
                return;
            }
            if a.y <= b.y
            {
                if b.y < min.y || a.y >= max.y
                {
                    return;
                }
                a.y = a.y.max(min.y);
                b.y = b.y.min(max.y);
                for y in a.y..b.y
                {
                    self.draw_pixel((a.x, y), col);
                }
            }
            else
            {
                if a.y < min.y || b.y >= max.y
                {
                    return;
                }
                a.y = a.y.min(max.y);
                b.y = b.y.max(min.y);
                for y in (b.y..a.y).rev()
                {
                    self.draw_pixel((a.x, y), col);
                }
            }
            return;
        }

        // Horizontal line
        if a.y == b.y
        {
            if a.y < min.y || a.y >= max.y
            {
                return;
            }

            if a.x <= b.x
            {
                if b.x < min.x || a.x >= max.x
                {
                    return;
                }
                a.x = a.x.max(min.x);
                b.x = b.x.min(max.x);
                for x in a.x..b.x
                {
                    self.draw_pixel((x, a.y), col);
                }
            }
            else
            {
                if a.x < min.x || b.x >= max.x
                {
                    return;
                }
                a.x = a.x.min(max.x);
                b.x = b.x.max(min.x);
                for x in b.x..a.x
                {
                    self.draw_pixel((x, a.y), col);
                }
            }
            return;
        }

        // Now simple cases are handled, perform clipping checks.
        let sign_x;
        let sign_y;

        if a.x < b.x
        {
            if a.x >= max.x || b.x < min.x
            {
                return;
            }
            sign_x = 1;
        }
        else
        {
            if b.x >= max.x || a.x < min.x
            {
                return;
            }
            sign_x = -1;

            // Invert sign, invert again right before plotting.
            a.x = -a.x;
            b.x = -b.x;

            min.x = -min.x;
            max.x = -max.x;

            std::mem::swap(&mut min.x, &mut max.x);
        }

        if a.y < b.y
        {
            if a.y >= max.y || b.y < min.y
            {
                return;
            }
            sign_y = 1;
        }
        else
        {
            if b.y >= max.y || a.y < min.y
            {
                return;
            }
            sign_y = -1;

            // Invert sign, invert again right before plotting.
            a.y = -a.y;
            b.y = -b.y;

            min.y = -min.y;
            max.y = -max.y;

            std::mem::swap(&mut min.y, &mut max.y);
        }

        let delta_x = b.x - a.x;
        let delta_y = b.y - a.y;

        let mut delta_x_step = 2 * delta_x;
        let mut delta_y_step = 2 * delta_y;

        // Plotting values
        let mut x_pos = a.x;
        let mut y_pos = a.y;

        if delta_x >= delta_y
        {
            let mut error = delta_y_step - delta_x;
            let mut set_exit = false;

            // Line starts below the clip window.
            if a.y < min.y
            {
                let temp = (2 * (min.y - a.y) - 1) * delta_x;
                let msd = temp / delta_y_step;
                x_pos += msd;

                // Line misses the clip window entirely.
                if x_pos >= max.x
                {
                    return;
                }

                // Line starts.
                if x_pos >= min.x
                {
                    let rem = temp - msd * delta_y_step;

                    y_pos = min.y;
                    error -= rem + delta_x;

                    if rem > 0
                    {
                        x_pos += 1;
                        error += delta_y_step
                    }
                    set_exit = true;
                }
            }

            // Line starts left of the clip window.
            if !set_exit && a.x < min.x
            {
                let temp = delta_y_step * (min.x - a.x);
                let msd = temp / delta_x_step;
                y_pos += msd;
                let rem = temp % delta_x_step;

                // Line misses clip window entirely.
                if y_pos > max.y || (y_pos == max.y && rem >= delta_x)
                {
                    return;
                }

                x_pos = min.x;
                error += rem;

                if rem >= delta_x
                {
                    y_pos += 1;
                    error -= delta_x_step;
                }
            }

            let mut x_pos_end = b.x;
            if b.y >= max.y
            {
                let temp = delta_x_step * (max.y - a.y) + delta_x;
                let msd = temp / delta_y_step;
                x_pos_end = a.x + msd;

                if (temp - msd * delta_y_step) == 0
                {
                    x_pos_end -= 1;
                }
            }

            x_pos_end = x_pos_end.min(max.x);

            if sign_y == -1
            {
                y_pos = -y_pos
            }

            if sign_x == -1
            {
                x_pos = -x_pos;
                x_pos_end = -x_pos_end;
            }

            delta_x_step -= delta_y_step;

            while x_pos != x_pos_end
            {
                self.draw_pixel((x_pos, y_pos), col);

                if error >= 0
                {
                    y_pos += sign_y;
                    error -= delta_x_step;
                }
                else
                {
                    error += delta_y_step;
                }

                x_pos += sign_x;
            }
        }
        else
        {
            // Line is steep '/' (delta_x < delta_y).
            // Same as previous block of code with std::mem::swapped x/y axis.

            let mut error = delta_x_step - delta_y;
            let mut set_exit = false;

            // Line starts left of the clip window.
            if a.x < min.x
            {
                let temp = (2 * (min.x - a.x) - 1) * delta_y;
                let msd = temp / delta_x_step;
                y_pos += msd;

                // Line misses the clip window entirely.
                if y_pos >= max.y
                {
                    return;
                }

                // Line starts.
                if y_pos >= min.y
                {
                    let rem = temp - msd * delta_x_step;

                    x_pos = min.x;
                    error -= rem + delta_y;

                    if rem > 0
                    {
                        y_pos += 1;
                        error += delta_x_step;
                    }
                    set_exit = true;
                }
            }

            // Line starts below the clip window.
            if !set_exit && a.y < min.y
            {
                let temp = delta_x_step * (min.y - a.y);
                let msd = temp / delta_y_step;
                x_pos += msd;
                let rem = temp % delta_y_step;

                // Line misses clip window entirely.
                if x_pos > max.x || (x_pos == max.x && rem >= delta_y)
                {
                    return;
                }

                y_pos = min.y;
                error += rem;

                if rem >= delta_y
                {
                    x_pos += 1;
                    error -= delta_y_step;
                }
            }

            let mut y_pos_end = b.y;
            if b.x >= max.x
            {
                let temp = delta_y_step * (max.x - a.x) + delta_y;
                let msd = temp / delta_x_step;
                y_pos_end = a.y + msd;

                if (temp - msd * delta_x_step) == 0
                {
                    y_pos_end -= 1;
                }
            }

            y_pos_end = y_pos_end.min(max.y);
            if sign_x == -1
            {
                x_pos = -x_pos;
            }

            if sign_y == -1
            {
                y_pos = -y_pos;
                y_pos_end = -y_pos_end;
            }
            delta_y_step -= delta_x_step;

            while y_pos != y_pos_end
            {
                self.draw_pixel((x_pos, y_pos), col);

                if error >= 0
                {
                    x_pos += sign_x;
                    error -= delta_y_step;
                }
                else
                {
                    error += delta_x_step;
                }

                y_pos += sign_y;
            }
        }
    }
}

/// blanket implementation
impl<T: AsRef<[u8]> + AsMut<[u8]>> Buf for T { }