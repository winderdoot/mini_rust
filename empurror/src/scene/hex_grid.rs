use bevy::{
    asset::RenderAssetUsages, color::palettes::{css::*, tailwind::*}, mesh::Indices, platform::collections::HashMap,
    prelude::*, render::render_resource::PrimitiveTopology, picking::pointer::PointerInteraction
};
use hexx::{shapes, *};
use std::{f32::consts::{FRAC_PI_2, FRAC_PI_3, PI}, time::*};
use crate::{game_logic::{province::*, province_generator::*}, scene::entity_picking::Highlightable};

const HEX_SIZE: f32 = 1.0;
const PRISM_HEIGHT: f32 = 1.0;
const GRASS: Color = Color::linear_rgb(0.235, 0.549, 0.129);

#[derive(Resource)]
pub struct HexGridSettings {
    pub hex_size: f32,
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
        materials: map.clone(),
        hover_materials: map
    });
}

fn load_color_materials(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
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

    commands.insert_resource(HexGridSettings {
        hex_size: HEX_SIZE,
        materials: map,
        hover_materials: hover_map
    });
}

pub fn load_hexgird_settings(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    load_color_materials(&mut commands, &mut materials);
}

#[derive(Resource, Default)]
pub struct HexGrid {
    pub layout: HexLayout,
    pub entities: HashMap<Hex, Entity>,
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
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
    .with_generated_tangents()
    .unwrap()
}

fn compute_hex_prism_mesh(hex_size: f32, height: f32) -> Mesh {
    Extrusion::new(RegularPolygon::new(hex_size, 6), height).into()
}

pub fn setup_hexgrid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<HexGridSettings>
) {
    let layout = HexLayout::flat().with_hex_size(settings.hex_size);

    let hex_tile_mesh = compute_hex_prism_mesh(HEX_SIZE, PRISM_HEIGHT);
    let mesh_handle = meshes.add(hex_tile_mesh);
    let seed : u32= SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32;
    let generator = ProvinceGenerator::new(seed, 0.0, 4.5);

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

pub fn tile_hover<E: EntityEvent>(
    event: On<E>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &mut Highlightable, &Province)>,
    settings: Res<HexGridSettings>
) {
    let entity = event.event_target();
    if let Ok((mut material, mut h, prov)) = query.get_mut(entity) {
        h.highlighted = !h.highlighted;

        if h.highlighted {
            material.0 = settings.hover_material(&prov.prov_type).clone();
        }
        else {
            material.0 = settings.province_material(&prov.prov_type)
        }
    }
}   
