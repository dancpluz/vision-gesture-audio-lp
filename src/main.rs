mod aruco_processor; // Novo módulo
mod audio;
mod audio_commands;
mod audio_controller;
mod camera;
mod config;

use aruco_processor::ArucoProcessor;
use audio::AudioPlayer;
use audio_commands::{KeyCommand, key_to_command};
use audio_controller::AudioController;
use opencv::{
    core::Mat,
    highgui::{WINDOW_AUTOSIZE, imshow, named_window, wait_key},
    prelude::MatTraitConst,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎬 === Projeto Visão + Gestos + Áudio ===");
    println!("🎮 Controles de teclado:");
    println!("  ESC     - Sair");
    println!("  ESPAÇO  - Iniciar/Parar áudio");
    println!("  + / -   - Ajuste fino de pitch");
    println!("  R       - Resetar pitch para normal");
    println!();
    println!("🎯 Controles por Marcadores ArUco:");
    println!("  Marcador 0 - Alternar áudio (play/pause)");
    println!("  Marcador 1 - Resetar pitch para 1.0");
    println!("  Marcador 2 - Aumentar pitch");
    println!("  Marcador 3 - Diminuir pitch");
    println!("  Marcador 4 - Parar áudio");
    println!();

    // Inicializar sistema de áudio
    let audio_player = AudioPlayer::new()?;
    println!("🔊 Sistema de áudio inicializado com sucesso");

    // Encontrar arquivo de áudio
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

    // Criar controlador de áudio
    let mut audio_controller = AudioController::new(audio_player, audio_file);

    // Inicializar câmera ou vídeo
    let (mut cam, is_camera) = camera::initialize_capture()?;

    // Inicializar processador ArUco
    let mut aruco_processor = match ArucoProcessor::new() {
        Ok(mut processor) => {
            println!("🎯 Processador ArUco inicializado com sucesso");

            // Configurar tamanho inicial baseado na câmera
            let test_frame_size = if is_camera {
                (640, 480) // Tamanho típico da webcam
            } else {
                (1280, 720) // Tamanho típico de vídeo
            };

            if let Err(e) =
                processor.configure_for_better_detection(test_frame_size.0, test_frame_size.1)
            {
                println!("⚠️  Erro na configuração ArUco: {}", e);
            }

            println!("{}", processor.get_parameters_info());
            Some(processor)
        }
        Err(e) => {
            println!("⚠️  Erro ao inicializar ArUco: {}", e);
            println!("ℹ️  Continuando apenas com controles de teclado...");
            None
        }
    };

    // Criar janela
    named_window("Video", WINDOW_AUTOSIZE)?;

    println!("🎥 Iniciando captura de vídeo...");
    println!(
        "🎵 Pitch atual: {:.2}",
        audio_controller.get_current_pitch()
    );
    println!();

    let mut frame_counter = 0;

    // Loop principal
    loop {
        frame_counter += 1;

        // Capturar frame
        let mut frame = Mat::default();
        if !camera::read_frame(&mut cam, &mut frame, is_camera)? {
            println!("📹 Fim do vídeo/câmera");
            break;
        }

        // Obter dimensões do frame para configuração
        let frame_width = frame.cols();
        let frame_height = frame.rows();

        // Configurar processador ArUco no primeiro frame
        if frame_counter == 1 {
            if let Some(processor) = &mut aruco_processor {
                if let Err(e) = processor.configure_for_better_detection(frame_width, frame_height)
                {
                    println!("⚠️  Erro na configuração ArUco: {}", e);
                }
            }
        }

        // Processar marcadores ArUco
        if let Some(processor) = &mut aruco_processor {
            match processor.detect_markers(&frame) {
                Ok(markers) => {
                    // Desenhar marcadores no frame para visualização
                    if let Err(e) = processor.draw_markers(&mut frame, &markers) {
                        eprintln!("⚠️  Erro ao desenhar marcadores: {}", e);
                    }

                    // Mostrar estatísticas a cada 30 frames
                    if frame_counter % 30 == 0 && !markers.is_empty() {
                        println!(
                            "📊 Frame {}: {} marcador(es) válido(s)",
                            frame_counter,
                            markers.len()
                        );
                        for marker in &markers {
                            let center = marker.center();
                            let perimeter = marker.perimeter();
                            println!(
                                "   - ID {} em ({:.1}, {:.1}), perímetro: {:.1}px",
                                marker.id, center.x, center.y, perimeter
                            );
                        }
                    }

                    // Processar comandos dos marcadores
                    let aruco_commands = processor.process_commands(&markers);

                    // Executar comandos detectados
                    for command in aruco_commands {
                        println!(
                            "🎯 Comando ArUco: {} (ID {})",
                            command.command_name, command.marker_id
                        );

                        // ... processar comandos como antes ...
                    }
                }
                Err(e) => {
                    // Mostrar erro apenas se for relevante
                    if !e.to_string().contains("empty") && frame_counter % 60 == 0 {
                        eprintln!("⚠️  Erro na detecção de ArUco: {}", e);
                    }
                }
            }
        }

        // Mostrar frame
        imshow("Video", &frame)?;

        // Check for key presses
        let key = wait_key(30)?;
        let command = key_to_command(key);

        // Processar teclas de ajuste de sensibilidade ArUco
        match key as u8 as char {
            '[' => {
                // Diminuir sensibilidade (aumentar tamanho mínimo)
                if let Some(processor) = &mut aruco_processor {
                    let current_size = 30.0; // Você pode armazenar isso em uma variável
                    processor.set_min_marker_size(current_size * 1.2);
                    println!("🔧 Sensibilidade ArUco diminuída");
                }
            }
            ']' => {
                // Aumentar sensibilidade (diminuir tamanho mínimo)
                if let Some(processor) = &mut aruco_processor {
                    let current_size = 30.0;
                    processor.set_min_marker_size(current_size * 0.8);
                    println!("🔧 Sensibilidade ArUco aumentada");
                }
            }
            _ => {}
        }

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

    println!(
        "🎉 Até logo! Total de frames processados: {}",
        frame_counter
    );
    Ok(())
}
