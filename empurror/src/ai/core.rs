
use bevy::{prelude::*, platform::collections::*};

use std::cmp::*;
use hexx::*;
use strum::IntoEnumIterator;

use crate::{game_logic::{armies::*, empire::*, province::*, resources::*}, game_systems::StartupSystems, scene::hex_grid::HexGrid};

/* Constants */
pub struct ResourceCost {
    free_pops: u32,
    resources: HashMap<ResourceType, f32>,
}

impl ResourceCost {
    pub fn pops(p: u32) -> Self {
        Self {
            free_pops: p,
            resources: HashMap::new()
        }
    }

    pub fn with_resources(self, resources: HashMap<ResourceType, f32>) -> Self {
        Self {
            free_pops: self.free_pops,
            resources
        }
    }

    pub fn resources(resources: HashMap<ResourceType, f32>) -> Self {
        Self {
            free_pops: 0,
            resources
        }
    }
}

pub enum AIAction {
    ClaimProvince,
    AddHouse,
    AssignPop,
    BuildLumberMill,
    BuildFarm,
    BuildStoneMine,
    BuildGoldMine,
    MakeNewArmy
}

impl AIAction {
    pub fn cost(&self) -> ResourceCost {
        match self {
            AIAction::ClaimProvince => {
                ResourceCost::pops(1)
                    .with_resources(House::build_cost())
            },
            AIAction::AddHouse => ResourceCost::resources(House::build_cost()),
            AIAction::AssignPop => ResourceCost::pops(1),
            AIAction::BuildLumberMill => ResourceCost::resources(SpecialBuilding::LumberMill.build_cost()),
            AIAction::BuildFarm => ResourceCost::resources(SpecialBuilding::Farm.build_cost()),
            AIAction::BuildStoneMine => ResourceCost::resources(SpecialBuilding::StoneMine.build_cost()),
            AIAction::BuildGoldMine => ResourceCost::resources(SpecialBuilding::GoldMine.build_cost()),
            AIAction::MakeNewArmy => {
                ResourceCost::pops(1)
                    .with_resources(SoldierType::Infantry.recruit_cost())
            },
        }
    }
}

/* Resources */
#[derive(Default)]
pub struct AIContext {
    pub path_to_gold: Vec<Hex>,
    pub path_claimed: u32,
    pub provinces_total: u32,
    pub buildings_built: u32
}

#[derive(Resource, Default)]
pub struct AIContexts {
    pub map: HashMap<u32, Option<AIContext>>,
}

impl AIContexts {
    pub fn new(ai_empires: u32) -> Self {
        let map : HashMap<u32, Option<AIContext>> = (0..ai_empires)
            .map(|i| {
                (i + 1, None)
            })
            .collect();

        Self {
            map
        }
    }
}

/* Events */
#[derive(Event, Debug)]
pub struct AIInitContext {
    pub empire_id: u32
}

#[derive(Event, Debug)]
pub struct AIPlayTurn {
    pub empire_id: u32
}

#[derive(Event, Debug)]
pub struct MakeArmies {
    pub empire: Entity
}

#[derive(Event, Debug)]
pub struct AIConstructBuildings {
    pub empire: Entity
}

#[derive(Event, Debug)]
pub struct AIClaimProvinces {
    pub empire: Entity
}

#[derive(Event, Debug)]
pub struct AIAssignPops {
    pub empire: Entity
}

/* Systems */
fn ai_claim_provinces(
    event: On<AIClaimProvinces>,
    mut contexts: ResMut<AIContexts>,
    mut q_empires: Query<&mut Empire>,
    grid: Res<HexGrid>,
    mut commands: Commands
) {
    let Ok(mut ai_empire) = q_empires.get_mut(event.empire) else {
        error!("{}:{} :((", file!(), line!());
        return;
    };
    let Some(Some(context)) = contexts.map.get_mut(&ai_empire.id) else {
        error!("{}:{} :((", file!(), line!());
        return;
    };
    if !ai_empire.has_free_pops() {
        return;
    }

    if let Some(next_hex) = context.path_to_gold.get(context.path_claimed as usize) {
        let Some(province_e) = grid.get_entity(next_hex) else {
            error!("{}:{} :((", file!(), line!());
            return;
        };

        let cost = House::build_cost();
        if ai_empire.can_afford(&cost) {
            ai_empire.remove_resources(&cost);
            commands.trigger(ProvinceClaimed { empire: event.empire, province: *province_e });
            context.path_claimed += 1;
        }
    }
}

fn ai_construct_buildings(
    event: On<AIConstructBuildings>,
    mut contexts: ResMut<AIContexts>,
    mut q_empires: Query<&mut Empire>,
    mut q_provinces: Query<(Entity, &mut Province, &ControlledBy)>,
    mut commands: Commands
) {
    let Ok(mut ai_empire) = q_empires.get_mut(event.empire) else {
        error!("{}:{} :((", file!(), line!());
        return;
    };
    let Some(Some(context)) = contexts.map.get_mut(&ai_empire.id) else {
        error!("{}:{} :((", file!(), line!());
        return;
    };
    context.buildings_built = 0;
    
    q_provinces
        .iter_mut()
        .for_each(|(province_e, mut province_c, controlled_by)| {
            if controlled_by.entity() != event.empire {
                return;
            }
            if !province_c.has_special_building() {
                let Some(building) = province_c.special_building_type() else {
                    return;
                };
                let cost = building.build_cost();
                if ai_empire.can_afford(&cost) {
                    context.buildings_built += 1;
                    ai_empire.remove_resources(&cost);
                    commands.trigger(SpecialBuildingAdded { province: province_e, castle: false });
                }
            }
            else if province_c.get_pops() == province_c.get_max_pops() - 1 && province_c.get_houses() < MAX_HOUSES {
                let cost = House::build_cost();
                if ai_empire.can_afford(&cost) {
                    context.buildings_built += 1;
                    ai_empire.remove_resources(&cost);
                    commands.trigger(HouseAdded { province: province_e });
                }
            }
        });
    
}

fn ai_assign_pops(
    event: On<AIAssignPops>,
    mut q_empires: Query<&mut Empire>,
    mut q_provinces: Query<(&mut Province, &ControlledBy)>
) {
    let Ok(mut ai_empire) = q_empires.get_mut(event.empire) else {
        error!("{}:{} :((", file!(), line!());
        return;
    };
    info!("[{}]: {}", ai_empire.id, resource_string(&ai_empire.resource_total));

    if ai_empire.get_free_pops() == 0 {
        return;
    }

    q_provinces
        .iter_mut()
        .for_each(|(mut province_c, controlled_by)| {
            if controlled_by.entity() != event.empire || ai_empire.get_free_pops() == 0 {
                return;
            }
            let space = max(province_c.get_max_pops() - province_c.get_pops(), 1) - 1;
            (0..space)
                .for_each(|_| {
                    ai_empire
                        .try_remove_free_pop()
                        .then(|| province_c.try_add_pop());
                });
        });
}

fn setup_ai(
    empires: Res<Empires>,
    mut commands: Commands
) {
    commands
        .insert_resource(
            AIContexts::new(empires.count())
    );

    (1..empires.count)
        .for_each(|empire_id| {
            commands.trigger(AIInitContext { empire_id });
        });
}

fn setup_ai_context(
    event: On<AIInitContext>,
    mut contexts: ResMut<AIContexts>,
    empires: Res<Empires>,
    grid: Res<HexGrid>,
    q_owned: Query<(&Province, &ControlledBy)>,
    q_provinces: Query<(&Province, Option<&ControlledBy>)>,
    mut commands: Commands
) {
    let Some(context) = contexts.map.get_mut(&event.empire_id) else {
        error!("{}:{} bad error :((", file!(), line!());
        return;
    };
    let Some(ai_empire) = empires.get_entity(event.empire_id) else {
        error!("{}:{} bad!", file!(), line!());
        return;
    };
    let Some(start_hex) = q_owned
        .iter()
        .find_map(|(province_c, owner)| {
            if owner.entity() == *ai_empire {
                Some(province_c.hex())
            }
            else {
                None
            }
        }) else {
            error!("{}:{} can't find empire province", file!(), line!());
            return;
        };
    
    let cost = |_, b: Hex| {
        let Some(a_b) = grid.get_entity(&b) else {
            return None;
        };
        let Ok((pb_c, cb_c)) = q_provinces.get(*a_b) else {
            return None;
        };
        if let ProvinceType::Water = pb_c.ptype {
            return None;
        }
        if let Some(owner) = cb_c && owner.entity() != *ai_empire{
            return None;
        }

        Some(1u32)
    };
    
    let Some(end_hex) = q_provinces
        .iter()
        .flat_map(|(province_c, _)| {
            let ProvinceType::Mountains = province_c.ptype else {
                return None;
            };
            let path = hexx::algorithms::a_star(start_hex, province_c.hex(), cost);
            let Some(path) = path else {
                return None;
            };
           
            Some((province_c.hex(), path.len()))
        })
        .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
        .map(|(hex, _)| hex) else {
            error!("{}:{} Can't find end hex!", file!(), line!());
            return;
        };

    let Some(path) = hexx::algorithms::a_star(start_hex, end_hex, cost) else {
        error!("{}:{} Can't find path", file!(), line!());
        return;
    };

    let mut claimed = 0;
    path
        .iter()
        .take_while(|hex| {
            let Some(province_e) = grid.get_entity(*hex) else {
                error!("{}:{}", file!(), line!());
                return false;
            };
            let Ok((_, controlled_by)) = q_provinces.get(*province_e) else {
                error!("{}:{}", file!(), line!());
                return false;
            };
            let Some(controlled_by) = controlled_by else {
                return false;
            };
            controlled_by.entity() == *ai_empire
        })
        .for_each(|_| {
            claimed += 1;
        });

    // let field_cost = |h: Hex| {
    //     let Some(a_b) = grid.get_entity(&b) else {
    //         return None;
    //     };
    //     let Ok((pb_c, cb_c)) = q_provinces.get(*a_b) else {
    //         return None;
    //     };
    //     if let ProvinceType::Water = pb_c.ptype {
    //         return None;
    //     }
    //     if let Some(owner) = cb_c && owner.entity() != *ai_empire{
    //         return None;
    //     }

    //     Some(1u32)
    // }; 

    // let field_set = hexx::algorithms::field_of_movement(start_hex, cost) else {
    //     error!("{}:{} Can't find path", file!(), line!());
    //     return;
    // };

    *context = Some(
        AIContext {
            path_to_gold: path,
            path_claimed: claimed,
            provinces_total: 2,
            .. Default::default()
        }
    );
}


fn play_ai_turn(
    event: On<AIPlayTurn>,
    mut contexts: ResMut<AIContexts>,
    empires: Res<Empires>,
    mut commands: Commands
) {
    let Some(Some(context)) = contexts.map.get_mut(&event.empire_id) else {
        error!("{}:{} bad error :((", file!(), line!());
        return;
    };
    let Some(empire_e) = empires.get_entity(event.empire_id) else {
        error!("{}:{} Missing empire entity", file!(), line!());
        return;
    };

    commands.trigger(AIAssignPops { empire: *empire_e });
    commands.trigger(AIConstructBuildings { empire: *empire_e });
    commands.trigger(AIClaimProvinces { empire: *empire_e });
}


/* Plugin */
pub struct EmpireAIPlugin;

impl Plugin for EmpireAIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, 
                setup_ai.in_set(StartupSystems::CreateAI)
            )
            .add_observer(play_ai_turn)
            .add_observer(ai_assign_pops)
            .add_observer(ai_construct_buildings)
            .add_observer(ai_claim_provinces)
            .add_observer(setup_ai_context);
    }
}