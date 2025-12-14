use opencv::{
    core::{Mat, Point},
    imgproc::{FONT_HERSHEY_SIMPLEX, LINE_AA, line, put_text},
};

use crate::{
    config::{COLOR_WHITE, COLOR_YELLOW},
    ui::DetectedMarker,
};
pub struct DebugManager {
    pub debug_mode: bool,
    pub window_name: String,
    pub window_created: bool,
}

impl DebugManager {
    pub fn new() -> Self {
        DebugManager {
            debug_mode: false,
            window_name: "Debug View".to_string(),
            window_created: false,
        }
    }

    pub fn toggle_debug_mode(&mut self) -> bool {
        self.debug_mode = !self.debug_mode;
        self.debug_mode
    }

    pub fn draw_debug_overlay(
        &self,
        frame: &mut Mat,
        markers_count: usize,
        frame_size: (i32, i32),
        processing_time: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.debug_mode {
            return Ok(());
        }

        let debug_text = format!(
            "DEBUG | Marcadores: {} | Res: {}x{} | Tempo: {:.1}ms",
            markers_count, frame_size.0, frame_size.1, processing_time
        );

        put_text(
            frame,
            &debug_text,
            Point::new(10, 90),
            FONT_HERSHEY_SIMPLEX,
            0.6,
            COLOR_YELLOW,
            1,
            LINE_AA,
            false,
        )?;

        Ok(())
    }

    /// cruz + id + linhas dos marcadores para debug
    pub fn create_debug_image(
        &self,
        processed_frame: &Mat,
        markers: &[DetectedMarker],
    ) -> Result<Mat, Box<dyn std::error::Error>> {
        let mut debug_frame = processed_frame.clone();

        for marker in markers {
            let corners = &marker.corners;
            if corners.len() != 4 {
                continue;
            }

            let color = COLOR_WHITE;

            for i in 0..4 {
                let start_point = Point::new(corners[i].x as i32, corners[i].y as i32);
                let end_point =
                    Point::new(corners[(i + 1) % 4].x as i32, corners[(i + 1) % 4].y as i32);
                line(
                    &mut debug_frame,
                    start_point,
                    end_point,
                    color,
                    2,
                    LINE_AA,
                    0,
                )?;
            }

            // ID do marcador
            if marker.id == 0 {
                let center_x = marker.center.x as i32;
                let center_y = marker.center.y as i32;

                let cross_size = 8;
                line(
                    &mut debug_frame,
                    Point::new(center_x - cross_size, center_y),
                    Point::new(center_x + cross_size, center_y),
                    color,
                    2,
                    LINE_AA,
                    0,
                )?;

                line(
                    &mut debug_frame,
                    Point::new(center_x, center_y - cross_size),
                    Point::new(center_x, center_y + cross_size),
                    color,
                    2,
                    LINE_AA,
                    0,
                )?;

                // texto com ID
                let id_text = format!("ID: {}", marker.id);
                put_text(
                    &mut debug_frame,
                    &id_text,
                    Point::new(center_x + 10, center_y - 10),
                    FONT_HERSHEY_SIMPLEX,
                    0.5,
                    color,
                    1,
                    LINE_AA,
                    false,
                )?;
            }
        }

        Ok(debug_frame)
    }

    pub fn print_debug_info(
        &self,
        frame_counter: u32,
        markers_count: usize,
        marker_detected: bool,
        marker_position: Option<(f32, f32)>,
    ) {
        if !self.debug_mode || frame_counter % 30 != 0 {
            return;
        }

        let position_info = if let Some((x, y)) = marker_position {
            format!("Pos: ({:.3}, {:.3})", x, y)
        } else {
            "Pos: N/A".to_string()
        };

        println!(
            "[DEBUG] Frame: {} | {} | Marcadores: {} | Detectado: {}",
            frame_counter,
            position_info,
            markers_count,
            if marker_detected { "SIM" } else { "NÃO" }
        );
    }
}
