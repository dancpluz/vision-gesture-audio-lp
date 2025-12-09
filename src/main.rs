mod audio;
mod audio_controller;
mod camera;
mod commands;
mod config;

use audio::AudioPlayer;
use audio_controller::AudioController;
use commands::{KeyCommand, key_to_command};
use opencv::core::Mat;
use opencv::highgui::{WINDOW_AUTOSIZE, imshow, named_window, wait_key};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎬 === Projeto Visão + Gestos + Áudio ===");
    println!("🎮 Controles:");
    println!("  ESC     - Sair");
    println!("  ESPAÇO  - Iniciar/Parar áudio");
    println!("  + / -   - Ajuste fino de pitch");
    println!("  R       - Resetar pitch para normal");
    println!();

    // Initialize audio system
    let audio_player = AudioPlayer::new()?;
    println!("🔊 Sistema de áudio inicializado com sucesso");

    // Find audio file
    let audio_file = match config::find_audio_file() {
        Some(path) => {
            println!("🎵 Arquivo de áudio encontrado: {}", path);
            path
        }
        None => {
            println!("⚠️  Nenhum arquivo de áudio encontrado");
            println!("📁 Adicione um arquivo audio.mp3 na pasta do projeto");
            "audio.mp3".to_string()
        }
    };

    // Create audio controller
    let mut audio_controller = AudioController::new(audio_player, audio_file);

    // Initialize camera or video
    let (mut cam, is_camera) = camera::initialize_capture()?;

    // Create window
    named_window("Video", WINDOW_AUTOSIZE)?;

    println!("🎥 Iniciando captura de vídeo...");
    println!("🎵 Pitch atual: {:.2}", audio_controller.get_current_pitch());
    println!();

    // Main loop
    loop {
        // Capture frame
        let mut frame = Mat::default();
        if !camera::read_frame(&mut cam, &mut frame, is_camera)? {
            println!("📹 Fim do vídeo/câmera");
            break;
        }

        // Show frame
        imshow("Video", &frame)?;

        // Check for key presses
        let key = wait_key(30)?;
        let command = key_to_command(key);

        match command {
            KeyCommand::Exit => {
                println!("\n✅ Programa encerrado!");
                audio_controller.stop();
                break;
            }
            KeyCommand::ToggleAudio => {
                audio_controller.toggle_audio_playback()?;
            }
            KeyCommand::ResetPitch => {
                audio_controller.reset_pitch()?;
            }
            KeyCommand::AdjustPitch(_) => {
                audio_controller.process_pitch_adjustment(key)?;
            }
            KeyCommand::None => {}
        }
    }

    println!("🎉 Até logo!");
    Ok(())
}
