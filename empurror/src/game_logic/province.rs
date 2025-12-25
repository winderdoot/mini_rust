use bevy::{color::palettes::{css::*, tailwind::*}, platform::collections::HashMap, prelude::*};
use hexx::Hex;

use std::{cmp::{Eq, min}, f32::consts::PI};
use strum_macros::{Display, EnumIter};

use crate::game_logic::{armies::*, empire::{Controls, Empire}, resources::ResourceType};
use crate::scene::assets::Models;   
use crate::scene::{hex_grid};

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
    hex: Hex,
    special_building: bool,
    castle: bool,
    house_count: u32,
    pops: u32,
    max_pops: u32,
    upkeep: HashMap<ResourceType, f32>,
    income: HashMap<ResourceType, f32>,
    soldiers: Vec<Soldier>, /* Free soldiers unassigned to armies */
    army_model: Option<Entity>
}

impl Province {
    pub fn from_type(t: &ProvinceType, hex: &Hex) -> Self {
        Self {
            ptype: t.clone(),
            hex: hex.clone(),
            special_building: false,
            castle: false,
            house_count: 0,
            pops: 0,
            max_pops: 0,
            upkeep: Default::default(),
            income: Default::default(),
            soldiers: Default::default(),
            army_model: None
        }
    }

    pub fn get_army_model(&self) -> Option<Entity> {
        self.army_model
    }

    pub fn set_army_model(&mut self, to: Option<Entity>) {
        self.army_model = to;
    }

    pub fn hex(&self) -> Hex {
        self.hex.clone()
    }

    pub fn has_pops_room(&self) -> bool {
        self.pops < self.max_pops
    }

    pub fn soldier_count(&self) -> usize {
        self.soldiers.len()
    }

    pub fn add_soldier(&mut self, soldier: Soldier) {
        self.soldiers.push(soldier);
    }

    pub fn soldiers_iter(&self) -> impl Iterator<Item = &Soldier> {
        self.soldiers.iter()
    }

    pub fn try_remove_soldier(&mut self) -> Option<Soldier> {
        self.soldiers.pop()
    }

    pub fn try_remove_soldier_type(&mut self, typ: &SoldierType) -> Option<Soldier> {
        let Some(ind) = self.soldiers
            .iter()
            .enumerate()
            .find_map(|(i, s)| {
                if s.stype == *typ {
                    Some(i)
                }
                else {
                    None
                }
            }) 
            else {
                return None;
            };

        Some(self.soldiers.remove(ind))
    }

    pub fn building_name(&self) -> String {
        match self.ptype {
            ProvinceType::BlackSoil | ProvinceType::Plains => String::from("Farm"),
            ProvinceType::Woods => String::from("Lumber Mill"),
            ProvinceType::Hills => String::from("Stone Mine"),
            ProvinceType::Mountains => String::from("Gold Mine"),
            _ => {
                error!("Bad error, we don't like it woo");
                return String::new()
            }
        }
    }

    pub fn special_building_type(&self) -> Option<SpecialBuilding> {
        match self.ptype {
            ProvinceType::BlackSoil | ProvinceType::Plains => Some(SpecialBuilding::Farm),
            ProvinceType::Woods => Some(SpecialBuilding::LumberMill),
            ProvinceType::Hills => Some(SpecialBuilding::StoneMine),
            ProvinceType::Mountains => Some(SpecialBuilding::GoldMine),
            _ => None
        }
    }

    pub fn try_add_pop(&mut self) -> bool {
        if self.pops < self.max_pops {
            self.pops += 1;
            true
        }
        else {
            false
        }
    }

    pub fn try_remove_pop(&mut self) -> bool {
        if self.pops > 1 {
            self.pops -= 1;
            true
        }
        else {
            false
        }
    }

    /// Use carefulyy, doesn't perform checks
    pub fn add_pop(&mut self) {
        self.pops += 1;
    }

    pub fn can_build_special_building(&self) -> bool {
        /* Castle is also a special building, so this is overkill */
        if self.special_building || self.castle {
            false
        }
        else if let ProvinceType::Desert | ProvinceType::Water = self.ptype {
            false
        }
        else {
            true
        }
    }

    pub fn has_castle(&self) -> bool {
        self.castle
    }

    pub fn has_special_building(&self) -> bool {
        self.special_building
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
        min(1, self.max_pops - self.pops)
    }

    fn upkeep(&self) -> HashMap<ResourceType, f32> {
        let mut pop_cost = match self.ptype {
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
        if self.castle {
            pop_cost *= 2.5;
        }
        let gold_cost = if self.castle {
            5.0
        } else {
            0.0
        };
        let food_cost = pop_cost * (self.pops as f32);

        return HashMap::from([(ResourceType::Grain, food_cost), (ResourceType::Gold, gold_cost)]);
    }

    fn base_income(&self) -> HashMap<ResourceType, f32> {
        if self.castle {
            return HashMap::new();
        }

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

    pub fn march_cost(&self) -> Option<u32> {
        match self.ptype {
            ProvinceType::Water => None,
            ProvinceType::BlackSoil => Some(2),
            ProvinceType::Plains => Some(2),
            ProvinceType::Woods => Some(3),
            ProvinceType::Desert => Some(5),
            ProvinceType::Hills => Some(4),
            ProvinceType::Mountains => Some(6),
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

impl House {
    pub fn build_cost() -> HashMap<ResourceType, f32> {
        [(ResourceType::Lumber, 5.0)].into()
    }
}

#[derive(Component, Debug, Display)]
pub enum SpecialBuilding {
    Farm,
    LumberMill,
    StoneMine,
    GoldMine,
    Castle
}

impl SpecialBuilding {
    pub fn income(&self, ptype: &ProvinceType, workers: u32) -> HashMap<ResourceType, f32> {
        match self {
            SpecialBuilding::Castle => return Default::default(),
            _ => {}
        }

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

    pub fn build_cost(&self) -> HashMap<ResourceType, f32> {
        match self {
            SpecialBuilding::Farm => [(ResourceType::Lumber, 10.0)].into(),
            SpecialBuilding::LumberMill => [(ResourceType::Lumber, 15.0)].into(),
            SpecialBuilding::StoneMine => [(ResourceType::Lumber, 20.0)].into(),
            SpecialBuilding::GoldMine => [(ResourceType::Lumber, 25.0), (ResourceType::Stone, 6.0)].into(),
            SpecialBuilding::Castle => [(ResourceType::Lumber, 10.0), (ResourceType::Stone, 1.0)].into(),
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
    pub castle: bool
}
/// Used to recalculate province upkeep/production/population
#[derive(Event, Debug)]
pub struct ProvinceIncomeChanged {
    pub province: Entity
}

#[derive(Event, Debug)]
pub struct ArmyCreated {
    pub empire: Entity,
    pub province: Entity
}

#[derive(Event, Debug)]
pub struct ArmyDisbanded {
    pub army: Entity
}

// Used to update army models displayed on the province tile
#[derive(Event, Debug)]
pub struct ProvinceArmyChanged {
    pub province: Entity
}

/* Systems */
pub fn update_army_model(
    event: On<ProvinceArmyChanged>,
    models: Res<Models>,
    mut q_provinces: Query<(&Transform, &mut Province, Option<&ProvinceArmies>)>,
    mut commands: Commands
) {
    let Ok((province_t, mut province_c, armies_c_o)) = q_provinces.get_mut(event.province) else {
        error!("{}:{} Missing province component", file!(), line!());
        return;
    };
    let army_count = armies_c_o.map_or(0, |armies_c| armies_c.count());
    match (province_c.get_army_model(), army_count) {
        (None, 0) => {
            warn!("{}:{} update_army_model has no effect", file!(), line!());
            return;
        },
        (None, 1..) => {
            let transl = Vec3::new(0.4, 0.8, 0.3);
            let desired = Transform::from_translation(transl);
            let transform = hex_grid::hextile_rel_transform(&province_t, &desired)
                .with_scale(Vec3::splat(0.35));

            province_c.set_army_model(Some(
                commands.spawn((
                    ArmyModel, /* Marker component - not used for now */
                    transform,

                    SceneRoot(models.knight.clone())
                )).id())
            );
        },
        (Some(model_e), 0) => {
            province_c.set_army_model(None);
            commands
                .entity(model_e)
                .despawn();
        },
        (Some(_), 1..) => {
            /* We don't have to do anything */ 
            info!(" both model and army are present");
        }
    }
}

pub fn create_army(
    event: On<ArmyCreated>,
    mut q_provinces: Query<&mut Province>,
    mut q_empires: Query<&mut Empire>,
    mut commands: Commands
) {
    let Ok(mut province_c) = q_provinces.get_mut(event.province) else {
        error!("{}:{} Missing province component", file!(), line!());
        return;
    };
    let Ok(mut empire_c) = q_empires.get_mut(event.empire) else {
        error!("{}:{} Missing empire component", file!(), line!());
        return;
    };
    let Some(soldier) = province_c.try_remove_soldier() else {
        error!("{}:{} Create army called, but no soldiers found in province", file!(), line!());
        return;
    };
    let army = Army::new(soldier, event.empire, empire_c.new_army_id());

    commands
        .spawn((
            army,
            ArmyProvince(event.province)
        ));

    commands.trigger(ProvinceArmyChanged { province: event.province });
}

pub fn disband_army(
    event: On<ArmyDisbanded>,
    mut q_provinces: Query<&mut Province>,
    q_armies: Query<(&Army, &ArmyProvince)>,
    mut commands: Commands
) {
    let Ok((army_c, province_e)) = q_armies.get(event.army) else {
        error!("{}:{} Missing army component", file!(), line!());
        return;
    };
    let Ok(mut province_c) = q_provinces.get_mut(province_e.entity()) else {
        error!("{}:{} Missing province component", file!(), line!());
        return;
    };

    army_c
        .soldiers_iter()
        .for_each(|soldier| {
            province_c.add_soldier(soldier.clone());
        });

    commands
        .entity(event.army)
        .despawn();

    commands.trigger(ProvinceArmyChanged { province: province_e.entity() });
}

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

    commands.trigger(ProvinceIncomeChanged { province: event.province });
}


pub fn add_special_building(
    event: On<SpecialBuildingAdded>,
    models: Res<Models>,
    mut q_provinces: Query<(&Transform, &mut Province)>,
    mut commands: Commands,
) {
    let Ok((prov_transform, mut prov)) = q_provinces.get_mut(event.province) else {
        return;
    };

    if !prov.can_build_special_building() {
        error!("Cannot build special building in this province");
        return;
    }
    prov.special_building = true;
    prov.castle = event.castle;

    if event.castle {
        /* Resettle all non soldier pops */
        prov.pops = 0;
    }

    let transl = Vec3::new(0.0, 0.5, 0.0);
    let desired = Transform::from_translation(transl);
    let transform = hex_grid::hextile_rel_transform(&prov_transform, &desired);
    
    let (model, building_type) = match (prov.ptype.clone(), event.castle) {
        (_, true) => {
            (models.castle.clone(), SpecialBuilding::Castle)
        }
        (ProvinceType::BlackSoil | ProvinceType::Plains, false) => {
            (models.farm.clone(), SpecialBuilding::Farm)
        },
        (ProvinceType::Woods, false) => {
            (models.farm.clone(), SpecialBuilding::LumberMill)
        },
        (ProvinceType::Hills, false) => {
            (models.farm.clone(), SpecialBuilding::StoneMine)
        },
        (ProvinceType::Mountains, false) => {
            (models.farm.clone(), SpecialBuilding::GoldMine)
        },
        _ => {
            warn!("add_resource_building called on {:?} province type", prov.ptype);
            return;
        }
    };

    commands.spawn((
        LocatedIn(event.province),
        building_type,
        SceneRoot(model),
        transform
    ));

    commands.trigger(ProvinceIncomeChanged { province: event.province });
}
