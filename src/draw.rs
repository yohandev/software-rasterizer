use framework::util::{ Bresenham, Triangle };
use framework::math::*;
use framework::*;

/// draws a line on top of the given bitmap. pixels out of bound
/// will be clipped
pub fn line(frame: &mut Frame, a: Vec2<i32>, b: Vec2<i32>, col: Rgba<u8>)
{
    // convert
    let max = frame.size().as_();

    for p in Bresenham::new_bounded(a, b, max)
    {
        // draw line
        frame.set(p, col);
    }
}

/// draw a triangle on top of the given bitmap. pixels out of
/// bound will be clipped
pub fn triangle(frame: &mut Frame, depth: &mut [f32], tri: [Vec3<f32>; 3], col: Rgba<u8>)
{
    // convert
    let max = frame.size().as_();
    let pts = [tri[0].xy().as_(), tri[1].xy().as_(), tri[2].xy().as_()];

    for (pt, br) in Triangle::new_bounded(pts, max)
    {
        // triangle point depth
        let pt_z = tri[0].z * br.x + tri[1].z * br.y + tri[2].z * br.z;
        // depth buffer z
        let bf_z = &mut depth[pt.x as usize * frame.width() + pt.y as usize];

        // depth comparison
        if *bf_z < pt_z
        {
            *bf_z = pt_z;

            // draw triangle
            frame.set(pt, col);
        }
    }
}