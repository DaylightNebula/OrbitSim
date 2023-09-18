use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Orbit {
    pub mass: f32,
    pub acceleration: Vec3,
    pub velocity: Vec3
}

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct OrbitInfo {
    pub acceleration: Vec3
}

pub struct OrbitPlugin;
impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_acceleration)
            .add_systems(Update, handle_infos.after(update_acceleration))
            .add_systems(Update, update_transforms.after(handle_infos));
    }
}

fn update_acceleration(
    mut commands: Commands,
    mut orbits: Query<(Entity, &mut Orbit, &Transform)>
) {
    // get list of orbits
    let orbits = orbits.iter_mut().collect::<Vec<(Entity, Mut<Orbit>, &Transform)>>();
    let g_const = 6.67 * (10.0_f32.powf(-11.0));

    // calculate orbits
    let mut i = 0;
    while i < orbits.len() {
        // get current orbit and create a vec for combined acceleration
        let (entity, orbit, orbit_transform) = &orbits[i];
        let mut net_force = Vec3::ZERO;

        // loop through other orbits
        let mut j = 0;
        while j < orbits.len() {
            if i == j { j += 1; continue; }

            // get other orbit
            let (_, other, other_transform) = &orbits[j];
            let distance_squared: f32 = orbit_transform.translation.distance_squared(other_transform.translation);
            let force = (g_const * orbit.mass * other.mass) / distance_squared;

            // turn force into vector and add to net force
            net_force += (other_transform.translation - orbit_transform.translation).normalize() * force;

            j += 1;
        }

        // turn net force into acceleration and update acceleration
        commands.entity(*entity).insert(OrbitInfo {
            acceleration: net_force / orbit.mass
        });

        i += 1;
    }
}

fn handle_infos(
    mut orbits: Query<(&mut Transform, &mut Orbit, &OrbitInfo)>,
    time: Res<Time>
) {
    orbits.for_each_mut(|(mut transform, mut orbit, info)| {
        orbit.acceleration = info.acceleration;
        orbit.velocity += info.acceleration * time.delta_seconds();
        transform.translation += orbit.velocity * time.delta_seconds();
    });
}

fn update_transforms(
    // mut orbits: Query<(&mut Transform, &Orbit)>,
    // time: Res<Time>
) {

}