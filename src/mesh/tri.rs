use std::fmt::Display;

use super::{Coord, HalfEdge, Mesh};

impl Mesh {
    /// move a edge and edge.neighbour into new triangle
    pub fn orphan_edge(&mut self, edge_id: u32) {
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
    pub fn concave_triangulate(&mut self) {
        let n_faces = self.faces.len() as u32;
        for i in 0..n_faces {
            self.concave_triangulate_face(i);
        }
    }
    pub fn concave_triangulate_face(&mut self, face: u32) {
        let mut travler = self.get_traverser(self.faces[face as usize]);
        let start_id = travler.get_id();
        travler.next().next().next();
        let mut targets = Vec::with_capacity(10);
        while travler.current_edge != start_id {
            targets.push(travler.get_id());
            travler.next();
        }
        for edge_id in targets {
            self.orphan_edge(edge_id);
        }
    }

    pub fn ear_clip(&mut self, face: u32) {
        let edge_iter = self.face_edges(face);
        let edge_count = edge_iter.clone().count();
        if edge_count <= 3 {
            println!("face {} already tri or less", face);
            return;
        };

        for _ in 0..(edge_count - 3) {
            let ear = self.find_ear(face, edge_count);
            self.orphan_edge(ear);
        }
    }
    fn find_ear(&self, face: u32, n_edges: usize) -> u32 {
        let mut coords = Vec::with_capacity(n_edges);
        for coord in self
            .face_edges(face)
            .map(|e| self.verts[e.origin as usize].coord)
        {
            coords.push(coord)
        }
        let n_others = n_edges - 3;
        let mut travler = self.get_traverser(self.faces[face as usize]);
        let mut coord_cycle = coords.into_iter().cycle().skip(1);
        loop {
            let candidate = travler.get_id();
            let other_coords = coord_cycle.clone().take(n_others).collect();
            if self.is_ear(candidate, other_coords) {
                return candidate;
            };
            travler.next();
            coord_cycle.next();
        }
    }
    fn is_ear(&self, edge_id: u32, other_coords: Vec<Coord>) -> bool {
        let mut trav = self.get_traverser(edge_id);
        let v0 = trav.prev().get_edge().origin;
        let v1 = trav.next().get_edge().origin;
        let v2 = trav.next().get_edge().origin;
        let tri: Tri = [
            self.verts[v0 as usize].coord,
            self.verts[v1 as usize].coord,
            self.verts[v2 as usize].coord,
        ]
        .into();
        println!("{}", tri);
        !other_coords.into_iter().any(|x| tri.inside(x))
    }
}
// type Tri = [Coord; 3];
struct Tri {
    coords: [Coord; 3],
}
impl Display for Tri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.coords {
            writeln!(f, "{}\t{}\t{}", c.x, c.y, c.z)?;
        }
        Ok(())
    }
}

impl From<[Coord; 3]> for Tri {
    fn from(coords: [Coord; 3]) -> Self {
        Self { coords }
    }
}

impl Tri {
    fn normalish(&self) -> Coord {
        let v1 = self.coords[1] - self.coords[0];
        let v2 = self.coords[2] - self.coords[1];
        v1.cross(v2)
    }
    fn inside(&self, coord: Coord) -> bool {
        let edges = [
            (self.coords[1], self.coords[0]),
            (self.coords[2], self.coords[1]),
            (self.coords[0], self.coords[2]),
        ];
        let normal = self.normalish();
        edges.into_iter().all(|e| {
            let edge_v = e.1 - e.0;
            let v = coord - e.0;
            let x = v.cross(edge_v);
            dbg!(x.dot(normal)) > 0.0
        })
    }
}

#[cfg(test)]
mod test_helpers {
    use super::{Mesh, Tri};
    use std::assert;

    fn star() -> Mesh {
        todo!()
    }

    #[test]
    fn test_inside() {
        let c0 = [0.0, 0.0, 0.0].into();
        let c1 = [1.0, 0.0, 0.0].into();
        let c2 = [1.0, 1.0, 0.0].into();
        let tri = Tri::from([c0, c1, c2]);

        let inside_coord = [0.6, 0.5, 0.5];
        assert!(tri.inside(inside_coord.into()));
        let outside1 = [10.0, 0.5, 0.5];
        assert!(!tri.inside(outside1.into()));
        let outside2 = [0.5, 10., 0.5];
        assert!(!tri.inside(outside2.into()));
        let outside3 = [-10.0, 10.0, 0.5];
        assert!(!tri.inside(outside3.into()));
    }
}

#[cfg(test)]
mod test_mesh_triangulation {
    use std::{assert_eq, vec};

    use super::Mesh;

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
        mesh.concave_triangulate();
        println!("post: {}", mesh);
        let expected: Vec<usize> = vec![3, 3, 3];
        let res: Vec<_> = mesh.face_edge_count().collect();
        assert_eq!(expected, res);
    }
    #[test]
    fn test_ear_clip() {
        let points = vec![
            [0.0, 0.0, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
            [0.2, 0.2, 0.0].into(),
            [0.0, 1.0, 0.0].into(),
        ];
        let faces = vec![vec![0, 1, 2, 3, 4]];
        let mesh = Mesh::from_verts_faces(points, faces);
    }
}
