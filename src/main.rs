mod aruco;
mod camera;
mod config;
mod debug;
mod theremin;
mod ui;

use aruco::ArucoProcessor;
use debug::DebugManager;
use opencv::{
    core::Mat,
    highgui::{WINDOW_AUTOSIZE, destroy_window, imshow, named_window, wait_key},
    prelude::MatTraitConst,
};
use std::error::Error;
use theremin::ThereminController;
use ui::{draw_markers, draw_position_info, draw_theremin_info, draw_debug_status};

fn main() -> Result<(), Box<dyn Error>> {
    println!("===== ArUco + Theremin =====");
    println!("Controles:");
    println!("  ESC     - Sair");
    println!("  ESPAÇO  - Ativar/Desativar som");
    println!("  V       - Alternar visualização debug");
    println!("============================");

    let mut theremin_controller = ThereminController::new()?;
    println!("[START] Theremin inicializado. Som ativo.");

    let (mut cam, is_camera) = camera::initialize_capture()?;

    let mut aruco_processor = match ArucoProcessor::new() {
        Ok(processor) => {
            println!("[START] Processador ArUco inicializado");
            Some(processor)
        }
        Err(e) => {
            println!("[ERROR] Erro ao inicializar ArUco: {}", e);
            println!("[INFO] Continuando apenas com visualização de vídeo...");
            None
        }
    };

    let mut debug_manager = DebugManager::new();
    
    named_window("Video", WINDOW_AUTOSIZE)?;
    println!("[START] Iniciando captura de vídeo...");
    println!("============================");

    let mut frame_counter = 0;
    let mut last_position = (0.0, 0.0);

    loop {
        frame_counter += 1;

        let mut frame = Mat::default();
        if !camera::read_frame(&mut cam, &mut frame, is_camera)? {
            println!("[INFO] Fim do vídeo/câmera");
            break;
        }

        let frame_width = frame.cols();
        let frame_height = frame.rows();

        if let Some(processor) = &mut aruco_processor {
            match processor.detect_markers(&frame) {
                Ok((markers, processing_time)) => {
                    if let Err(e) = draw_markers(&mut frame, &markers) {
                        eprintln!("[ERROR] Erro ao desenhar marcadores: {}", e);
                    }

                    let marker_position =
                        processor.calculate_marker0_position(frame_width, frame_height, &markers);

                    if let Err(e) = draw_position_info(&mut frame, &marker_position) {
                        eprintln!("[ERROR] Erro ao desenhar informações: {}", e);
                    }

                    if debug_manager.debug_mode {
                        let processed_frame = processor.get_processed_frame();
                        if !processed_frame.empty() {
                            if !debug_manager.window_created {
                                match named_window(&debug_manager.window_name, WINDOW_AUTOSIZE) {
                                    Ok(_) => {
                                        debug_manager.window_created = true;
                                        println!("[DEBUG] Janela de debug criada");
                                    }
                                    Err(e) => eprintln!("[ERROR] Erro ao criar janela de debug: {}", e),
                                }
                            }
                            
                            match debug_manager.create_debug_image(processed_frame, &markers) {
                                Ok(debug_frame) => {
                                    imshow(&debug_manager.window_name, &debug_frame)?;
                                }
                                Err(e) => eprintln!("[ERROR] Erro ao criar imagem de debug: {}", e),
                            }
                            
                            debug_manager.draw_debug_overlay(
                                &mut frame,
                                markers.len(),
                                (frame_width, frame_height),
                                processing_time,
                            )?;
                        }
                    } else {
                        if debug_manager.window_created {
                            match destroy_window(&debug_manager.window_name) {
                                Ok(_) => {
                                    debug_manager.window_created = false;
                                    println!("[DEBUG] Janela de debug fechada");
                                }
                                Err(e) => {
                                    // ignorar erro se a janela já estiver fechada
                                    if !e.to_string().contains("NULL window") {
                                        eprintln!("[ERROR] Erro ao fechar janela de debug: {}", e);
                                    } else {
                                        debug_manager.window_created = false;
                                    }
                                }
                            }
                        }
                    }

                    draw_debug_status(&mut frame, debug_manager.debug_mode)?;
                    
                    let marker_pos = if marker_position.detected {
                        Some((marker_position.x, marker_position.y))
                    } else {
                        None
                    };
                    
                    debug_manager.print_debug_info(
                        frame_counter,
                        markers.len(),
                        marker_position.detected,
                        marker_pos,
                    );

                    // atualiza theremin
                    if marker_position.detected {
                        last_position = (marker_position.x, marker_position.y);
                        theremin_controller
                            .update_from_position(marker_position.x, marker_position.y);
                    } else {
                        theremin_controller.update_from_position(last_position.0, last_position.1);
                    }

                    draw_theremin_info(&mut frame, &theremin_controller)?;
                }
                Err(e) => {
                    if !e.to_string().contains("empty") && frame_counter % 60 == 0 {
                        eprintln!("[ERROR] Erro na detecção: {}", e);
                    }
                }
            }
        }

        imshow("Video", &frame)?;

        let key = wait_key(30)?;

        match key {
            27 => {
                // ESC
                println!("[INFO] Esc pressionado. Saindo...");
                theremin_controller.stop();
                break;
            }
            32 => {
                // ESPAÇO
                theremin_controller.toggle_sound();
            }
            86 | 118 => {
                // 'V' ou 'v' - Alterna modo debug
                if debug_manager.toggle_debug_mode() {
                    println!("[DEBUG] Modo debug ATIVADO");
                } else {
                    println!("[DEBUG] Modo debug DESATIVADO");
                }
            }
            _ => {}
        }
    }

    println!("============================");
    println!("[INFO] Liberando recursos...");

    if debug_manager.window_created {
        let _ = destroy_window(&debug_manager.window_name);
        debug_manager.window_created = false;
    }

    if let Err(e) = camera::release_capture(&mut cam) {
        eprintln!("[ERROR] Erro ao liberar câmera: {}", e);
    }

    println!("[INFO] Programa finalizado com sucesso.");
    Ok(())
}