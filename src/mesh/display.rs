use std::fmt;

use super::{HalfEdge, Mesh, Vertex};
// mod half

fn op_id2string(op_num: Option<u32>) -> String {
    match op_num {
        Some(num) => num.to_string(),
        None => "-".to_string(),
    }
}

impl Mesh {
    fn pretty_edges(&self) -> String {
        let header = format!("id\t{}", HalfEdge::pretty_header());
        let body: String = self
            .half_edges
            .iter()
            .enumerate()
            .map(|(i, e)| format!("{}\t{}\n", i, e.pretty_data()))
            .collect();
        format!("{}\n{}", header, body)
    }

    fn pretty_verts(&self) -> String {
        let header = format!("id\t{}", Vertex::pretty_header());
        let body: String = self
            .verts
            .iter()
            .enumerate()
            .map(|(i, e)| format!("{}\t{}\n", i, e.pretty_data()))
            .collect();
        format!("{}\n{}", header, body)
    }

    fn pretty_faces(&self) -> String {
        let header = "id\tedge".to_string();
        let body: String = self
            .faces
            .iter()
            .enumerate()
            .map(|(i, face_edge)| format!("{}\t{}\n", i, face_edge))
            .collect();
        format!("{}\n{}", header, body)
    }

    pub fn pretty_face_vecs(&self) -> String {
        let mut s = String::new();
        for (face_i, face) in self.face_inds().enumerate() {
            let mut row = format!("f{}:", face_i);
            for i in face {
                row += &format!(" {}", i);
            }
            row += "\n";
            s += &row;
        }
        s
    }
}

impl Vertex {
    fn pretty_header() -> &'static str {
        "x\ty\tz\tedge"
    }
    fn pretty_data(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t",
            self.coord[0], self.coord[1], self.coord[2], self.half_edge
        )
    }
}

impl HalfEdge {
    pub fn pretty_header() -> &'static str {
        "origin\ttwin\tface\tnext\tprev"
    }

    pub fn pretty_data(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}",
            self.origin + 1,
            op_id2string(self.twin),
            op_id2string(self.face),
            self.next,
            self.prev,
        )
    }
}

impl fmt::Display for HalfEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", HalfEdge::pretty_header())?;
        writeln!(f, "{}", self.pretty_data())
    }
}

impl fmt::Display for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "HalfEdges:\n{}", self.pretty_edges())?;
        writeln!(f)?;
        writeln!(f, "Verts:\n{}", self.pretty_verts())?;
        writeln!(f)?;
        writeln!(f, "Faces:\n{}", self.pretty_faces())?;
        Ok(())
    }
}
