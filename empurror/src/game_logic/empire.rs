use bevy::{platform::collections::{HashMap, HashSet}, prelude::*};
use strum::IntoEnumIterator;
use std::cmp::*;

use crate::game_logic::{armies::*, province::*, resources::ResourceType};
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

#[derive(Component, Default)]
pub struct Empire {
    pub id: u32, /* 0..empire_count  */
    pub color: Color,
    pub name: String,

    /* Armies */
    pub armies_created: u32, /* Counts how many armies were ever created */

    /* Treasury */
    pub resource_total: HashMap<ResourceType, f32>,
    pub resource_income: HashMap<ResourceType, f32>,
    pub pops_total: u32,
    pub pops_free: u32,
    pub pops_income: u32,

    pub soldiers: u32,    /* Number of recruited soldiers */
    pub max_soldiers: u32, /* Max soldier number, empire could recruit */

    /* Others */
    pub is_playing: bool
}

pub fn resource_amount(map: &HashMap<ResourceType, f32>, typ: &ResourceType) -> f32 {
    map.get(typ).cloned().unwrap_or(0.0)
}

impl Empire {
    pub fn is_bankrupt(&self) -> bool {
        ResourceType::iter()
            .any(|t| {
                self.get_total(&t) < 0.0
            })
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn loose(&mut self) {
        self.is_playing = false;
    }

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
            pops_income: 0,
            soldiers: 0,
            armies_created: 0,
            is_playing: true,
            .. Default::default()
        }
    }

    pub fn total_armies_created(&self) -> u32 {
        self.armies_created
    }

    pub fn new_army_id(&mut self) -> u32 {
        let ret = self.armies_created;
        self.armies_created += 1;
        ret
    }

    pub fn new_army(&mut self) {
        self.armies_created += 1;
    }

    pub fn get_soldiers(&self) -> u32 {
        self.soldiers
    }

    pub fn add_soldier(&mut self) {
        self.soldiers += 1;
    }

    pub fn remove_soldier(&mut self) {
        self.soldiers -= 1;
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

    pub fn add_free_pops(&mut self, amount: u32) {
        self.pops_free += amount
    }

    pub fn get_total(&self, typ: &ResourceType) -> f32 {
        self.resource_total.get(typ).cloned().unwrap_or(0.0)
    }

    pub fn total_income(&self) -> &HashMap<ResourceType, f32> {
        &self.resource_income
    }

    pub fn get_income(&self, typ: &ResourceType) -> f32 {
        self.resource_income.get(typ).cloned().unwrap_or(0.0)
    }

    pub fn get_pops_income(&self) -> u32 {
        self.pops_income
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

    pub fn has_income_for(&self, cost: &HashMap<ResourceType, f32>) -> bool {
        cost
            .iter()
            .all(|(k, v)| {
                self.get_income(k) >= *v
            })
    }

    pub fn remove_resources(&mut self, cost: &HashMap<ResourceType, f32>) {
        cost
            .iter()
            .for_each(|(k, v)| {
                let new_total = self.get_total(k) - v;
                self.resource_total.insert(*k, new_total);
            });
    }

    pub fn apply_income(&mut self) {
        for (k, v) in &self.resource_income {
            let new_total = self.get_total(k) + *v;
            self.resource_total.insert(*k, new_total);
        }
        self.pops_total += self.pops_income;
        self.pops_free += self.pops_income;
    }
}

/// Only used a single time, when so that we can insert the number of provinces into the system that spawns them
#[derive(Resource)]
pub struct EmpireCount(u32);

#[derive(Resource)]
pub struct Empires {
    pub count: u32,
    pub empire_entity: HashMap<u32, Entity>,
    war: HashSet<(u32, u32)>, /* Hold the lower empire id first */
    peace: HashSet<(u32, u32)>,
    peace_time: HashMap<(u32, u32), u32>
}

impl Empires {
    pub fn new(count: u32, map: HashMap<u32, Entity>) -> Self {
        Self {
            count,
            empire_entity: map,
            war: Default::default(),
            peace: Default::default(),
            peace_time: Default::default()
        }
    }
    
    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn get_entity(&self, empire_id: u32) -> Option<&Entity> {
        self.empire_entity.get(&empire_id)
    }

    pub fn player_empire(&self) -> Option<&Entity> {
        self.empire_entity.get(&PLAYER_EMPIRE)
    }

    fn order(a: u32, b: u32) -> (u32, u32) {
        (min(a, b), max(a, b))
    }

    pub fn at_war(&self, empire_a: u32, empire_b: u32) -> bool {
        let (a, b) = Self::order(empire_a, empire_b);
        self.war.contains(&(a, b))
    }

    pub fn set_war(&mut self, empire_a: u32, empire_b: u32) {
        let (a, b) = Self::order(empire_a, empire_b);
        self.peace.remove(&(a, b));
        self.war.insert((a,b));
    }

    pub fn at_peace(&self, empire_a: u32, empire_b: u32) -> bool {
        let (a, b) = Self::order(empire_a, empire_b);
        self.peace.contains(&(a, b))
    }

    pub fn set_peace(&mut self, empire_a: u32, empire_b: u32, turns: u32) {
        let (a, b) = Self::order(empire_a, empire_b);
        self.war.remove(&(a, b));
        self.peace.insert((a,b));
        self.peace_time.insert((a, b), turns);
    }

    pub fn peace_time(&self, empire_a: u32, empire_b: u32) -> Option<u32> {
        let (a, b) = Self::order(empire_a, empire_b);
        self.peace_time.get(&(a, b)).cloned()
    }   

    pub fn update_peace_time(&mut self) {
        self.peace_time
            .iter_mut()
            .for_each(|((a, b), v)| {
                if *v > 0 {
                    *v -= 1;
                }

                if *v == 0 {
                    self.peace.remove(&(*a, *b));
                }
            });
        
        let to_remove = self.peace_time
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(k, _)| *k)
            .collect::<Vec<(u32, u32)>>();

        to_remove
            .iter()
            .for_each(|k| {
                self.peace_time.remove(k);
            })
    }
}

/* Gameplay Events */
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
#[derive(Event, Debug)]
pub struct SoldierRecruited {
    pub soldier: SoldierType,
    pub empire: Entity,
    pub province: Entity
}
#[derive(Event, Debug)]
pub struct ResetEmpireArmies {
    pub empire: Entity
}
#[derive(Event, Debug)]
pub struct EmpireBankrupt {
    pub empire: Entity
}

/* Systems */
pub fn bankrupt_empire(
    event: On<EmpireBankrupt>,
    mut q_provinces: Query<&mut Province>,
    mut q_empires: Query<(&mut Empire, Option<&Controls>)>,
    q_armies: Query<(Entity, &Army, &ArmyProvince)>,
    mut commands: Commands
) {
    let Ok((_, controls)) = q_empires.get_mut(event.empire) else {
        error!("{}:{} Missing empire comonent", file!(), line!());
        return;
    };
    let Some(controls) = controls else {
        /* Nothing needs to be done */
        return;
    };
    
    let empire_armies = q_armies
        .iter()
        .filter(|(_, army_c, _)| {
            army_c.empire() == event.empire
        })
        .map(|(army_e, _, army_province)| (army_e, army_province))
        .collect::<Vec<_>>();

    empire_armies
        .iter()
        .for_each(|(army_e, province_e)| {

            commands
                .entity(*army_e)
                .despawn();

            commands.trigger(ProvinceArmyChanged { province: province_e.entity() });
            commands.trigger(ProvinceIncomeChanged { province: province_e.entity() });
        });

    controls.0
        .iter()
        .for_each(|province_e| {
            let Ok(mut province_c) = q_provinces.get_mut(*province_e) else {
                error!("{}:{} Missing province comonent", file!(), line!());
                return;
            };

            province_c
                .has_castle()
                .then(|| province_c.remove_soliers());
            
            province_c.remove_pops();

            commands
                .entity(*province_e)
                .remove::<ControlledBy>();
        })
}

pub fn reset_armies_moves(
    event: On<ResetEmpireArmies>,
    mut q_armies: Query<&mut Army>
) {
    q_armies
        .iter_mut()
        .filter(|army_c| army_c.empire() == event.empire)
        .for_each(|mut army_c| {
            army_c.reset_moved();
        })
}

pub fn recruit_soldier(
    event: On<SoldierRecruited>,
    mut q_provinces: Query<&mut Province>,
    mut q_empires: Query<&mut Empire>,
    mut commands: Commands
) {
    let Ok(mut empire_c) = q_empires.get_mut(event.empire) else {
        error!("{}:{} Missing empire comonent", file!(), line!());
        return;
    };
    let Ok(mut province_c) = q_provinces.get_mut(event.province) else {
        error!("{}:{} Missing province comonent", file!(), line!());
        return;
    };

    /* We already know these should succeed */
    empire_c.remove_resources(&event.soldier.recruit_cost());
    empire_c.try_remove_free_pop();
    empire_c.add_soldier();
    province_c.try_add_pop(); /* Pop in this province is used to calculate the province upkeep for the soldier */
    province_c.add_soldier(Soldier::new_infantry(&event.province));

    commands.trigger(ProvinceIncomeChanged { province: event.province });
    commands.trigger(ResourceIncomeChanged { empire: event.empire });
    commands.trigger(PopsIncomeChanged { empire: event.empire });
}

/// Used in special situations (startup) to force all provinces to recalculate their income/upkeep. Would be faster with exclusive world access
/// but there aren't many controlled provinces at the start so whatever.
pub fn calculate_all_provinces_income(
    q_empires: Query<(Entity, &Controls), With<Empire>>,
    mut commands: Commands
) {
    q_empires
        .iter()
        .for_each(|(_, controls)| {
            controls
                .get_provinces()
                .for_each(|province| {
                    commands.trigger(ProvinceIncomeChanged { province: *province });
                });

            // This is not neccessary, because we do it at the beginning of each turn
            // commands.trigger(ResourceIncomeChanged { empire });
            // commands.trigger(PopsIncomeChanged { empire });
        });
}

pub fn calculate_empire_resource_net_income(
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
        .map(|province| (province.get_income(), province.get_upkeep()))
        .map(|(income, upkeep)| {
            ResourceType::iter()
                .map(|typ| {
                    let typ_income = resource_amount(&income, &typ);
                    let typ_cost = resource_amount(&upkeep, &typ);
                    (typ, typ_income - typ_cost)
                })
                .collect()
        })
        .fold(HashMap::<ResourceType, f32>::new(), |total, net_income| {
            ResourceType::iter()
                .map(|typ| {
                    let combined = resource_amount(&total, &typ) + resource_amount(&net_income, &typ);
                    (typ, combined)
                })
                .collect()
        });

    /* Consider making homeless pops eat too, and then raise the food consumption for working pops */
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

    empire_c.max_soldiers = controls.0
        .iter()
        .flat_map(|p_ent| q_provinces.get(*p_ent))
        .filter(|province_c| province_c.has_castle())
        .map(|province| province.get_max_pops())
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
        Empires ::new(count.0, map)
    );
}

pub fn claim_province(
    event: On<ProvinceClaimed>,
    mut q_provinces: Query<&mut Province>,
    mut q_empires: Query<&mut Empire>,
    mut commands: Commands,
    systems: Res<GameSystems>
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
    let Some(system) = systems.get(stringify!(reset_province_materials)) else {
        error!("{}:{} missing game system", file!(), line!());
        return;
    };
    commands.run_system(*system);
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
            let Some((hex, tile)) = grid.get_random_tile() else {
                error!("{}:{} this should never occur", file!(), line!());
                return;
            };
            let Ok(prov) = provinces.get(*tile) else {
                error!("{}:{} missing province entity", file!(), line!());
                return;
            };

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

                    if assigned.contains(tile) {
                        return false;
                    }
                    
                    true
                });
            
            let Some(plains_hex) = plains_neighbor else {
                continue;
            };
            let Some(plains_ent) = grid.get_entity(&plains_hex) else {
                error!("{}:{} missing grid entity", file!(), line!());
                return;
            };
            let woods_ent = tile;
            /* We have 2 starting provinces we can use */
            assigned.insert(woods_ent);
            assigned.insert(plains_ent);

            commands.trigger(ProvinceClaimed { empire: empire.clone(), province: plains_ent.clone() });
            commands.trigger(ProvinceClaimed { empire: empire.clone(), province: woods_ent.clone() });
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