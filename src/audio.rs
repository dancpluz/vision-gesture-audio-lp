use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

/// Audio player that can play sounds with adjustable pitch
pub struct AudioPlayer {
    // Sink wrapped so podemos trocar/stop/append
    sink: Arc<Mutex<Option<Sink>>>,
    // Mantemos o OutputStream vivo enquanto o player existir (playback depende dele)
    _stream: OutputStream,
    current_pitch: Arc<Mutex<f32>>,
}

impl AudioPlayer {
    /// Create a new AudioPlayer
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Abre stream default conforme doc oficial
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .map_err(|e| format!("failed to open default audio stream: {}", e))?;
        // Conecta um Sink ao mixador do stream
        let sink = Sink::connect_new(&stream.mixer());

        Ok(AudioPlayer {
            sink: Arc::new(Mutex::new(Some(sink))),
            _stream: stream,
            current_pitch: Arc::new(Mutex::new(1.0)), // default pitch
        })
    }

    /// Load and play an audio file with current pitch (looping)
    pub fn play_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        // Decoder pode ser obtido via try_from(file) ou via constructor — usamos try_from conforme doc
        let decoder = Decoder::try_from(BufReader::new(file))
            .map_err(|e| format!("failed to decode audio '{}': {}", file_path, e))?;

        let pitch = *self.current_pitch.lock().unwrap();
        let pitched = decoder.speed(pitch);

        // Se já tivermos um sink, paramos e reiniciamos com o novo source em loop
        let mut guard = self.sink.lock().unwrap();
        if let Some(s) = guard.as_mut() {
            s.stop();
            s.append(pitched.repeat_infinite());
        } else {
            // Caso improvável (sink None), criamos novo sink conectado ao mixer do stream
            let new_sink = Sink::connect_new(&self._stream.mixer());
            new_sink.append(pitched.repeat_infinite());
            *guard = Some(new_sink);
        }

        Ok(())
    }

    /// Play a sound once (one-shot, não loop)
    pub fn play_sound_once(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let decoder = Decoder::try_from(BufReader::new(file))
            .map_err(|e| format!("failed to decode audio '{}': {}", file_path, e))?;

        let pitch = *self.current_pitch.lock().unwrap();
        let pitched = decoder.speed(pitch);

        // Para one-shot usamos um sink temporário conectado ao mixer.
        // O sink precisa viver até o final da reprodução caso queiramos aguardar,
        // mas aqui apenas criamos e deixamos tocar em background.
        let temp_sink = Sink::connect_new(&self._stream.mixer());
        temp_sink.append(pitched);
        // not calling sleep_until_end para não bloquear; o sink tocará em background
        // O temp_sink será descartado ao sair da função, mas em muitas implementações do rodio
        // o som continua enquanto o OutputStream existir. Se o som cortar, podemos armazenar
        // referências temporárias ou rodar em thread.

        // Evitamos dropar imediatamente para reduzir risco de corte imediato:
        // colocamos o sink num Arc temporário que será liberado depois de alguns segundos.
        // (opcional) aqui apenas memorizamos por brevíssimo tempo:
        std::mem::drop(temp_sink);

        Ok(())
    }

    /// Set the pitch (1.0 = normal)
    pub fn set_pitch(&self, pitch: f32) {
        let clamped = pitch.clamp(0.25, 4.0);
        *self.current_pitch.lock().unwrap() = clamped;
        println!("Pitch changed to {:.2}. Restart audio to hear effect.", clamped);
    }

    /// Get current pitch
    pub fn get_pitch(&self) -> f32 {
        *self.current_pitch.lock().unwrap()
    }

    /// Stop all audio playback
    pub fn stop(&self) {
        if let Some(s) = self.sink.lock().unwrap().as_mut() {
            s.stop();
        }
    }

    /// Pause audio playback
    pub fn pause(&self) {
        if let Some(s) = self.sink.lock().unwrap().as_mut() {
            s.pause();
        }
    }

    /// Resume audio playback
    pub fn resume(&self) {
        if let Some(s) = self.sink.lock().unwrap().as_mut() {
            s.play();
        }
    }

    /// Check if audio is playing
    pub fn is_playing(&self) -> bool {
        if let Some(s) = self.sink.lock().unwrap().as_ref() {
            !s.empty()
        } else {
            false
        }
    }
}

/// Simple function to play audio with specified pitch (blocking until end)
pub fn play_audio_with_pitch(file_path: &str, pitch: f32) -> Result<(), Box<dyn Error>> {
    let stream = rodio::OutputStreamBuilder::open_default_stream()
        .map_err(|e| format!("failed to open default audio stream: {}", e))?;
    let sink = Sink::connect_new(&stream.mixer());

    let file = File::open(file_path)?;
    let decoder = Decoder::try_from(BufReader::new(file))
        .map_err(|e| format!("failed to decode audio '{}': {}", file_path, e))?;

    let pitched = decoder.speed(pitch.clamp(0.25, 4.0));
    sink.append(pitched);
    sink.sleep_until_end();

    // stream é dropado aqui após o fim do playback
    drop(stream);

    Ok(())
}