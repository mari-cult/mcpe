use bevy::{
	app::{App, Startup},
	asset::AssetServer,
	image::{ImageLoaderSettings, ImageSampler},
	prelude::{
		BuildChildren as _, Camera3d, ChildBuild as _, Commands, ImageNode,
		PickingBehavior, Res,
	},
	sprite::{SliceScaleMode, TextureSlicer},
	ui::{
		widget::NodeImageMode, Display, GridPlacement, GridTrack,
		IsDefaultUiCamera, Node, UiBoxShadowSamples, UiRect, Val,
	},
	DefaultPlugins,
};

fn main() {
	App::new()
		.add_plugins((DefaultPlugins,))
		.add_systems(Startup, setup)
		// .add_systems(Update, keyboard_input)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((
		Camera3d::default(),
		IsDefaultUiCamera,
		UiBoxShadowSamples(6),
	));

	let dirt = asset_server.load_with_settings(
		"gui/bg32.png",
		|settings: &mut ImageLoaderSettings| {
			settings.sampler = ImageSampler::nearest();
		},
	);
	let logo = asset_server.load_with_settings(
		"gui/title.png",
		|settings: &mut ImageLoaderSettings| {
			settings.sampler = ImageSampler::nearest();
		},
	);

	let slicer = TextureSlicer {
		center_scale_mode: SliceScaleMode::Tile { stretch_value: 4.0 },
		..Default::default()
	};

	commands
		.spawn((
			Node {
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				display: Display::Grid,
				grid_template_columns: vec![
					GridTrack::fr(1.0),
					GridTrack::fr(1.0),
					GridTrack::fr(1.0),
				],
				grid_template_rows: vec![
					GridTrack::fr(1.0),
					GridTrack::fr(1.0),
					GridTrack::fr(1.0),
				],
				padding: UiRect::all(Val::Px(6.0)),
				..Default::default()
			},
			ImageNode {
				image: dirt.clone(),
				image_mode: NodeImageMode::Sliced(slicer.clone()),
				..Default::default()
			},
		))
		.insert(PickingBehavior::IGNORE)
		.with_children(|parent| {
			parent.spawn((
				Node {
					display: Display::Grid,
					grid_column: GridPlacement::start(2),
					max_height: Val::Px(128.0),
					..Default::default()
				},
				ImageNode {
					image: logo.clone(),
					..Default::default()
				},
			));
		});
}
