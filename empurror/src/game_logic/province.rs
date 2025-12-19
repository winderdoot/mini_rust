use bevy::{prelude::*, color::palettes::{tailwind::*, css::*}};
use std::{cmp::Eq, f32::consts::PI};
// use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::game_logic::empire::Controls;
use crate::scene::assets::Models;   
use crate::scene::{hex_grid, mesh_highlight::*};

#[derive(Hash, Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum ProvinceType {
    Water,
    BlackSoil,
    Plains,
    Woods,
    Desert,   
    Hills,
    Mountains,
}

impl ProvinceType {
    pub fn terrain_color(&self) -> Color {
        match self {
            ProvinceType::Water => Color::Srgba(DODGER_BLUE),
            ProvinceType::BlackSoil => Color::Srgba(YELLOW_950),
            ProvinceType::Plains => Color::Srgba(OLIVE_DRAB),
            ProvinceType::Woods => Color::Srgba(SEA_GREEN),
            ProvinceType::Desert => Color::Srgba(KHAKI),
            ProvinceType::Hills => Color::Srgba(LIGHT_SLATE_GRAY),
            ProvinceType::Mountains => Color::Srgba(WHITE_SMOKE),
        }
    }

    pub fn terrain_roughness(&self) -> f32 {
        match self {
            ProvinceType::Water => 0.1,
            ProvinceType::BlackSoil => 0.9,
            ProvinceType::Plains => 0.9,
            ProvinceType::Woods => 0.8,
            ProvinceType::Desert => 0.9,
            ProvinceType::Hills => 0.6,
            ProvinceType::Mountains => 0.35,
        }
    }
}

/* Source of truth in the ControlledBy <-> Controls relationship */
#[derive(Component, Deref)]
#[relationship(relationship_target = Controls)]
pub struct ControlledBy(pub Entity);

#[derive(Component)]
#[require(Highlightable, Selectable)]
pub struct Province {
    pub prov_type: ProvinceType,
    pub house_count: u32,
    pub resource_building: bool
}

impl Province {
    pub fn from_type(t: &ProvinceType) -> Self {
        Self {
            prov_type: t.clone(),
            house_count: 0,
            resource_building: false
        }
    }
}

/* Source of truth in LocatedIn <-> ProvinceBuildings  */
#[derive(Component, Deref)]
#[relationship(relationship_target = ProvinceBuildings)]
pub struct LocatedIn(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = LocatedIn)]
pub struct ProvinceBuildings(Vec<Entity>);


/* Buildings */
#[derive(Component, Default)]
pub struct Building;

#[derive(Component)]
#[require(Building)]
pub struct House {
    pub population: u32,
    pub max_population: u32
}

#[derive(Component)]
#[require(Building)]
pub struct Farm {
    pub level: u32
}

#[derive(Component)]
#[require(Building)]
pub struct StoneMine {
    pub level: u32
}

#[derive(Component)]
#[require(Building)]
pub struct LumberMill {
    pub level: u32
}

#[derive(Component)]
#[require(Building)]
pub struct GoldMine {
    pub level: u32
}

#[derive(Component)]
#[require(Building)]
pub struct Castle {
    pub level: u32
}


/* Building events  */
#[derive(Event, Debug)]
pub struct HouseAdded {
    pub province: Entity
}

/* Building type is deduced based on the province type */
#[derive(Event, Debug)]
pub struct ResourceBuildingAdded {
    pub province: Entity,
}

/* Systems */
pub fn add_house(
    event: On<HouseAdded>,
    models: Res<Models>,
    mut q_provinces: Query<(&Transform, &mut Province)>,
    mut commands: Commands,
) {
    let Ok((prov_transform, mut prov)) = q_provinces.get_mut(event.province) else {
        return;
    };
    if prov.house_count >= 3 {
        warn!("add_house called when province has 3 houses");
        return;
    }
    let angle = 2.0*PI/3.0;
    let dir = Vec3::X.rotate_y((prov.house_count + 1) as f32 * angle);
    let rot_ang = match prov.house_count {
        0 => PI / 6.0,
        1 => -PI / 6.0,
        2.. => PI / 2.0
    };

    prov.house_count += 1;

    let transl = dir.clamp_length_max(0.65) + Vec3::new(0.0, 0.5, 0.0);
    let desired = Transform::from_translation(transl).with_rotation(Quat::from_rotation_y(rot_ang));
    let transform = hex_grid::hextile_rel_transform(&prov_transform, &desired);

    commands.spawn((
        House { population: 0, max_population: 5 },
        LocatedIn(event.province),
        SceneRoot(models.house.clone()),
        transform
    ));
}


pub fn add_resource_building(
    event: On<ResourceBuildingAdded>,
    models: Res<Models>,
    mut q_provinces: Query<(&Transform, &mut Province)>,
    mut commands: Commands,
) {
    let Ok((prov_transform, prov)) = q_provinces.get_mut(event.province) else {
        return;
    };

    let transl = Vec3::new(0.0, 0.5, 0.0);
    let desired = Transform::from_translation(transl);
    let transform = hex_grid::hextile_rel_transform(&prov_transform, &desired);
    
    let building_id = commands.spawn((
        LocatedIn(event.province),
        transform
    )).id();

    match prov.prov_type {
        ProvinceType::BlackSoil | ProvinceType::Plains => {
            commands
                .entity(building_id)
                .insert(Farm { level: 1 })
                .insert(SceneRoot(models.farm.clone()));
        },
        ProvinceType::Woods => {
            commands
                .entity(building_id)
                .insert(LumberMill { level: 1 })
                .insert(SceneRoot(models.farm.clone()));
        },
        ProvinceType::Hills => {
            commands
                .entity(building_id)
                .insert(StoneMine { level: 1 });
        },
        ProvinceType::Mountains => {
            commands
                .entity(building_id)
                .insert(GoldMine { level: 1 });
        },
        _ => {
            warn!("add_resource_building called on province: {:?}", prov.prov_type);
            return;
        }
    };
}


