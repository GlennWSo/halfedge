use halfedge::plot::{default_window, edge_transformations, vertex_transformations};
use three_d::*;

// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let positions = Positions::F32(vec![vec3(0., 0., 0.), vec3(1., 0., 0.), vec3(1., 0., 1.)]);
    let indices = Indices::U32(vec![0, 1, 2]);
    // let mesh = CpuMesh::cube();
    let mut mesh = CpuMesh {
        positions,
        indices,
        ..Default::default()
    };
    mesh.compute_normals();

    render(mesh).await;
}

pub async fn render(mesh: CpuMesh) {
    let window = default_window().unwrap();
    let context = window.gl();
    let target = vec3(0.0, 0.5, 0.5);
    let scene_radius = 6.0;
    let mut camera = Camera::new_orthographic(
        window.viewport(),
        target + scene_radius * vec3(0.6, 0.3, 1.0).normalize(),
        target,
        vec3(0.0, 0.0, 1.0),
        5.,
        0.1,
        100.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 0.1 * scene_radius, 100.0 * scene_radius);

    let mut model_material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Color::new_opaque(50, 50, 50),
            roughness: 0.7,
            metallic: 0.8,
            ..Default::default()
        },
    );
    model_material.render_states.cull = Cull::Back;
    let model = Gm::new(Mesh::new(&context, &mesh), model_material);
    let mut wireframe_material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Color::new_opaque(220, 50, 50),
            roughness: 0.7,
            metallic: 0.8,
            ..Default::default()
        },
    );
    wireframe_material.render_states.cull = Cull::Back;
    let mut cylinder = CpuMesh::cylinder(10);
    cylinder
        .transform(&Mat4::from_nonuniform_scale(1.0, 0.007, 0.007))
        .unwrap();
    let edges = Gm::new(
        InstancedMesh::new(&context, &edge_transformations(&mesh), &cylinder),
        wireframe_material.clone(),
    );

    let mut sphere = CpuMesh::sphere(8);
    sphere.transform(&Mat4::from_scale(0.015)).unwrap();
    let vertices = Gm::new(
        InstancedMesh::new(&context, &vertex_transformations(&mesh), &sphere),
        wireframe_material,
    );

    let ambient = AmbientLight::new(&context, 0.7, Color::WHITE);
    let directional0 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0));
    let directional1 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(1.0, 1.0, 1.0));

    // main loop
    window.render_loop(move |mut frame_input| {
        let mut redraw = frame_input.first_frame;
        redraw |= camera.set_viewport(frame_input.viewport);
        redraw |= control.handle_events(&mut camera, &mut frame_input.events);

        if redraw {
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .render(
                    &camera,
                    model.into_iter().chain(&vertices).chain(&edges),
                    &[&ambient, &directional0, &directional1],
                );
        }

        FrameOutput {
            swap_buffers: redraw,
            ..Default::default()
        }
    });
}
