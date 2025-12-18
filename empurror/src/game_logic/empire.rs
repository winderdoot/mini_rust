use std::f32::consts::PI;

use bevy::platform::collections::HashSet;
use bevy::{platform::collections::HashMap, prelude::*};

use crate::game_logic::province::*;
use crate::scene::assets::Models;
use crate::scene::hex_grid::{self, HexGrid};
use crate::system_sets::*;

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
    pub color: Color
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
}


/* Systems */
fn spawn_empires(
    mut commands: Commands,
    count: Res<EmpireCount>
) {
    let map: HashMap<u32, Entity> = (0..count.0)
        .map(|i| {
            let empire_hue = 0.0.lerp(360.0, (i as f32) / (count.0 as f32));
            let entity = commands.spawn(
                Empire { 
                    id: i,
                    color: Hsla::new(empire_hue, 1.0, 0.3, 1.0).into()
                }
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

fn add_province_to_empire(
    empire: &Entity,
    province: &Entity,
    commands: &mut Commands,
    models: &Res<Models>,
    q_transforms: &Query<&Transform, With<Province>>,
) {
    let Ok(p_transform) = q_transforms.get(*province) else {
        return;
    };
    let desired = Transform::from_xyz(0.0, 0.5, 0.0);
    let transform = hex_grid::hextile_rel_transform(p_transform, &desired);
    
    /* Spawn house */
    commands.spawn((
        House { population: 0, max_population: 5 },
        LocatedIn(*province),
        SceneRoot(models.house.clone()),
        transform
    ));
    /* Assign the province to the empire */
    commands
        .entity(*province)
        .insert(ControlledBy(*empire));
}

/// Add some starter provinces to each empire
fn assign_provinces(
    empires: Res<Empires>,
    q_transforms: Query<&Transform, With<Province>>,
    provinces: Query<&Province>,
    grid: Res<HexGrid>,
    models: Res<Models>,
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
            if prov.prov_type != ProvinceType::Woods {
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

                    let Ok(ProvinceType::Plains) = provinces.get(*tile).map(|p| &p.prov_type) else {
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

            add_province_to_empire(empire, plains_ent, &mut commands, &models, &q_transforms);
            add_province_to_empire(empire, woods_ent, &mut commands, &models, &q_transforms);
            
            break;
        }
    }
}

fn debug_empire(
    empires: Res<Empires>,
    provinces: Query<&Controls>,
) {
    let empire = empires.get_entity(0).unwrap();
    let x = provinces.get(*empire).unwrap();

    info!("empire 0 controls: {} provinces", x.0.len());

    // let empire = empires.get_entity(1).unwrap();
    // let x = provinces.get(*empire).unwrap();

    // info!("empire 1 controls: {} provinces", x.0.len());
}

/* Init Plugin */
pub struct EmpirePlugin {
    pub empire_count: u32
}

impl Plugin for EmpirePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(EmpireCount(self.empire_count))
            .add_systems(Startup, 
                spawn_empires
                .in_set(StartupSystems::CreateEmpires)
            )
            .add_systems(Startup, 
                (assign_provinces, debug_empire).chain()
                .in_set(StartupSystems::AssignEmpireProvinces)
            );
    }
}