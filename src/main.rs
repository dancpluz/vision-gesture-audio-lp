use opencv::{
    videoio,
    highgui,
    prelude::*,
    core::Mat,
    Result,
};
use std::io::{self, Write};

/// Scan available webcams by probing indices
fn find_available_cameras(max_index: i32) -> Result<Vec<i32>> {
    let mut available = Vec::new();

    for index in 0..=max_index {
        let mut cam = videoio::VideoCapture::new(index, videoio::CAP_ANY)?;

        if videoio::VideoCapture::is_opened(&cam)? {
            let mut frame = Mat::default();

            if cam.read(&mut frame)? && !frame.empty() {
                available.push(index);
            }
        }
    }

    Ok(available)
}

/// Select a camera index interactively
fn select_camera() -> Result<i32> {
    let cameras = find_available_cameras(10)?; // scan 0–10

    if cameras.is_empty() {
        panic!("No cameras detected!");
    }

    println!("Available cameras:");
    for cam in &cameras {
        println!("  - Camera {}", cam);
    }

    print!("Choose a camera index: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let chosen: i32 = input.trim().parse().unwrap();
    if !cameras.contains(&chosen) {
        panic!("Invalid camera index!");
    }

    Ok(chosen)
}

fn main() -> Result<()> {
    let cam_index = select_camera()?;
    println!("Opening camera {}...", cam_index);

    let mut cam = videoio::VideoCapture::new(cam_index, videoio::CAP_ANY)?;
    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Failed to open selected camera!");
    }

    let mut frame = Mat::default();

    highgui::named_window("Selected Camera", highgui::WINDOW_AUTOSIZE)?;

    loop {
        cam.read(&mut frame)?;
        if frame.empty() {
            continue;
        }

        highgui::imshow("Selected Camera", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 27 { break; } // ESC
    }

    Ok(())
}