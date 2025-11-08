// Declara los módulos (los otros archivos .rs)
mod gamestate;
mod player;
mod audio;

use bevy::prelude::*;
use gamestate::GameStatePlugin; // Nuestro plugin de estados (Inicio, Pausa)
use player::PlayerPlugin;       // Nuestro plugin del jugador (movimiento)
use audio::AudioPlugin;        // Nuestro plugin de audio y puntuación

fn main() {
    App::new()
        // Carga los plugins básicos de Bevy (gráficos, sonido, input, etc.)
        .add_plugins(DefaultPlugins)
        
        // Carga NUESTROS plugins personalizados
        .add_plugins((
            GameStatePlugin,
            PlayerPlugin,
            AudioPlugin,
        ))
        .run();
}