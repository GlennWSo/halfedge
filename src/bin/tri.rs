use halfedge::plot::show_wireframes;
use halfedge::Mesh;

fn edge_case() -> Mesh {
    let points = vec![
        [0.0, 0.0, 0.0].into(),
        [1.0, 0.0, 0.0].into(),
        [1.0, 1.0, 0.0].into(),
        [0.5, 0.5, 0.0].into(),
    ];
    let faces = vec![vec![0, 1, 2, 3]];

    Mesh::from_verts_faces(points, faces)
}

fn main() {
    let mut mesh1 = edge_case();
    let mut mesh2 = mesh1.clone();
    println!("pre{}", mesh1);
    // mesh.ear_clip(0);

    mesh1.concave_triangulate();
    mesh2.ear_clip(0);
    println!("post{}", mesh1);
    let tris: Vec<_> = mesh1.tri_inds().collect();
    println!("tri {:?}", tris);
    let edge_count: Vec<_> = mesh1.face_edge_count().collect();
    println!("{:#?}", edge_count);
    show_wireframes(vec![mesh1.into(), mesh2.into()]);
}
