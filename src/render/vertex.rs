use crate::utils::Vec3;

pub struct Vertex {
    pub pos: Vec3,
    pub clr: Vec3,
}

impl Vertex {
    pub fn new(pos: Vec3, clr: Vec3) -> Vertex {
        Vertex { pos, clr }
    }

    pub fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>();

        let location = 0;
        let offset = 0;

        unsafe {
            Vec3::vertex_attrib_pointer(stride, location, offset);
        }

        let location = 1;
        let offset = offset + std::mem::size_of::<Vec3>();

        unsafe {
            Vec3::vertex_attrib_pointer(stride, location, offset);
        }
    }
}
