use super::{HalfEdge, Mesh};

impl Mesh {
    /// divide a polyogon by moving a edge and edge.neighbour into new triangle
    pub fn clip_polygon(&mut self, edge_id: u32) {
        let mut travler = self.get_traverser(edge_id);
        let edge = travler.get_edge();
        // let face = self.faces[edge.face.expect("edge should be part of a face") as usize];
        let prev = travler.prev().get_edge();

        let prev_id = edge.prev;
        let next_id = self.half_edges[edge_id as usize].next;
        let pp_id = prev.prev;
        let nn_id = next_id;

        let new_edge_id = self.half_edges.len() as u32;
        let new_twin_id = new_edge_id + 1;
        let face = self.half_edges[edge_id as usize].face;
        let new_face_id = self.faces.len() as u32;
        let face_id = edge.face.expect("edge should be part of a face]") as usize;
        self.faces[face_id] = next_id;
        self.faces.push(edge_id);

        // add new edge
        let new_edge = HalfEdge {
            origin: self.half_edges[next_id as usize].origin,
            twin: Some(new_twin_id),
            face: Some(new_face_id),
            next: prev_id,
            prev: edge_id,
        };
        self.half_edges.push(new_edge);

        let new_twin = HalfEdge {
            origin: self.half_edges[prev_id as usize].origin,
            twin: Some(new_edge_id),
            face,
            next: nn_id,
            prev: pp_id,
        };
        self.half_edges.push(new_twin);

        // update edge
        let edge = &mut self.half_edges[edge_id as usize];
        edge.face = Some(new_face_id);
        edge.next = new_edge_id;

        let prev_edge = &mut self.half_edges[prev_id as usize];
        prev_edge.face = Some(new_face_id);
        prev_edge.prev = new_edge_id;

        let next_edge = &mut self.half_edges[next_id as usize];
        next_edge.prev = new_twin_id;

        let pp_edge = &mut self.half_edges[pp_id as usize];
        pp_edge.next = new_twin_id;
    }
    pub fn concave_triangulate(&mut self, face: u32) {
        let mut travler = self.get_traverser(self.faces[face as usize]);
        let start_id = travler.get_id();
        travler.next().next().next();
        let mut targets = Vec::with_capacity(10);
        while travler.current_edge != start_id {
            targets.push(travler.get_id());
            travler.next();
        }
        for edge_id in dbg!(targets) {
            self.clip_polygon(edge_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, vec};

    use super::Mesh;
    use crate::plot::show_wireframe;

    /// mesh with a single face that have 5 edges
    fn pentagon() -> Mesh {
        let points = vec![
            [0.0, 0.0, 0.0].into(),
            [0.5, -0.5, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
            [1.0, 1.0, 0.0].into(),
            [0.0, 1.0, 0.0].into(),
        ];
        let faces = vec![vec![0, 1, 2, 3, 4]];

        let mesh = Mesh::from_verts_faces(points, faces);
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.face_edge_count().next().unwrap(), 5);
        mesh
    }

    #[test]
    fn test_concave_triangulate_pentagon() {
        let mut mesh = pentagon();
        println!("pre: {}", mesh);
        mesh.concave_triangulate(0);
        println!("post: {}", mesh);
        let expected: Vec<usize> = vec![3, 3, 3];
        let res: Vec<_> = mesh.face_edge_count().collect();
        assert_eq!(expected, res);
    }
}
