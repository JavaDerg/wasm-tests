mod addons;

use wasmer::{imports, Function, Instance, Module, Store, Value, Array, import_namespace, UniversalEngine};
use wasmer_wasi::{WasiEnv, WasiState, WasiVersion};
use bevy::prelude::*;
use crate::addons::loader::ModLoaderPlugin;
use heron::PhysicsPlugin;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(ModLoaderPlugin)
        .add_startup_system(setup.system())
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shapes::RegularPolygon {
            sides: 6,
            center: Default::default(),
            feature: shapes::RegularPolygonFeature::Radius(120.0),
        },
        ShapeColors::outlined(Color::ORANGE_RED, Color::WHITE),
        DrawMode::Stroke {
            0: StrokeOptions::default().with_line_width(5.0),
        },
        Transform::default(),
    ));
}
