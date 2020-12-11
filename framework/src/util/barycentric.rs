use std::ops::{ Mul, Sub, Div };
use std::cmp::PartialOrd;
use num::{ One, Signed, Float };

use crate::math::*;

pub fn barycentric<T, V: Float>(p: Vec2<T>, [a, b, c]: [Vec2<T>; 3]) -> Vec3<V>
where T: Sub<T, Output = T>
    + Mul<T, Output = T>
    + Div<T, Output = T>
    + Into<V>
    + PartialOrd
    + One
    + Signed
    + Copy
{
    // looking for vector <u, v, 1> orthogonal to <ab.x, ac.x, pa.x>
    // and <ab.y, ac.y, pa.y>. this uses cramer's rule
    let ba = b - a;
    let ca = c - a;
    let pa = p - a;

    // determinant
    let d00 = ba.dot(ba);
    let d01 = ba.dot(ca);
    let d11 = ca.dot(ca);
    let d20 = pa.dot(ba);
    let d21 = pa.dot(ca);

    // denominator
    let den: V = (d00 * d11 - d01 * d01).into();

    let v: V = (d11 * d20 - d01 * d21).into() / den;
    let w: V = (d00 * d21 - d01 * d20).into() / den;
    let u: V = V::one() - v - w;

    Vec3::new(u, v, w)
}