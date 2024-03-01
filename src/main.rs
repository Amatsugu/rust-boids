use bevy::prelude::*;
mod boids;

fn main() {
	App::new().add_plugins((DefaultPlugins, boids::Boids)).run();
}
