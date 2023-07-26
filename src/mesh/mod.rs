mod display;
mod flip;
mod plane;
mod point;
mod primative;
mod split;
mod traverse;
mod tri;

// use std::fmt;
pub use point::Point;
use std::iter;

// type Point = [f64; 3];

#[derive(Debug, Clone)]
pub struct Vertex {
    pub coord: Point,
    half_edge: u32,
}

pub type Verts = Vec<Vertex>;
pub type Faces = Vec<u32>;
pub type HalfEdges = Vec<HalfEdge>;
type FaceList = Vec<Vec<u32>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HalfEdge {
    origin: u32,
    twin: Option<u32>,
    face: Option<u32>,
    next: u32,
    prev: u32,
}

impl HalfEdge {
    pub fn new(origin: u32, twin: Option<u32>, face: Option<u32>, next: u32, prev: u32) -> Self {
        Self {
            origin,
            twin,
            face,
            next,
            prev,
        }
    }
}

impl From<[u32; 5]> for HalfEdge {
    fn from(value: [u32; 5]) -> Self {
        let face = if value[2] < u32::MAX {
            Some(value[2])
        } else {
            None
        };
        HalfEdge {
            origin: value[0],
            twin: Some(value[1]),
            face,
            next: value[3],
            prev: value[4],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub verts: Verts, // TODO make private
    faces: Faces,
    half_edges: HalfEdges,
}

impl Mesh {
    pub fn verts(&self) -> &Verts {
        &self.verts
    }
    pub fn faces(&self) -> &Faces {
        &self.faces
    }
    pub fn half_edges(&self) -> &HalfEdges {
        &self.half_edges
    }
}

/// # construct new Self
impl Mesh {
    pub fn from_verts_faces(points: Vec<Point>, face_list: FaceList) -> Self {
        let mut half_edges: Vec<HalfEdge> = Vec::new();
        let mut faces = Vec::with_capacity(face_list.len());

        // create non boundary edges
        for (face, verts) in face_list.iter().enumerate() {
            let n_edges = half_edges.len();
            let origins = verts.iter();
            let edge_ids: Vec<u32> = (n_edges..(n_edges + verts.len()))
                .map(|i| i as u32)
                .collect();
            let last = iter::once(edge_ids.last().unwrap());

            let mut next_edge = edge_ids[1..].iter().chain(iter::once(&edge_ids[0]));
            let mut prev_edge = last.chain(edge_ids.iter().take(edge_ids.len() - 1));
            let mut new_edges: Vec<HalfEdge> = origins
                .map(|e| HalfEdge {
                    origin: *e,
                    face: Some(face as u32),
                    twin: None,
                    next: *next_edge.next().unwrap(),
                    prev: *prev_edge.next().unwrap(),
                })
                .collect();

            let n_edges = half_edges.len();
            faces.push(n_edges as u32);

            let next_origins: Vec<u32> = half_edges
                .iter()
                .map(|e| half_edges[e.next as usize].origin)
                .collect();

            for (i_old, old) in half_edges
                .iter_mut()
                .enumerate()
                .filter(|e| e.1.twin.is_none())
            {
                let nn_origins: Vec<u32> = new_edges
                    .iter()
                    .map(|e| new_edges[e.next as usize - n_edges].origin)
                    .collect();

                for (i_new, new) in new_edges.iter_mut().enumerate() {
                    if (old.origin == nn_origins[i_new]) && (new.origin == next_origins[i_old]) {
                        new.twin = Some(i_old as u32);
                        old.twin = Some((i_new + n_edges) as u32);
                    }
                }
            }
            half_edges.extend(new_edges);
        }
        let mut verts: Vec<Vertex> = points
            .into_iter()
            .map(|coord| Vertex {
                coord,
                half_edge: 0,
            })
            .collect();
        for (i, edge) in half_edges.iter().enumerate() {
            verts[edge.origin as usize].half_edge = i as u32;
        }

        // add boundary edges to edges with no twin
        let n_surface_edges = half_edges.len();
        let loners_ids: Vec<u32> = half_edges
            .iter_mut()
            .enumerate()
            .filter(|(_i, e)| e.twin.is_none())
            .enumerate()
            .map(|(i, (old_i, e))| {
                e.twin = Some((n_surface_edges + i) as u32);
                old_i as u32
            })
            .collect();

        let loner_edges = loners_ids.iter().map(|id| &half_edges[*id as usize]);

        let boundary_prevs: Vec<u32> = loner_edges
            .clone()
            .map(|lone_edge| {
                let prev_o = half_edges[lone_edge.next as usize].origin;
                loner_edges.clone().find_map(|e| {
                    if e.origin == prev_o {
                        return Some(e.twin.unwrap());
                    }
                    None
                })
            })
            .map(|some_e| some_e.unwrap())
            .collect();

        let mut boundary_nexts = boundary_prevs.clone();
        for (i, b_next) in boundary_prevs.iter().enumerate() {
            let next_i = *b_next as usize - n_surface_edges;
            boundary_nexts[next_i] = i as u32 + n_surface_edges as u32;
        }

        let boundary_edges: Vec<HalfEdge> = loners_ids
            .iter()
            .enumerate()
            .map(|(i, old_id)| {
                let old_edge = &half_edges[*old_id as usize];
                let old_next = &half_edges[old_edge.next as usize];
                let next_id = boundary_prevs[i];
                let prev_id = boundary_nexts[i];
                HalfEdge {
                    origin: old_next.origin,
                    face: None,
                    twin: Some(*old_id),
                    next: prev_id,
                    prev: next_id,
                }
            })
            .collect();

        half_edges.extend(boundary_edges);
        Mesh {
            verts,
            faces,
            half_edges,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{mesh::HalfEdge, Mesh};

    fn square() -> Mesh {
        let points = vec![
            [0.0, 1.0, 0.0].into(),
            [1.0, 1.0, 0.0].into(),
            [0.0, 0.0, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
        ];
        let faces = vec![vec![0, 2, 3], vec![0, 3, 1]];
        Mesh::from_verts_faces(points, faces)
    }

    #[test]
    fn from_verts_faces() {
        let mesh = square();
        let expected_edges: Vec<HalfEdge> = vec![
            [1 - 1, 6, 0, 1, 2].into(),
            [3 - 1, 7, 0, 2, 0].into(),
            [4 - 1, 3, 0, 0, 1].into(),
            [1 - 1, 2, 1, 4, 5].into(),
            [4 - 1, 8, 1, 5, 3].into(),
            [2 - 1, 9, 1, 3, 4].into(),
            [3 - 1, 0, u32::MAX, 9, 7].into(),
            [4 - 1, 1, u32::MAX, 6, 8].into(),
            [2 - 1, 4, u32::MAX, 7, 9].into(),
            [1 - 1, 5, u32::MAX, 8, 6].into(),
        ];
        // expected_edges.sort();
        println!("{}", mesh);

        let edges = mesh.half_edges();

        for (expected, edge) in expected_edges.iter().zip(edges) {
            assert_eq!(expected, edge);
            println!("ok edge: {}", edge);
        }
    }
    #[test]
    fn flip_square() {
        let mut mesh = square();
        mesh.flip_edge(3);
        let edges = mesh.half_edges();
        let expected_edges: Vec<HalfEdge> = vec![
            [1 - 1, 6, 0, 3, 5].into(),
            [3 - 1, 7, 0, 4, 2].into(),
            [2 - 1, 3, 0, 1, 4].into(),
            [3 - 1, 2, 1, 5, 0].into(),
            [4 - 1, 8, 1, 2, 1].into(),
            [2 - 1, 9, 1, 0, 3].into(),
            [3 - 1, 0, u32::MAX, 9, 7].into(),
            [4 - 1, 1, u32::MAX, 6, 8].into(),
            [2 - 1, 4, u32::MAX, 7, 9].into(),
            [1 - 1, 5, u32::MAX, 8, 6].into(),
        ];
        for (expected, edge) in expected_edges.iter().zip(edges) {
            assert_eq!(expected, edge);
            println!("{}", edge);
        }
    }
}
