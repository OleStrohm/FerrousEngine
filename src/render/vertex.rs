use crate::utils::Vec3;

#[derive(Debug, Copy, Clone, VertexAttribPointers)]
#[repr(C, packed)]
pub struct Vertex {
    #[location = 0]
    pub pos: Vec3,
    #[location = 1]
    pub clr: Vec3,
}

impl Vertex {
    pub fn new(pos: Vec3, clr: Vec3) -> Vertex {
        Vertex { pos, clr }
    }
}
