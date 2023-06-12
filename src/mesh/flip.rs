use super::Mesh;

impl Mesh {
    /// rotates edge and its twin
    pub fn flip_edge(&mut self, edge_id: u32) {
        let face_id = self.half_edges[edge_id as usize]
            .face
            .expect("flip can only be applied to interior edges");

        // get values to use
        let mut trav = self.get_traverser(edge_id);

        let mut twin_trav = trav.clone();
        let twin_id = twin_trav.twin().get_id();
        let twin_face_id = self.half_edges[twin_id as usize]
            .face
            .expect("flip can only be applied to interior edges");

        let twin_next_id = twin_trav.clone().next().get_id();
        let twin_prev_id = twin_trav.prev().get_id();
        let twin_prev_origin = twin_trav.get_edge().origin;

        let next_id = trav.clone().next().get_id();
        let prev_id = trav.prev().get_id();
        let prev_origin = trav.get_edge().origin;

        {
            // ensure flipped edges is not used as insident edges
            let face = &self.faces[face_id as usize];
            face.set(next_id);
        }
        {
            let twin_face = &self.faces[twin_face_id as usize];
            twin_face.set(twin_next_id);
        }

        {
            let edge = &mut self.half_edges[edge_id as usize];
            edge.next = prev_id;
            edge.prev = twin_next_id;
            edge.origin = twin_prev_origin;
        }

        {
            let twin = &mut self.half_edges[twin_id as usize];
            twin.next = twin_prev_id;
            twin.prev = next_id;
            twin.origin = dbg!(prev_origin);
        }

        {
            let next = &mut self.half_edges[next_id as usize];
            next.next = twin_id;
            next.prev = twin_prev_id;
        }

        {
            let twin_next = &mut self.half_edges[twin_next_id as usize];
            twin_next.next = edge_id;
            twin_next.prev = prev_id;
        }

        {
            let prev = &mut self.half_edges[prev_id as usize];
            prev.next = twin_next_id;
            prev.prev = edge_id;
        }

        {
            let twin_prev = &mut self.half_edges[twin_prev_id as usize];
            twin_prev.next = next_id;
            twin_prev.prev = twin_id;
        }
    }
}
