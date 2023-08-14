use halfedge;
use halfedge::plot::show_wireframe;
use halfedge::Mesh;

fn main() {
    let mut mesh = Mesh::unit_cube();
    mesh.concave_triangulate();
    show_wireframe(mesh.into());
}
