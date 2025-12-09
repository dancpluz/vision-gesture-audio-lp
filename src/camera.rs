use opencv::{
    core::Mat,
    prelude::{MatTraitConst, VideoCaptureTrait, VideoCaptureTraitConst},
    videoio::{CAP_ANY, CAP_PROP_POS_FRAMES, VideoCapture},
};
use std::error::Error;

use crate::config::VIDEO_PATHS;

/// Inicializa a captura de vídeo (câmera ou arquivo)
pub fn initialize_capture() -> Result<(VideoCapture, bool), Box<dyn Error>> {
    // Tentar abrir webcam
    println!("📷 Tentando abrir a câmera...");
    let mut cam = VideoCapture::new(0, CAP_ANY)?;
    
    if cam.is_opened()? {
        println!("✅ Câmera iniciada com sucesso");
        return Ok((cam, true));
    }
    
    // Fallback para vídeo
    println!("📽️  Câmera não encontrada. Carregando vídeo de exemplo...");
    
    for video_path in VIDEO_PATHS {
        println!("🔍 Tentando: {}", video_path);
        cam = VideoCapture::from_file(video_path, CAP_ANY)?;
        if cam.is_opened()? {
            println!("✅ Vídeo carregado: {}", video_path);
            return Ok((cam, false));
        }
    }
    
    println!("❌ ERRO: Não foi possível abrir câmera nem vídeo!");
    println!("💡 Dica: Coloque um arquivo de vídeo (ex: video.mp4) na pasta do projeto.");
    println!("💡 Ou conecte uma webcam e verifique as permissões.");
    
    Err("Nenhuma fonte de vídeo disponível".into())
}

/// Lê um frame da captura de vídeo
pub fn read_frame(
    cam: &mut VideoCapture,
    frame: &mut Mat,
    is_camera: bool,
) -> Result<bool, Box<dyn Error>> {
    cam.read(frame)?;
    
    if frame.empty() {
        if !is_camera {
            // Reiniciar vídeo
            println!("🔄 Fim do vídeo. Reiniciando...");
            cam.set(CAP_PROP_POS_FRAMES, 0.0)?;
            cam.read(frame)?;
            
            if frame.empty() {
                println!("❌ Não foi possível reiniciar o vídeo.");
                return Ok(false);
            }
        }
    }
    
    Ok(true)
}

pub fn release_capture(cam: &mut VideoCapture) -> Result<(), Box<dyn Error>> {
    cam.release()?;
    println!("📷 Recurso da câmera liberado");
    Ok(())
}