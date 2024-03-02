use bevy::{
	prelude::*,
	sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
pub struct Boids;

impl Plugin for Boids {
	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(Color::rgb(0.09, 0., 0.0390625)))
			.add_systems(Startup, init)
			.add_systems(Update, (simulate_boids, update_boid_vel).chain());
	}
}

fn simulate_boids(time: Res<Time>, mut boids: Query<(&Transform, &mut Vel), With<Boid>>) {
	let mut combos = boids.iter_combinations_mut();
	while let Some([mut a, mut b]) = combos.fetch_next() {
		let mut a_total_vel = a.1.value;
		let mut b_total_vel = b.1.value;
		//Match Speed
		let dist = a.0.translation - b.0.translation;
		let d = dist.length();
		if d <= 100. {
			let avg_vel = (a.1.value + b.1.value) / 2.;
			a_total_vel += ((avg_vel - a_total_vel) / 8.) * time.delta_seconds();
			b_total_vel -= ((avg_vel - b_total_vel) / 8.) * time.delta_seconds();
		}

		//Collision Avoidance
		if d < 15. {
			a_total_vel += dist * time.elapsed_seconds();
			b_total_vel += -dist * time.elapsed_seconds();
		} else {
			//Flocking
			// let avg_pos = (a.0.translation + b.0.translation) / 2.;
			// a_total_vel += (avg_pos - a.0.translation) * 0.0001 * time.elapsed_seconds();
			// b_total_vel += (avg_pos - b.0.translation) * 0.0001 * time.elapsed_seconds();
		}

		// println!("{}", a_total_vel.length());
		a.1.value = a_total_vel;
		b.1.value = b_total_vel;
	}

	for (t, mut v) in &mut boids {
		//Tend to Center
		let mut d = t.translation.length();
		if d > 500. {
			d -= 500.;
			d /= 1000.;
			v.value += -t.translation * time.elapsed_seconds() * remap(d, 0., 1., 0., 0.03);
		}

		//Limit Velocity
		if v.value == Vec3::ZERO {
			continue;
		}
		v.value = limit_velocity(&v);
	}
}

fn remap(value: f32, low1: f32, high1: f32, low2: f32, high2: f32) -> f32 {
	return low2 + (value - low1) * (high2 - low2) / (high1 - low1);
}

fn limit_velocity(v: &Vel) -> Vec3 {
	let max_speed = 200.;
	if v.value.length() > max_speed {
		return v.value.normalize() * max_speed;
	}
	return v.value;
}

fn update_boid_vel(time: Res<Time>, mut query: Query<(&mut Transform, &Vel), With<Boid>>) {
	for (mut t, v) in &mut query {
		t.translation += v.value * time.delta_seconds();
		if v.value == Vec3::ZERO {
			continue;
		}
		t.rotation = Quat::from_rotation_arc(Vec3::Y, v.value.normalize());
	}
}

fn init(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn(Camera2dBundle::default());

	let size = 50;

	for x in 0..size {
		for y in 0..size {
			let shape = Mesh2dHandle(meshes.add(Triangle2d::new(
				Vec2::Y * 5.0,
				Vec2::new(-5.0, -5.0),
				Vec2::new(5.0, -5.0),
			)));
			let color = Color::hsl(
				360.0 * (x as f32 / size as f32),
				y as f32 / size as f32,
				0.7,
			);
			commands.spawn((
				Boid,
				Vel { value: Vec3::ZERO },
				Name(format!("B{x}").into()),
				MaterialMesh2dBundle {
					mesh: shape,
					material: materials.add(color),
					transform: Transform::from_xyz(x as f32 * 8.0, y as f32 * 8.0, 0.0),
					..default()
				},
			));
		}
	}
}

#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Vel {
	pub value: Vec3,
}
