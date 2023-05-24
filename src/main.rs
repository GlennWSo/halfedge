use halfedge::Mesh;

fn main() {
    let points = vec![
        [1.0, 4.0, 0.0],
        [3.0, 4.0, 0.0],
        [0.0, 2.0, 0.0],
        [2.0, 2.0, 0.0],
        [4.0, 2.0, 0.0],
        [1.0, 0.0, 0.0],
        [3.0, 0.0, 0.0],
    ];
    // let points = vec![
    //     [0.0, 1.0, 0.0],
    //     [1.0, 1.0, 0.0],
    //     [0.0, 2.0, 0.0],
    //     [1.0, 0.0, 0.0],
    // ];
    let mut faces = vec![vec![1, 3, 4], vec![1, 4, 2]];

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

    let mut hmesh = Mesh::from_verts_faces(points, faces);
    // println!("{}", hmesh);

    // println!("{}", hmesh.get_traverser(2).get_edge());
    hmesh.flip_edge(3);
    println!("{}", hmesh);
    // println!("{}", hmesh.get_traverser(2).get_edge());
}
