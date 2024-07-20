#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::borrow::ToOwned;

use bevy::asset::AssetMetaCheck;
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle::default());
    commands.spawn(PointLightBundle::default());
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::default()),
            material: materials.add(Color::from(css::GRAY)),
            transform: Transform::from_xyz(0.0, 0.0, -5.0),
            ..default()
        },
        PickableBundle::default(),
        HIGHLIGHT_TINT,
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::default()),
            material: materials.add(Color::from(css::BLUE)),
            transform: Transform::from_xyz(1.0, 0.0, -5.0),
            ..default()
        },
        PickableBundle::default(),
        HIGHLIGHT_TINT,
    ));
}

const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl
            .base_color
            .mix(&Color::srgba(-0.5, -0.3, 0.9, 0.8), 0.5), // hovered is blue
        ..matl.to_owned()
    })),
    selected: None,
    pressed: None,
};
