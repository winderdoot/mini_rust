use bevy::{color::palettes::{css::*, tailwind::*}, platform::collections::HashMap, prelude::*};
use std::{cmp::{Eq, min}, f32::consts::PI};
// use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::game_logic::{empire::Controls, resources::ResourceType};
use crate::scene::assets::Models;   
use crate::scene::{hex_grid, mesh_highlight::*};

pub const MAX_HOUSES: u32 = 3;

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

impl std::fmt::Display for ProvinceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvinceType::Water => write!(f, "Water"),
            ProvinceType::BlackSoil => write!(f, "Black soil"),
            ProvinceType::Plains => write!(f, "Plains"),
            ProvinceType::Woods => write!(f, "Woods"),
            ProvinceType::Desert => write!(f, "Desert"),
            ProvinceType::Hills => write!(f, "Rocky hills"),
            ProvinceType::Mountains => write!(f, "Mountains"),
        }
    }
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

#[derive(Component)]
pub struct Province {
    pub ptype: ProvinceType,
    house_count: u32,
    pops: u32,
    max_pops: u32,
    upkeep: HashMap<ResourceType, f32>,
    income: HashMap<ResourceType, f32>,
    pops_income: u32,
}

impl Province {
    pub fn from_type(t: &ProvinceType) -> Self {
        Self {
            ptype: t.clone(),
            house_count: 0,
            pops: 1,
            max_pops: 0,
            upkeep: Default::default(),
            income: Default::default(),
            pops_income: 0,
        }
    }

    pub fn get_houses(&self) -> u32 {
        self.house_count
    }

    pub fn get_pops(&self) -> u32 {
        self.pops
    }

    pub fn get_max_pops(&self) -> u32 {
        self.max_pops
    }

    pub fn get_upkeep(&self) -> &HashMap<ResourceType, f32> {
        &self.upkeep
    }

    pub fn get_income(&self) -> &HashMap<ResourceType, f32> {
        &self.income
    }

    pub fn get_pops_income(&self) -> u32 {
        self.pops_income
    }

    fn upkeep(&self) -> HashMap<ResourceType, f32> {
        let pop_cost = match self.ptype {
            ProvinceType::Water => {
                error!("Water province can't be owned");
                return HashMap::new();
            },
            ProvinceType::BlackSoil => 1.0,
            ProvinceType::Plains => 1.0,
            ProvinceType::Woods => 1.25,
            ProvinceType::Desert => 4.0,
            ProvinceType::Hills => 2.25,
            ProvinceType::Mountains => 3.5,
        };
        let food_cost = pop_cost * (self.pops as f32);

        return HashMap::from([(ResourceType::Grain, food_cost)]);
    }

    fn base_income(&self) -> HashMap<ResourceType, f32> {
        match self.ptype {
            ProvinceType::BlackSoil => {
                return [(ResourceType::Grain, self.pops as f32 * 2.25)].into();
            },
            ProvinceType::Plains => {
                return [(ResourceType::Grain, self.pops as f32 * 1.5)].into();
            },
            ProvinceType::Woods => {
                return [(ResourceType::Lumber, self.pops as f32 * 1.5)].into();
            },
            ProvinceType::Hills => {
                return [(ResourceType::Stone, self.pops as f32 * 1.0)].into();
            },
            ProvinceType::Mountains => {
                return [(ResourceType::Gold, self.pops as f32 * 1.0)].into();
            },
            _ => Default::default()
        }
    }
}

/* Relationships */

/* Source of truth in the ControlledBy <-> Controls relationship */
#[derive(Component, Deref)]
#[relationship(relationship_target = Controls)]
pub struct ControlledBy(pub Entity);

/* Source of truth in LocatedIn <-> ProvinceBuildings  */
#[derive(Component, Deref)]
#[relationship(relationship_target = ProvinceBuildings)]
pub struct LocatedIn(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = LocatedIn)]
pub struct ProvinceBuildings(Vec<Entity>);

impl ProvinceBuildings {
    pub fn get_buildings(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

/* Buildings */
#[derive(Component)]
pub struct House {
    pub max_residents: u32
}

#[derive(Component)]
pub enum SpecialBuilding {
    Farm,
    LumberMill,
    StoneMine,
    GoldMine,
    Castle
}

impl SpecialBuilding {
    pub fn income(&self, ptype: &ProvinceType, workers: u32) -> HashMap<ResourceType, f32> {
        match ptype {
            ProvinceType::BlackSoil => {
                return [(ResourceType::Grain, workers as f32 * 6.0)].into();
            },
            ProvinceType::Plains => {
                return [(ResourceType::Grain, workers as f32 * 3.0)].into();
            },
            ProvinceType::Woods => {
                return [(ResourceType::Lumber, workers as f32 * 4.0)].into();
            },
            ProvinceType::Hills => {
                return [(ResourceType::Stone, workers as f32 * 3.0)].into();
            },
            ProvinceType::Mountains => {
                return [(ResourceType::Gold, workers as f32 * 3.0)].into();
            },
            _ => Default::default()
        }
    }
}

/* Building events  */
#[derive(Event, Debug)]
pub struct HouseAdded {
    pub province: Entity
}

/* Building type is deduced based on the province type */
#[derive(Event, Debug)]
pub struct SpecialBuildingAdded {
    pub province: Entity,
}
/// Used to recalculate province upkeep/production/population
#[derive(Event, Debug)]
pub struct ProvinceIncomeChanged {
    pub province: Entity
}

/* Systems */
// Recalculate all province income/upkeep values
pub fn calculate_province_income(
    event: On<ProvinceIncomeChanged>,
    mut q_provinces: Query<(&mut Province, &ProvinceBuildings)>,
    q_houses: Query<&House>,
    q_special_buldings: Query<&SpecialBuilding>
) {
    let Ok((mut p, buildings)) = q_provinces.get_mut(event.province) else {
        return;
    };
    p.max_pops = 0;
    p.income = HashMap::new();
    
    /* Applied only if there is no resource building */
    let mut building_income = HashMap::<ResourceType, f32>::new();

    buildings
        .get_buildings()
        .for_each(|building| {
            if let Ok(btype) = q_special_buldings.get(*building) {
                building_income.extend(btype.income(&p.ptype, p.pops));
            }
            else {
                let Ok(house) = q_houses.get(*building) else {
                    error!("Invalid building entity");
                    return;
                };
                p.max_pops += house.max_residents;
            }
        });

    p.upkeep = p.upkeep();
    p.pops_income = min(2, p.max_pops - p.pops);

    p.income =  
    if building_income.is_empty() {
        p.base_income()
    } else {
        building_income
    };
}

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
        House { max_residents: 5 },
        LocatedIn(event.province),
        SceneRoot(models.house.clone()),
        transform
    ));
}


pub fn add_resource_building(
    event: On<SpecialBuildingAdded>,
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
    
    let (model, building_type) = match prov.ptype {
        ProvinceType::BlackSoil | ProvinceType::Plains => {
            (models.farm.clone(), SpecialBuilding::Farm)
        },
        ProvinceType::Woods => {
            (models.farm.clone(), SpecialBuilding::LumberMill)
        },
        ProvinceType::Hills => {
            (models.farm.clone(), SpecialBuilding::StoneMine)
        },
        ProvinceType::Mountains => {
            (models.farm.clone(), SpecialBuilding::GoldMine)
        },
        _ => {
            warn!("add_resource_building called on {:?} province type", prov.ptype);
            return;
        }
    };

    let building_id = commands.spawn((
        LocatedIn(event.province),
        building_type,
        SceneRoot(model),
        transform
    )).id();
}
