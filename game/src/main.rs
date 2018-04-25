#![allow(dead_code)]
// Configure Clippy to run when testing
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
// Use QuickCheck only when testing
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;

extern crate amethyst;
extern crate cgmath;
extern crate genmesh;

use amethyst::assets::Loader;
use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::core::cgmath::prelude::InnerSpace;
use amethyst::core::cgmath::{Quaternion, Rotation3};
use amethyst::core::transform::{LocalTransform, Transform, TransformBundle};
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::ecs::World;
use amethyst::prelude::*;
use amethyst::input::InputBundle;
use amethyst::renderer::{AmbientColor, Camera, DisplayConfig, DrawShaded, Event, KeyboardInput,
                         Light, Mesh, Pipeline, PointLight, PosNormTex, Projection, RenderBundle,
                         RenderSystem, Rgba, Stage, VirtualKeyCode, WindowEvent};
use amethyst::utils::fps_counter::FPSCounterBundle;
use genmesh::{generators, MapToVertices, Triangulate, Vertices};

mod voxel_grid;
mod camera_bundle;
mod fly_cam;

use camera_bundle::CameraBundle;

const SPHERE_COLOUR: [f32; 4] = [0.0, 0.0, 1.0, 1.0]; // blue
const AMBIENT_LIGHT_COLOUR: Rgba = Rgba(0.01, 0.01, 0.01, 1.0); // near-black
const POINT_LIGHT_COLOUR: Rgba = Rgba(1.0, 1.0, 1.0, 1.0); // white
const BACKGROUND_COLOUR: [f32; 4] = [0.0, 0.0, 0.0, 0.0]; // black
const LIGHT_POSITION: [f32; 3] = [2.0, 2.0, 2.0];
const LIGHT_RADIUS: f32 = 5.0;
const LIGHT_INTENSITY: f32 = 3.0;

struct VallenGameState;

impl State for VallenGameState {
    fn on_start(&mut self, world: &mut World) {
        // Initialize the scene with an object, a light and a camera.
        initialise_terrain(world);
        initialise_lights(world);
        initialise_camera(world);
    }

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }
}

fn run() -> Result<(), amethyst::Error> {
    let display_config_path = format!("{}/resources/display.ron", env!("CARGO_MANIFEST_DIR"));
    let key_bindings_path = format!("{}/resources/controls.ron", env!("CARGO_MANIFEST_DIR"));
    let resources = format!("{}/resources/assets/", env!("CARGO_MANIFEST_DIR"));

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target(BACKGROUND_COLOUR, 1.0)
            .with_pass(DrawShaded::<PosNormTex>::new()),
    );

    let config = DisplayConfig::load(&display_config_path);

    let mut game = Application::build(resources, VallenGameState)?
        .with_bundle(RenderBundle::new())?
        .with_local(RenderSystem::build(pipe, Some(config))?)
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path),
        )?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 0)
        .with_bundle(FPSCounterBundle::default())?
        .with_bundle(CameraBundle)?
        .with_bundle(TransformBundle::new().with_dep(&["fly_cam_system"]))?
        .build()?;
    Ok(game.run())
}

fn initialise_terrain(world: &mut World) {
    use amethyst::assets::Handle;
    use amethyst::renderer::{Material, MaterialDefaults};

    let mut vg = voxel_grid::VoxelGrid::new();

    {
        // Generate a plane of grass in the voxel grid
        let mut first_chunk = voxel_grid::Chunk::new(32);

        for outer in 0..32 {
            for inner in 0..32 {
                first_chunk.set_voxel_at(
                    Vector3::new(outer, 16, inner),
                    voxel_grid::Material::Grass,
                    voxel_grid::QuantizedFloat::new(255),
                );
            }
        }
        vg.insert_chunk(&Vector3::new(0, 0, 0), first_chunk)
    }

    // Turn voxel grid into triangles
    let chunk_size: f32 = 60.0;
    let voxel_size = (chunk_size / 32.0) / 4.0;

    let mut vertex_data: Vec<PosNormTex> = Vec::new();

    for outer in 0..32 {
        for inner in 0..32 {
            vertex_data.extend(
                generators::Cube::new()
                    .vertex(|v| PosNormTex {
                        position: [
                            v.pos[0] + outer as f32 * voxel_size,
                            v.pos[1],
                            v.pos[2] + inner as f32 * voxel_size,
                        ],
                        normal: Vector3::from(v.normal).normalize().into(),
                        tex_coord: [0.1, 0.1],
                    })
                    .triangulate()
                    .vertices()
                    .collect::<Vec<PosNormTex>>(),
            )
        }
    }

    println!("vertices: {:?}", vertex_data.len());

    let (mesh, material) = {
        let loader = world.read_resource::<Loader>();

        let mesh: Handle<Mesh> =
            loader.load_from_data(vertex_data.into(), (), &world.read_resource());

        let albedo = SPHERE_COLOUR.into();

        let tex_storage = world.read_resource();
        let mat_defaults = world.read_resource::<MaterialDefaults>();

        let albedo = loader.load_from_data(albedo, (), &tex_storage);

        let mat = Material {
            albedo,
            ..mat_defaults.0.clone()
        };

        (mesh, mat)
    };

    world
        .create_entity()
        .with(Transform::default())
        .with(mesh)
        .with(material)
        .build();
}

/// This function adds an ambient light and a point light to the world.
fn initialise_lights(world: &mut World) {
    // Add ambient light.
    world.add_resource(AmbientColor(AMBIENT_LIGHT_COLOUR));

    let light: Light = PointLight {
        center: LIGHT_POSITION.into(),
        radius: LIGHT_RADIUS,
        intensity: LIGHT_INTENSITY,
        color: POINT_LIGHT_COLOUR,
        ..Default::default()
    }.into();

    // Add point light.
    world.create_entity().with(light).build();
}

/// This function initializes a camera and adds it to the world.
fn initialise_camera(world: &mut World) {
    let mut local = LocalTransform::default();
    local.translation = Vector3::new(0.0, 0.0, -4.0);
    local.rotation = Quaternion::from_angle_x(Deg(180.0)).into();
    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.3, Deg(60.0))))
        .with(local)
        .with(Transform::default())
        .build();
}

fn main() {
    if let Err(e) = run() {
        println!("Failed to execute example: {}", e);
        ::std::process::exit(1);
    }
}
