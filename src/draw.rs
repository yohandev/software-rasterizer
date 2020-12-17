use framework::util::{ Bresenham, Triangle };
use framework::math::*;
use framework::*;

use crate::obj::Vertex;

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
pub fn triangle(frame: &mut Frame, depth: &mut [f32], tex: &Image, tri: [Vertex; 3], col: Rgba<u8>)
{
    // convert
    let max = frame.size().as_();
    let pts = [tri[0].pos.xy().as_(), tri[1].pos.xy().as_(), tri[2].pos.xy().as_()];

    for (pt, br) in Triangle::new_bounded(pts, max)
    {
        // triangle point depth
        let pt_z = tri[0].pos.z * br.x + tri[1].pos.z * br.y + tri[2].pos.z * br.z;
        // depth buffer z
        let bf_z = &mut depth[pt.y as usize * frame.width() + pt.x as usize];

        // depth comparison
        if *bf_z < pt_z
        {
            *bf_z = pt_z;

            // triangle UV
            let u = tri[0].tex.x * br.x + tri[1].tex.x * br.y + tri[2].tex.x * br.z;
            let v = tri[0].tex.y * br.y + tri[1].tex.y * br.y + tri[2].tex.y * br.z;

            // draw triangle
            frame.set(pt, col);
        }
    }
}