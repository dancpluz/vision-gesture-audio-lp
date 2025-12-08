use opencv::{
    core::{self, Mat, MatTraitConst, Point, Rect, Scalar, Vector},
    highgui::{WINDOW_AUTOSIZE, imshow, named_window, wait_key},
    imgcodecs::imwrite,
    imgproc::{
        self, CHAIN_APPROX_SIMPLE, COLOR_BGR2GRAY, FILLED, FONT_HERSHEY_SIMPLEX, LINE_8,
        RETR_EXTERNAL, THRESH_BINARY, cvt_color,
    },
    prelude::*,
    videoio::{CAP_ANY, CAP_PROP_POS_FRAMES, VideoCapture},
};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Detecção de Movimento de Mãos em Tempo Real ===");

    let mut cam;
    let mut is_camera = true; // Flag para saber se estamos usando câmera ou vídeo

    // Primeiro tenta abrir a webcam
    println!("Tentando abrir a câmera...");
    cam = VideoCapture::new(0, CAP_ANY)?;

    if !cam.is_opened()? {
        // Se a câmera falhar, tenta carregar um vídeo de exemplo
        println!("Câmera não encontrada. Carregando vídeo de exemplo...");

        // Tenta diferentes caminhos de vídeo
        let video_paths = [
            "hand_video.mp4",        // Na raiz do projeto
            "videos/hand_video.mp4", // Em subpasta
            "test_video.mp4",        // Nome alternativo
        ];

        let mut video_loaded = false;
        for video_path in &video_paths {
            println!("Tentando: {}", video_path);
            cam = VideoCapture::from_file(video_path, CAP_ANY)?;
            if cam.is_opened()? {
                video_loaded = true;
                is_camera = false;
                println!("✓ Vídeo carregado: {}", video_path);
                break;
            }
        }

        if !video_loaded {
            println!("ERRO: Não foi possível abrir câmera nem vídeo!");
            println!("Coloque um arquivo de vídeo (ex: hand_video.mp4) na pasta do projeto.");
            println!("Ou conecte uma webcam e verifique as permissões.");
            return Ok(());
        }
    } else {
        println!("✓ Câmera iniciada com sucesso");
    }

    println!("Controles:");
    println!("  - ESPAÇO: Calibrar fundo");
    println!("  - S: Salvar frame atual");
    println!("  - ESC: Sair");
    println!();

    // Criar janelas
    named_window("Camera", WINDOW_AUTOSIZE)?;
    named_window("Movimento", WINDOW_AUTOSIZE)?;

    // Variáveis para processamento
    let mut frame = Mat::default();
    let mut prev_frame = Mat::default();
    let mut background = Mat::default();
    let mut motion_mask = Mat::default();
    let mut first_frame = true;
    let mut frame_count = 0;
    let mut last_save = Instant::now();
    let mut _hand_detected_frames = 0;

    // Parâmetros de detecção
    let motion_threshold = 25.0; // Sensibilidade ao movimento
    let min_contour_area = 500.0; // Área mínima para detectar mão
    let max_contour_area = 50000.0; // Área máxima (para filtrar objetos grandes)

    loop {
        // Capturar frame
        cam.read(&mut frame)?;

        if frame.empty() {
            if !is_camera {
                // Se for vídeo e chegar ao fim, reinicia
                println!("Fim do vídeo. Reiniciando...");
                cam.set(CAP_PROP_POS_FRAMES, 0.0)?;
                cam.read(&mut frame)?;

                if frame.empty() {
                    println!("ERRO: Não foi possível reiniciar o vídeo.");
                    break;
                }
            } else {
                // Se for câmera e frame vazio, continua tentando
                println!("Aviso: Frame vazio da câmera. Continuando...");
                continue;
            }
        }

        // Restante do processamento permanece igual...
        frame_count += 1;

        // Converter para escala de cinza (lembre-se do parâmetro AlgorithmHint!)
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

        // Calcular diferença entre frames (absdiff está em core, não imgproc)
        core::absdiff(&gray, &prev_frame, &mut motion_mask)?;

        // Aplicar threshold para binarizar
        let mut motion_bin = Mat::default();
        imgproc::threshold(
            &motion_mask,
            &mut motion_bin,
            motion_threshold,
            255.0,
            THRESH_BINARY,
        )?;
        motion_mask = motion_bin;

        // Encontrar contornos
        let mut contours = Vector::<Vector<Point>>::new();
        imgproc::find_contours(
            &motion_mask,
            &mut contours,
            RETR_EXTERNAL,
            CHAIN_APPROX_SIMPLE,
            Point::new(0, 0),
        )?;

        // Desenhar retângulos ao redor dos contornos (potenciais mãos)
        let mut hand_detected = false;
        let mut largest_contour_area = 0.0;
        let mut hand_rect = Rect::new(0, 0, 0, 0);

        for contour in contours {
            let contour_area = imgproc::contour_area(&contour, false)?;

            // Filtrar por tamanho (típico para mãos)
            if contour_area > min_contour_area && contour_area < max_contour_area {
                let rect = imgproc::bounding_rect(&contour)?;

                // Verificar proporção (mãos geralmente são mais altas que largas)
                let aspect_ratio = rect.width as f64 / rect.height as f64;
                if aspect_ratio > 0.5 && aspect_ratio < 2.0 {
                    // Desenhar retângulo verde ao redor da mão detectada
                    imgproc::rectangle(
                        &mut frame,
                        rect,
                        Scalar::new(0.0, 255.0, 0.0, 0.0),
                        2,
                        LINE_8,
                        0,
                    )?;

                    // Adicionar label
                    let label = format!("Mao ({:.0})", contour_area);
                    imgproc::put_text(
                        &mut frame,
                        &label,
                        Point::new(rect.x, rect.y - 10),
                        FONT_HERSHEY_SIMPLEX,
                        0.6,
                        Scalar::new(0.0, 255.0, 0.0, 0.0),
                        2,
                        LINE_8,
                        false,
                    )?;

                    hand_detected = true;
                    _hand_detected_frames += 1;

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
            imgproc::circle(
                &mut frame,
                Point::new(center_x, center_y),
                5,
                Scalar::new(0.0, 0.0, 255.0, 0.0),
                FILLED,
                LINE_8,
                0,
            )?;

            // Desenhar cruz no centro
            imgproc::line(
                &mut frame,
                Point::new(center_x - 15, center_y),
                Point::new(center_x + 15, center_y),
                Scalar::new(0.0, 0.0, 255.0, 0.0),
                2,
                LINE_8,
                0,
            )?;
            imgproc::line(
                &mut frame,
                Point::new(center_x, center_y - 15),
                Point::new(center_x, center_y + 15),
                Scalar::new(0.0, 0.0, 255.0, 0.0),
                2,
                LINE_8,
                0,
            )?;

            // Indicador visual de mão detectada
            let frame_cols = frame.cols(); // <-- Capture o valor AQUI
            imgproc::circle(
                &mut frame,
                Point::new(frame_cols - 30, 30), // <-- Use a variável capturada
                15,
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                FILLED,
                LINE_8,
                0,
            )?;
        } else {
            // Indicador visual de sem mão detectada
            let frame_cols = frame.cols(); // <-- Capture o valor AQUI
            imgproc::circle(
                &mut frame,
                Point::new(frame_cols - 30, 30), // <-- Use a variável capturada
                15,
                Scalar::new(0.0, 0.0, 255.0, 0.0),
                FILLED,
                LINE_8,
                0,
            )?;
        }

        // Mostrar informações na tela
        let info_text = format!(
            "Frame: {} | Mao detectada: {} ({:.0} pixels)",
            frame_count, hand_detected, largest_contour_area
        );
        imgproc::put_text(
            &mut frame,
            &info_text,
            Point::new(10, 30),
            FONT_HERSHEY_SIMPLEX,
            0.7,
            Scalar::new(255.0, 255.0, 255.0, 0.0),
            2,
            LINE_8,
            false,
        )?;

        let controls_text = "ESPACO: Calibrar | S: Salvar | ESC: Sair";
        let frame_rows = frame.rows(); // <-- Capture o valor AQUI
        imgproc::put_text(
            &mut frame,
            controls_text,
            Point::new(10, frame_rows - 10), // <-- Use a variável capturada
            FONT_HERSHEY_SIMPLEX,
            0.6,
            Scalar::new(255.0, 255.0, 255.0, 0.0),
            2,
            LINE_8,
            false,
        )?;

        // Mostrar imagens
        imshow("Camera", &frame)?;
        imshow("Movimento", &motion_mask)?;

        // Salvar automaticamente quando detectar mão (a cada 5 segundos)
        if hand_detected && last_save.elapsed() > Duration::from_secs(5) {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            let filename = format!(
                "hand_detected_{}_{}.png",
                timestamp, largest_contour_area as u32
            );
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
            32 => {
                // ESPAÇO
                println!("📸 Calibrando fundo...");
                gray.copy_to(&mut background)?;
                println!("✓ Fundo calibrado!");
            }
            115 => {
                // S
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
