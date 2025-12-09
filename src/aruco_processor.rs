//! Processador para detecção de marcadores ArUco e mapeamento para comandos.
use opencv::{
    core::{Point, Point2f, Scalar, Vector},
    imgproc::{FONT_HERSHEY_SIMPLEX, LINE_AA, put_text},
    objdetect::{
        ArucoDetector, DetectorParameters, PredefinedDictionaryType, RefineParameters,
        get_predefined_dictionary,
    },
    prelude::{ArucoDetectorTraitConst, MatTraitConst},
};
use std::error::Error;

/// Representa um marcador ArUco detectado
#[derive(Debug, Clone)]
pub struct DetectedMarker {
    pub id: i32,
    pub corners: Vec<Point2f>,
}

impl DetectedMarker {
    /// Calcula o ponto central do marcador
    pub fn center(&self) -> Point2f {
        if self.corners.len() >= 4 {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for corner in &self.corners {
                sum_x += corner.x;
                sum_y += corner.y;
            }
            Point2f::new(sum_x / 4.0, sum_y / 4.0)
        } else {
            Point2f::new(0.0, 0.0)
        }
    }

    /// Calcula o perímetro do marcador
    pub fn perimeter(&self) -> f32 {
        let mut perimeter = 0.0;
        let n = self.corners.len();
        for i in 0..n {
            let j = (i + 1) % n;
            let dx = self.corners[i].x - self.corners[j].x;
            let dy = self.corners[i].y - self.corners[j].y;
            perimeter += (dx * dx + dy * dy).sqrt();
        }
        perimeter
    }
}

/// Posição normalizada de um marcador
#[derive(Debug, Clone, Copy)]
pub struct NormalizedPosition {
    pub x: f32, // -1 (esquerda) a 1 (direita), 0 = centro
    pub y: f32, // -1 (cima) a 1 (baixo), 0 = centro
    pub detected: bool,
}

impl NormalizedPosition {
    pub fn new(x: f32, y: f32, detected: bool) -> Self {
        NormalizedPosition { x, y, detected }
    }
}

/// Processador principal para detecção de ArUco
pub struct ArucoProcessor {
    detector: ArucoDetector,
    min_marker_size: f32, // Tamanho mínimo do marcador em pixels
    last_position: NormalizedPosition,
}

impl ArucoProcessor {
    /// Cria um novo processador ArUco
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let dictionary = get_predefined_dictionary(PredefinedDictionaryType::DICT_ARUCO_ORIGINAL)?;
        let parameters = DetectorParameters::default()?;
        let refine_params = RefineParameters {
            min_rep_distance: 10.0,
            error_correction_rate: 3.0,
            check_all_orders: true,
        };

        let detector = ArucoDetector::new(&dictionary, &parameters, refine_params)?;

        Ok(ArucoProcessor {
            detector,
            min_marker_size: 30.0,
            last_position: NormalizedPosition::new(0.0, 0.0, false),
        })
    }

    /// Detecta marcadores em um frame de vídeo com filtro de tamanho
    pub fn detect_markers(
        &self,
        frame: &opencv::core::Mat,
    ) -> Result<Vec<DetectedMarker>, Box<dyn Error>> {
        let mut corners = Vector::<Vector<Point2f>>::new();
        let mut ids = Vector::<i32>::new();
        let mut rejected = Vector::<Vector<Point2f>>::new();

        self.detector
            .detect_markers(frame, &mut corners, &mut ids, &mut rejected)?;

        let mut markers = Vec::new();
        for (i, id) in ids.iter().enumerate() {
            if let Ok(corner_vec) = corners.get(i) {
                let corners_vec: Vec<Point2f> = corner_vec.iter().collect();
                let marker = DetectedMarker {
                    id,
                    corners: corners_vec,
                };

                if self.is_marker_valid(&marker) {
                    markers.push(marker);
                }
            }
        }

        Ok(markers)
    }

    /// Verifica se um marcador é válido
    fn is_marker_valid(&self, marker: &DetectedMarker) -> bool {
        let perimeter = marker.perimeter();
        if perimeter < self.min_marker_size {
            return false;
        }

        let corners = &marker.corners;
        if corners.len() != 4 {
            return false;
        }

        let mut side_lengths = Vec::new();
        for i in 0..4 {
            let j = (i + 1) % 4;
            let dx = corners[i].x - corners[j].x;
            let dy = corners[i].y - corners[j].y;
            side_lengths.push((dx * dx + dy * dy).sqrt());
        }

        let avg_length: f32 = side_lengths.iter().sum::<f32>() / 4.0;
        let max_variation = side_lengths
            .iter()
            .map(|&l| (l - avg_length).abs() / avg_length)
            .fold(0f32, |a, b| a.max(b));

        max_variation < 0.3
    }

    /// Calcula a posição normalizada do marcador 0
    pub fn calculate_marker0_position(
        &mut self,
        frame_width: i32,
        frame_height: i32,
        markers: &[DetectedMarker],
    ) -> NormalizedPosition {
        for marker in markers {
            if marker.id == 0 {
                let center = marker.center();

                // Normalizar posição para [-1, 1]
                let x_normalized = ((center.x * 2.0) / frame_width as f32) - 1.0;
                let y_normalized = ((center.y * 2.0) / frame_height as f32) - 1.0;

                let position = NormalizedPosition::new(x_normalized, y_normalized, true);
                self.last_position = position;
                return position;
            }
        }

        // Marcador não detectado
        let position = NormalizedPosition::new(0.0, 0.0, false);
        self.last_position = position;
        position
    }

    /// Desenha marcadores detectados no frame
    pub fn draw_markers(
        &self,
        frame: &mut opencv::core::Mat,
        markers: &[DetectedMarker],
    ) -> Result<(), Box<dyn Error>> {
        if markers.is_empty() {
            return Ok(());
        }

        let mut corners_vec = Vector::<Vector<Point2f>>::new();
        let mut ids_vec = Vector::<i32>::new();

        for marker in markers {
            let mut corner_vec = Vector::<Point2f>::new();
            for corner in &marker.corners {
                corner_vec.push(*corner);
            }
            corners_vec.push(corner_vec);
            ids_vec.push(marker.id);
        }

        opencv::objdetect::draw_detected_markers(
            frame,
            &corners_vec,
            &ids_vec,
            Scalar::new(0.0, 255.0, 0.0, 0.0),
        )?;

        Ok(())
    }

    /// Desenha informações de posição do marcador 0 na tela
    pub fn draw_position_info(
        &self,
        frame: &mut opencv::core::Mat,
        position: &NormalizedPosition,
    ) -> Result<(), Box<dyn Error>> {
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

        // Desenhar cruz de referência no centro da tela
        let center_x = frame.cols() / 2;
        let center_y = frame.rows() / 2;

        // Linha horizontal (vermelha)
        opencv::imgproc::line(
            frame,
            Point::new(center_x - 20, center_y),
            Point::new(center_x + 20, center_y),
            Scalar::new(0.0, 0.0, 255.0, 0.0),
            1,
            LINE_AA,
            0,
        )?;

        // Linha vertical (vermelha)
        opencv::imgproc::line(
            frame,
            Point::new(center_x, center_y - 20),
            Point::new(center_x, center_y + 20),
            Scalar::new(0.0, 0.0, 255.0, 0.0),
            1,
            LINE_AA,
            0,
        )?;

        Ok(())
    }

    /// Ajusta o tamanho mínimo do marcador
    pub fn set_min_marker_size(&mut self, size: f32) {
        self.min_marker_size = size.max(10.0);
        println!(
            "🔧 Tamanho mínimo ajustado para: {} pixels",
            self.min_marker_size
        );
    }
}
