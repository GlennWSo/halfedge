use halfedge::plot::show_wireframe;
use halfedge::Mesh;
// use std::thread;
// use three_d::CpuMesh;

fn main() {
    let points = vec![
        [0.0, 1.0, 0.0].into(),
        [1.0, 1.0, 0.0].into(),
        [0.0, 0.0, 0.0].into(),
    ];
    let mut faces = vec![vec![1, 2, 3]];

    for face in faces.iter_mut() {
        for num in face {
            *num -= 1;
        }
    }

    let mesh = Mesh::from_verts_faces(points, faces);
    show_wireframe(mesh.into());
}
