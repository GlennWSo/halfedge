use halfedge::plot::show_wireframes;
use halfedge::{Mesh, Plane};

fn main() {
    let mut mesh1 = Mesh::unit_cube();
    mesh1.concave_triangulate();

    let mut mesh2 = mesh1.clone();
    let plane = Plane::new([0.5, 0.5, 0.5].into(), [1.0, 0.0, 0.0].into());
    mesh2.sub_divide_plane(plane);
    mesh2.concave_triangulate();
    println!("{}", mesh2);
    println!("{}", mesh2.pretty_face_vecs());
    show_wireframes(vec![mesh1.into(), mesh2.into()]);
}
