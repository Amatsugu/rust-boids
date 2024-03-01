use bevy::prelude::*;
mod boids;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					title: "Boids".into(),
					name: Some("rest.boids".into()),
					resolution: (1920.0, 1080.0).into(),
					resizable: false,
					enabled_buttons: bevy::window::EnabledButtons{
						maximize: false,
						..Default::default()
					},
					..default()
				}),
				..default()
			}),
			boids::Boids,
		))
		.run();
}
