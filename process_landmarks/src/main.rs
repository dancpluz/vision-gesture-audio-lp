use mediapipe_rs::tasks::vision::HandLandmarkerBuilder; // Check exact struct name

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It's common to load the model from a file included in the binary
    let model_path = "assets/models/hand_landmark_detection/hand_landmarker.task";
    
    // Create the hand landmark detector using the builder pattern
    let hand_landmarker = HandLandmarkerBuilder::new() // Check exact builder name
        .max_results(2) // Set the number of hands to detect
        .build_from_file(model_path)?;

    // Imagine this image data is received from another process
    let image_data: Vec<u8> = receive_image_data();
    let input_image = image::load_from_memory(&image_data)?;

    // Run inference on the image
    let detection_result = hand_landmarker.detect(&input_image)?; // Check exact function name

    // Now send the result (e.g., as JSON) back to the visualizer
    send_landmarks_to_visualizer(detection_result);
    
    Ok(())
}