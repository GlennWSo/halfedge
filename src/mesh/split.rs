use super::{Coord, HalfEdge, Mesh, Vertex};

impl Mesh {
    /// split_edge with mid point
    pub fn divide_edge(&mut self, edge_id: u32) {
        let mut travler = self.get_traverser(edge_id);
        let id1 = travler.get_edge().origin;
        let p1 = self.verts[id1 as usize].coord;
        travler.next();
        let id2 = travler.get_edge().origin;
        let p2 = self.verts[id2 as usize].coord;
        let mid = (p1 + p2) / 2.0;
        self.split_edge(edge_id, mid);
    }
    pub fn split_edge(&mut self, edge_id: u32, coord: Coord) {
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

        // create split point
        let new_point = Vertex {
            coord,
            half_edge: new_edge_id,
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
