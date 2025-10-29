use opencv::{
    prelude::*,
    videoio,
    highgui,
};

fn main() -> opencv::Result<()> { // Retorna um Result para lidar com erros
    // 1. Inicializar a captura da webcam
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)
        .expect("Não foi possível abrir a câmera"); 

    // 2. Verificar se a câmera abriu
    let opened = videoio::VideoCapture::is_opened(&cam)
        .expect("Falha ao verificar se a câmera estava aberta");

    if !opened {
        panic!("Câmera não pode ser aberta");
    }

    // 3. Criar uma janela para exibir
    highgui::named_window("Webcam", highgui::WINDOW_NORMAL)?;

    Ok(())
}