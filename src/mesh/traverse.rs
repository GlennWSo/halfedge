use super::{HalfEdge, Mesh};
use std::iter::Iterator;

pub struct MeshTraverser<'a> {
    mesh: &'a Mesh,
    pub current_edge: u32,
}

impl<'a> Clone for MeshTraverser<'a> {
    fn clone(&self) -> Self {
        Self {
            mesh: self.mesh,
            current_edge: self.current_edge,
        }
    }
}

impl<'a> MeshTraverser<'a> {
    pub fn get_id(&self) -> u32 {
        self.current_edge
    }
    pub fn get_edge(&self) -> &'a HalfEdge {
        &self.mesh.half_edges[self.current_edge as usize]
    }
    pub fn next(&mut self) -> &mut Self {
        self.current_edge = self.get_edge().next;
        self
    }
    pub fn prev(&mut self) -> &mut Self {
        self.current_edge = self.get_edge().prev;
        self
    }
    pub fn twin(&mut self) -> &mut Self {
        self.current_edge = self.get_edge().twin.unwrap();
        self
    }
}

pub struct FaceEdgesIter<'a> {
    traverser: MeshTraverser<'a>,
    start_edge: u32,
    stop: bool,
}

impl<'a> Iterator for FaceEdgesIter<'a> {
    type Item = &'a HalfEdge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        self.traverser.next();
        if self.traverser.current_edge == self.start_edge {
            self.stop = true;
        }
        return Some(&self.traverser.get_edge());
    }
}
pub struct VertexEdgesIter<'a> {
    traverser: MeshTraverser<'a>,
    start_edge: u32,
    stop: bool,
}
impl<'a> Iterator for VertexEdgesIter<'a> {
    type Item = &'a HalfEdge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        self.traverser.prev();
        self.traverser.twin();
        if self.traverser.current_edge == self.start_edge {
            self.stop = true;
        }
        return Some(&self.traverser.get_edge());
    }
}

/// # traversing
impl Mesh {
    pub fn get_traverser(&self, edge_id: u32) -> MeshTraverser {
        MeshTraverser {
            mesh: self,
            current_edge: edge_id,
        }
    }

    /// # gets iterator over half edges around a face
    pub fn face_edges(&self, face: u32) -> FaceEdgesIter {
        let start_edge = self.faces[face as usize].get();
        let traverser = self.get_traverser(start_edge);
        FaceEdgesIter {
            traverser,
            start_edge,
            stop: false,
        }
    }

    pub fn face_inds(&self) -> impl Iterator<Item = impl Iterator<Item = u32> + '_> + '_ {
        self.faces
            .iter()
            .enumerate()
            .map(|(i, _face)| self.face_edges(i as u32).map(|edge| edge.origin))
    }

    pub fn tri_inds(&self) -> impl Iterator<Item = impl Iterator<Item = u32> + '_> + '_ {
        self.faces
            .iter()
            .enumerate()
            .map(|(i, _face)| self.face_edges(i as u32).map(|edge| edge.origin).take(3))
    }

    /// # gets iterator over half edges around a vertex
    pub fn vertex_edges(&self, vertex: u32) -> VertexEdgesIter {
        let start_edge = self.verts[vertex as usize].half_edge;
        let traverser = self.get_traverser(start_edge);
        VertexEdgesIter {
            traverser,
            start_edge,
            stop: false,
        }
    }
}
