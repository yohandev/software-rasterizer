use framework::math::*;

/// represents a wavefront object, with faces that can be iterated
pub struct Obj
{
    inner: obj::Obj
}

impl Obj
{
    /// read a new wavefront object from its path 
    pub fn load(path: &str) -> Self
    {
        use std::io::BufReader;
        use std::fs::File;

        let input = BufReader::new(File::open(path).unwrap());

        Self
        {
            inner: obj::load_obj(input).unwrap()
        }
    }

    /// iterate this model's faces
    pub fn iter_faces(&self) -> impl Iterator<Item = [Vec3<f32>; 3]> + '_
    {
        self.inner.indices.chunks_exact(3).map(move |ind|
        {
            [
                self.inner.vertices[ind[0] as usize].position.into(),
                self.inner.vertices[ind[1] as usize].position.into(),
                self.inner.vertices[ind[2] as usize].position.into(),
            ]
        })
    }
}

// /// represents an iterator that yields an [Obj]'s faces
// ///
// /// [Obj]: Obj
// pub struct IterFaces<'a>
// {
//     /// object we're iterating
//     obj: &'a Obj,
//     /// next index
//     i: usize,
// }

// impl Iterator for IterFaces
// {
//     type Item = [Vec2<f32>; 3];

//     fn next(&mut self) -> Option<Self::Item>
//     {
//         //self.obj.inner.indices
//     }
// }