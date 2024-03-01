use bevy::prelude::*;
pub struct Boids;

impl Plugin for Boids {
	fn build(&self, app: &mut App) {
		app.insert_resource(PrintTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
			.add_systems(Startup, add_boids)
			.add_systems(Update, (update_boid, print_boid).chain());
	}
}

fn print_boid(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<&Name, With<Boid>>) {
	if !timer.0.tick(time.delta()).just_finished() {
		return;
	}
	for name in &query {
		println!("{}", name.0);
	}
}

fn update_boid(mut query: Query<&mut Name, With<Boid>>) {
	for mut name in &mut query {
		if name.0 == "B1" {
			name.0 = "B1.Updated".to_string();
			break;
		}
	}
}

fn add_boids(mut commands: Commands) {
	commands.spawn((Boid, Name("B1".to_string())));
	commands.spawn((Boid, Name("B2".to_string())));
	commands.spawn((Boid, Name("B3".to_string())));
}

#[derive(Resource)]
struct PrintTimer(Timer);

#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Name(String);
