use bevy::{prelude::*, platform::collections::*};

use strum_macros::{Display, EnumIter};
use hexx::{Hex};

use crate::{game_logic::{province::*, resources::*}, scene::hex_grid::*};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, Display)]
pub enum SoldierType {
    Infantry
}

impl SoldierType {
    pub fn recruit_cost(&self) -> HashMap<ResourceType, f32> {
        match self {
            SoldierType::Infantry => [(ResourceType::Gold, 1.0), (ResourceType::Grain, 5.0)].into(),
        }
    }

    pub fn march_budget(&self) -> u32 {
        match self {
            SoldierType::Infantry => 10,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Soldier {
    pub stype: SoldierType,
    pub home_province: Entity
}

impl Soldier {
    pub fn new_infantry(province: &Entity) -> Self {
        Self {
            stype: SoldierType::Infantry,
            home_province: province.clone()
        }
    }
}

#[derive(Component, Debug)]
pub struct Army {
    pub atype: SoldierType,
    pub soldiers: Vec<Soldier>,
    pub empire: Entity,
    pub id: u32,
    moved: bool /* If moved this turn */
}

impl Army {
    pub fn new(soldier: Soldier, empire: Entity, id: u32) -> Self {
        Self {
            atype: soldier.stype,
            soldiers: vec![soldier],
            empire,
            id,
            moved: false
        }
    }

    pub fn march_budget(&self) -> u32 {
        if self.soldiers.len() < 10 {
            return self.atype.march_budget();
        }

        return self.atype.march_budget() / 2;
    }

    pub fn soldier_type(&self) -> SoldierType {
        self.atype.clone()
    }

    pub fn try_add_soldier(&mut self, soldier: Soldier) -> bool {
        if soldier.stype != self.atype {
            return false;
        }
        self.soldiers.push(soldier);

        true
    }

    pub fn try_remove_soldier(&mut self) -> Option<Soldier> {
        self.soldiers.pop()
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn soldiers(&self) -> &Vec<Soldier> {
        return &self.soldiers
    }
    
    pub fn soldiers_iter(&self) -> impl Iterator<Item = &Soldier> {
        return self.soldiers.iter()
    }

    pub fn soldier_count(&self) -> usize {
        self.soldiers.len()
    }

    pub fn set_moved(&mut self) {
        self.moved = true;
    }

    pub fn moved(&self) -> bool {
        self.moved
    }

    pub fn reset_moved(&mut self) {
        self.moved = false;
    }
}

impl std::fmt::Display for Army {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Army {} ({})", self.id, self.atype)
    }
}

/* Relationships */

/* Source of truth in ArmyProvince <-> ProvinceArmies */
#[derive(Component, Deref)]
#[relationship(relationship_target = ProvinceArmies)]
pub struct ArmyProvince(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ArmyProvince)]
pub struct ProvinceArmies(Vec<Entity>);

impl ProvinceArmies {
    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = Entity> {
        self.0.iter().cloned()
    }

    pub fn armies(&self) -> &Vec<Entity> {
        &self.0
    }
}

/* Events */
#[derive(Event, Debug)]
pub struct ArmyMoved {
    pub army: Entity,
    pub province: Entity
}

/* Systems */

pub fn move_army(
    event: On<ArmyMoved>,
    mut q_armies: Query<&mut Army>,
    mut commands: Commands
) {
    commands
        .entity(event.army)
        .remove::<ArmyProvince>()
        .insert(ArmyProvince(event.province));

    let Ok(mut army_c) = q_armies.get_mut(event.army) else {
        error!("{}:{} Missing army component", file!(), line!());
        return;
    };
    army_c.set_moved();
}

pub fn get_reachable_tiles(
    army_c: &Army,
    province_c: &Province,
    q_provinces: &Query<&Province>,
    grid: &Res<HexGrid>
) -> HashSet<Entity> {

    let budget = army_c.march_budget();
    let const_func = |hex: Hex| {
        let Some(province_e) = grid.get_entity(&hex) else {
            return None;
        };
        let Ok(province_c) = q_provinces.get(*province_e) else {
            return None;
        };

        return province_c.march_cost();
    };

    let hex_set = hexx::algorithms::field_of_movement(province_c.hex(), budget, const_func);

    return hex_set
        .into_iter()
        .filter_map(|hex| grid.get_entity(&hex))
        .cloned()
        .collect();
}