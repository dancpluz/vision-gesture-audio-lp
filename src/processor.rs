use opencv::{
    core::{self, Mat, MatTraitConst, Point, Scalar, Vector},
    imgproc::{
        self, CHAIN_APPROX_SIMPLE, COLOR_BGR2GRAY, RETR_EXTERNAL, THRESH_BINARY,
        cvt_color, find_contours,
    },
};
use std::error::Error;

use crate::config::DetectionConfig;
use crate::detector::{self, PalmDetection};

/// Estado da aplicação
pub struct AppState {
    pub prev_frame: Mat,
    pub background: Mat,
    pub first_frame: bool,
    pub frame_count: u32,
    pub show_camera_view: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            prev_frame: Mat::default(),
            background: Mat::default(),
            first_frame: true,
            frame_count: 0,
            show_camera_view: true,
        }
    }
    
    pub fn update_previous_frame(&mut self) {
        // Implementado no loop principal
    }
}

/// Resultado do processamento de detecção
pub struct DetectionResult {
    pub palm: Option<PalmDetection>,
    pub total_contours: usize,
    pub other_contours: Vec<(Vector<Point>, f64)>,
}

/// Processa um frame e detecta movimento/palmas
pub fn process_frame(
    frame: &mut Mat,
    state: &mut AppState,
    config: &DetectionConfig,
) -> Result<DetectionResult, Box<dyn Error>> {
    state.frame_count += 1;
    
    // Converter para escala de cinza
    let mut gray = Mat::default();
    cvt_color(frame, &mut gray, COLOR_BGR2GRAY, 0)?;
    
    // Inicializar no primeiro frame
    if state.first_frame {
        gray.copy_to(&mut state.prev_frame)?;
        gray.copy_to(&mut state.background)?;
        state.first_frame = false;
        println!("✓ Frame inicial capturado! Iniciando detecção de movimento...");
        return Ok(DetectionResult {
            palm: None,
            total_contours: 0,
            other_contours: Vec::new(),
        });
    }
    
    // Detectar movimento
    let motion_mask = detect_motion(&gray, &state.prev_frame, config)?;
    
    // Encontrar contornos
    let mut contours = Vector::<Vector<Point>>::new();
    find_contours(
        &motion_mask,
        &mut contours,
        RETR_EXTERNAL,
        CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )?;
    
    // Detectar melhor palma
    let palm = detector::detect_best_palm(&contours, config)?;
    
    // Coletar outros contornos (se não detectou palma)
    let other_contours = if palm.is_none() {
        collect_other_contours(&contours)?
    } else {
        Vec::new()
    };
    
    // Atualizar frame anterior
    gray.copy_to(&mut state.prev_frame)?;
    
    Ok(DetectionResult {
        palm,
        total_contours: contours.len(),
        other_contours,
    })
}

/// Detecta movimento entre dois frames
fn detect_motion(
    gray: &Mat,
    prev_frame: &Mat,
    config: &DetectionConfig,
) -> Result<Mat, Box<dyn Error>> {
    // Calcular diferença
    let mut diff = Mat::default();
    core::absdiff(&gray, &prev_frame, &mut diff)?;
    
    // Binarizar
    let mut binary = Mat::default();
    imgproc::threshold(
        &diff,
        &mut binary,
        config.motion_threshold,
        255.0,
        THRESH_BINARY,
    )?;
    
    // Operações morfológicas
    let kernel = imgproc::get_structuring_element(
        imgproc::MORPH_ELLIPSE,
        core::Size::new(config.kernel_size, config.kernel_size),
        Point::new(-1, -1),
    )?;
    
    let mut result = Mat::default();
    imgproc::morphology_ex(
        &binary,
        &mut result,
        imgproc::MORPH_CLOSE,
        &kernel,
        Point::new(-1, -1),
        config.morph_iterations,
        core::BORDER_CONSTANT,
        Scalar::all(0.0),
    )?;
    
    Ok(result)
}

/// Coleta contornos menores que não são palmas
fn collect_other_contours(
    contours: &Vector<Vector<Point>>,
) -> Result<Vec<(Vector<Point>, f64)>, Box<dyn Error>> {
    let mut result = Vec::new();
    
    for contour in contours {
        let area = imgproc::contour_area(&contour, false)?;
        if area > 500.0 {
            result.push((contour.clone(), area));
        }
    }
    
    Ok(result)
}