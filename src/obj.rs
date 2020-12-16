use framework::math::*;

/// represents a collection of [Mesh]'s
///
/// [Mesh]: self::Mesh
#[derive(Debug, Clone, PartialEq)]
pub struct Obj
{
    pub meshes: Vec<Mesh>
}

/// represents a mesh within a wavefront [Obj]
///
/// [Obj]: self::Obj
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh
{
    /// this mesh("model")'s name
    pub name: String,

    /// arbitrarily sorted list of vertices
    pub verts: Vec<Vertex>,
    /// list of triangular faces
    pub inds: Vec<[usize; 3]>,
}

/// represents a vertex within a [Mesh]
///
/// [Mesh]: self::Mesh
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vertex
{
    /// this vertex's position
    pub pos: Vec3<f32>,
    /// this vertex's normal
    pub nor: Vec3<f32>,
    /// this vertex's UV/texture coordinate
    pub tex: Vec2<f32>,
}

impl Obj
{
    /// read a new wavefront object from its path. assumes the vertices
    /// have all position, normal, and texture coordinates. all faces must
    /// be triangular
    pub fn load(path: &str) -> Self
    {
        // load .obj
        let (models, _) = tobj::load_obj(path, true).unwrap();

        // go through every mesh
        let meshes = models.into_iter().map(|tobj::Model { name, mesh }|
        {
            /// maps a slice(assumed length 3) to an array of usize
            fn map_indices(i: &[u32]) -> [usize; 3]
            {
                [i[0] as usize, i[1] as usize, i[2] as usize]
            }
            /// maps position, normal, and texture coordinate slices to
            /// a vertex
            fn map_vertex(((pos, nor), tex): ((&[f32], &[f32]), &[f32])) -> Vertex
            {
                Vertex
                {
                    pos: Vec3::new(pos[0], pos[1], pos[2]),
                    nor: Vec3::new(nor[0], nor[1], nor[2]),
                    tex: Vec2::new(tex[0], tex[1]),
                }
            }

            // map indices
            let inds: Vec<[usize; 3]> = mesh.indices
                .chunks_exact(3)
                .map(map_indices)
                .collect();
            // map vertices
            let verts: Vec<Vertex> = mesh.positions
                .chunks_exact(3)
                .zip(mesh.normals.chunks_exact(3))
                .zip(mesh.texcoords.chunks_exact(2))
                .map(map_vertex)
                .collect();

            // build the mesh
            Mesh { name, verts, inds }

        }).collect();

        Self { meshes }
    }

    /// iterate all the faces in all the meshes in this wavefront scene
    pub fn iter_faces(&self)  -> impl Iterator<Item = [Vertex; 3]> + '_
    {
        self.meshes
            .iter()
            .flat_map(|m| m.iter_faces())
    }
}

impl Mesh
{
    /// iterate this mesh's faces
    pub fn iter_faces(&self) -> impl Iterator<Item = [Vertex; 3]> + '_
    {
        self.inds
            .iter()
            .map(move |[a, b, c]| [self.verts[*a], self.verts[*b], self.verts[*c]])
    }
}