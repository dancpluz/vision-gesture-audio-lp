//! Controller para gerenciamento de operações de áudio

use crate::audio::AudioPlayer;
use crate::audio_commands::get_pitch_factor;
use crate::config;

/// Controlador de operações de áudio
pub struct AudioController {
    audio_player: AudioPlayer,
    audio_file: String,
    is_audio_playing: bool,
}

impl AudioController {
    /// Cria um novo controlador de áudio
    pub fn new(audio_player: AudioPlayer, audio_file: String) -> Self {
        AudioController {
            audio_player,
            audio_file,
            is_audio_playing: false,
        }
    }

    /// Ajusta o pitch e reinicia o áudio se estiver tocando
    pub fn adjust_pitch_with_restart(
        &mut self,
        new_pitch: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_player.set_pitch(new_pitch);

        if self.is_audio_playing {
            self.audio_player.stop();
            self.audio_player.play_file(&self.audio_file)?;
        }

        Ok(())
    }

    /// Processa comando de ajuste de pitch baseado na tecla
    pub fn process_pitch_adjustment(&mut self, key: i32) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(factor) = get_pitch_factor(key) {
            let new_pitch = (self.audio_player.get_pitch() * factor)
                .clamp(config::MIN_PITCH, config::MAX_PITCH);

            let current_pitch = self.audio_player.get_pitch();
            if (new_pitch - current_pitch).abs() > 0.001 {
                let message = if factor > 1.0 {
                    "↑ Pitch aumentado"
                } else {
                    "↓ Pitch diminuído"
                };

                println!("{} para: {:.2}", message, new_pitch);

                self.adjust_pitch_with_restart(new_pitch)?;
            }
        }

        Ok(())
    }

    /// Alterna entre reproduzir e parar o áudio
    pub fn toggle_audio_playback(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_audio_playing {
            self.audio_player.stop();
            println!("⏹️  Áudio parado");
            self.is_audio_playing = false;
        } else {
            self.audio_player.play_file(&self.audio_file)?;
            println!(
                "▶️  Áudio iniciado (pitch: {:.2})",
                self.audio_player.get_pitch()
            );
            self.is_audio_playing = true;
        }

        Ok(())
    }

    /// Reseta o pitch para o valor padrão
    pub fn reset_pitch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_player.set_pitch(config::DEFAULT_PITCH);
        println!("🔄 Pitch resetado para 1.0");

        if self.is_audio_playing {
            self.audio_player.stop();
            self.audio_player.play_file(&self.audio_file)?;
            println!("🔁 Áudio reiniciado com pitch normal");
        }

        Ok(())
    }

    /// Para toda reprodução de áudio
    pub fn stop(&mut self) {
        self.audio_player.stop();
        self.is_audio_playing = false;
    }

    /// Obtém o pitch atual
    pub fn get_current_pitch(&self) -> f32 {
        self.audio_player.get_pitch()
    }
}
