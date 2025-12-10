mod aruco;
mod camera;
mod config;
mod theremin;
use aruco::ArucoProcessor;
use opencv::{
    core::Mat,
    highgui::{WINDOW_AUTOSIZE, imshow, named_window, wait_key},
    prelude::MatTraitConst,
};
use std::error::Error;
use theremin::ThereminController;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎬 === Rastreamento ArUco + Theremin ===");
    println!("🎮 Controles de teclado:");
    println!(" ESC - Sair");
    println!(" ESPAÇO - Ativar/Desativar som");
    println!(" [ / ] - Ajustar sensibilidade ArUco");
    println!();
    println!("🎯 Rastreamento de Marcador ArUco:");
    println!(" - Apenas marcador ID 0 é rastreado");
    println!(" - Posição normalizada: [-1, 1] em ambos os eixos");
    println!(" - Centro da tela: (0, 0)");
    println!(" - Esquerda: x = -1, Direita: x = 1");
    println!(" - Cima: y = -1, Baixo: y = 1");
    println!();

    // Inicializar o theremin
    let mut theremin_controller = ThereminController::new()?;
    println!("🔊 Theremin inicializado. Som ativo.");

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
    let mut last_position = (0.0, 0.0); // Armazena a última posição (x, y)

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

                    // Se o marcador foi detectado, atualizar a posição
                    if marker_position.detected {
                        last_position = (marker_position.x, marker_position.y);
                        theremin_controller
                            .update_from_position(marker_position.x, marker_position.y);
                    } else {
                        // Marcador não detectado, usar a última posição
                        theremin_controller.update_from_position(last_position.0, last_position.1);
                    }

                    // Desenhar informações do theremin na tela
                    draw_theremin_info(&mut frame, &theremin_controller, marker_position.detected)?;

                    // Imprimir posição no console a cada frame
                    if frame_counter % 30 == 0 {
                        if marker_position.detected {
                            println!(
                                "📍 Frame {}: Marcador em (x: {:.3}, y: {:.3}) | Frequência: {:.1} Hz, Amplitude: {:.2}",
                                frame_counter,
                                marker_position.x,
                                marker_position.y,
                                theremin_controller.get_frequency(),
                                theremin_controller.get_amplitude()
                            );
                        } else {
                            println!(
                                "📍 Frame {}: Marcador não detectado | Usando últimos valores",
                                frame_counter
                            );
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
            27 => {
                // ESC
                println!("\n✅ Programa encerrado!");
                theremin_controller.stop();
                break;
            }
            32 => {
                // ESPAÇO
                theremin_controller.toggle_sound();
            }
            91 => {
                // '['
                if let Some(processor) = &mut aruco_processor {
                    processor.set_min_marker_size(30.0 * 1.2);
                }
            }
            93 => {
                // ']'
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

/// Desenha informações do theremin na tela
fn draw_theremin_info(
    frame: &mut opencv::core::Mat,
    controller: &ThereminController,
    marker_detected: bool,
) -> Result<(), Box<dyn Error>> {
    use opencv::{
        core::Point,
        imgproc::{FONT_HERSHEY_SIMPLEX, LINE_AA, put_text},
        prelude::MatTrait,
    };

    let mut y_offset = 60;

    // Status do som
    let sound_status = if controller.is_enabled() { "ON" } else { "OFF" };
    let sound_color = if controller.is_enabled() {
        opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0) // Verde
    } else {
        opencv::core::Scalar::new(0.0, 0.0, 255.0, 0.0) // Vermelho
    };

    put_text(
        frame,
        &format!("Som: {}", sound_status),
        Point::new(10, y_offset),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        sound_color,
        2,
        LINE_AA,
        false,
    )?;

    y_offset += 30;

    // Informações de áudio
    put_text(
        frame,
        &format!("Freq: {:.1} Hz", controller.get_frequency()),
        Point::new(10, y_offset),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        opencv::core::Scalar::new(255.0, 255.0, 255.0, 0.0),
        2,
        LINE_AA,
        false,
    )?;

    y_offset += 30;

    put_text(
        frame,
        &format!("Amp: {:.2}", controller.get_amplitude()),
        Point::new(10, y_offset),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        opencv::core::Scalar::new(255.0, 255.0, 255.0, 0.0),
        2,
        LINE_AA,
        false,
    )?;

    y_offset += 30;

    // Status do marcador
    let marker_status = if marker_detected {
        "DETECTADO"
    } else {
        "NÃO DETECTADO"
    };
    let marker_color = if marker_detected {
        opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0) // Verde
    } else {
        opencv::core::Scalar::new(0.0, 0.0, 255.0, 0.0) // Vermelho
    };

    put_text(
        frame,
        &format!("Marcador: {}", marker_status),
        Point::new(10, y_offset),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        marker_color,
        2,
        LINE_AA,
        false,
    )?;

    Ok(())
}
