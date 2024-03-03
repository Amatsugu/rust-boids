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
const PROTECTED_RANGE: f32 = 40.;
const AVOID_RANGE: f32 = 20.;
const ALIGN_RANGE: f32 = 300.;
const CO_RANGE: f32 = 200.;

const ALIGN_FACTOR: f32 = 0.125;
const CO_FACTOR: f32 = 0.05;

const MAX_RANGE: f32 = 500.;
const MAX_SPEED: f32 = 200.;

const GRID_SIZE: i32 = 30;

fn simulate_boids(time: Res<Time>, mut boids: Query<(&Transform, &mut Vel), With<Boid>>) {
	let mut combos = boids.iter_combinations_mut();
	while let Some([mut a, mut b]) = combos.fetch_next() {
		let mut a_total_vel = a.1.value;
		let mut b_total_vel = b.1.value;
		//Align
		let dist = a.0.translation - b.0.translation;
		let d = dist.length();
		if d <= ALIGN_RANGE && d > PROTECTED_RANGE {
			let avg_vel = (a.1.value + b.1.value) / 2.;
			a_total_vel -= (avg_vel - a_total_vel) * ALIGN_FACTOR * time.delta_seconds();
			b_total_vel -= (avg_vel - b_total_vel) * ALIGN_FACTOR * time.delta_seconds();
		}

		//Collision Avoidance
		if d < AVOID_RANGE {
			a_total_vel += dist * time.delta_seconds();
			b_total_vel += -dist * time.delta_seconds();
		}
		if d < CO_RANGE && d > PROTECTED_RANGE {
			//Choesian
			let avg_pos = ((b.0.translation - a.0.translation) / 2.) + a.0.translation;
			a_total_vel +=
				tend_to_point(a.0.translation, avg_pos) * CO_FACTOR * time.delta_seconds();
			b_total_vel +=
				tend_to_point(b.0.translation, avg_pos) * CO_FACTOR * time.delta_seconds();
		}

		a.1.value = a_total_vel;
		b.1.value = b_total_vel;
	}

	for (t, mut v) in &mut boids {
		//Tend to Center
		let mut d = t.translation.length();
		if d > MAX_RANGE {
			d -= MAX_RANGE;
			d /= MAX_RANGE;
			v.value += tend_to_point(t.translation, Vec3::ZERO)
				* time.elapsed_seconds()
				* d.remap(0., 1., 0., 0.5);
		}
		//Limit Velocity
		if v.value == Vec3::ZERO {
			continue;
		}
		v.value = limit_velocity(&v);
	}
}

fn tend_to_point(from: Vec3, to: Vec3) -> Vec3 {
	let diff = to - from;
	return diff.normalize();
}

fn limit_velocity(v: &Vel) -> Vec3 {
	if v.value.length() > MAX_SPEED {
		return v.value.normalize() * MAX_SPEED;
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

	for x in 0..GRID_SIZE {
		for y in 0..GRID_SIZE {
			let shape = Mesh2dHandle(meshes.add(Triangle2d::new(
				Vec2::Y * 5.0,
				Vec2::new(-5.0, -5.0),
				Vec2::new(5.0, -5.0),
			)));
			let color = Color::hsl(360.0 * (x as f32 / GRID_SIZE as f32), 1., 0.7);
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
