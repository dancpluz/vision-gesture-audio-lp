mod audio;
mod camera;
mod config;

use audio::AudioPlayer;
use opencv::core::Mat;
use opencv::highgui::{WINDOW_NORMAL, imshow, named_window, wait_key};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Vision Gesture Audio Project ===");
    println!("Controls:");
    println!("  ESC     - Exit");
    println!("  SPACE   - Start/Stop audio");
    println!("  UP/DOWN - Increase/Decrease pitch");
    println!("  R       - Reset pitch to normal");
    println!("  P       - Play short sound effect");
    println!();

    // Initialize audio system
    let audio_player = AudioPlayer::new()?;
    println!("✓ Audio system initialized");
    
    // Try to load default audio
    let audio_file = config::DEFAULT_AUDIO_FILE;
    println!("Trying to load audio: {}", audio_file);
    
    // Initialize camera or video
    let (mut cam, is_camera) = camera::initialize_capture()?;

    // Create window
    named_window("Video", WINDOW_NORMAL)?;

    println!("▶️  Starting capture...");
    println!("Current pitch: {:.2}", audio_player.get_pitch());
    println!();

    let mut audio_playing = false;

    // Main loop
    loop {
        // Capture frame
        let mut frame = Mat::default();
        if !camera::read_frame(&mut cam, &mut frame, is_camera)? {
            break;
        }

        // Show frame
        imshow("Video", &frame)?;

        // Check for key presses
        let key = wait_key(30)?;
        
        match key {
            27 => { // ESC - Exit
                println!("\n✅ Encerrando...");
                audio_player.stop();
                break;
            }
            32 => { // SPACE - Toggle audio playback
                if audio_playing {
                    audio_player.stop();
                    audio_playing = false;
                    println!("Audio stopped");
                } else {
                    match audio_player.play_file(audio_file) {
                        Ok(_) => {
                            audio_playing = true;
                            println!("Audio started with pitch: {:.2}", audio_player.get_pitch());
                        }
                        Err(e) => {
                            println!("Failed to play audio: {}", e);
                            println!("Please add an audio file (e.g., audio.mp3) to the project folder");
                        }
                    }
                }
            }
            112 => { // 'p' - Play short sound effect
                match audio_player.play_sound_once(audio_file) {
                    Ok(_) => println!("Played sound effect"),
                    Err(_) => println!("No audio file found for sound effect"),
                }
            }
            114 => { // 'r' - Reset pitch
                audio_player.set_pitch(config::DEFAULT_PITCH);
                println!("Pitch reset to: {:.2}", audio_player.get_pitch());
                
                // If audio is playing, restart with new pitch
                if audio_playing {
                    audio_player.stop();
                    if let Ok(_) = audio_player.play_file(audio_file) {
                        println!("Audio restarted with new pitch");
                    }
                }
            }
            82 | 83 => { // Arrow Up (82) / Down (83) in some systems
                // Alternative key codes for arrow keys
                let current_pitch = audio_player.get_pitch();
                let new_pitch = if key == 82 { // Up
                    (current_pitch * 1.1).clamp(config::MIN_PITCH, config::MAX_PITCH)
                } else { // Down
                    (current_pitch * 0.9).clamp(config::MIN_PITCH, config::MAX_PITCH)
                };
                
                if (new_pitch - current_pitch).abs() > 0.01 {
                    audio_player.set_pitch(new_pitch);
                    println!("Pitch changed to: {:.2}", new_pitch);
                    
                    // If audio is playing, restart with new pitch
                    if audio_playing {
                        audio_player.stop();
                        if let Ok(_) = audio_player.play_file(audio_file) {
                            println!("Audio restarted with new pitch");
                        }
                    }
                }
            }
            0 => { // No key pressed
                // Continue
            }
            _ => {
                // You can add more key controls here
                // println!("Key pressed: {}", key);
            }
        }
    }

    println!("Program finished!");
    Ok(())
}