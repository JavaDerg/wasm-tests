mod addons;

use wasmer::{imports, Function, Instance, Module, Store, Value, Array, import_namespace, UniversalEngine};
use wasmer_wasi::{WasiEnv, WasiState, WasiVersion};
use bevy::prelude::*;
use crate::addons::loader::ModLoaderPlugin;
use heron::{Acceleration, AxisAngle, CollisionShape, PhysicMaterial, PhysicsPlugin, RigidBody, RotationConstraints, Velocity};
use bevy_prototype_lyon::prelude::*;
use fastrand as rng;
use bevy::math::Vec3A;

struct Ship(f32);

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(ModLoaderPlugin)
        .add_startup_system(setup.system())
        .add_system(wrap_system.system())
        .add_system(handle_kb.system())
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    for _ in 0..1 {
        commands.spawn()
            .insert(Ship(100.0))
            .insert_bundle(GeometryBuilder::build_as(
            &shapes::Polygon {
                points: vec![
                    Vec2::new(0.0, 10.0),
                    Vec2::new(-10.0, -20.0),
                    Vec2::new(10.0, -20.0),
                ],
                closed: true,
            },
            ShapeColors::outlined(Color::WHITE, Color::NONE),
            DrawMode::Stroke(
                StrokeOptions::default().with_line_width(1.0),
            ),
            Transform::from_scale(Vec3::ONE),
        )).insert_bundle((
            RigidBody::Dynamic,
            CollisionShape::ConvexHull {
                points: vec![
                    Vec3::new(0.0, 10.0, 0.0),
                    Vec3::new(-10.0, -20.0, 0.0),
                    Vec3::new(10.0, -20.0, 0.0),
                ],
                border_radius: Some(1.0),
            },
            // Velocity::from(Vec2::new(100.0 - rng::f32() * 200.0, 100.0 - rng::f32() * 200.0)),
            Velocity::default(),
            Acceleration::default(),
            PhysicMaterial {
                restitution: 0.2,
                density: 10.0,
                friction: 0.1,
            },
            RotationConstraints::restrict_to_z_only(),
        ));
    }
}

fn wrap_system(mut query: Query<&mut Transform>, win_dim: Res<Windows>) {
    let prim = win_dim.get_primary().unwrap();

    for mut tf in query.iter_mut() {
        while tf.translation.x <= -prim.width() / 2.0 { tf.translation.x += prim.width();}
        while tf.translation.x >= prim.width() / 2.0 { tf.translation.x -= prim.width();}

        while tf.translation.y <= -prim.height() / 2.0 { tf.translation.y += prim.height();}
        while tf.translation.y >= prim.height() / 2.0 { tf.translation.y -= prim.height();}
    }
}

fn handle_kb(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &Transform), With<Ship>>) {

    if keys.pressed(KeyCode::W) | keys.pressed(KeyCode::Up) {
        query.iter_mut().for_each(|(mut vel, tf)| vel.linear += Vec3::from(tf.rotation.mul_vec3a(Vec3A::Y)));
    }
    if keys.pressed(KeyCode::S) | keys.pressed(KeyCode::Down) {
        query.iter_mut().for_each(|(mut vel, tf)| vel.linear -= Vec3::from(tf.rotation.mul_vec3a(Vec3A::Y)));
    }
    if keys.pressed(KeyCode::A) | keys.pressed(KeyCode::Left) {
        query.iter_mut().for_each(|(mut vel, _)| apply_rotation(&mut vel, 0.01));
    }
    if keys.pressed(KeyCode::D) | keys.pressed(KeyCode::Right) {
        query.iter_mut().for_each(|(mut vel, _)| apply_rotation(&mut vel, -0.01));
    }
}

fn apply_rotation(vel: &mut Velocity, angle: f32) {
    let onorm = vel.angular.axis().z.is_sign_positive().then(|| 1.0).unwrap_or(-1.0);
    let new_angle = onorm * vel.angular.angle() + angle;
    let nnorm = new_angle.is_sign_positive().then(|| 1.0).unwrap_or(-1.0);
    vel.angular = AxisAngle::new(Vec3::Z * nnorm, new_angle.abs());
}
