use bevy::{
    asset::RenderAssetUsages, color::palettes::{css::*, tailwind::*}, mesh::Indices, platform::collections::HashMap,
    prelude::*, render::render_resource::PrimitiveTopology
};
use hexx::{shapes, *};
use std::{f32::consts::{FRAC_PI_2, FRAC_PI_3, PI}, time::*};

use crate::{game_logic::{province::*, province_generator::*}, scene::entity_picking::*};

/* Constants */
const HEX_SIZE: f32 = 1.0;
const PRISM_HEIGHT: f32 = 1.0;
const MIN_HEIGHT: f32 = 0.0;
const MAX_HEIGHT: f32 = 5.0;

#[derive(Resource)]
pub struct HexGridSettings {
    pub hex_size: f32,
    pub prism_height: f32,
    pub max_height: f32,
    pub min_height: f32,
    pub materials: HashMap<ProvinceType, Handle<StandardMaterial>>,
    pub hover_materials: HashMap<ProvinceType, Handle<StandardMaterial>>
}

impl HexGridSettings {
    pub fn province_material(&self, province: &ProvinceType) -> Handle<StandardMaterial> {
        self.materials.get(province).unwrap().clone()
    }

    pub fn hover_material(&self, province: &ProvinceType) -> Handle<StandardMaterial> {
        self.hover_materials.get(province).unwrap().clone()
    }
}

fn load_color_materials(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> (HashMap<ProvinceType, Handle<StandardMaterial>>, HashMap<ProvinceType, Handle<StandardMaterial>>) {
    let mut map = HashMap::<ProvinceType, Handle<StandardMaterial>>::new();
    let mut hover_map = map.clone();
    let emission = LinearRgba::rgb(0.3, 0.3, 0.3);

    map.insert(ProvinceType::Water, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(DODGER_BLUE),
            perceptual_roughness: 0.1,
            ..Default::default()
        })
    );
    hover_map.insert(ProvinceType::Water, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(DODGER_BLUE),
            perceptual_roughness: 0.1,
            emissive: emission,
            ..Default::default()
        })
    );

    map.insert(ProvinceType::Hills, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(LIGHT_SLATE_GRAY),
            perceptual_roughness: 0.6,
            ..Default::default()
        })
    );
    hover_map.insert(ProvinceType::Hills, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(LIGHT_SLATE_GRAY),
            perceptual_roughness: 0.6,
            emissive: emission,
            ..Default::default()
        })
    );
    

    map.insert(ProvinceType::Desert, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(KHAKI),
            perceptual_roughness: 0.9,
            ..Default::default()
        })
    );
    hover_map.insert(ProvinceType::Desert, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(KHAKI),
            perceptual_roughness: 0.9,
            emissive: emission,
            ..Default::default()
        })
    );

    map.insert(ProvinceType::Woods, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(SEA_GREEN),
            perceptual_roughness: 0.8,
            ..Default::default()
        })
    );
    hover_map.insert(ProvinceType::Woods, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(SEA_GREEN),
            perceptual_roughness: 0.8,
            emissive: emission,
            ..Default::default()
        })
    );

    map.insert(ProvinceType::Plains, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(OLIVE_DRAB),
            perceptual_roughness: 0.9,
            ..Default::default()
        })
    );
    hover_map.insert(ProvinceType::Plains, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(OLIVE_DRAB),
            perceptual_roughness: 0.9,
            emissive: emission,
            ..Default::default()
        })
    );

    map.insert(ProvinceType::Mountains, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(WHITE_SMOKE),
            perceptual_roughness: 0.35,
            ..Default::default()
        })
    );
    hover_map.insert(ProvinceType::Mountains, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(WHITE_SMOKE),
            perceptual_roughness: 0.35,
            emissive: emission,
            ..Default::default()
        })
    );

    map.insert(ProvinceType::BlackSoil, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(YELLOW_950),
            perceptual_roughness: 0.9,
            ..Default::default()
        })
    );
    hover_map.insert(ProvinceType::BlackSoil, 
        materials.add(StandardMaterial {
            base_color: Color::Srgba(YELLOW_950),
            perceptual_roughness: 0.9,
            emissive: emission,
            ..Default::default()
        })
    );

    (map, hover_map)    
}

pub fn load_hexgird_settings(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (materials, hover_materials) = load_color_materials(&mut materials);

    commands.insert_resource(HexGridSettings {
        hex_size: HEX_SIZE,
        prism_height: PRISM_HEIGHT,
        min_height: MIN_HEIGHT,
        max_height: MAX_HEIGHT,
        materials,
        hover_materials,
    });
}

#[derive(Resource, Default)]
pub struct HexGrid {
    pub layout: HexLayout,
    pub entities: HashMap<Hex, Entity>,
}

#[allow(dead_code)]
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
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
    .with_generated_tangents()
    .unwrap()
}

fn compute_hex_prism_mesh(hex_size: f32, height: f32) -> Mesh {
    Extrusion::new(RegularPolygon::new(hex_size, 6), height).into()
}

/// System responsible for setting up the entire hexgird game board.
/// Also sets up tile hover mechanic via entity observers
pub fn setup_hexgrid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    settings: Res<HexGridSettings>
) {
    let layout = HexLayout::flat().with_hex_size(settings.hex_size);

    let hex_tile_mesh = compute_hex_prism_mesh(HEX_SIZE, PRISM_HEIGHT);
    let mesh_handle = meshes.add(hex_tile_mesh);
    let seed : u32= SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32;
    let generator = ProvinceGenerator::new(seed, MIN_HEIGHT, MAX_HEIGHT);

    let tiles = generator.generate(
        shapes::flat_rectangle([-20, 20, -20, 20]),
        &layout
    );

    let mut hover_observer = Observer::new(tile_hover::<Pointer<Over>>);
    let mut leave_observer = Observer::new(tile_hover::<Pointer<Out>>);

    let tile_entities = tiles
        .into_iter()
        .map(|(hex, mut pos, province)| {
            let mat = settings.province_material(&province);
            
            /* The prism mesh is extruded along the z axis, we have to translate it and rotate it properly */
            pos.y -= PRISM_HEIGHT * 0.05f32; 
            let mut transform = Transform::from_translation(pos);
            transform.rotate_axis(Dir3::X, PI * 0.5);
            transform.rotate_axis(Dir3::Y, PI / 6.0);

            let id = commands.spawn((
                Province { prov_type: province },
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(mat.clone()),
                transform
            )).id();

            hover_observer.watch_entity(id);
            leave_observer.watch_entity(id);

            (hex, id)
        })
        .collect();

    commands.insert_resource(HexGrid {
        layout,
        entities: tile_entities
    });

    commands.spawn(hover_observer);
    commands.spawn(leave_observer);
}

/* Init Plugin */
pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_hexgird_settings, setup_hexgrid).chain());
    }
}