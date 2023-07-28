use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

mod pan_orbit_cam;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
            ResourceInspectorPlugin::<Time>::default(),
            pan_orbit_cam::CamPlugin,
        ))
        .register_type::<Tower>()
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, (tower_shooting, despawn_after_life_time))
        .run();
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.254, 0.92, 0.90).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1., TimerMode::Repeating),
        })
        .insert(Name::new("Cube"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 5.,
                subdivisions: 5,
            })),
            material: materials.add(Color::rgb(0.254, 0.92, 0.90).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -1., 0.0)),
            ..Default::default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .insert(Name::new("Light"));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform = Transform::from_xyz(0., 0.7, 0.6);

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                    material: materials.add(StandardMaterial {
                        emissive: Color::rgb_linear(13.99, 5.32, 2.0),
                        ..default()
                    }),
                    transform: spawn_transform,
                    ..Default::default()
                })
                .insert(LifeTime {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                })
                .insert(Name::new("Bullet"));
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct LifeTime {
    timer: Timer,
}

fn despawn_after_life_time(
    mut commands: Commands,
    mut life_times: Query<(Entity, &mut LifeTime)>,
    time: Res<Time>,
) {
    for (entity, mut life_time) in &mut life_times {
        life_time.timer.tick(time.delta());
        if life_time.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
