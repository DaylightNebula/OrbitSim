use bevy::prelude::*;

#[derive(Component, Debug, PartialEq)]
pub struct Orbit {
    pub mass: f32,
    pub acceleration: Vec3,
    pub velocity: Vec3
}

pub struct OrbitPlugin;
impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_acceleration)
            .add_systems(Update, handle_acceleration.after(update_acceleration))
            .add_systems(Update, handle_velocity.after(handle_acceleration));
    }
}

fn update_acceleration(
    mut orbits: Query<(&mut Orbit, &Transform)>
) {
    // get list of orbits
    let mut orbits = orbits.iter_mut().collect::<Vec<(Mut<Orbit>, &Transform)>>();
    let g_const = 6.67 * (10.0_f32.powf(-11.0));

    // calculate orbits
    let mut i = 0;
    while i < orbits.len() {
        // get current orbit and create a vec for combined acceleration
        let (orbit, orbit_transform) = orbits.get(i).unwrap();
        let mut net_force = Vec3::ZERO;

        // loop through other orbits
        let mut j = 0;
        while j < orbits.len() {
            if i == j { j += 1; continue; }

            // get other orbit
            let (other, other_transform) = orbits.get(j).unwrap();
            let distance_squared: f32 = orbit_transform.translation.distance_squared(other_transform.translation);
            let force = (g_const * orbit.mass * other.mass) / distance_squared;

            // todo turn force into vector and add to net force

            j += 1;
        }

        // todo turn net force into acceleration and update acceleration

        i += 1;
    }
}

fn handle_acceleration(

) {

}

fn handle_velocity(

) {

}