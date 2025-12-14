use bevy::{
    asset::RenderAssetUsages, color::palettes::css::*, mesh::Indices, platform::collections::HashMap, prelude::*, render::render_resource::PrimitiveTopology, window::PrimaryWindow
};
use hexx::{shapes, *};
use std::f32::consts::{FRAC_PI_2};
use crate::game_logic::province::*;

const GRASS: Color = Color::linear_rgb(0.235, 0.549, 0.129);

#[derive(Resource)]
pub struct HexGridSettings {
    pub hex_size: f32
}

impl Default for HexGridSettings {
    fn default() -> Self {
        Self { hex_size: 1.0 }
    }
}

#[derive(Resource, Default)]
pub struct HexGrid {
    pub layout: HexLayout,
    pub entities: HashMap<Hex, Entity>,
    pub materals: HashMap<ProvinceType, StandardMaterial>,
}

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<HexGridSettings>
) {
    let layout = HexLayout::flat().with_hex_size(settings.hex_size);
    let green_mat = materials.add(Color::Srgba(GREEN));
    let blue_mat = materials.add(Color::Srgba(LIGHT_SEA_GREEN));
    let sandy_mat = materials.add(Color::Srgba(SANDY_BROWN));
    let olive_mat = materials.add(Color::Srgba(OLIVE));

    let mesh = compute_hex_mesh(&layout);
    let mesh_handle = meshes.add(mesh);
    let hexes: Vec<Entity> = shapes::hexagon(hex(0, 0), 10)
        .map(|hex| {
            let mut mat;
            if hex.x % 2 == 0 {
                if hex.y % 2 == 0{
                    mat = &green_mat;
                }
                else {
                    mat = &blue_mat;
                }
            }
            else {
                if hex.y % 2 == 0{
                    mat = &sandy_mat;
                }
                else {
                    mat = &olive_mat;
                }
            }
            let pos = layout.hex_to_world_pos(hex);
            let id = commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(mat.clone()),
                Transform::from_xyz(pos.x, 0.0, pos.y)
            )).id();

            id
        })
        .collect();

    // commands.insert_resource(
    //     HexGrid {
    //         layout,
    //         entities: HashMap::<Hex, Entity>::new(),
    //         materal: grass_mat,
    //     }
    // );


}

fn compute_hex_mesh(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Y)
        .with_scale(Vec3::splat(0.98))
        .build();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}