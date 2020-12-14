use crate::util::Barycentric;
use crate::math::*;

/// iterator to trace/rasterize triangles using the bounding box
/// and baryncentric coordinates approach
pub struct Triangle
{
    // (current x, current y)
    cur: Vec2<i32>,

    /// points composing the triangle.
    pts: [Vec2<i32>; 3],

    /// minimum(upper-left corner) of the bounding box
    min: Vec2<i32>,
    /// maximum(lower-right corner) of the bounding box
    max: Vec2<i32>,
}

impl Triangle
{
    /// create a new iterator that yields points inside `pts`
    /// the tuple returned is in (cartesian, barycentric) coordinates
    ///
    /// Differs from [Triangle::new] in that it skips points out of the
    /// canvas's bounds. `size` is higher-bound exclusive.
    ///
    /// [Triangle::new]: crate::util::Triangle::new
    #[inline]
    pub fn new_bounded(pts: [Vec2<i32>; 3], size: Extent2<i32>) -> impl Iterator<Item = (Vec2<i32>, Vec3<f32>)>
    {
        // bounds
        let size: Vec2<i32> = size
            .map(|n| n - 1)
            .into();

        // bounding box
        let mut min: Vec2<i32> = size.clone();
        let mut max: Vec2<i32> = Vec2::zero();

        // compute bounding box
        for vert in &pts
        {
            min = min.map2(*vert, |m, v| m.min(v).max(0));
            max = max.map3(*vert, size, |m, v, b| m.max(v).min(b));
        }

        // starting point
        let cur = min.clone();

        Self { cur, pts, min, max }
    }

    /// create a new iterator that yields points inside `pts`
    #[inline]
    pub fn new(pts: [Vec2<i32>; 3]) -> impl Iterator<Item = (Vec2<i32>, Vec3<f32>)>
    {
        Self::new_bounded(pts, Extent2::broadcast(i32::MAX))
    }
}

impl Iterator for Triangle
{
    type Item = (Vec2<i32>, Vec3<f32>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        // inclusive upper bound
        while self.cur.y <= self.max.y
        {
            // cartesian and barycentric coords
            let p = self.cur;
            let b: Vec3<f32> = p.into_barycentric(self.pts);
            
            // step, either for next Iterator::next call, or for
            // next while loop attempt
            self.cur.x += 1;
            // inclusive upper bound
            if self.cur.x > self.max.x
            {
                self.cur.x = self.min.x;
                self.cur.y += 1;
            }

            // inside the triangle
            if b.x >= 0.0 && b.y >= 0.0 && b.z >= 0.0
            {
                return Some((p, b));
            }
        }
        // done
        None
    }
}