use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use rand::distributions::Standard;
use rand::prelude::*;
use bevy::color::palettes::css::CRIMSON;

use crate::map::{Coordinate, Map};
use crate::ui::GameState;
use crate::{AppState, ENEMY_Z};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaveState {
            last_spawn: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .add_event::<EnemyBreach>()
        .add_systems(
            PostUpdate,
            remove_enemies.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                update_enemy_colors
                    .in_set(EnemySet::UpdateColor)
                    .after(EnemySet::Damage),
                spawn_enemies.before(EnemySet::UpdateColor),
                move_enemies.in_set(EnemySet::Move).before(EnemySet::Damage),
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::InGame), break_down_enemies);
    }
}

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub enum EnemySet {
    UpdateColor,
    Damage,
    Move,
}

#[derive(Event)]
pub struct EnemyBreach;

#[derive(Resource)]
struct WaveState {
    pub last_spawn: Timer,
}

#[derive(Component)]
pub struct Tameable;

#[derive(Clone, Component)]
pub struct Enemy {
    current_waypoint_index: usize,
    pub form: EnemyForm,
    pub color: EnemyColor,
    pub bullets: Vec<Entity>,
    pub colored_health: i32,
    pub travelled: f32,
    pub max_health: i32,
}

#[derive(Clone, Component)]
pub struct Health {
    pub value: i32,
}

impl Enemy {
    pub fn get_color(&self, health: i32) -> Color {
        Color::srgb(0.79, 0.277, 0.)
        // let health_factor = if health > 0 {
        //     health as f32 / self.max_health as f32
        // } else {
        //     0.
        // };
        // Color::GRAY * health_factor + self.color.to_color() * (1. - health_factor)
    }
}

impl Distribution<EnemyForm> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyForm {
        match rng.gen_range(0..3) {
            0 => EnemyForm::Circle,
            1 => EnemyForm::Triangle,
            _ => EnemyForm::Quadratic,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnemyColor {
    Red,
    Lilac,
    Green,
    Blue,
    Pink,
}

impl Distribution<EnemyColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyColor {
        match rng.gen_range(0..4) {
            0 => EnemyColor::Red,
            1 => EnemyColor::Green,
            2 => EnemyColor::Blue,
            3 => EnemyColor::Pink,
            _ => EnemyColor::Lilac,
        }
    }
}

impl EnemyColor {
    pub fn to_color(&self) -> Color {
        match self {
            EnemyColor::Lilac => Color::srgb(84. / 255., 13. / 255., 110. / 255.),
            EnemyColor::Red => Color::srgb(235. / 255., 66. / 255., 102. / 255.),
            EnemyColor::Green => Color::srgb(83. / 255., 145. / 255., 126. / 255.),
            EnemyColor::Pink => Color::srgb(217. / 255., 154. / 255., 197. / 255.),
            EnemyColor::Blue => Color::srgb(88. / 255., 84. / 255., 129. / 255.),
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    map: Res<Map>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut wave_state: ResMut<WaveState>,
) {

    if wave_state.last_spawn.tick(time.delta()).just_finished() {
        if game_state.health < 1 {
            return;
        }
        game_state.enemy_health += 1;

        let form: EnemyForm = random();
        let color: EnemyColor = random();

        let mut health = game_state.enemy_health;
        let one_percent = health / 100;
        let mut rng = rand::thread_rng();
        let percent: i32 = rng.gen_range(0..50);
        health += percent * one_percent;

        create_enemy(&mut commands, color, &map, health, form);
    }
}

fn create_enemy(
    commands: &mut Commands,
    color: EnemyColor,
    map: &Res<Map>,
    health: i32,
    form: EnemyForm,
) {
    let enemy = Enemy {
        current_waypoint_index: 0,
        form: form.clone(),
        max_health: health,
        bullets: vec![],
        colored_health: health,
        color,
        travelled: 0.,
    };
    commands
        .spawn(form.build_bundle(
            Transform::from_translation(Vec3::new(map.spawn.x, map.spawn.y, ENEMY_Z)),
            enemy.get_color(health),
            Some(enemy.get_color(health)),
        ))
        .insert(enemy)
        .insert(Health { value: health });
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnemyForm {
    Circle,
    Triangle,
    Quadratic,
}

impl EnemyForm {
    pub fn build_bundle(
        &self,
        transform: Transform,
        outline_color: Color,
        fill_color: Option<Color>,
    ) -> impl Bundle {
        let shape = shapes::RegularPolygon {
            sides: match self {
                EnemyForm::Circle => 5,
                EnemyForm::Triangle => 3,
                EnemyForm::Quadratic => 4,
            },
            feature: shapes::RegularPolygonFeature::Radius(12.0),
            ..shapes::RegularPolygon::default()
        };

        (
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform,
                ..default()
            },
            Fill::color(fill_color.unwrap_or(Color::NONE)),
            Stroke::new(outline_color, 2.0),
        )
    }
}

fn remove_enemies(
    mut commands: Commands,
    map: Res<Map>,
    mut game_state: ResMut<GameState>,
    mut enemy_breach: EventWriter<EnemyBreach>,
    mut enemy_query: Query<(Entity, &mut Enemy, &Health), Without<Tameable>>,
) {
    for (entity, mut enemy, health) in enemy_query.iter_mut() {
        if health.value < 0 {
            if game_state.health > 0 {
                game_state.score += enemy.max_health as usize;
            }
            commands.entity(entity).insert(Tameable);
            for id in enemy.bullets.drain(..) {
                commands.entity(id).despawn();
            }
            continue;
        }
        if enemy.current_waypoint_index >= map.waypoints.len() {
            if game_state.health > 0 {
                game_state.health -= 1;
                enemy_breach.send(EnemyBreach);
            }
            for id in enemy.bullets.drain(..) {
                commands.entity(id).despawn();
            }
            commands.entity(entity).despawn();
            continue;
        }
    }
}

fn move_enemies(
    time: Res<Time>,
    map: Res<Map>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform), Without<Tameable>>,
) {
    let delta = time.delta().as_millis() as f32;
    let speed = 0.1;

    for (mut enemy, mut transform) in enemy_query.iter_mut() {
        if enemy.current_waypoint_index >= map.waypoints.len() {
            continue;
        }
        let destination = map.waypoints.get(enemy.current_waypoint_index).unwrap();
        let mut distance = Vec3::new(destination.x, destination.y, ENEMY_Z) - transform.translation;
        distance.z = 0.;
        if distance == Vec3::ZERO {
            enemy.current_waypoint_index += 1;
            continue;
        }
        let movement = distance.normalize() * delta * speed;
        if movement.length() > distance.length() {
            transform.translation = Vec3::new(destination.x, destination.y, ENEMY_Z);
            enemy.travelled += distance.length();
            enemy.current_waypoint_index += 1;
        } else {
            enemy.travelled += movement.length();
            transform.translation += movement;
        }
    }
}

fn update_enemy_colors(
    mut damaged_enemies: Query<(&mut Fill, &mut Stroke, &Health, &Enemy), Changed<Health>>,
) {
    for (mut fill, mut stroke, health, enemy) in damaged_enemies.iter_mut() {
        if health.value == enemy.colored_health {
            continue;
        }
        fill.color = enemy.get_color(health.value);
        stroke.color = enemy.get_color(health.value);
    }
}

fn break_down_enemies(mut commands: Commands, enemies_query: Query<Entity, With<Enemy>>) {
    for entity in enemies_query.iter() {
        commands.entity(entity).despawn();
    }
}