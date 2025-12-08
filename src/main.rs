use opencv::core::{Mat, Vector, CV_8UC1, CV_8UC3, Scalar, Point, Rect, Size};
use opencv::videoio::{VideoCapture, CAP_ANY};
use opencv::imgproc::{cvt_color, COLOR_BGR2GRAY, absdiff, threshold, THRESH_BINARY, find_contours, RETR_EXTERNAL, CHAIN_APPROX_SIMPLE, bounding_rect, circle, line, put_text, FONT_HERSHEY_SIMPLEX, rectangle};
use opencv::imgcodecs::imwrite;
use opencv::prelude::*;
use opencv::highgui::{imshow, wait_key, create_window, WINDOW_AUTOSIZE};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Detecção de Movimento de Mãos em Tempo Real ===");
    println!("Iniciando câmera...");

    // Abrir câmera
    let mut cam = VideoCapture::new(0, CAP_ANY)?;
    if !cam.is_opened()? {
        println!("ERRO: Não foi possível abrir a câmera!");
        println!("Verifique se uma câmera está conectada e não está sendo usada por outro aplicativo.");
        return Ok(());
    }

    println!("✓ Câmera iniciada com sucesso");
    println!("Controles:");
    println!("  - ESPAÇO: Calibrar fundo");
    println!("  - S: Salvar frame atual");
    println!("  - ESC: Sair");
    println!();

    // Criar janelas
    create_window("Camera", WINDOW_AUTOSIZE)?;
    create_window("Movimento", WINDOW_AUTOSIZE)?;

    // Variáveis para processamento
    let mut frame = Mat::default();
    let mut prev_frame = Mat::default();
    let mut background = Mat::default();
    let mut motion_mask = Mat::default();
    let mut first_frame = true;
    let mut frame_count = 0;
    let mut last_save = Instant::now();
    let mut hand_detected_frames = 0;

    // Parâmetros de detecção
    let motion_threshold = 25.0; // Sensibilidade ao movimento
    let min_contour_area = 500.0; // Área mínima para detectar mão
    let max_contour_area = 50000.0; // Área máxima (para filtrar objetos grandes)

    loop {
        // Capturar frame
        cam.read(&mut frame)?;
        if frame.empty() {
            continue;
        }

        frame_count += 1;

        // Converter para escala de cinza
        let mut gray = Mat::default();
        cvt_color(&frame, &mut gray, COLOR_BGR2GRAY, 0)?;

        // Primeiro frame - inicializar
        if first_frame {
            gray.copy_to(&mut prev_frame)?;
            gray.copy_to(&mut background)?;
            first_frame = false;
            println!("✓ Frame inicial capturado! Iniciando detecção de movimento...");
            continue;
        }

        // Calcular diferença entre frames
        absdiff(&gray, &prev_frame, &mut motion_mask)?;

        // Aplicar threshold para binarizar
        threshold(&motion_mask, &mut motion_mask, motion_threshold, 255.0, THRESH_BINARY)?;

        // Encontrar contornos
        let mut contours = Vector::new();
        find_contours(&motion_mask, &mut contours, RETR_EXTERNAL, CHAIN_APPROX_SIMPLE, Point::default())?;

        // Desenhar retângulos ao redor dos contornos (potenciais mãos)
        let mut hand_detected = false;
        let mut largest_contour_area = 0.0;
        let mut hand_rect = Rect::default();

        for i in 0..contours.len() {
            let contour_area = opencv::imgproc::contour_area(&contours.get(i)?, false)?;

            // Filtrar por tamanho (típico para mãos)
            if contour_area > min_contour_area && contour_area < max_contour_area {
                let rect = bounding_rect(&contours.get(i)?)?;

                // Verificar proporção (mãos geralmente são mais altas que largas)
                let aspect_ratio = rect.width as f64 / rect.height as f64;
                if aspect_ratio > 0.5 && aspect_ratio < 2.0 {
                    // Desenhar retângulo verde ao redor da mão detectada
                    rectangle(&mut frame, rect, Scalar::new(0.0, 255.0, 0.0, 0.0), 2)?;

                    // Adicionar label
                    let label = format!("Mao ({:.0})", contour_area);
                    put_text(&mut frame, &label, Point::new(rect.x, rect.y - 10),
                             FONT_HERSHEY_SIMPLEX, 0.6, Scalar::new(0.0, 255.0, 0.0, 0.0), 2)?;

                    hand_detected = true;
                    hand_detected_frames += 1;

                    if contour_area > largest_contour_area {
                        largest_contour_area = contour_area;
                        hand_rect = rect;
                    }
                }
            }
        }

        // Desenhar centro da mão detectada
        if hand_detected {
            let center_x = hand_rect.x + hand_rect.width / 2;
            let center_y = hand_rect.y + hand_rect.height / 2;
            circle(&mut frame, Point::new(center_x, center_y), 5, Scalar::new(0.0, 0.0, 255.0, 0.0), -1)?;

            // Desenhar cruz no centro
            line(&mut frame, Point::new(center_x - 15, center_y), Point::new(center_x + 15, center_y),
                 Scalar::new(0.0, 0.0, 255.0, 0.0), 2)?;
            line(&mut frame, Point::new(center_x, center_y - 15), Point::new(center_x, center_y + 15),
                 Scalar::new(0.0, 0.0, 255.0, 0.0), 2)?;

            // Indicador visual de mão detectada
            circle(&mut frame, Point::new(frame.cols() - 30, 30), 15, Scalar::new(0.0, 255.0, 0.0, 0.0), -1)?;
        } else {
            // Indicador visual de sem mão detectada
            circle(&mut frame, Point::new(frame.cols() - 30, 30), 15, Scalar::new(0.0, 0.0, 255.0, 0.0), -1)?;
        }

        // Mostrar informações na tela
        let info_text = format!("Frame: {} | Mao detectada: {} ({:.0} pixels)",
                               frame_count, hand_detected, largest_contour_area);
        put_text(&mut frame, &info_text, Point::new(10, 30),
                 FONT_HERSHEY_SIMPLEX, 0.7, Scalar::new(255.0, 255.0, 255.0, 0.0), 2)?;

        put_text(&mut frame, "ESPACO: Calibrar | S: Salvar | ESC: Sair", Point::new(10, frame.rows() - 10),
                 FONT_HERSHEY_SIMPLEX, 0.6, Scalar::new(255.0, 255.0, 255.0, 0.0), 2)?;

        // Mostrar imagens
        imshow("Camera", &frame)?;
        imshow("Movimento", &motion_mask)?;

        // Salvar automaticamente quando detectar mão (a cada 5 segundos)
        if hand_detected && last_save.elapsed() > Duration::from_secs(5) {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            let filename = format!("hand_detected_{}_{}.png", timestamp, largest_contour_area as u32);
            imwrite(&filename, &frame, &Vector::new())?;
            println!("✋ MÃO DETECTADA! Salvo: {}", filename);
            last_save = Instant::now();
        }

        // Atualizar frame anterior
        gray.copy_to(&mut prev_frame)?;

        // Processar teclas
        let key = wait_key(30)?; // ~33 FPS

        match key {
            27 => break, // ESC
            32 => { // ESPAÇO
                println!("📸 Calibrando fundo...");
                gray.copy_to(&mut background)?;
                println!("✓ Fundo calibrado!");
            }
            115 => { // S
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs();
                let filename = format!("manual_capture_{}.png", timestamp);
                imwrite(&filename, &frame, &Vector::new())?;
                println!("💾 Imagem salva como: {}", filename);
            }
            _ => {}
        }
    }

    println!("Programa finalizado!");
    Ok(())
}