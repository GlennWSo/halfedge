use halfedge::plot::show_wireframe;
use halfedge::{Mesh, Plane};

fn main() {
    let mut mesh = Mesh::unit_cube();
    mesh.concave_triangulate();
    let plane = Plane::new([0.5, 0.5, 0.5].into(), [0.0, 0.0, 1.0].into());
    mesh.split_plane(plane);
    show_wireframe(mesh.into());
}
