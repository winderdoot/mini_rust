use bevy::prelude::*;

/// This and RecentlyMoved components are there to allow any system
/// to check if a given entity has moved in the last frame. This avoids concurrency issues,
/// when the two systems cannot be ordered in relation to another
#[derive(Component, Deref, DerefMut, Default)]
pub struct CurrentlyMoving(pub bool);

#[derive(Component, Deref, DerefMut, Default)]
pub struct RecentlyMoved(pub bool);

/// Update the status of all entities that have decalred themselves as currently moving in their own systems,
/// as having recently moved, so that other systems can make use of that information next frame.
pub fn recently_moved_update(
    mut query: Query<(&mut CurrentlyMoving, &mut RecentlyMoved)>
) {
    query
        .iter_mut()
        .for_each(|(mut curr, mut recently)| {
            recently.0 = curr.0;
            curr.0 = false;
        })
}

/* Init Plugin */
pub struct RecentlyMovedPlugin;

impl Plugin for RecentlyMovedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, recently_moved_update);
    }
}
