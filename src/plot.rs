use std::collections::HashMap;

use crate::{Coord, Mesh};
// use std::collections::HashMap;
use three_d::{renderer::*, FrameInputGenerator, Mesh as GPUMesh, Srgba as Color, WindowedContext};
use winit::{
    self,
    // event_loop::EventLoop,
    event_loop::EventLoopBuilder,
    window::Window,
};

// struct Plot {
//     window: Window,
//     wc: WindowedContext,
//     inp_gen: FrameInputGenerator,
//     camera: Camera,
//     objects: Objects,
// }

// struct Plotter {
//     evl: EventLoop<()>,
//     windows: HashMap<u32, Box<Plot>>,
// }

struct WireFrame {
    model: Gm<GPUMesh, PhysicalMaterial>,
    edges: Gm<InstancedMesh, PhysicalMaterial>,
    vertices: Gm<InstancedMesh, PhysicalMaterial>,
}

impl WireFrame {
    fn get_object_iter(&self) -> impl IntoIterator<Item = impl Object + '_> {
        self.model
            .into_iter()
            .chain(&self.edges)
            .chain(&self.vertices)
    }
    fn new(mesh: CpuMesh, context: &WindowedContext) -> Self {
        let mut model_material = PhysicalMaterial::new_opaque(
            context,
            &CpuMaterial {
                albedo: Color::new_opaque(50, 50, 50),
                roughness: 0.7,
                metallic: 0.8,
                ..Default::default()
            },
        );
        model_material.render_states.cull = Cull::Back;
        let model = Gm::new(GPUMesh::new(&context, &mesh), model_material);

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

        let edges = {
            let mut cylinder = CpuMesh::cylinder(10);
            cylinder
                .transform(&Mat4::from_nonuniform_scale(1.0, 0.007, 0.007))
                .unwrap();
            let imesh = InstancedMesh::new(&context, &edge_transformations(&mesh), &cylinder);
            Gm::new(imesh, wireframe_material.clone())
        };

        let vertices = {
            let mut sphere = CpuMesh::sphere(8);
            sphere.transform(&Mat4::from_scale(0.015)).unwrap();
            let imesh = InstancedMesh::new(&context, &vertex_transformations(&mesh), &sphere);
            Gm::new(imesh, wireframe_material)
        };
        WireFrame {
            model,
            edges,
            vertices,
        }
    }
}

impl Into<Vector3<f64>> for Coord {
    fn into(self) -> Vector3<f64> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl Into<CpuMesh> for Mesh {
    fn into(self) -> CpuMesh {
        let positions = Positions::F64(self.verts().iter().map(|v| v.coord.into()).collect());
        let indices = Indices::U32(self.tri_inds().flatten().collect());
        let mut mesh = CpuMesh {
            positions,
            indices,
            ..Default::default()
        };
        mesh.compute_normals();
        mesh
    }
}

impl Mesh {
    pub fn plot(self) -> ! {
        show_wireframe(self.into())
    }
}

pub fn show_wireframes(meshes: Vec<CpuMesh>) -> ! {
    let mut builder = EventLoopBuilder::new();
    // OBS! By default, the window is only allowed to be created on the main thread, to make platform compatibility easier.
    let event_loop = builder.build();

    let plots = meshes.into_iter().map(|mesh| {
        let window_builder = winit::window::WindowBuilder::new()
            .with_title("winit window")
            .with_min_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .with_maximized(true);
        let window = window_builder.build(&event_loop).unwrap();
        let context = WindowedContext::from_winit_window(
            &window,
            three_d::SurfaceSettings {
                vsync: false, // Wayland hangs in swap_buffers when one window is minimized or occluded
                ..three_d::SurfaceSettings::default()
            },
        )
        .unwrap();

        let target = vec3(0.0, 0.5, 0.5);
        let scene_radius = 6.0;
        let camera = Camera::new_orthographic(
            Viewport::new_at_origo(1, 1),
            target + scene_radius * vec3(0.6, 0.3, 1.0).normalize(),
            target,
            vec3(0.0, 0.0, 1.0),
            5.,
            0.1,
            100.0,
        );
        let control = OrbitControl::new(*camera.target(), 0.1 * scene_radius, 100.0 * scene_radius);

        let ambient_light = AmbientLight::new(&context, 0.7, Color::WHITE);
        let directional0 =
            DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0));
        let directional1 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(1.0, 1.0, 1.0));

        let wf = WireFrame::new(mesh, &context);

        // main loop
        let input_gen = three_d::FrameInputGenerator::from_winit_window(&window);
        WFPlot {
            wf,
            input_gen,
            control,
            camera,
            ambient_light,
            dir_lights: vec![directional0, directional1],
            window,
            ctx: context,
        }
    });
    let mut windows = HashMap::new();
    for p in plots {
        windows.insert(p.window.id(), p);
    }
    event_loop.run(move |event, _, control_flow| match &event {
        winit::event::Event::MainEventsCleared => {
            for p in windows.values() {
                p.window.request_redraw();
            }
        }
        winit::event::Event::RedrawRequested(window_id) => {
            if let Some(plot) = windows.get_mut(window_id) {
                plot.ctx.make_current().unwrap();
                let mut frame_input = plot.input_gen.generate(&plot.ctx);
                plot.control
                    .handle_events(&mut plot.camera, &mut frame_input.events);
                plot.camera.set_viewport(frame_input.viewport);
                frame_input
                    .screen()
                    .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                    .render(
                        &plot.camera,
                        plot.wf.get_object_iter(),
                        &[
                            &plot.ambient_light,
                            &plot.dir_lights[0],
                            &plot.dir_lights[1],
                        ],
                    );
                plot.ctx.swap_buffers().unwrap();
                control_flow.set_poll();
                plot.window.request_redraw();
            }
        }
        winit::event::Event::WindowEvent { window_id, event } => {
            if let Some(plot) = windows.get_mut(window_id) {
                plot.input_gen.handle_winit_window_event(event);
                match event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        plot.ctx.resize(*physical_size)
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        plot.ctx.resize(**new_inner_size)
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        plot.ctx.make_current().unwrap();
                        windows.remove(window_id);
                        if windows.is_empty() {
                            control_flow.set_exit();
                        }
                    }

                    _ => (),
                }
            }
        }
        _ => (),
    })
}

/// stuff needed to plot a WireFrame
struct WFPlot {
    wf: WireFrame,
    input_gen: FrameInputGenerator,
    control: OrbitControl,
    camera: Camera,
    ambient_light: AmbientLight,
    dir_lights: Vec<DirectionalLight>,
    window: Window,
    ctx: WindowedContext,
}

pub fn show_wireframe(mesh: CpuMesh) -> ! {
    let mut builder = EventLoopBuilder::new();
    // OBS! By default, the window is only allowed to be created on the main thread, to make platform compatibility easier.
    let event_loop = builder.build();

    let window_builder = winit::window::WindowBuilder::new()
        .with_title("winit window")
        .with_min_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .with_maximized(true);

    let window = window_builder.build(&event_loop).unwrap();
    let context = WindowedContext::from_winit_window(
        &window,
        three_d::SurfaceSettings {
            vsync: false, // Wayland hangs in swap_buffers when one window is minimized or occluded
            ..three_d::SurfaceSettings::default()
        },
    )
    .unwrap();

    let target = vec3(0.0, 0.5, 0.5);
    let scene_radius = 6.0;
    let mut camera = Camera::new_orthographic(
        Viewport::new_at_origo(1, 1),
        target + scene_radius * vec3(0.6, 0.3, 1.0).normalize(),
        target,
        vec3(0.0, 0.0, 1.0),
        5.,
        0.1,
        100.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 0.1 * scene_radius, 100.0 * scene_radius);

    let ambient = AmbientLight::new(&context, 0.7, Color::WHITE);
    let directional0 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0));
    let directional1 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(1.0, 1.0, 1.0));

    let wf = WireFrame::new(mesh, &context);

    // main loop
    let mut frame_input_generator = three_d::FrameInputGenerator::from_winit_window(&window);
    event_loop.run(move |event, _, control_flow| {
        match &event {
            winit::event::Event::MainEventsCleared => window.request_redraw(),
            winit::event::Event::RedrawRequested(_) => {
                // context.make_current().unwrap();
                let mut frame_input = frame_input_generator.generate(&context);
                control.handle_events(&mut camera, &mut frame_input.events);
                camera.set_viewport(frame_input.viewport);
                // scene.model.animate(frame_input.accumulated_time as f32);
                frame_input
                    .screen()
                    .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                    .render(
                        &camera,
                        wf.get_object_iter(),
                        &[&ambient, &directional0, &directional1],
                    );

                context.swap_buffers().unwrap();
                control_flow.set_poll();
                window.request_redraw();
            }
            winit::event::Event::WindowEvent {
                event,
                window_id: _,
            } => {
                frame_input_generator.handle_winit_window_event(event);
                match event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        context.resize(*physical_size);
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        context.resize(**new_inner_size);
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        // context.make_current().unwrap();
                        control_flow.set_exit();
                    }

                    _ => (),
                }
            }

            _ => (),
        }
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
