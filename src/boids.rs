use bevy::{
	prelude::*,
	sprite::{MaterialMesh2dBundle, Mesh2dHandle},
	window::PrimaryWindow,
};
pub struct Boids;

impl Plugin for Boids {
	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(Color::rgb(0.09, 0., 0.0390625)))
			.add_systems(Startup, init)
			.add_systems(Update, (update_boid_vel, wrap_boids, simulate_boids));
	}
}

fn simulate_boids(time: Res<Time>, mut boids: Query<(&Transform, &mut Vel), With<Boid>>) {
	let mut combos = boids.iter_combinations_mut();
	while let Some([mut a, mut b]) = combos.fetch_next() {
		let mut a_total_vel = a.1.value;
		let mut b_total_vel = b.1.value;
		//Match Speed
		let avg_vel = (a.1.value + b.1.value) / 2.;
		a_total_vel += ((avg_vel - a_total_vel) / 8.) * time.delta_seconds();
		b_total_vel -= ((avg_vel - b_total_vel) / 8.) * time.delta_seconds();

		//Collision Avoidance
		let dist = a.0.translation - b.0.translation;
		if dist.length_squared() < 50. * 50. {
			a_total_vel += dist * time.elapsed_seconds();
			b_total_vel += -dist * time.elapsed_seconds();
		} else {
			//Flocking
			let avg = (a.0.translation + b.0.translation) / 2.;
			a_total_vel -= avg * 0.05 * time.elapsed_seconds();
			b_total_vel += avg * 0.05 * time.elapsed_seconds();
		}

		// println!("{}", a_total_vel.length());
		a.1.value = a_total_vel;
		b.1.value = b_total_vel;
	}

	for (t, mut v) in &mut boids {
		//Tend to Center
		if t.translation.length_squared() > 500. * 500. {
			v.value += -t.translation * time.elapsed_seconds() * 0.3;
		}

		//Limit Velocity
		if v.value == Vec3::ZERO {
			continue;
		}
		v.value = limit_velocity(&v);
	}
}

fn limit_velocity(v: &Vel) -> Vec3 {
	let max_speed = 200.;
	if v.value.length_squared() > max_speed * max_speed {
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

fn wrap_boids(
	cam_query: Query<(&Camera, &GlobalTransform)>,
	window: Query<&Window, With<PrimaryWindow>>,
	mut boids: Query<(&mut Transform, &mut Vel), With<Boid>>,
) {
	let (cam, cam_pos) = cam_query.single();
	let w = window.single();
	let (min, max) = (
		cam.viewport_to_world_2d(cam_pos, (0., w.height()).into())
			.unwrap(),
		cam.viewport_to_world_2d(cam_pos, (w.width(), 0.).into())
			.unwrap(),
	);

	for (mut t, mut v) in &mut boids {
		if t.translation.x < min.x || t.translation.x > max.x {
			v.value.x = -v.value.x;
		}
		if t.translation.y < min.y || t.translation.y > max.y {
			v.value.y = -v.value.y;
		}

		t.translation = t.translation.min(max.extend(0.)).max(min.extend(0.));
	}
}

fn init(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn(Camera2dBundle::default());

	let size = 20;

	for x in 0..size {
		for y in 0..size {
			let shape = Mesh2dHandle(meshes.add(Triangle2d::new(
				Vec2::Y * 5.0,
				Vec2::new(-5.0, -5.0),
				Vec2::new(5.0, -5.0),
			)));
			let color = Color::hsl(360.0 * x as f32 / 10.0, 0.75, 0.7);
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
