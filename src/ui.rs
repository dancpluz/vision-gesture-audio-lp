use opencv::{
    core::{Mat, MatTraitConst, Point, Scalar, Vector},
    highgui::{WINDOW_AUTOSIZE, destroy_window, imshow, named_window, wait_key},
    imgcodecs::imwrite,
    imgproc::{self, FILLED, FONT_HERSHEY_SIMPLEX, LINE_8, bounding_rect},
};
use std::error::Error;

use crate::config::ui_constants::*;
use crate::processor::{AppState, DetectionResult};

/// Exibe controles no console
pub fn print_controls() {
    println!("Controles:");
    println!("  - S: Salvar frame atual");
    println!("  - V: Alternar entre visão da câmera e do movimento");
    println!("  - ESC: Sair");
    println!();
}

/// Cria as janelas necessárias
pub fn create_windows() -> Result<(), Box<dyn Error>> {
    named_window("Camera", WINDOW_AUTOSIZE)?;
    named_window("Movimento", WINDOW_AUTOSIZE)?;
    Ok(())
}

/// Renderiza informações visuais no frame
pub fn render_frame(
    frame: &mut Mat,
    result: &DetectionResult,
    state: &AppState,
) -> Result<(), Box<dyn Error>> {
    if let Some(palm) = &result.palm {
        render_palm_detection(frame, palm)?;
        render_indicator(frame, true)?;
    } else {
        render_other_contours(frame, &result.other_contours)?;
        render_indicator(frame, false)?;
    }

    render_info_text(frame, result, state)?;
    render_controls_text(frame)?;

    Ok(())
}

/// Renderiza a palma detectada
fn render_palm_detection(
    frame: &mut Mat,
    palm: &crate::detector::PalmDetection,
) -> Result<(), Box<dyn Error>> {
    // Centro de massa (círculo azul)
    imgproc::circle(
        frame,
        palm.center,
        CENTER_CIRCLE_RADIUS,
        Scalar::new(255.0, 0.0, 0.0, 0.0),
        FILLED,
        LINE_8,
        0,
    )?;

    // Cruz vermelha no centro
    imgproc::line(
        frame,
        Point::new(palm.center.x - CROSS_SIZE, palm.center.y),
        Point::new(palm.center.x + CROSS_SIZE, palm.center.y),
        Scalar::new(0.0, 0.0, 255.0, 0.0),
        3,
        LINE_8,
        0,
    )?;
    imgproc::line(
        frame,
        Point::new(palm.center.x, palm.center.y - CROSS_SIZE),
        Point::new(palm.center.x, palm.center.y + CROSS_SIZE),
        Scalar::new(0.0, 0.0, 255.0, 0.0),
        3,
        LINE_8,
        0,
    )?;

    // Círculo delimitador (ciano)
    let radius = ((palm.rect.width.max(palm.rect.height) as f32) * 0.5) as i32;
    imgproc::circle(
        frame,
        palm.center,
        radius,
        Scalar::new(0.0, 255.0, 255.0, 0.0),
        2,
        LINE_8,
        0,
    )?;

    // Texto informativo
    let info_text = format!(
        "PALMA: Score {:.2}, Area {:.0}, Circ {:.2}",
        palm.score, palm.area, palm.circularity
    );
    imgproc::put_text(
        frame,
        &info_text,
        Point::new(palm.rect.x, palm.rect.y - 20),
        FONT_HERSHEY_SIMPLEX,
        0.6,
        Scalar::new(0.0, 255.0, 255.0, 0.0),
        2,
        LINE_8,
        false,
    )?;

    Ok(())
}

/// Renderiza contornos menores (quando não há palma)
fn render_other_contours(
    frame: &mut Mat,
    contours: &[(Vector<Point>, f64)],
) -> Result<(), Box<dyn Error>> {
    for (contour, _) in contours {
        let rect = bounding_rect(&contour)?;
        imgproc::rectangle(
            frame,
            rect,
            Scalar::new(100.0, 100.0, 100.0, 0.0),
            1,
            LINE_8,
            0,
        )?;
    }
    Ok(())
}

/// Renderiza indicador visual (canto superior direito)
fn render_indicator(frame: &mut Mat, detected: bool) -> Result<(), Box<dyn Error>> {
    let color = if detected {
        Scalar::new(0.0, 255.0, 0.0, 0.0) // Verde
    } else {
        Scalar::new(0.0, 0.0, 255.0, 0.0) // Vermelho
    };

    let x = frame.cols() - INDICATOR_X_OFFSET;
    imgproc::circle(
        frame,
        Point::new(x, INDICATOR_Y_OFFSET),
        INDICATOR_RADIUS,
        color,
        FILLED,
        LINE_8,
        0,
    )?;

    Ok(())
}

/// Renderiza texto informativo
fn render_info_text(
    frame: &mut Mat,
    result: &DetectionResult,
    state: &AppState,
) -> Result<(), Box<dyn Error>> {
    let info_text = format!(
        "Frame: {} | Palma detectada: {} | Contornos: {}",
        state.frame_count,
        if result.palm.is_some() { "SIM" } else { "NAO" },
        result.total_contours
    );

    imgproc::put_text(
        frame,
        &info_text,
        Point::new(10, 30),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
        2,
        LINE_8,
        false,
    )?;

    Ok(())
}

/// Renderiza texto de controles
fn render_controls_text(frame: &mut Mat) -> Result<(), Box<dyn Error>> {
    let controls_text = "S: Salvar | V: Alternar | ESC: Sair";

    imgproc::put_text(
        frame,
        controls_text,
        Point::new(10, frame.rows() - 10),
        FONT_HERSHEY_SIMPLEX,
        0.6,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
        2,
        LINE_8,
        false,
    )?;

    Ok(())
}

/// Exibe o frame na janela apropriada
pub fn display_frame(frame: &Mat, state: &AppState) -> Result<(), Box<dyn Error>> {
    if state.show_camera_view {
        imshow("Camera", frame)?;
        destroy_window("Movimento")?;
    }
    // Note: A visualização de movimento requer acesso à motion_mask
    // que pode ser armazenada no AppState se necessário

    Ok(())
}

/// Processa input do teclado
/// Retorna false se deve sair do programa
pub fn handle_keyboard_input(state: &mut AppState, frame: &Mat) -> Result<bool, Box<dyn Error>> {
    let key = wait_key(30)?;

    match key {
        27 => Ok(false), // ESC
        115 => {
            // S - Salvar
            save_frame(frame)?;
            Ok(true)
        }
        118 => {
            // V - Alternar visualização
            toggle_view(state)?;
            Ok(true)
        }
        _ => Ok(true),
    }
}

/// Salva o frame atual
fn save_frame(frame: &Mat) -> Result<(), Box<dyn Error>> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let filename = format!("manual_capture_{}.png", timestamp);
    imwrite(&filename, &frame, &Vector::new())?;
    println!("💾 Imagem salva como: {}", filename);
    Ok(())
}

/// Alterna entre visualizações
fn toggle_view(state: &mut AppState) -> Result<(), Box<dyn Error>> {
    state.show_camera_view = !state.show_camera_view;
    let view_name = if state.show_camera_view {
        "Camera"
    } else {
        "Movimento"
    };
    println!("Alternando para visualização: {}", view_name);

    if state.show_camera_view {
        named_window("Camera", WINDOW_AUTOSIZE)?;
        destroy_window("Movimento")?;
    } else {
        named_window("Movimento", WINDOW_AUTOSIZE)?;
        destroy_window("Camera")?;
    }

    Ok(())
}
