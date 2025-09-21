use bevy::{
	DefaultPlugins,
	app::{App, Startup},
	asset::AssetServer,
	camera::Camera2d,
	image::{ImageLoaderSettings, ImageSampler},
	math::Rect,
	picking::Pickable,
	prelude::{Camera3d, Commands, ImageNode, Res, Text},
	sprite::{SliceScaleMode, TextureSlicer},
	text::{Justify, TextFont, TextLayout},
	ui::{
		AlignItems, Display, FlexDirection, IsDefaultUiCamera, JustifyContent,
		Node, UiRect, Val, widget::NodeImageMode,
	},
};

fn main() {
	tracing_subscriber::fmt::init();
	App::new()
		.add_plugins((DefaultPlugins,))
		.add_systems(Startup, setup)
		// .add_systems(Update, keyboard_input)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((Camera3d::default(), IsDefaultUiCamera));

	let font = asset_server.load("font/Monocraft-SemiBold.otf");

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

	let touchgui = asset_server.load_with_settings(
		"gui/touchgui.png",
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
				display: Display::Flex,
				flex_direction: FlexDirection::Column,
				align_items: AlignItems::Center,
				padding: UiRect::all(Val::Px(12.0)),
				..Default::default()
			},
			ImageNode {
				image: dirt.clone(),
				image_mode: NodeImageMode::Sliced(slicer.clone()),
				..Default::default()
			},
		))
		.insert(Pickable::IGNORE)
		.with_children(|parent| {
			parent.spawn((
				Node {
					display: Display::Flex,
					width: Val::Percent(33.0),
					..Default::default()
				},
				ImageNode {
					image: logo.clone(),
					..Default::default()
				},
			));

			parent.spawn((
				Node {
					display: Display::Flex,
					..Default::default()
				},
				Text::new("v0.2.0 alpha"),
				TextLayout::new_with_justify(Justify::Center),
				TextFont {
					font: font.clone(),
					font_size: 28.0,
					..Default::default()
				},
			));

			parent
				.spawn((Node {
					display: Display::Flex,
					flex_direction: FlexDirection::Row,
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					padding: UiRect::top(Val::Px(50.0)),
					justify_content: JustifyContent::SpaceEvenly,

					..Default::default()
				},))
				.with_children(|parent| {
					parent
						.spawn((
							Node {
								display: Display::Flex,
								width: Val::Px(300.0),
								height: Val::Px(300.0),
								padding: UiRect::all(Val::Px(36.0)),
								justify_content: JustifyContent::Center,
								..Default::default()
							},
							ImageNode {
								image: touchgui.clone(),
								rect: Some(Rect::new(0.0, 101.0, 75.0, 175.0)),
								..Default::default()
							},
						))
						.with_children(|parent| {
							parent.spawn((
								Node {
									display: Display::Flex,
									justify_content: JustifyContent::Center,
									..Default::default()
								},
								Text::new("Start Game"),
								TextLayout::new_with_justify(Justify::Center),
								TextFont {
									font: font.clone(),
									font_size: 28.0,
									..Default::default()
								},
							));
						});
				});
		});
}
