mod aruco_processor;
mod camera;
mod config;

use aruco_processor::ArucoProcessor;
use opencv::{
    core::Mat,
    highgui::{WINDOW_AUTOSIZE, imshow, named_window, wait_key},
    prelude::MatTraitConst,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎬 === Rastreamento ArUco ===");
    println!("🎮 Controles de teclado:");
    println!(" ESC - Sair");
    println!();
    println!("🎯 Rastreamento de Marcador ArUco:");
    println!(" - Apenas marcador ID 0 é rastreado");
    println!(" - Posição normalizada: [-1, 1] em ambos os eixos");
    println!(" - Centro da tela: (0, 0)");
    println!(" - Esquerda: x = -1, Direita: x = 1");
    println!(" - Cima: y = -1, Baixo: y = 1");
    println!();

    let (mut cam, is_camera) = camera::initialize_capture()?;

let mut aruco_processor = match ArucoProcessor::new() {
    Ok(processor) => {
        println!("🎯 Processador ArUco inicializado");
        Some(processor)
    }
    Err(e) => {
        println!("⚠️  Erro ao inicializar ArUco: {}", e);
        println!("ℹ️  Continuando apenas com visualização de vídeo...");
        None
    }
};

named_window("Video", WINDOW_AUTOSIZE)?;
println!("🎥 Iniciando captura de vídeo...");
println!();

let mut frame_counter = 0;

loop {
    frame_counter += 1;

    let mut frame = Mat::default();
    if !camera::read_frame(&mut cam, &mut frame, is_camera)? {
        println!("📹 Fim do vídeo/câmera");
        break;
    }

    let frame_width = frame.cols();
    let frame_height = frame.rows();

    if let Some(processor) = &mut aruco_processor {
        match processor.detect_markers(&frame) {
            Ok(markers) => {
                // Desenhar marcadores no frame
                if let Err(e) = processor.draw_markers(&mut frame, &markers) {
                    eprintln!("⚠️  Erro ao desenhar marcadores: {}", e);
                }

                // Calcular posição do marcador
                let marker_position =
                    processor.calculate_marker0_position(frame_width, frame_height, &markers);

                // Desenhar informações de posição na tela
                if let Err(e) = processor.draw_position_info(&mut frame, &marker_position) {
                    eprintln!("⚠️  Erro ao desenhar informações: {}", e);
                }

                // Imprimir posição no console a cada frame
                if frame_counter % 30 == 0 {
                    if marker_position.detected {
                        println!(
                            "📍 Frame {}: Marcador em (x: {:.3}, y: {:.3})",
                            frame_counter, marker_position.x, marker_position.y
                        );
                    } else {
                        println!("📍 Frame {}: Marcador não detectado", frame_counter);
                    }
                }
            }
            Err(e) => {
                if !e.to_string().contains("empty") && frame_counter % 60 == 0 {
                    eprintln!("⚠️  Erro na detecção: {}", e);
                }
            }
        }
    }

    imshow("Video", &frame)?;

    let key = wait_key(30)?;

    // Controles de teclado
    match key {
        27 => { // ESC
            println!("\n✅ Programa encerrado!");
            break;
        }
        // Teclas para ajustar sensibilidade ArUco
        91 => { // '['
            if let Some(processor) = &mut aruco_processor {
                processor.set_min_marker_size(30.0 * 1.2);
            }
        }
        93 => { // ']'
            if let Some(processor) = &mut aruco_processor {
                processor.set_min_marker_size(30.0 * 0.8);
            }
        }
        _ => {}
    }
}

// Liberação final antes de sair
println!("\n🧹 Liberando recursos...");

// Liberar câmera
if let Err(e) = camera::release_capture(&mut cam) {
    eprintln!("⚠️  Erro ao liberar câmera: {}", e);
}

println!("🎉 Recursos liberados. Até logo!");
Ok(())
}