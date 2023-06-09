use three_d::*;

pub fn default_window() -> Result<Window<()>, WindowError> {
    Window::new(WindowSettings {
        title: "Wireframe!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
}

pub fn vertex_transformations(cpu_mesh: &CpuMesh) -> Instances {
    Instances {
        transformations: cpu_mesh
            .positions
            .to_f32()
            .into_iter()
            .map(|p| Mat4::from_translation(p))
            .collect(),
        ..Default::default()
    }
}

pub fn edge_transformations(cpu_mesh: &CpuMesh) -> Instances {
    let indices = cpu_mesh.indices.to_u32().unwrap();
    let positions = cpu_mesh.positions.to_f32();
    let mut transformations = Vec::new();
    let mut keys = Vec::new();
    for f in 0..indices.len() / 3 {
        let mut fun = |i1, i2| {
            let key = if i1 < i2 { (i1, i2) } else { (i2, i1) };
            if !keys.contains(&key) {
                keys.push(key);
                let p1: Vec3 = positions[i1];
                let p2: Vec3 = positions[i2];
                transformations.push(
                    Mat4::from_translation(p1)
                        * Into::<Mat4>::into(Quat::from_arc(
                            vec3(1.0, 0.0, 0.0),
                            (p2 - p1).normalize(),
                            None,
                        ))
                        * Mat4::from_nonuniform_scale((p1 - p2).magnitude(), 1.0, 1.0),
                );
            }
        };
        let i1 = indices[3 * f] as usize;
        let i2 = indices[3 * f + 1] as usize;
        let i3 = indices[3 * f + 2] as usize;
        fun(i1, i2);
        fun(i2, i3);
        fun(i3, i1);
    }
    Instances {
        transformations,
        ..Default::default()
    }
}
