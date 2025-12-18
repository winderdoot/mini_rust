use bevy::{
    asset::RenderAssetUsages, color::palettes::{css::*, tailwind::*}, mesh::Indices, platform::collections::HashMap,
    prelude::*, render::render_resource::{PrimitiveTopology}
};

use hexx::{shapes, *};
use std::{f32::consts::{PI}, time::*};
use indexmap::IndexMap;
use rand::Rng;

use crate::{game_logic::{empire::{Empire}, province::*, province_generator::*}, scene::mesh_highlight::*, system_sets::StartupSystems};

/* Constants */
pub const HEX_SIZE: f32 = 1.0;
pub const PRISM_HEIGHT: f32 = 1.0;
const MIN_HEIGHT: f32 = 0.0;
const MAX_HEIGHT: f32 = 5.0;
const HEX_X_ROT: f32 = PI / 2.0;
const HEX_Y_ROT: f32 = PI / 6.0;

#[derive(Resource)]
pub struct HexGridSettings {
    pub hex_size: f32,
    pub prism_height: f32,
    pub max_height: f32,
    pub min_height: f32,
    pub materials: HashMap<ProvinceType, Handle<StandardMaterial>>,
    pub hover_materials: HashMap<ProvinceType, Handle<StandardMaterial>>,
    pub empire_materials: HashMap<(ProvinceType, u32), Handle<StandardMaterial>>
}

impl HexGridSettings {
    pub fn province_material(&self, province: &ProvinceType) -> Handle<StandardMaterial> {
        self.materials.get(province).unwrap().clone()
    }

    pub fn hover_material(&self, province: &ProvinceType) -> Handle<StandardMaterial> {
        self.hover_materials.get(province).unwrap().clone()
    }

    pub fn empire_material(&self, province: &ProvinceType, empire_id: u32) -> Handle<StandardMaterial> {
        self.empire_materials.get(&(province.clone(), empire_id)).unwrap().clone()
    }
}

fn load_color_materials(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> (HashMap<ProvinceType, Handle<StandardMaterial>>, HashMap<ProvinceType, Handle<StandardMaterial>>) {
    let mut map = HashMap::<ProvinceType, Handle<StandardMaterial>>::new();
    let mut hover_map = map.clone();
    let emission = LinearRgba::rgb(0.2, 0.2, 0.2);

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

fn create_empire_materials(
    material_map: &HashMap<ProvinceType, Handle<StandardMaterial>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    empires: &Query<&Empire>
) -> HashMap<(ProvinceType, u32), Handle<StandardMaterial>> {
    let empires = empires.iter().collect::<Vec<&Empire>>();

    material_map
        .iter()
        .flat_map(|(p, h)| {
            empires
                .iter()
                .map(|e| {
                    let mat = materials.get(h).unwrap();
                    let mut new_mat = mat.clone();
                    new_mat.emissive = LinearRgba::from(e.color);
                    // TODO: Add clearcoat to water material
                    let new_handle = materials.add(new_mat);
                    
                    ((p.clone(), e.id), new_handle)
                })
                .collect::<HashMap<(ProvinceType, u32), Handle<StandardMaterial>>>()
            }
        )
        .collect()
}

pub fn load_hexgird_settings(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    empires: Query<&Empire>
) {
    let (material_map, hover_map) = load_color_materials(&mut materials);
    let empire_map = create_empire_materials(&material_map, &mut materials, &empires);

    commands.insert_resource(HexGridSettings {
        hex_size: HEX_SIZE,
        prism_height: PRISM_HEIGHT,
        min_height: MIN_HEIGHT,
        max_height: MAX_HEIGHT,
        materials: material_map,
        hover_materials: hover_map,
        empire_materials: empire_map
    });
}

#[derive(Resource, Default)]
pub struct HexGrid {
    pub layout: HexLayout,
    pub entities: IndexMap<Hex, Entity>,
}

impl HexGrid {
    pub fn get_random_tile(&self) -> (&Hex, &Entity) {
        let index = rand::rng().random_range(0..self.entities.len());
        self.entities.get_index(index).unwrap()
    }

    pub fn get_entity(&self, hex: &Hex) -> Option<&Entity> {
        self.entities.get(hex)
    }
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

    let tile_entities: IndexMap<Hex, Entity> = tiles
        .into_iter()
        .map(|(hex, mut pos, province)| {
            let mat = settings.province_material(&province);
            
            /* The prism mesh is extruded along the z axis, we have to translate it and rotate it properly */
            pos.y -= PRISM_HEIGHT * 0.5; 
            let mut transform = Transform::from_translation(pos);
            transform.rotate_axis(Dir3::X, HEX_X_ROT);
            transform.rotate_axis(Dir3::Y, HEX_Y_ROT);

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
        entities: tile_entities,
    });

    commands.spawn(hover_observer);
    commands.spawn(leave_observer);
}

/// Cursed function that only exists because my province tiles have incorrectly oriented meshes and it causes all province children to be horribly
/// oriented
pub fn hextile_rel_transform(tile: &Transform, rel: &Transform) -> Transform {
    let pos = tile.translation + rel.translation;

    Transform::from_translation(pos).with_rotation(rel.rotation)
}



/* Init Plugin */
pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                load_hexgird_settings, 
                setup_hexgrid
            )
            .chain()
            .in_set(StartupSystems::CreateHexGrid)
        );
    }
}