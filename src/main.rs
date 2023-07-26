use halfedge::plot::show_wireframe;
use halfedge::Mesh;
// use std::thread;
// use three_d::CpuMesh;

fn main() {
    // let points = vec![
    //     [1.0, 4.0, 0.0],
    //     [3.0, 4.0, 0.0],
    //     [0.0, 2.0, 0.0],
    //     [2.0, 2.0, 0.0],
    //     [4.0, 2.0, 0.0],
    //     [1.0, 0.0, 0.0],
    //     [3.0, 0.0, 0.0],
    // ];
    let points = vec![
        [0.0, 1.0, 0.0].into(),
        [1.0, 1.0, 0.0].into(),
        [0.0, 0.0, 0.0].into(),
        // [1.0, 0.0, 0.0].into(),
    ];
    let mut faces = vec![vec![1, 2, 3]];

    // let mut faces = vec![
    //     vec![1, 3, 4],
    //     vec![1, 4, 2],
    //     vec![2, 4, 5],
    //     vec![3, 6, 4],
    //     vec![4, 6, 7],
    //     vec![4, 7, 5],
    // ];
    for face in faces.iter_mut() {
        for num in face {
            *num -= 1;
        }
    }

    let mesh = Mesh::from_verts_faces(points, faces);
    let mut mesh2 = mesh.clone();
    // mesh2.flip_edge(2);
    mesh2.divide_edge(2);
    println!("{}", mesh2);
    mesh2.concave_triangulate();
    mesh2.verts.last_mut().unwrap().coord.x = -0.1;
    for verts in mesh2.face_inds() {
        for v in verts {
            print!("{} ", v);
        }
        println!("");
    }
    // mesh.plot();
    // show_wireframe(mesh.into());
    show_wireframe(mesh2.into());

    // println!("i never run");

    // hmesh.flip_edge(2);
    // println!("{}", hmesh);
    // hmesh.plot();
}
