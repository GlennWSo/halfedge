use halfedge::plot::show_wireframes;
use halfedge::{Mesh, Plane};

fn main() {
    let plane = Plane::new([0.5, 0.5, 0.5].into(), [1.0, 0.0, 0.0].into());
    let mut mesh1 = Mesh::unit_cube();
    mesh1.triangulate();
    let mut mesh2 = mesh1.clone();
    mesh1.sub_divide_plane(&plane);
    mesh1.triangulate();
    mesh2.trim_plane(&plane);
    mesh2.triangulate();
    // println!("{}", mesh2);
    // println!("{}", mesh2.pretty_face_vecs());
    show_wireframes(vec![mesh1.into(), mesh2.into()]);
}
