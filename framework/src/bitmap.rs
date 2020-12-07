/// represents an RGBA bitmap, which can be combined/compared using
/// various operations
pub trait Bitmap
{
    /// get this bitmap's height, in pixels
    fn width(&self) -> usize;

    /// get this bitmap's height, in pixels
    fn height(&self) -> usize;

    /// get the raw pixel bytes in this bitmap
    fn pixels(&self) -> &[u8];

    /// get the raw pixel bytes in this bitmap, mutably
    fn pixels_mut(&mut self) -> &mut [u8];
// }

// impl dyn Bitmap
// {
    /// paste another bitmap on top of this one, clipping any invisible
    /// pixels and (optionally) translating it
    ///
    /// the source bitmap isn't affected
    fn paste(&mut self, src: &impl Bitmap, dx: isize, dy: isize)
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
}