use framework::math::*;
use framework::*;

struct BasicShader;

impl Shader for BasicShader
{
    type Vertex = crate::obj::Vertex;
    type Uniforms = (Mat4<f32>,); // (mvp,)
    type Varyings = (f32,); // (depth,)

    fn vertex(v: Self::Vertex, (mvp,): &Self::Uniforms) -> (Vec4<f32>, Self::Varyings)
    {
        let pos = Vec4::new(v.pos.x, v.pos.y, v.pos.z, 1.0);
        let mvp = *mvp;

        (pos * mvp,(v.pos.z,))
    }

    fn fragment(v: Self::Varyings, u: &Self::Uniforms) -> Rgba<u8>
    {
        todo!()
    }
}

pub trait Shader
{
    /// type of vertex used by this shader
    type Vertex;
    /// read-only uniforms. constant across the vertex/fragment calls
    /// it's fed to
    type Uniforms;
    /// additional outputs of the vertex shader, which are interpolated
    /// and fed into the fragment shader
    type Varyings;

    /// run the vertex shader, taking in a vertex(and other uniforms),
    /// and returning a screen-space vector(NDC is -1.0..=1.0) as well
    /// as outputs to be fed into the fragment shader
    fn vertex(v: Self::Vertex, u: &Self::Uniforms) -> (Vec4<f32>, Self::Varyings);

    fn fragment(v: Self::Varyings, u: &Self::Uniforms) -> Rgba<u8>;
}

trait VertexShaderOutput
{

}

/// represents a type that can be interpolated using barycentric
/// coordinates. ie, a triangle with 3 vertice(one red, one blue,
/// and one yellow) will need to interpolate these colors for any
/// point within its area
trait Varying: Sized
{
    /// interpolate self using barycentric coordinates
    fn interpolate(tri: Vec3<Self>, bar: Vec3<f32>) -> Self;
}

impl Varying for f32
{
    fn interpolate(tri: Vec3<Self>, bar: Vec3<f32>) -> Self
    {
        tri.dot(bar)
    }
}

impl Varying for Rgba<f32>
{
    fn interpolate(tri: Vec3<Self>, bar: Vec3<f32>) -> Self
    {
        let r = Vec3::new(tri.x.r, tri.y.r, tri.z.r).dot(bar);
        let g = Vec3::new(tri.x.g, tri.y.g, tri.z.g).dot(bar);
        let b = Vec3::new(tri.x.b, tri.y.b, tri.z.b).dot(bar);
        let a = Vec3::new(tri.x.a, tri.y.a, tri.z.a).dot(bar);

        Rgba::new(r, g, b, a)
    }
}

impl Varying for Rgba<u8>
{
    fn interpolate(tri: Vec3<Self>, bar: Vec3<f32>) -> Self
    {
        Rgba::<f32>::interpolate(tri.map(|n| n.as_()), bar).as_()
    }
}

impl Varying for Rgb<f32>
{
    fn interpolate(tri: Vec3<Self>, bar: Vec3<f32>) -> Self
    {
        let r = Vec3::new(tri.x.r, tri.y.r, tri.z.r).dot(bar);
        let g = Vec3::new(tri.x.g, tri.y.g, tri.z.g).dot(bar);
        let b = Vec3::new(tri.x.b, tri.y.b, tri.z.b).dot(bar);

        Rgb::new(r, g, b)
    }
}

impl Varying for Rgb<u8>
{
    fn interpolate(tri: Vec3<Self>, bar: Vec3<f32>) -> Self
    {
        Rgb::<f32>::interpolate(tri.map(|n| n.as_()), bar).as_()
    }
}