use opencv::{
    core::{AlgorithmHint, Mat, Point2f, Vector},
    imgproc::{COLOR_BGR2GRAY, cvt_color},
    objdetect::{
        ArucoDetector, CornerRefineMethod, DetectorParameters, RefineParameters,
        get_predefined_dictionary,
    },
    prelude::{ArucoDetectorTraitConst, DetectorParametersTrait},
};
use std::{error::Error, time::Instant};

use crate::{
    config::{DEFAULT_MIN_MARKER_SIZE, DICTIONARY_TYPE},
    ui::{DetectedMarker, NormalizedPosition},
};

pub struct ArucoProcessor {
    detector: ArucoDetector,
    min_marker_size: f32,
    last_position: NormalizedPosition,
    processed_frame: Mat,
    last_processing_time: f32,
}

impl ArucoProcessor {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let dictionary = get_predefined_dictionary(DICTIONARY_TYPE)?;

        let mut parameters = DetectorParameters::default()?;

        parameters.set_adaptive_thresh_win_size_min(3);
        parameters.set_adaptive_thresh_win_size_max(23);
        parameters.set_adaptive_thresh_constant(7.0);
        parameters.set_min_marker_perimeter_rate(0.03);
        parameters.set_max_marker_perimeter_rate(4.0);
        parameters.set_polygonal_approx_accuracy_rate(0.05);
        parameters.set_min_corner_distance_rate(0.05);
        parameters.set_min_distance_to_border(3);
        parameters.set_marker_border_bits(1);
        parameters.set_min_otsu_std_dev(5.0);
        parameters.set_perspective_remove_pixel_per_cell(8);
        parameters.set_perspective_remove_ignored_margin_per_cell(0.33);
        parameters.set_max_erroneous_bits_in_border_rate(0.35);
        parameters.set_error_correction_rate(0.6);
        parameters.set_use_aruco3_detection(true);
        parameters.set_corner_refinement_method(CornerRefineMethod::CORNER_REFINE_SUBPIX as i32);
        parameters.set_corner_refinement_win_size(5);
        parameters.set_corner_refinement_max_iterations(30);
        parameters.set_corner_refinement_min_accuracy(0.1);

        let refine_params = RefineParameters {
            min_rep_distance: 10.0,
            error_correction_rate: 3.0,
            check_all_orders: true,
        };

        let detector = ArucoDetector::new(&dictionary, &parameters, refine_params)?;

        Ok(ArucoProcessor {
            detector,
            min_marker_size: DEFAULT_MIN_MARKER_SIZE,
            last_position: NormalizedPosition::new(0.0, 0.0, false),
            processed_frame: Mat::default(),
            last_processing_time: 0.0,
        })
    }

    pub fn detect_markers(
        &mut self,
        frame: &Mat,
    ) -> Result<(Vec<DetectedMarker>, f32), Box<dyn Error>> {
        let start_time = Instant::now();

        self.processed_frame = Mat::default();

        cvt_color(
            frame,
            &mut self.processed_frame,
            COLOR_BGR2GRAY,
            0,
            AlgorithmHint::ALGO_HINT_DEFAULT,
        )?;

        let mut corners = Vector::<Vector<Point2f>>::new();
        let mut ids = Vector::<i32>::new();
        let mut rejected = Vector::<Vector<Point2f>>::new();

        self.detector.detect_markers(
            &self.processed_frame,
            &mut corners,
            &mut ids,
            &mut rejected,
        )?;

        let mut markers = Vec::new();
        for (i, id) in ids.iter().enumerate() {
            if let Ok(corner_vec) = corners.get(i) {
                let corners_vec: Vec<Point2f> = corner_vec.iter().collect();
                let marker = DetectedMarker::new(id, corners_vec);

                if self.is_marker_valid(&marker) {
                    markers.push(marker);
                }
            }
        }

        self.last_processing_time = start_time.elapsed().as_secs_f32() * 1000.0;

        Ok((markers, self.last_processing_time))
    }

    fn is_marker_valid(&self, marker: &DetectedMarker) -> bool {
        // calcular perímetro
        let corners = &marker.corners;
        if corners.len() != 4 {
            return false;
        }

        let mut perimeter = 0.0;
        for i in 0..4 {
            let j = (i + 1) % 4;
            let dx = corners[i].x - corners[j].x;
            let dy = corners[i].y - corners[j].y;
            perimeter += (dx * dx + dy * dy).sqrt();
        }

        if perimeter < self.min_marker_size {
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

        max_variation < 0.5
    }

    pub fn calculate_marker0_position(
        &mut self,
        frame_width: i32,
        frame_height: i32,
        markers: &[DetectedMarker],
    ) -> NormalizedPosition {
        for marker in markers {
            if marker.id == 0 {
                let center = marker.center;

                // normalizar posição para [-1, 1]
                let x_normalized = ((center.x * 2.0) / frame_width as f32) - 1.0;
                let y_normalized = ((center.y * 2.0) / frame_height as f32) - 1.0;

                let position = NormalizedPosition::new(x_normalized, y_normalized, true);
                self.last_position = position;
                return position;
            }
        }

        let position = NormalizedPosition::new(0.0, 0.0, false);
        self.last_position = position;
        position
    }

    pub fn get_processed_frame(&self) -> &Mat {
        &self.processed_frame
    }
}
