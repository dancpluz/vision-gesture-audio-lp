mod camera;
mod config;
mod detector;
mod ui;
mod processor;

use opencv::core::Mat;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Detecção de Movimento de Mãos em Tempo Real ===");
    
    // Inicializar câmera ou vídeo
    let (mut cam, is_camera) = camera::initialize_capture()?;
    
    // Exibir controles
    ui::print_controls();
    
    // Criar janelas
    ui::create_windows()?;
    
    // Estado da aplicação
    let mut app_state = processor::AppState::new();
    let config = config::DetectionConfig::default();
    
    // Loop principal
    loop {
        // Capturar frame
        let mut frame = Mat::default();
        if !camera::read_frame(&mut cam, &mut frame, is_camera)? {
            break;
        }
        
        // Processar frame
        let detection_result = processor::process_frame(
            &mut frame,
            &mut app_state,
            &config,
        )?;
        
        // Renderizar UI
        ui::render_frame(&mut frame, &detection_result, &app_state)?;
        
        // Mostrar frame
        ui::display_frame(&frame, &app_state)?;
        
        // Atualizar estado anterior
        app_state.update_previous_frame();
        
        // Processar input do teclado
        if !ui::handle_keyboard_input(&mut app_state, &frame)? {
            break;
        }
    }
    
    println!("Programa finalizado!");
    Ok(())
}