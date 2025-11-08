use bevy::prelude::*;

// Este es el plugin que importaremos en main.rs
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // 1. Define los estados
            .init_state::<AppState>()
            .set_state(AppState::Inicio) // Estado inicial
            
            // 2. Sistemas para el estado de INICIO
            .add_systems(OnEnter(AppState::Inicio), setup_menu_inicio)
            .add_systems(Update, menu_inicio_input.run_if(in_state(AppState::Inicio)))
            .add_systems(OnExit(AppState::Inicio), cleanup_menu) // Limpia la UI

            // 3. Sistemas para el estado de PAUSA
            .add_systems(OnEnter(AppState::Pausa), setup_menu_pausa)
            .add_systems(Update, menu_pausa_input.run_if(in_state(AppState::Pausa)))
            .add_systems(OnExit(AppState::Pausa), cleanup_menu) // Limpia la UI
            
            // 4. Sistema global para PAUSAR el juego
            .add_systems(Update, check_pause_input);
    }
}

// --- Enum de Estados ---
// 'pub' para que otros módulos (como player.rs) puedan verlo
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Inicio,
    JuegoActivo,
    Pausa,
}

// Componente "marcador" para toda la UI que queramos borrar al cambiar de estado
#[derive(Component)]
struct MenuUI;

// --- Sistemas de INICIO ---
fn setup_menu_inicio(mut commands: Commands) {
    // Spawnea la cámara de UI
    commands.spawn(Camera2dBundle::default());
    
    // Spawnea el texto de bienvenida
    commands.spawn((
        TextBundle::from_section(
            "Proyecto Sistemas Dinámicos\nPresiona 'Enter' para comenzar",
            TextStyle { font_size: 40.0, color: Color::WHITE, ..default() }
        ).with_style(Style {
            margin: UiRect::all(Val::Auto), // Centrar
            ..default()
        }),
        MenuUI, // Marca esto como UI del menú
    ));
}

fn menu_inicio_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        next_state.set(AppState::JuegoActivo); // Cambia al estado de juego
    }
}

// --- Sistemas de PAUSA ---
fn setup_menu_pausa(mut commands: Commands) {
    // Muestra el texto de pausa
    commands.spawn((
        TextBundle::from_section(
            "PAUSA\nPresiona 'P' para reanudar",
            TextStyle { font_size: 40.0, color: Color::WHITE, ..default() }
        ).with_style(Style {
            margin: UiRect::all(Val::Auto), // Centrar
            ..default()
        }),
        MenuUI, // Marca esto como UI del menú
    ));
}

fn menu_pausa_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Reanuda el juego
    if keys.just_pressed(KeyCode::KeyP) {
        next_state.set(AppState::JuegoActivo);
    }
}

// --- Sistema Global ---
fn check_pause_input(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>, // El estado actual
    mut next_state: ResMut<NextState<AppState>>, // Para cambiar al próximo estado
) {
    // Solo pausa si estamos en el juego activo
    if keys.just_pressed(KeyCode::KeyP) && *current_state.get() == AppState::JuegoActivo {
        next_state.set(AppState::Pausa); // Pausa el juego
    }
}

// Sistema genérico que borra toda entidad con el componente 'MenuUI'
fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive(); // Borra la entidad y sus hijos
    }
}