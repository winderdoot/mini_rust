use bevy::{platform::collections::{HashMap, HashSet}, prelude::*};
use strum::IntoEnumIterator;

use crate::game_logic::{province::*, resources::ResourceType};
use crate::scene::hex_grid::{HexGrid};
use crate::game_systems::*;

/* Constants */
pub const MAX_EMPIRES: u32 = 10;
pub const PLAYER_EMPIRE: u32 = 0;

#[derive(Component, Deref)]
#[relationship_target(relationship = ControlledBy)]
pub struct Controls(Vec<Entity>);

impl Controls {
    /// I think you aren't supposed to use the vec directly, but rather query for child components that have a given parent entity. We'll see
    pub fn get_provinces(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

#[derive(Component)]
pub struct Empire {
    pub id: u32, /* 0..empire_count  */
    pub color: Color,
    pub name: String,

    /* Treasury */
    pub resource_total: HashMap<ResourceType, f32>,
    pub resource_income: HashMap<ResourceType, f32>,
    pub pops_total: u32,
    pub pops_free: u32,
    pub pops_income: u32,
}

pub fn resource_amount(map: &HashMap<ResourceType, f32>, typ: &ResourceType) -> f32 {
    map.get(typ).cloned().unwrap_or(0.0)
}

impl Empire {
    fn starting_resources() -> HashMap<ResourceType, f32> {
        [
            (ResourceType::Gold, 5.0), 
            (ResourceType::Grain, 20.0),
            (ResourceType::Lumber, 10.0),
            (ResourceType::Stone, 5.0)
        ].into()
    }

    pub fn new(id: u32, color: Color, name: String) -> Self {
        Empire {
            id,
            color,
            name,
            resource_total: Self::starting_resources(),
            resource_income: Default::default(),
            pops_total: 2,
            pops_free: 2,
            pops_income: 0
        }
    }

    pub fn get_pops(&self) -> u32 {
        self.pops_total
    }

    pub fn get_free_pops(&self) -> u32 {
        self.pops_free
    }

    pub fn try_remove_free_pop(&mut self) -> bool {
        if self.pops_free == 0 {
            false
        }
        else {
            self.pops_free -= 1;
            true
        }
    }

    pub fn add_free_pop(&mut self) {
        self.pops_free += 1
    }

    pub fn get_total(&self, typ: &ResourceType) -> f32 {
        self.resource_total.get(typ).cloned().unwrap_or(0.0)
    }

    pub fn get_income(&self, typ: &ResourceType) -> f32 {
        self.resource_income.get(typ).cloned().unwrap_or(0.0)
    }

    pub fn has_free_pops(&self) -> bool {
        self.pops_free > 0
    }

    pub fn can_afford(&self, cost: &HashMap<ResourceType, f32>) -> bool {
        cost
            .iter()
            .all(|(k, v)| {
                self.get_total(k) >= *v
            })
    }
}

/// Only used a single time, when so that we can insert the number of provinces into the system that spawns them
#[derive(Resource)]
pub struct EmpireCount(u32);

#[derive(Resource)]
pub struct Empires {
    pub count: u32,
    pub empire_entity: HashMap<u32, Entity>
}

impl Empires {
    pub fn get_entity(&self, empire_id: u32) -> Option<&Entity> {
        self.empire_entity.get(&empire_id)
    }

    pub fn player_empire(&self) -> Option<&Entity> {
        self.empire_entity.get(&PLAYER_EMPIRE)
    }
}

/* Events */
#[derive(Event, Debug)]
pub struct ProvinceClaimed {
    pub empire: Entity,
    pub province: Entity
}

#[derive(Event, Debug)]
pub struct ResourceIncomeChanged {
    pub empire: Entity
}
#[derive(Event, Debug)]
pub struct PopsIncomeChanged {
    pub empire: Entity
}

/* Systems */
pub fn calculate_empire_resource_income(
    event: On<ResourceIncomeChanged>,
    mut q_empires: Query<(&mut Empire, &Controls)>,
    q_provinces: Query<&Province>
) {
    let Ok((mut empire_c, controls)) = q_empires.get_mut(event.empire) else {
        error!("Empire component missing");
        return;
    };
    empire_c.resource_income = controls.0
        .iter()
        .flat_map(|p_ent| q_provinces.get(*p_ent))
        .map(|province| province.get_income())
        .fold(HashMap::<ResourceType, f32>::new(), |total, income| {
            ResourceType::iter()
                .map(|typ| {
                    let combined = resource_amount(&total, &typ) + resource_amount(&income, &typ);

                    (typ, combined)
                })
                .collect()
        });
}

pub fn calculate_empire_pops_income(
    event: On<PopsIncomeChanged>,
    mut q_empires: Query<(&mut Empire, &Controls)>,
    q_provinces: Query<&Province>
) {
    let Ok((mut empire_c, controls)) = q_empires.get_mut(event.empire) else {
        error!("Empire component missing");
        return;
    };
    empire_c.pops_income = controls.0
        .iter()
        .flat_map(|p_ent| q_provinces.get(*p_ent))
        .map(|province| province.get_pops_income())
        .sum();
        
}

fn spawn_empires(
    mut commands: Commands,
    count: Res<EmpireCount>
) {
    let map: HashMap<u32, Entity> = (0..count.0)
        .map(|i| {
            let empire_hue = 0.0.lerp(360.0, (i as f32) / (count.0 as f32));
            let entity = commands.spawn(
                Empire::new (
                    i,
                    Hsla::new(empire_hue, 1.0, 0.3, 1.0).into(),
                    format!("Empire {}", i)
                )
            ).id();

            (i, entity)
        })
        .collect();
    
    commands.insert_resource(
        Empires {
            count: count.0,
            empire_entity: map,
        }
    );
}

pub fn claim_province(
    event: On<ProvinceClaimed>,
    mut q_provinces: Query<&mut Province>,
    mut q_empires: Query<&mut Empire>,
    mut commands: Commands
) {
    let Ok(mut prov) = q_provinces.get_mut(event.province) else {
        error!("Missing province component");
        return;
    };
    let Ok(mut empire) = q_empires.get_mut(event.empire) else {
        error!("Missing empire component");
        return;
    };
    if !empire.has_free_pops() {
        error!("Called claim_province when no free pops available!");
        return;
    }
    empire.pops_free -= 1;
    prov.add_pop();
    commands.trigger(HouseAdded { province: event.province });

    /* Assign the province to the empire */
    commands
        .entity(event.province)
        .insert(ControlledBy(event.empire));

    commands.trigger(ProvinceIncomeChanged { province: event.province });
    commands.trigger(ResourceIncomeChanged { empire: event.empire });
    commands.trigger(PopsIncomeChanged { empire: event.empire });
}

/// Add some starter provinces to each empire
fn assign_provinces(
    empires: Res<Empires>,
    provinces: Query<&Province>,
    grid: Res<HexGrid>,
    mut commands: Commands
) {
    /* Store which provinces we have already assigned to some empire,
     * because commands and buffered and we cannot check if a province is available otherwise */
    let mut assigned = HashSet::<&Entity>::new();

    for (_id, empire) in empires.empire_entity.iter() {
        
        loop {
            let (hex, tile) = grid.get_random_tile();
            let prov = provinces.get(*tile).unwrap();

            if assigned.contains(tile) {
                continue;
            }
            if prov.ptype != ProvinceType::Woods {
                continue;
            }
            /* We found a random unowned woods province  */
            let plains_neighbor = hex
                .all_neighbors()
                .into_iter()
                .find(|nei| {
                    let Some(tile) = grid.get_entity(nei) else {
                        return false;
                    };

                    let Ok(ProvinceType::Plains) = provinces.get(*tile).map(|p| &p.ptype) else {
                        return false;
                    };
                    
                    true
                });
            
            let Some(plains_hex) = plains_neighbor else {
                continue;
            };
            let plains_ent = grid.get_entity(&plains_hex).unwrap();
            let woods_ent = tile;
            /* We have 2 starting provinces we can use */
            assigned.insert(woods_ent);
            assigned.insert(plains_ent);

            commands.trigger(ProvinceClaimed { empire: empire.clone(), province: plains_ent.clone() });
            commands.trigger(ProvinceClaimed { empire: empire.clone(), province: woods_ent.clone() });
            // commands.trigger();
            break;
        }
    }
}


/* Init Plugin */
pub struct EmpirePlugin {
    pub empire_count: u32
}

impl Plugin for EmpirePlugin {
    fn build(&self, app: &mut App) {
        let mut empires = self.empire_count;
        if self.empire_count > MAX_EMPIRES {
            warn!("Empire count cannot exceed {}", MAX_EMPIRES);
            empires = MAX_EMPIRES;
        }

        app
            .insert_resource(EmpireCount(empires))
            .add_systems(Startup, 
                spawn_empires
                .in_set(StartupSystems::CreateEmpires)
            )
            .add_systems(Startup, 
                assign_provinces
                .in_set(StartupSystems::AssignEmpireProvinces)
            );
    }
}