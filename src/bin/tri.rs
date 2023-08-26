use halfedge::plot::show_wireframe;
use halfedge::Mesh;

fn pentagon() -> Mesh {
    let points = vec![
        [0.0, 0.0, 0.0].into(),
        [0.5, -0.5, 0.0].into(),
        [1.0, 0.0, 0.0].into(),
        [1.0, 1.0, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
    ];
    let faces = vec![vec![0, 1, 2, 3, 4]];

    Mesh::from_verts_faces(points, faces)
}

fn main() {
    let mut mesh = pentagon();
    println!("pre{}", mesh);
    mesh.concave_triangulate();
    println!("post{}", mesh);
    let tris: Vec<_> = mesh.tri_inds().collect();
    println!("tri {:?}", tris);
    let edge_count: Vec<_> = mesh.face_edge_count().collect();
    println!("{:#?}", edge_count);
    show_wireframe(mesh.into());
}
