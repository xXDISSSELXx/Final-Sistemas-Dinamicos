use bevy::prelude::*;
use bevy_rand::prelude::*; // Para el generador de números aleatorios
use rand::prelude::Rng;
use crate::gamestate::AppState; // Importamos el estado

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugin para poder usar 'GlobalEntropy' (generador aleatorio)
            .add_plugins(EntropyPlugin::<WyRand>::default())
            
            // Inicializa los 'Recursos' (datos globales)
            .init_resource::<Score>()
            .init_resource::<AudioCommand>()
            .insert_resource(AudioCommandTimer(Timer::from_seconds(3.0, TimerMode::Repeating))) // Timer de 3 seg
            
            // Sistema para crear la UI de la puntuación
            .add_systems(OnEnter(AppState::JuegoActivo), setup_score_ui)

            // Sistemas que corren cada frame en JuegoActivo
            .add_systems(Update, 
                (
                    audio_command_system, // Da el comando de audio
                    scoring_system,     // Revisa si el jugador acertó
                    update_score_ui,    // Actualiza el texto de la puntuación
                )
                .run_if(in_state(AppState::JuegoActivo))
            );
    }
}

// --- Recursos (Datos Globales) ---
#[derive(Resource, Default)]
struct Score { value: i32 }

#[derive(Resource, Default, PartialEq, Clone, Copy, Debug)]
pub enum AudioCommand {
    #[default]
    None, // Ningún comando activo
    Left,
    Right,
    Jump,
}

// Un 'Resource' que envuelve un 'Timer'
#[derive(Resource)]
struct AudioCommandTimer(Timer);

// --- Componentes (para la UI) ---
#[derive(Component)]
struct ScoreText;

// --- Sistemas ---

// Crea el texto de la puntuación en la esquina
fn setup_score_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Puntuación: ",
                TextStyle { font_size: 30.0, color: Color::WHITE, ..default() }
            ),
            TextSection::new(
                "0", // Valor inicial
                TextStyle { font_size: 30.0, color: Color::GOLD, ..default() }
            ),
        ]).with_style(Style {
            position_type: PositionType::Absolute, // Fijo en la pantalla
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText, // Componente marcador
    ));
}

// Actualiza el texto de la puntuación
fn update_score_ui(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    // 'score.is_changed()' asegura que esto solo corra si la puntuación cambió
    if score.is_changed() {
        if let Ok(mut text) = query.get_single_mut() {
            text.sections[1].value = score.value.to_string();
        }
    }
}

// Sistema que genera los comandos de audio
fn audio_command_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<AudioCommandTimer>,
    mut current_command: ResMut<AudioCommand>,
    mut rng: ResMut<GlobalEntropy<WyRand>>, // Generador aleatorio
) {
    // Si el comando actual NO es 'None', significa que estamos esperando respuesta
    // No damos un nuevo comando hasta que el jugador responda.
    if *current_command != AudioCommand::None {
        return;
    }

    // Avanza el temporizador
    if timer.0.tick(time.delta()).just_finished() {
        // Elige un comando aleatorio (0, 1, o 2)
        let choice = rng.gen_range(0..3);
        let (next_command, sound_file) = match choice {
            0 => (AudioCommand::Left, "audio/left.ogg"),
            1 => (AudioCommand::Right, "audio/right.ogg"),
            _ => (AudioCommand::Jump, "audio/jump.ogg"),
        };
        
        // Guarda el comando actual en el Recurso
        *current_command = next_command;
        println!("¡Nuevo comando: {:?}", next_command); // Útil para depurar

        // Reproduce el sonido
        commands.spawn(AudioBundle {
            source: asset_server.load(sound_file),
            settings: PlaybackSettings::DESPAWN, // Se destruye al terminar
        });
    }
}

// Sistema que revisa la entrada del jugador y puntúa
fn scoring_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut current_command: ResMut<AudioCommand>, // 'mut' para poder resetearlo
    mut score: ResMut<Score>,
) {
    // Si no hay un comando activo, no hagas nada
    if *current_command == AudioCommand::None { 
        return; 
    }

    // Detectar qué tecla se presionó
    let mut pressed_key = None;
    if keys.just_pressed(KeyCode::ArrowLeft) { pressed_key = Some(AudioCommand::Left); }
    if keys.just_pressed(KeyCode::ArrowRight) { pressed_key = Some(AudioCommand::Right); }
    if keys.just_pressed(KeyCode::ArrowUp) { pressed_key = Some(AudioCommand::Jump); }

    // Si se presionó una de las teclas del juego
    if let Some(key) = pressed_key {
        // Compara la tecla con el comando esperado
        if key == *current_command {
            score.value += 1; // Correcto
            println!("¡Correcto! Puntuación: {}", score.value);
        } else {
            score.value -= 1; // Incorrecto
            println!("¡Error! Puntuación: {}", score.value);
        }
        
        // Resetea el comando a 'None' para que el sistema sepa que ya respondiste
        *current_command = AudioCommand::None;
    }
}