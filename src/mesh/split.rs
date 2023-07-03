use super::{HalfEdge, Mesh, Vertex};

impl Mesh {
    pub fn split_edge(&mut self, edge_id: u32) {
        // get all effected ids
        let travler = self.get_traverser(edge_id);
        let mut twin_travler = travler.clone();
        let twin_id = twin_travler.twin().get_id();
        // let twin_next_id = twin_travler.clone().next().get_id();
        // let twin_prev_id = twin_travler.prev().get_id();

        // let next_id = travler.clone().next().get_id();
        // let prev_id = travler.prev().get_id();

        let new_edge_id = self.half_edges.len() as u32;
        let new_twin_id = new_edge_id + 1;
        let new_point_id = self.verts.len() as u32;

        let edge = &self.half_edges[edge_id as usize];
        let twin = &self.half_edges[twin_id as usize];
        let origin_id = edge.origin;
        let twin_origin_id = twin.origin;

        // create split point
        let new_point = {
            let p1 = &self.verts[origin_id as usize];
            let p2 = &self.verts[twin_origin_id as usize];
            Vertex {
                coord: p1.coord / 2.0 + p2.coord / 2.0,
                half_edge: new_edge_id,
            }
        };
        self.verts.push(new_point);

        // create new edges
        {
            let new_edge =
                HalfEdge::new(new_point_id, Some(twin_id), edge.face, edge.next, edge_id);
            let new_twin =
                HalfEdge::new(new_point_id, Some(edge_id), twin.face, twin.next, twin_id);
            self.half_edges.push(new_edge);
            self.half_edges.push(new_twin);
        }

        //update target edge and twin
        self.half_edges[edge_id as usize].next = new_edge_id;
        self.half_edges[twin_id as usize].next = new_twin_id;
    }
}
