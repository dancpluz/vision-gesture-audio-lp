//! Processador para detecção de marcadores ArUco e mapeamento para comandos.
use opencv::{
    core::{Point2f, Scalar, Vector},
    objdetect::{
        ArucoDetector, DetectorParameters, PredefinedDictionaryType, RefineParameters,
        get_predefined_dictionary,
    },
    prelude::ArucoDetectorTraitConst,
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

/// Processador principal para detecção de ArUco
pub struct ArucoProcessor {
    detector: ArucoDetector,
    last_detected_ids: Vec<i32>,
    min_marker_size: f32, // Tamanho mínimo do marcador em pixels
}

impl ArucoProcessor {
    /// Cria um novo processador ArUco com parâmetros otimizados
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // 1. Obter dicionário ARUCO_ORIGINAL
        let dictionary = get_predefined_dictionary(PredefinedDictionaryType::DICT_ARUCO_ORIGINAL)?;

        // 2. Criar parâmetros do detector com valores padrão
        let parameters = DetectorParameters::default()?;

        // 3. Criar parâmetros de refinamento
        let refine_params = RefineParameters {
            min_rep_distance: 10.0,
            error_correction_rate: 3.0,
            check_all_orders: true,
        };

        // 4. Criar o detector Aruco
        let detector = ArucoDetector::new(&dictionary, &parameters, refine_params)?;

        Ok(ArucoProcessor {
            detector,
            last_detected_ids: Vec::new(),
            min_marker_size: 30.0, // Tamanho mínimo de 30 pixels
        })
    }

    /// Configura parâmetros de detecção para evitar quadrados pequenos
    pub fn configure_for_better_detection(
        &mut self,
        image_width: i32,
        image_height: i32,
    ) -> Result<(), Box<dyn Error>> {
        // Vamos usar pós-processamento para filtrar marcadores pequenos
        // Definir tamanho mínimo baseado no tamanho da imagem
        let min_dimension = image_width.min(image_height) as f32;
        self.min_marker_size = min_dimension * 0.5; // 10% da dimensão menor

        println!(
            "🔧 Configuração de detecção: tamanho mínimo = {} pixels",
            self.min_marker_size
        );

        Ok(())
    }

    /// Detecta marcadores em um frame de vídeo com filtro de tamanho
    pub fn detect_markers(
        &self,
        frame: &opencv::core::Mat,
    ) -> Result<Vec<DetectedMarker>, Box<dyn Error>> {
        let mut corners = Vector::<Vector<Point2f>>::new();
        let mut ids = Vector::<i32>::new();
        let mut rejected = Vector::<Vector<Point2f>>::new();

        // Executar detecção principal
        self.detector
            .detect_markers(frame, &mut corners, &mut ids, &mut rejected)?;

        // Converter resultados para nossa estrutura e filtrar por tamanho
        let mut markers = Vec::new();
        for (i, id) in ids.iter().enumerate() {
            if let Ok(corner_vec) = corners.get(i) {
                let corners_vec: Vec<Point2f> = corner_vec.iter().collect();
                let marker = DetectedMarker {
                    id,
                    corners: corners_vec,
                };

                // Filtrar marcadores muito pequenos
                if self.is_marker_valid(&marker, frame) {
                    markers.push(marker);
                }
            }
        }

        Ok(markers)
    }

    /// Verifica se um marcador é válido (tamanho suficiente, forma adequada)
    fn is_marker_valid(&self, marker: &DetectedMarker, frame: &opencv::core::Mat) -> bool {
        // 1. Verificar tamanho mínimo
        let perimeter = marker.perimeter();
        if perimeter < self.min_marker_size {
            return false;
        }

        // 2. Verificar se é aproximadamente quadrado
        let corners = &marker.corners;
        if corners.len() != 4 {
            return false;
        }

        // Calcular comprimentos dos lados
        let mut side_lengths = Vec::new();
        for i in 0..4 {
            let j = (i + 1) % 4;
            let dx = corners[i].x - corners[j].x;
            let dy = corners[i].y - corners[j].y;
            side_lengths.push((dx * dx + dy * dy).sqrt());
        }

        // Verificar se todos os lados têm comprimento similar
        let avg_length: f32 = side_lengths.iter().sum::<f32>() / 4.0;
        let max_variation = side_lengths
            .iter()
            .map(|&l| (l - avg_length).abs() / avg_length)
            .fold(0f32, |a, b| a.max(b));

        // Aceitar se a variação for menor que 30%
        max_variation < 0.3
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

        // Desenhar marcadores detectados (verde)
        opencv::objdetect::draw_detected_markers(
            frame,
            &corners_vec,
            &ids_vec,
            Scalar::new(0.0, 255.0, 0.0, 0.0), // Verde: BGR (0, 255, 0)
        )?;

        Ok(())
    }

    /// Identifica comandos baseados em marcadores recém-detectados
    pub fn process_commands(&mut self, markers: &[DetectedMarker]) -> Vec<ArucoCommand> {
        let current_ids: Vec<i32> = markers.iter().map(|m| m.id).collect();
        let mut commands = Vec::new();

        // Encontrar marcadores que apareceram agora (não estavam no frame anterior)
        let newly_detected: Vec<i32> = current_ids
            .iter()
            .filter(|id| !self.last_detected_ids.contains(id))
            .copied()
            .collect();

        // Mapear IDs para comandos de áudio
        for marker_id in newly_detected {
            if let Some(command) = self.map_marker_to_command(marker_id) {
                commands.push(command);
            }
        }

        // Atualizar estado para o próximo frame
        self.last_detected_ids = current_ids;

        commands
    }

    /// Mapeia IDs de marcadores para comandos de áudio
    fn map_marker_to_command(&self, marker_id: i32) -> Option<ArucoCommand> {
        let command_name = match marker_id {
            0 => "toggle_audio",
            1 => "reset_pitch",
            2 => "increase_pitch",
            3 => "decrease_pitch",
            4 => "stop_audio",
            5 => "test_sound",
            6 => "pitch_up_coarse",
            7 => "pitch_down_coarse",
            _ => return None,
        };

        Some(ArucoCommand {
            marker_id,
            command_name: command_name.to_string(),
        })
    }

    /// Retorna informações sobre os parâmetros usados
    pub fn get_parameters_info(&self) -> String {
        format!(
            "ArUco Parameters:\n\
             - Dictionary: DICT_ARUCO_ORIGINAL (1024 marcadores)\n\
             - Tamanho mínimo do marcador: {} pixels\n\
             - Filtro de forma ativado (verificação quadrática)\n\
             - RefineParameters: min_rep_distance=10.0, error_correction_rate=3.0",
            self.min_marker_size
        )
    }

    /// Ajusta o tamanho mínimo do marcador
    pub fn set_min_marker_size(&mut self, size: f32) {
        self.min_marker_size = size.max(10.0); // Mínimo de 10 pixels
        println!(
            "🔧 Tamanho mínimo ajustado para: {} pixels",
            self.min_marker_size
        );
    }
}

/// Representa um comando gerado por um marcador ArUco
#[derive(Debug, Clone)]
pub struct ArucoCommand {
    pub marker_id: i32,
    pub command_name: String,
}
