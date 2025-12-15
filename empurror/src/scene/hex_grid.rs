use bevy::{
    asset::RenderAssetUsages, color::palettes::css::*, mesh::Indices, platform::collections::HashMap, prelude::*, render::render_resource::PrimitiveTopology, window::PrimaryWindow
};
use hexx::{shapes, *};
use std::f32::consts::{FRAC_PI_2};
use crate::game_logic::province::*;
use noise::Perlin;


const HEX_SIZE: f32 = 1.0;
const GRASS: Color = Color::linear_rgb(0.235, 0.549, 0.129);

#[derive(Resource)]
pub struct HexGridSettings {
    pub hex_size: f32,
    pub materials: HashMap<ProvinceType, Handle<StandardMaterial>>
}

#[allow(dead_code)]
fn load_texture_materials(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>
) {
    let mut map = HashMap::<ProvinceType, Handle<StandardMaterial>>::new();

    let water_img: Handle<Image> = asset_server.load("textures/tex_Water.png");
    let stone_img: Handle<Image> = asset_server.load("textures/stone.png");
    let sand_img: Handle<Image> = asset_server.load("textures/sand.png");
    let darkgrass_img: Handle<Image> = asset_server.load("textures/grass_dark.png");
    let grass_img: Handle<Image> = asset_server.load("textures/grass.png");
    let snow_img: Handle<Image> = asset_server.load("textures/snow.png");
    
    map.insert(ProvinceType::Water, 
        materials.add(StandardMaterial {
            base_color_texture: Some(water_img),
            perceptual_roughness: 0.1,
            ..Default::default()
        })
    );
    map.insert(ProvinceType::Hills, 
        materials.add(StandardMaterial {
            base_color_texture: Some(stone_img),
            perceptual_roughness: 0.6,
            ..Default::default()
        })
    );
    map.insert(ProvinceType::Desert, 
        materials.add(StandardMaterial {
            base_color_texture: Some(sand_img),
            perceptual_roughness: 0.9,
            ..Default::default()
        })
    );
    map.insert(ProvinceType::Woods, 
        materials.add(StandardMaterial {
            base_color_texture: Some(darkgrass_img),
            perceptual_roughness: 0.8,
            ..Default::default()
        })
    );
    map.insert(ProvinceType::Plains, 
        materials.add(StandardMaterial {
            base_color_texture: Some(grass_img),
            perceptual_roughness: 0.9,
            ..Default::default()
        })
    );
    map.insert(ProvinceType::Mountains, 
        materials.add(StandardMaterial {
            base_color_texture: Some(snow_img),
            perceptual_roughness: 0.3,
            ..Default::default()
        })
    );

    commands.insert_resource(HexGridSettings {
        hex_size: HEX_SIZE,
        materials: map
    });
}

fn load_color_materials(
    mut commands: &Commands,
    mut materials: &ResMut<Assets<StandardMaterial>>
) {
    let mut map = HashMap::<ProvinceType, Handle<StandardMaterial>>::new();
    
    
}

pub fn load_hexgird_settings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut map = HashMap::<ProvinceType, Handle<StandardMaterial>>::new();

    // TODO: Zrobić emmisive materiał do tile'a na którym jest myszka
    let water_img: Handle<Image> = asset_server.load("textures/tex_Water.png");
    let stone_img: Handle<Image> = asset_server.load("textures/stone.png");
    let sand_img: Handle<Image> = asset_server.load("textures/sand.png");
    let darkgrass_img: Handle<Image> = asset_server.load("textures/grass_dark.png");
    let grass_img: Handle<Image> = asset_server.load("textures/grass.png");
    let snow_img: Handle<Image> = asset_server.load("textures/snow.png");
    
    map.insert(ProvinceType::Water, 
        materials.add(StandardMaterial {
            base_color_texture: Some(water_img),
            perceptual_roughness: 0.1,
            ..Default::default()
        })
    );
    map.insert(ProvinceType::Hills, 
        materials.add(StandardMaterial {
            base_color_texture: Some(stone_img),
            perceptual_roughness: 0.6,
            ..Default::default()
        })
    );
    map.insert(ProvinceType::Desert, 
        materials.add(StandardMaterial {
            base_color_texture: Some(sand_img),
            perceptual_roughness: 0.9,
            ..Default::default()
        })
    );
    map.insert(ProvinceType::Woods, 
        materials.add(StandardMaterial {
            base_color_texture: Some(darkgrass_img),
            perceptual_roughness: 0.8,
            ..Default::default()
        })
    );
    map.insert(ProvinceType::Plains, 
        materials.add(StandardMaterial {
            base_color_texture: Some(grass_img),
            perceptual_roughness: 0.9,
            ..Default::default()
        })
    );
    map.insert(ProvinceType::Mountains, 
        materials.add(StandardMaterial {
            base_color_texture: Some(snow_img),
            perceptual_roughness: 0.3,
            ..Default::default()
        })
    );

    commands.insert_resource(HexGridSettings {
        hex_size: HEX_SIZE,
        materials: map
    });
}

#[derive(Resource, Default)]
pub struct HexGrid {
    pub layout: HexLayout,
    pub entities: HashMap<Hex, Entity>,
}

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<HexGridSettings>
) {
    let layout = HexLayout::flat().with_hex_size(settings.hex_size);
    // let green_mat = materials.add(Color::Srgba(GREEN));
    // let blue_mat = materials.add(Color::Srgba(LIGHT_SEA_GREEN));
    // let sandy_mat = materials.add(Color::Srgba(SANDY_BROWN));
    // let olive_mat = materials.add(Color::Srgba(OLIVE));

    let mesh = compute_hex_mesh(&layout);
    let mesh_handle = meshes.add(mesh);
    let entities = shapes::hexagon(hex(0, 0), 10)
        .map(|hex| {
            let mat;
            if hex.x % 2 == 0 {
                if hex.y % 2 == 0{
                    mat = settings.materials.get(&ProvinceType::Mountains).unwrap();
                }
                else {
                    mat = settings.materials.get(&ProvinceType::Water).unwrap();
                }
            }
            else {
                if hex.y % 2 == 0{
                    mat = settings.materials.get(&ProvinceType::Woods).unwrap();
                }
                else {
                    mat = settings.materials.get(&ProvinceType::Hills).unwrap();
                }
            }
            let pos = layout.hex_to_world_pos(hex);

            let id = commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(mat.clone()),
                Transform::from_xyz(pos.x, 0.0, pos.y)
            )).id();

            (hex, id)
        })
        .collect();

    commands.insert_resource(HexGrid {
        layout,
        entities
    });
}

fn compute_hex_mesh(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Y)
        .with_scale(Vec3::splat(1.0))
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