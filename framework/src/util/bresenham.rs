use crate::math::*;

/// iterator to trace/draw lines using Bresenham's line drawing
/// algorithm
pub struct Bresenham
{
    // (current x, current y)
    cur: Vec2<isize>,

    // delta.x
    dx: isize,
    // delta.y.signum()
    dy: isize,

    // error
    e: isize,
    // delta error
    de: isize,

    // whether to flip (x,y) to (y, x)
    steep: bool,

    // last x position, inclusive
    end: isize,
}

impl Bresenham
{
    /// create a new iterator that yields points from a to b, inclusive
    #[inline]
    pub fn new(a: impl Into<Vec2<isize>>, b: impl Into<Vec2<isize>>) -> Self
    {
        // convert
        let mut a: Vec2<isize> = a.into();
        let mut b: Vec2<isize> = b.into();

        // adjust slope
        let steep = if (a.x - b.x).abs() < (a.y - b.y).abs()
        {
            std::mem::swap(&mut a.x, &mut a.y);
            std::mem::swap(&mut b.x, &mut b.y);

            true
        }
        else
        {
            false
        };

        // flip the x so that we always start with the lowest x
        if a.x > b.x
        {
            std::mem::swap(&mut a.x, &mut b.x); 
            std::mem::swap(&mut a.y, &mut b.y); 
        }

        // delta
        let d = b - a;
        
        Self
        {
            cur: a,             // starting x and y
            dx: d.x,            // delta x
            dy: d.y.signum(),   // delta y
            e: 0,               // slope error(0.5)
            de: d.y.abs() * 2,  // slope error increment(0.5)
            steep,              // steep or not
            end: b.x,           // ending x
        }
    }
}

impl Iterator for Bresenham
{
    type Item = Vec2<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        let out;
        if self.cur.x > self.end
        {
            // done
            out = None;
        }
        else
        {
            // next pixel
            out = Some(if self.steep { self.cur.yx() } else { self.cur });

            // increment slope error
            self.e += self.de; 
            if self.e > self.dx
            {
                self.cur.y += self.dy;
                self.e -= self.dx * 2;
            }
            // increment x
            self.cur.x += 1;
        }
        out
    }
}