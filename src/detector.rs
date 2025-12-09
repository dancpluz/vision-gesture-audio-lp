use opencv::{
    core::{Point, Rect, Vector},
    imgproc::{arc_length, bounding_rect, contour_area, moments},
};
use std::error::Error;

use crate::config::DetectionConfig;

/// Resultado da detecção de palma
#[derive(Clone)]
pub struct PalmDetection {
    pub contour: Vector<Point>,
    pub center: Point,
    pub rect: Rect,
    pub score: f64,
    pub area: f64,
    pub circularity: f64,
}

/// Detecta a melhor palma nos contornos fornecidos
pub fn detect_best_palm(
    contours: &Vector<Vector<Point>>,
    config: &DetectionConfig,
) -> Result<Option<PalmDetection>, Box<dyn Error>> {
    let mut best_palm: Option<PalmDetection> = None;
    let mut best_score = 0.0;
    
    for contour in contours {
        let area = contour_area(&contour, false)?;
        
        // Filtrar por área
        if area < config.min_contour_area || area > config.max_contour_area {
            continue;
        }
        
        let rect = bounding_rect(&contour)?;
        
        // Calcular métricas de forma
        let metrics = calculate_shape_metrics(&contour, &rect, area)?;
        
        // Calcular score composto
        let score = calculate_palm_score(&metrics);
        
        // Verificar se atende aos critérios mínimos
        if metrics.circularity >= config.min_circularity
            && metrics.solidity >= config.min_solidity
            && metrics.normalized_aspect >= config.min_aspect_ratio
            && score > best_score
        {
            best_score = score;
            best_palm = Some(PalmDetection {
                contour: contour.clone(),
                center: metrics.center,
                rect,
                score,
                area,
                circularity: metrics.circularity,
            });
        }
    }
    
    Ok(best_palm)
}

/// Métricas de forma de um contorno
struct ShapeMetrics {
    circularity: f64,
    solidity: f64,
    normalized_aspect: f64,
    center: Point,
}

/// Calcula métricas de forma para um contorno
fn calculate_shape_metrics(
    contour: &Vector<Point>,
    rect: &Rect,
    area: f64,
) -> Result<ShapeMetrics, Box<dyn Error>> {
    // Circularidade
    let perimeter = arc_length(&contour, true)?;
    let circularity = if perimeter > 0.0 {
        4.0 * std::f64::consts::PI * area / (perimeter * perimeter)
    } else {
        0.0
    };
    
    // Proporção de aspecto normalizada
    let aspect_ratio = rect.width as f64 / rect.height as f64;
    let normalized_aspect = if aspect_ratio > 1.0 {
        1.0 / aspect_ratio
    } else {
        aspect_ratio
    };
    
    // Solidez
    let rect_area = (rect.width * rect.height) as f64;
    let solidity = if rect_area > 0.0 {
        area / rect_area
    } else {
        0.0
    };
    
    // Centro de massa
    let m = moments(&contour, false)?;
    let center = if m.m00 != 0.0 {
        Point::new(
            (m.m10 / m.m00) as i32,
            (m.m01 / m.m00) as i32,
        )
    } else {
        Point::new(rect.x + rect.width / 2, rect.y + rect.height / 2)
    };
    
    Ok(ShapeMetrics {
        circularity,
        solidity,
        normalized_aspect,
        center,
    })
}

/// Calcula um score composto para identificar palmas
fn calculate_palm_score(metrics: &ShapeMetrics) -> f64 {
    // Ponderar: circularidade (0.4) + solidez (0.3) + aspecto normalizado (0.3)
    (metrics.circularity * 0.4) + (metrics.solidity * 0.3) + (metrics.normalized_aspect * 0.3)
}