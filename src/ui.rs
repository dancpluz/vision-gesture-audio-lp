use crate::theremin::ThereminController;
use opencv::{
    core::{Point, Point2f, Scalar},
    imgproc::{FONT_HERSHEY_SIMPLEX, LINE_AA, get_text_size, line, put_text},
    prelude::MatTraitConst,
};

/// Representa um marcador ArUco detectado (simplificado)
pub struct DetectedMarker {
    pub id: i32,
    pub corners: Vec<Point2f>,
    pub center: Point2f,
}

impl DetectedMarker {
    pub fn new(id: i32, corners: Vec<Point2f>) -> Self {
        let center = if corners.len() >= 4 {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for corner in &corners {
                sum_x += corner.x;
                sum_y += corner.y;
            }
            Point2f::new(sum_x / 4.0, sum_y / 4.0)
        } else {
            Point2f::new(0.0, 0.0)
        };

        DetectedMarker {
            id,
            corners,
            center,
        }
    }
}

/// Posição normalizada de um marcador
#[derive(Debug, Clone, Copy)]
pub struct NormalizedPosition {
    pub x: f32, // -1 (esquerda) a 1 (direita), 0 = centro
    pub y: f32, // -1 (cima) a 1 (baixo), 0 = centro
    pub detected: bool,
    pub center_px: Option<Point>, // Centro em pixels quando detectado
}

impl NormalizedPosition {
    pub fn new(x: f32, y: f32, detected: bool, center_px: Option<Point>) -> Self {
        NormalizedPosition {
            x,
            y,
            detected,
            center_px,
        }
    }
}

/// Desenha informações do theremin na tela
pub fn draw_theremin_info(
    frame: &mut opencv::core::Mat,
    controller: &ThereminController,
) -> Result<(), Box<dyn std::error::Error>> {
    let frame_height = frame.rows();
    let frame_width = frame.cols();

    // 1. Informações no canto superior direito - status do som
    let sound_status = if controller.is_enabled() { "ON" } else { "OFF" };
    let sound_color = if controller.is_enabled() {
        Scalar::new(0.0, 255.0, 0.0, 0.0) // Verde
    } else {
        Scalar::new(0.0, 0.0, 255.0, 0.0) // Vermelho
    };

    // Texto mais descritivo
    let sound_text = format!("Som: {} (Espaco: Liga/Desliga)", sound_status);
    let text_size = get_text_size(&sound_text, FONT_HERSHEY_SIMPLEX, 0.7, 2, &mut 0)?;

    let sound_text_x = frame_width - text_size.width - 10;
    put_text(
        frame,
        &sound_text,
        Point::new(sound_text_x, 30),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        sound_color,
        2,
        LINE_AA,
        false,
    )?;

    // 2. Informações na parte inferior - frequência à esquerda, amplitude à direita
    let freq_text = format!("Freq: {:.1} Hz", controller.get_frequency());
    let amp_text = format!("Amp: {:.2}", controller.get_amplitude());

    // Posição na parte inferior da tela
    let bottom_y = frame_height - 20;

    // Desenhar frequência no canto inferior esquerdo
    put_text(
        frame,
        &freq_text,
        Point::new(10, bottom_y),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
        2,
        LINE_AA,
        false,
    )?;

    // Desenhar amplitude no canto inferior direito
    let amp_text_size = get_text_size(&amp_text, FONT_HERSHEY_SIMPLEX, 0.7, 2, &mut 0)?;

    put_text(
        frame,
        &amp_text,
        Point::new(frame_width - amp_text_size.width - 10, bottom_y),
        FONT_HERSHEY_SIMPLEX,
        0.7,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
        2,
        LINE_AA,
        false,
    )?;

    Ok(())
}

/// Desenha marcadores detectados no frame SEM os IDs
pub fn draw_markers(
    frame: &mut opencv::core::Mat,
    markers: &[DetectedMarker],
) -> Result<(), Box<dyn std::error::Error>> {
    // Desenhar apenas as linhas dos marcadores (sem IDs)
    for marker in markers {
        let corners = &marker.corners;
        if corners.len() != 4 {
            continue;
        }

        // Desenhar as 4 linhas do quadrado
        for i in 0..4 {
            let start_point = Point::new(corners[i].x as i32, corners[i].y as i32);
            let end_point =
                Point::new(corners[(i + 1) % 4].x as i32, corners[(i + 1) % 4].y as i32);

            line(
                frame,
                start_point,
                end_point,
                Scalar::new(0.0, 255.0, 0.0, 0.0), // Verde
                2,                                 // Espessura
                LINE_AA,
                0,
            )?;
        }

        // Desenhar cruz vermelha no centro do marcador
        if marker.id == 0 {
            // Apenas para o marcador 0
            draw_marker_center_cross(frame, marker.center)?;
        }
    }

    Ok(())
}

/// Desenha uma cruz vermelha no centro do marcador
pub fn draw_marker_center_cross(
    frame: &mut opencv::core::Mat,
    center: Point2f,
) -> Result<(), Box<dyn std::error::Error>> {
    let center_x = center.x as i32;
    let center_y = center.y as i32;
    let cross_size = 8; // Tamanho da cruz

    // Linha horizontal (vermelha)
    line(
        frame,
        Point::new(center_x - cross_size, center_y),
        Point::new(center_x + cross_size, center_y),
        Scalar::new(0.0, 0.0, 255.0, 0.0), // Vermelho
        2,                                 // Espessura
        LINE_AA,
        0,
    )?;

    // Linha vertical (vermelha)
    line(
        frame,
        Point::new(center_x, center_y - cross_size),
        Point::new(center_x, center_y + cross_size),
        Scalar::new(0.0, 0.0, 255.0, 0.0), // Vermelho
        2,                                 // Espessura
        LINE_AA,
        0,
    )?;

    Ok(())
}

/// Desenha informações de posição do marcador na tela
pub fn draw_position_info(
    frame: &mut opencv::core::Mat,
    position: &NormalizedPosition,
) -> Result<(), Box<dyn std::error::Error>> {
    // Cor baseada na detecção
    let color = if position.detected {
        Scalar::new(0.0, 255.0, 0.0, 0.0) // Verde quando detectado
    } else {
        Scalar::new(0.0, 0.0, 255.0, 0.0) // Vermelho quando não detectado
    };

    // Texto principal com a posição
    let main_text = if position.detected {
        format!("({:.3}, {:.3})", position.x, position.y)
    } else {
        String::from("Nao detectado")
    };

    // Posição do texto (canto superior esquerdo)
    let text_position = Point::new(10, 30);

    // Desenhar o texto principal
    put_text(
        frame,
        &main_text,
        text_position,
        FONT_HERSHEY_SIMPLEX,
        0.7,
        color,
        2,
        LINE_AA,
        false,
    )?;

    Ok(())
}
