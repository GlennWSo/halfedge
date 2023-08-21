use halfedge::plot::show_wireframes;
use halfedge::{Mesh, Plane};

fn main() {
    let mut mesh1 = Mesh::unit_cube();
    mesh1.concave_triangulate();
    let mut mesh2 = mesh1.clone();
    // let plane = Plane::new([0.5, 0.5, 0.5].into(), [0.0, 0.0, 1.0].into());
    // mesh2.split_plane(plane);
    mesh2.divide_edge(0);
    // mesh2.concave_triangulate();
    for (face_i, face) in mesh2.face_inds().enumerate() {
        print!("face{}:", face_i);
        for i in face {
            print!(" {}", i);
        }
        println!();
    }
    show_wireframes(vec![mesh1.into(), mesh2.into()]);
}
