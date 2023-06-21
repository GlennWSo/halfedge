use halfedge::plot::show_wireframe;
use three_d::*;

// Entry point for non-wasm
fn main() {
    let positions = Positions::F32(vec![vec3(0., 0., 0.), vec3(1., 0., 0.), vec3(1., 0., 1.)]);
    let indices = Indices::U32(vec![0, 1, 2]);
    // let mut mesh = CpuMesh::cube();
    let mut mesh = CpuMesh {
        positions,
        indices,
        ..Default::default()
    };
    mesh.compute_normals();
    show_wireframe(mesh.clone());
    // show_wireframe(mesh, false);
}
