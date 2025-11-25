
use bevy::prelude::*;

use crate::constants::*;
use crate::model::GameData;

#[derive(Component)]
struct ScoreboardUi;


pub struct Scoreboard;
impl Plugin for Scoreboard {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scoreboard)
           .add_systems(Update, update_scoreboard);
    }
}


fn setup_scoreboard(mut commands: Commands)
{
    // Scoreboard
    commands.spawn((
        Text::new("Score: "),
        TextFont {
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
        ScoreboardUi,
        Node {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        )],
    ));
}


fn update_scoreboard(
    game: Res<GameData>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = game.current_score.to_string();
}
