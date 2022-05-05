#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::ScalingMode,
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
    window::PresentMode,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Template".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<MyMaterial>::default())
        .add_startup_system(spawn_quad)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_system(toggle_inspector)
        .run();
}

fn spawn_quad(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<MyMaterial>>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
        material: my_material_assets.add(MyMaterial),
        ..default()
    });
}

#[derive(TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct MyMaterial;

struct MyMaterialGPU {
    bind_group: BindGroup,
}

impl Material2d for MyMaterial {
    fn bind_group(material: &MyMaterialGPU) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[],
        })
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load("my_material.wgsl"))
    }
}

impl RenderAsset for MyMaterial {
    type ExtractedAsset = MyMaterial;
    type PreparedAsset = MyMaterialGPU;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<MyMaterial>>);

    fn extract_asset(&self) -> MyMaterial {
        self.clone()
    }

    fn prepare_asset(
        _extracted_asset: MyMaterial,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<MyMaterialGPU, PrepareAssetError<MyMaterial>> {
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[],
        });
        Ok(MyMaterialGPU { bind_group })
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}
