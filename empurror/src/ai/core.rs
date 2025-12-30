
use bevy::{prelude::*, platform::collections::*};

use std::{cmp::*, mem};
use hexx::*;

use crate::{game_logic::{armies::{self, *}, empire::*, province::*, resources::*, turns::Turns}, game_systems::StartupSystems, scene::hex_grid::HexGrid};


/* Resources */
#[derive(Default)]
pub struct AIContext {
    pub path_to_gold: Vec<Hex>,
    pub path_claimed: u32,
    pub gold_reached: bool,
    pub provinces_total: u32,
    pub lumber_mills: u32,
    pub farms: u32,
    pub stone_mines: u32,
    pub gold_mines: u32,
    pub castles: u32,
    pub provinces: u32,
    /* Province -> number of armies (1 soldier each) */
    pub army_heatmap: HashMap<Entity, u32>
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
pub struct AIClaimRandomProvinces {
    pub empire: Entity,
    pub grain_shortage: bool
}

#[derive(Event, Debug)]
pub struct AIAssignPops {
    pub empire: Entity
}

#[derive(Event, Debug)]
pub struct AICreateArmies {
    pub empire: Entity,
}

#[derive(Event, Debug)]
pub struct AIMoveArmies {
    pub empire: Entity
}

/* Systems */
/// Only moves armies defensively between controlled provinces, to spread them out evenly
fn ai_move_armies(
    event: On<AIMoveArmies>,
    mut contexts: ResMut<AIContexts>,
    q_empires: Query<&Empire>,
    q_owned: Query<(Entity, &ControlledBy, &ProvinceArmies), With<Province>>,
    q_provinces: Query<(&Province, Option<&ControlledBy>)>,
    q_armies: Query<&Army>,
    grid: Res<HexGrid>,
    empires: Res<Empires>,
    mut commands: Commands
) {
    let Ok(ai_empire) = q_empires.get(event.empire) else {
        error!("{}:{} :((", file!(), line!());
        return;
    };
    let Some(Some(context)) = contexts.map.get_mut(&ai_empire.id) else {
        error!("{}:{} :((", file!(), line!());
        return;
    };
    
    /* First build new heat map of armies */
    context.army_heatmap = q_owned
        .iter()
        .flat_map(|(province_e, controlled_by, armies_c)| {
            if controlled_by.entity() != event.empire  {
                return None;
            }
            Some((province_e, armies_c.count() as u32))
        })
        .collect::<HashMap<Entity, u32>>();

    /* Move armies to nearby 'cold' provinces */
    q_owned
        .iter()
        .for_each(|(province_e, controlled_by, armies_c)| {
            if controlled_by.entity() != event.empire  {
                return;
            }
            let Ok((province_c, _)) = q_provinces.get(province_e) else {
                error!("{}:{} :((", file!(), line!());
                return;
            };

            armies_c
                .iter()
                .for_each(|army_e| {
                    let Ok(army_c) = q_armies.get(army_e) else {
                        error!("{}:{} :((", file!(), line!());
                        return;
                    };
                    let reachable = armies::get_reachable_tiles(
                        &army_c, &province_c, &q_provinces,
                        &q_empires, &grid, &empires
                    );
                    let Some(target_province) = reachable
                        .iter()
                        .flat_map(|province_e| {
                            let Ok((_, owner)) = q_provinces.get(*province_e) else {
                                return None;
                            };
                            let Some(owner) = owner else {
                                return None;
                            };
                            if owner.entity() != event.empire {
                                return None;
                            }
                            let heat = context.army_heatmap.get(province_e).cloned().unwrap_or(0);

                            Some((province_e, heat))
                        })
                        .min_by_key(|(_, heat )| *heat)
                        .map(|(province_e, _)| *province_e) 
                    else {
                        return;
                    };

                    context
                        .army_heatmap
                        .entry(target_province)
                        .and_modify(|heat| *heat += 1)
                        .or_insert(1);

                    commands.trigger(ArmyMoved { army: army_e, province: target_province });
                });
        });

}

fn ai_create_armies(
    event: On<AICreateArmies>,
    mut contexts: ResMut<AIContexts>,
    q_empires: Query<&Empire>,
    mut s_provinces: ParamSet<(
        Query<(Entity, &Province, &ControlledBy)>,
        Query<&Province>,
        Query<(&Province, Option<&ControlledBy>)>,
    )>,
    mut commands: Commands
) {
    let Ok(ai_empire) = q_empires.get(event.empire) else {
        error!("{}:{} :((", file!(), line!());
        return;
    };
    let Some(Some(context)) = contexts.map.get_mut(&ai_empire.id) else {
        error!("{}:{} :((", file!(), line!());
        return;
    };
    if context.castles == 0 {
        return;
    }
    if !ai_empire.has_free_pops() {
        return;
    }
    let recruit_cost = SoldierType::Infantry.recruit_cost();
    if !ai_empire.can_afford(&recruit_cost) {
        return;
    }

    let q_owned = &s_provinces.p0();
    let ai_castle_provinces =  q_owned
        .iter()
        .filter_map(|(province_e, province_c, owner)| {
            if owner.entity() == event.empire && 
               province_c.has_castle()
            {
                Some(province_e)
            }
            else {
                None
            }
        })
        .collect::<Vec<Entity>>();
    
    let q_provinces = &mut s_provinces.p1();
    let recruited = ai_castle_provinces
        .iter()
        .any(|province_e| {
            let Ok(province_c) = q_provinces.get(*province_e) else {
                error!("{}:{} :(", file!(), line!());
                return false;
            };
            if province_c.pops_extra_room() == 0  {
                return false;
            }

            let upkeep_cost = province_c.soldier_upkeep();
            if ai_empire.has_free_pops() &&
               ai_empire.can_afford(&recruit_cost) &&
               ai_empire.has_income_for(&upkeep_cost) {
                /* Recruiting soldiers and making armies is implemented as events, meaning that we cannot easily do it in batches. 
                 * Hacky solition: schedule the same AIArmiesCreated from here and return early. Wasting cpu time, but whatever.
                 * This opaque type of buffered recursion fits very badly with iterator code, I wrote a bug here initially.
                 * Whatever. */
                
                commands.trigger(SoldierRecruited { soldier: SoldierType::Infantry, empire: event.empire, province: *province_e });
                commands.trigger(AICreateArmies { empire: event.empire });
                true
            }
            else {
                false
            }
        });
    
    if recruited {
        return;
    }
    
    /* If we got to this point, it means that no more soldiers could be recruited. We can try to assign armies */
    ai_castle_provinces
        .iter()
        .for_each(|province_e| {
            let Ok(province_c) = q_provinces.get(*province_e) else {
                error!("{}:{} :(", file!(), line!());
                return;
            };
            let soldiers = province_c.soldier_count();
            if soldiers == 0 {
                return;
            }
            (0..soldiers)
                .for_each(|_| {
                    commands.trigger(ArmyCreated { empire: event.empire, province: *province_e });
                });
        });
    

}

fn ai_claim_random_provinces(
    event: On<AIClaimRandomProvinces>,
    mut contexts: ResMut<AIContexts>,
    mut q_empires: Query<&mut Empire>,
    q_provinces: Query<(&Province, Option<&ControlledBy>)>,
    q_owned: Query<(Entity, &ControlledBy), With<Province>>,
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
        warn!("ai_claim_random_provinces called when no free pops available");
        return;
    }

    let last_ind = context.path_claimed.checked_sub(1);

    let empty_province = 
    if !context.gold_reached && let Some(last_ind) = last_ind {
        let Some(last_reached) = context.path_to_gold.get(last_ind as usize) else {
            error!("{}:{} :(", file!(), line!());
            return;
        };
        last_reached
            .all_neighbors()
            .iter()
            .find_map(|hex| {
                let Some(province_e) = grid.get_entity(hex) else {
                    return None;
                };
                let Ok((province_c, control)) = q_provinces.get(*province_e) else {
                    return None;
                };
                if let Some(_) = control {
                    return None;
                }
                if let ProvinceType::Desert = province_c.ptype {
                    return None;
                }
                if let ProvinceType::Water = province_c.ptype {
                    return None;
                }
                if event.grain_shortage {
                    if let ProvinceType::BlackSoil | ProvinceType::Plains = province_c.ptype {
                        Some(*province_e)
                    }
                    else {
                        None
                    }
                }
                else {
                    Some(*province_e)
                }
            })
    }
    else {
        let ai_provinces =  q_owned
            .iter()
            .filter_map(|(province_e, owner)| {
                if owner.entity() == event.empire {
                    Some(province_e)
                }
                else {
                    None
                }
            })
            .collect::<Vec<Entity>>();

        let mut attempts = 150;
        let found =
        loop {
            let ind = rand::random_range(0..ai_provinces.len());
            let Ok((province_c, _)) = q_provinces.get(ai_provinces[ind]) else {
                error!("{}:{} :((", file!(), line!());
                return;
            };
            if let Some(pick) = province_c
                .hex()
                .all_neighbors()
                .iter()
                .find_map(|hex| {
                    let Some(province_e) = grid.get_entity(hex) else {
                        return None;
                    };
                    let Ok((province_c, control)) = q_provinces.get(*province_e) else {
                        return None;
                    };
                    if let Some(_) = control {
                        return None;
                    }
                    if let ProvinceType::Desert = province_c.ptype {
                        return None;
                    }
                    if let ProvinceType::Water = province_c.ptype {
                        return None;
                    }
                    if event.grain_shortage {
                        if let ProvinceType::BlackSoil | ProvinceType::Plains = province_c.ptype {
                            Some(*province_e)
                        }
                        else {
                            None
                        }
                    }
                    else {
                        Some(*province_e)
                    }
            }) {
                break Some(pick);
            } 
            else if attempts > 0 {
                attempts -= 1;
                continue;
            }
            else {
                break None;
            }
        };

        found            
    };

    let Some(empty_province) = empty_province else {
        return;
    };
    ai_empire.remove_resources(&House::build_cost());
    context.provinces += 1;
    commands.trigger(ProvinceClaimed { empire: event.empire, province: empty_province });
}

fn ai_claim_provinces(
    event: On<AIClaimProvinces>,
    turns: Res<Turns>,
    mut contexts: ResMut<AIContexts>,
    mut q_empires: Query<&mut Empire>,
    q_owner: Query<Option<&ControlledBy>, With<Province>>,
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
    let cost = House::build_cost();
    if !ai_empire.can_afford(&cost) {
        return;
    }
    let grain_shortage = ai_empire.get_income(&ResourceType::Grain) < 5.0 + (turns.full_rounds() / 10) as f32;

    if !grain_shortage && let Some(next_hex) = context.path_to_gold.get(context.path_claimed as usize) {
        let Some(province_e) = grid.get_entity(next_hex) else {
            error!("{}:{} :((", file!(), line!());
            return;
        };

        if matches!(q_owner.get(*province_e), Ok(Some(_))){
            /* Our path to gold has been cut, get random province instead. */
            commands.trigger(AIClaimRandomProvinces { empire: event.empire, grain_shortage });
            return;
        };

        ai_empire.remove_resources(&cost);
        commands.trigger(ProvinceClaimed { empire: event.empire, province: *province_e });
        context.path_claimed += 1;
        context.provinces += 1;
        if context.path_claimed == context.path_to_gold.len() as u32 {
            context.gold_reached = true;
        }
    }
    else {
        /* Gold already reached, get random province instead. */
        commands.trigger(AIClaimRandomProvinces { empire: event.empire, grain_shortage });
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
    
    q_provinces
        .iter_mut()
        .for_each(|(province_e, mut province_c, controlled_by)| {
            if controlled_by.entity() != event.empire {
                return;
            }
            if province_c.can_build_special_building() {
                let ok_to_build_castle = 
                    context.castles < context.gold_mines && context.gold_mines > 0 &&
                    context.farms > std::cmp::max(2 * context.castles, 4) &&
                    matches!(province_c.ptype, ProvinceType::BlackSoil | ProvinceType::Plains | ProvinceType::Woods);
                let castle_cost = SpecialBuilding::Castle.build_cost();
                let random_factor = rand::random_range(0..=1) == 1;
                if ok_to_build_castle && ai_empire.can_afford(&castle_cost) && random_factor {
                    ai_empire.remove_resources(&castle_cost);
                    commands.trigger(SpecialBuildingAdded { province: province_e, castle: true });
                    context.castles += 1;
                    return;
                }

                let Some(building) = province_c.special_building_type() else {
                    return;
                };
                let cost = building.build_cost();
                if ai_empire.can_afford(&cost) {
                    match building {
                        SpecialBuilding::LumberMill => {
                            if context.lumber_mills >= 1 && context.farms < 2 * context.lumber_mills {
                                return;
                            }
                            context.lumber_mills += 1
                        },
                        SpecialBuilding::Farm => {
                            if context.lumber_mills < 1 {
                                return;
                            }
                            context.farms += 1;
                        },
                        SpecialBuilding::GoldMine => {
                            let ok_to_build = 
                                context.lumber_mills > context.gold_mines &&
                                context.farms > 3 * context.gold_mines &&
                                context.stone_mines as f32 > 1.5 * context.gold_mines as f32;
                            if !ok_to_build {
                                return;
                            }
                            context.gold_mines += 1
                        },
                        SpecialBuilding::StoneMine => {
                            let ok_to_build = 
                                context.lumber_mills > context.stone_mines &&
                                context.farms as f32 > 2.5 * context.stone_mines as f32;
                            if !ok_to_build {
                                return;
                            }
                            context.stone_mines += 1
                        },
                        _ => {
                            return;
                        }
                    }
                    ai_empire.remove_resources(&cost);
                    commands.trigger(SpecialBuildingAdded { province: province_e, castle: false });
                }
            }
            else if 
                (province_c.get_pops() == province_c.get_max_pops() - 1 || province_c.has_castle()) &&
                province_c.get_houses() < MAX_HOUSES &&
                context.lumber_mills >= 1 && context.farms >= 1 
            {
                let cost = House::build_cost();
                if ai_empire.can_afford(&cost) {
                    ai_empire.remove_resources(&cost);
                    commands.trigger(HouseAdded { province: province_e });
                }
            }
        });
    
}

fn order_provinces(a: &Province, b: &Province) -> Ordering {
    if mem::discriminant(&a.ptype) == mem::discriminant(&b.ptype) {
        /* If less room, then this province comes later  */
        let room_a = a.pops_extra_room();
        let room_b = b.pops_extra_room();
        return room_a.cmp(&room_b).reverse();
    }
    return a.ptype.order_id().cmp(&b.ptype.order_id());
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
    info!("[{}]: {}", ai_empire.id, resource_string(&ai_empire.total_income()));

    if ai_empire.get_free_pops() == 0 {
        return;
    }

    let mut ai_provinces = q_provinces
        .iter_mut()
        .filter_map(|(province_c, controlled_by)| {
            if controlled_by.entity() != event.empire || ai_empire.get_free_pops() == 0 {
                return None;
            }
            if let ProvinceType::Desert | ProvinceType::Water = province_c.ptype {
                return None;
            }
            /* Castle provinces do not work like this */
            if province_c.has_castle() {
                return None;
            }
            
            Some(province_c)
        })
        .collect::<Vec<Mut<Province>>>();
    ai_provinces.sort_by(|p_a, p_b| order_provinces(p_a, p_b));

    ai_provinces
        .into_iter()
        .for_each(|mut province_c| {
            let space = province_c.pops_extra_room();
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
    q_provinces: Query<(&Province, Option<&ControlledBy>)>
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

    let Some(mut path) = hexx::algorithms::a_star(start_hex, end_hex, cost) else {
        error!("{}:{} Can't find path", file!(), line!());
        return;
    };

    let claimed = 0;
    path = path
        .into_iter()
        .filter_map(|hex| {
            let Some(province_e) = grid.get_entity(&hex) else {
                error!("{}:{}", file!(), line!());
                return None;
            };
            let Ok((_, controlled_by)) = q_provinces.get(*province_e) else {
                error!("{}:{}", file!(), line!());
                return None;
            };
            if let None = controlled_by {
                Some(hex)
            }
            else {
                None
            }
        })
        .collect();

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
    let Some(Some(_)) = contexts.map.get_mut(&event.empire_id) else {
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
    commands.trigger(AICreateArmies { empire: *empire_e });
    commands.trigger(AIMoveArmies { empire: *empire_e });
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
            .add_observer(ai_claim_random_provinces)
            .add_observer(ai_create_armies)
            .add_observer(ai_move_armies)
            .add_observer(setup_ai_context);
    }
}