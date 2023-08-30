use super::Mesh;

impl Mesh {
    pub fn star() -> Mesh {
        let mut points = vec![
            [-1.0, -1.0, 0.0].into(),
            [0.0, -1.0, 0.0].into(),
            [1.0, -1.0, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
            [1.0, 1.0, 0.0].into(),
            [0.0, 1.0, 0.0].into(),
            [-1.0, 1.0, 0.0].into(),
            [-1.0, 0.0, 0.0].into(),
        ];
        for i in (1..points.len()).step_by(2) {
            points[i] = points[i] * 0.3;
        }
        let face1 = (0..points.len()).map(|n| n as u32).collect();

        let faces = vec![face1];

        Mesh::from_verts_faces(points, faces)
    }
    pub fn unit_triangle() -> Self {
        let points = vec![
            [0.0, 0.0, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
            [1.0, 1.0, 0.0].into(),
        ];
        let faces = vec![vec![0, 1, 2]];
        Mesh::from_verts_faces(points, faces)
    }

    pub fn unit_square() -> Self {
        let points = vec![
            [0.0, 0.0, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
            [1.0, 1.0, 0.0].into(),
            [0.0, 1.0, 0.0].into(),
        ];
        let faces = vec![vec![0, 1, 2, 3]];
        Mesh::from_verts_faces(points, faces)
    }

    pub fn unit_cube() -> Self {
        let points = vec![
            [0.0, 0.0, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
            [1.0, 1.0, 0.0].into(),
            [0.0, 1.0, 0.0].into(),
            [0.0, 0.0, 1.0].into(),
            [1.0, 0.0, 1.0].into(),
            [1.0, 1.0, 1.0].into(),
            [0.0, 1.0, 1.0].into(),
        ];
        let faces = vec![
            vec![3, 2, 1, 0],
            vec![0, 1, 5, 4],
            vec![1, 2, 6, 5],
            vec![2, 3, 7, 6],
            vec![3, 0, 4, 7],
            vec![4, 5, 6, 7],
        ];

        Mesh::from_verts_faces(points, faces)
    }
}
