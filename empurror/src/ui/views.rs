use bevy::prelude::*;

use crate::{game_logic::{armies::*, empire::*, game_states::*, province::*}, scene::hex_grid::*, ui::panels::*};

/* Systems */

pub fn toggle_province_view(
    keyboard: Res<ButtonInput<KeyCode>>,
    view: Res<State<GridViewMode>>,
    mut next_view: ResMut<NextState<GridViewMode>>
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        match view.get() {
            GridViewMode::Terrain => next_view.set(GridViewMode::Empire),
            GridViewMode::Empire => next_view.set(GridViewMode::Terrain),
        }
    }
}

pub fn toggle_movement_view(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut movement: ResMut<ArmyMovement>,
    state: Res<State<ArmyMovementView>>,
    mut next_state: ResMut<NextState<ArmyMovementView>>,
    mut pick: ResMut<PickedProvince>,
    mut army_panel: Single<&mut ArmiesPanel>,
    q_provinces: Query<(&Province, &ProvinceArmies)>,
    q_provinces_1: Query<(&Province, Option<&ControlledBy>)>,
    q_empires: Query<&Empire>,
    q_armies: Query<&Army>,
    grid: Res<HexGrid>,
    empires: Res<Empires>
) {
    let PickedProvince::Selected(province_e) = *pick else {
        return;
    };
    let Ok((province_c, prov_armies_c)) = q_provinces.get(province_e) else {
        return;
    };
    if prov_armies_c.count() == 0 {
        return;
    }
    let player_armies = prov_armies_c
        .iter()
        .filter(|army_e| {
            let Ok(army_c) = q_armies.get(*army_e) else {
                error!("{}:{} Missing army component", file!(), line!());
                return false;
            };
            let Ok(empire_c) = q_empires.get(army_c.empire()) else {
                error!("{}:{} Missing empire component", file!(), line!());
                return false;
            };

            return empire_c.id == PLAYER_EMPIRE;
        })
        .collect::<Vec<Entity>>();
    let army_e = player_armies[army_panel.curr_army as usize];
    let Ok(army_c) = q_armies.get(army_e) else {
        error!("{}:{} Missing army component", file!(), file!());
        return;
    };
    /* Cannot move army twice in turn */
    if army_c.moved() {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyM) {
        match state.get() {
            ArmyMovementView::On => {
                next_state.set(ArmyMovementView::Off);
                army_panel.curr_army = 0;
            },
            ArmyMovementView::Off => {
                army_panel.curr_army = 0;
                next_state.set(ArmyMovementView::On);
                movement.set_reachable(get_reachable_tiles(&army_c, &province_c, &q_provinces_1, &q_empires, &grid, &empires));
                movement.set_old_selected(&province_e);
                movement.set_army(army_e.clone());
                *pick = PickedProvince::None;
            }
        }
    }
}