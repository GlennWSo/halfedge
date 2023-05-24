mod display;
mod flip;
mod traverse;

use std::cell::Cell;
// use std::fmt;
use std::iter;

type Point = [f64; 3];

#[derive(Debug)]
struct Vertex {
    coord: Point,
    half_edge: u32,
}

#[derive(Debug, Clone)]
pub struct HalfEdge {
    origin: u32,
    face: Option<u32>,
    twin: Option<u32>,
    next: u32,
    prev: u32,
}

#[derive(Debug)]
pub struct Mesh {
    verts: Vec<Vertex>,
    faces: Vec<Cell<u32>>,
    half_edges: Vec<HalfEdge>,
}

type FaceList = Vec<Vec<u32>>;

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
            faces.push(Cell::new(n_edges as u32));

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
        let n_volume_edges = half_edges.len();
        let loners_ids: Vec<u32> = half_edges
            .iter_mut()
            .enumerate()
            .filter(|(_i, e)| e.twin.is_none())
            .enumerate()
            .map(|(i, (old_i, e))| {
                e.twin = Some((n_volume_edges + i) as u32);
                old_i as u32
            })
            .collect();

        let loner_edges = loners_ids.iter().map(|id| &half_edges[*id as usize]);

        let mut boundary_nexts = loner_edges.clone().map(|lone_edge| {
            let prev_o = half_edges[lone_edge.prev as usize].origin;
            loner_edges.clone().find_map(|e| {
                if e.origin == prev_o {
                    return Some(e.twin.unwrap());
                }
                None
            })
        });

        let mut boundary_prevs = loner_edges.clone().map(|lone_edge| {
            let next_o = half_edges[lone_edge.next as usize].origin;
            loner_edges.clone().find_map(|e| {
                if e.origin == next_o {
                    return Some(e.twin.unwrap());
                }
                None
            })
        });

        let boundary_edges: Vec<HalfEdge> = loners_ids
            .iter()
            .copied()
            .map(|old_id| {
                let old_edge = &half_edges[old_id as usize];
                let old_next = &half_edges[old_edge.next as usize];
                let next_id = boundary_nexts.next().unwrap().unwrap();
                let prev_id = boundary_prevs.next().unwrap().unwrap();
                HalfEdge {
                    origin: old_next.origin,
                    face: None,
                    twin: Some(old_id),
                    next: next_id,
                    prev: prev_id,
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
