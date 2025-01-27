use std::time::Instant;

use crate::AppState;
use bevy::{color::palettes::css::CRIMSON, prelude::*};
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default())
            .init_resource::<ButtonColors>()
            .add_systems(OnEnter(AppState::InGame), init_stats)
            .add_systems(
                Update,
                (update_game_state, retry_system, click_retry_button)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Resource)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct RetryButton;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct CoinsText;

#[derive(Resource)]
pub struct GameState {
    pub health: usize,
    pub score: usize,
    pub coins: usize,
    pub enemy_health: i32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            health: 1488,
            score: 1337,
            coins: 228,
            enemy_health: 1,
        }
    }
}

fn init_stats(mut commands: Commands, game_state: Res<GameState>) {
    // Health Node
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(10.),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Text::new(format!("Health: {}", game_state.health)))
                .insert(HealthText);
        });
    // Score Node
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(30.),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Text::new(format!("Score: {}", game_state.score)))
                .insert(ScoreText);
        });
    // Coins Node
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(50.),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Text::new(format!("Coins: {}", game_state.coins)))
                .insert(CoinsText);
        });
}


fn update_game_state(
    game_state: Res<GameState>,
    mut queries: ParamSet<(
        Query<&mut Text, With<HealthText>>,
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<CoinsText>>,
    )>,
) {
    if true {
    // if game_state.is_changed() {
        for mut text in queries.p0().iter_mut() {
            *text = Text::new(format!(
                "Health: {} - {:?}",
                game_state.health,
                Instant::now()
            ));
        }
        for mut text in queries.p1().iter_mut() {
            *text = Text::new(format!(
                "Score: {} - {:?}",
                game_state.score,
                Instant::now()
            ));
        }
        for mut text in queries.p2().iter_mut() {
            *text = Text::new(format!(
                "Coins: {} - {:?}",
                game_state.coins,
                Instant::now()
            ));
        }
    }
}

fn retry_system(
    mut commands: Commands,
    game_state: Res<GameState>,
) {
    // if true {
    if game_state.is_changed() && game_state.health < 1 {
        commands
            .spawn((
                Button,
                Node {
                width: Val::Px(300.0),
                height: Val::Px(75.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            }))
            .insert(RetryButton)
            .with_children(|parent| {
                parent.spawn((
                    Text::new("restart"),
                    TextColor(Color::srgb(0.647, 0.165, 0.165)),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    Node {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },
                ));
            });
    }
}

fn click_retry_button(
    mut commands: Commands,
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameState>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &Children),
        With<Button>,
    >,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, mut color, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *game_state = GameState::default();
                commands.entity(button).despawn();
                commands.entity(text).despawn();
                state.set(AppState::Restart);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}
