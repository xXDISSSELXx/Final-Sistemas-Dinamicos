use bevy::prelude::*;
use crate::gamestate::AppState; // Importamos nuestro Enum de estados

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // setup_player se ejecuta UNA VEZ al ENTRAR al estado JuegoActivo
            .add_systems(OnEnter(AppState::JuegoActivo), setup_player)
            
            // Estos sistemas se ejecutan CADA FRAME, pero SÓLO si estamos en JuegoActivo
            .add_systems(Update, 
                (player_movement_system, simple_physics_system)
                .run_if(in_state(AppState::JuegoActivo))
            );
    }
}

// --- Componentes ---
// Componente "marcador" para identificar al jugador
#[derive(Component)]
struct Player;

// Componente que guarda la velocidad vertical del jugador
#[derive(Component)]
struct Velocity {
    y: f32,
}

// --- Constantes de Física ---
const GRAVITY: f32 = -980.0; // Gravedad
const PLAYER_SPEED: f32 = 250.0;
const JUMP_IMPULSE: f32 = 400.0;
const GROUND_LEVEL: f32 = -150.0; // Nivel del "suelo"

// --- Sistemas ---
fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawnea (crea) al jugador
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("character.png"), // Carga la imagen
            transform: Transform::from_xyz(0.0, GROUND_LEVEL, 0.0),
            ..default()
        },
        Player, // Le añade el componente Player
        Velocity { y: 0.0 }, // Le añade el componente Velocity
    ));
}

// Sistema que maneja el input y el movimiento
fn player_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    // Obtenemos el jugador (si existe)
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        
        // Revisa si el jugador está en el suelo
        let on_ground = transform.translation.y <= GROUND_LEVEL;

        // Movimiento Izquierda / Derecha
        if keys.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::ArrowRight) {
            transform.translation.x += PLAYER_SPEED * time.delta_seconds();
        }

        // Salto
        if keys.just_pressed(KeyCode::ArrowUp) && on_ground {
            velocity.y = JUMP_IMPULSE; // Le da un impulso hacia arriba
        }
    }
}

// Sistema que aplica la gravedad y el movimiento vertical
fn simple_physics_system(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        // 1. Aplica gravedad a la velocidad
        velocity.y += GRAVITY * time.delta_seconds();
        
        // 2. Aplica la velocidad a la posición
        transform.translation.y += velocity.y * time.delta_seconds();

        // 3. Colisión con el suelo
        if transform.translation.y < GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.y = 0.0;
        }
    }
}