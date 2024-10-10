mod ascii_processor;

use std::{thread::sleep, time};

use ascii_processor::AsciiProcessor;
use opencv::videoio::{self, VideoCaptureTrait};

fn main() {
    // Set up video capture
    let mut cam =
        videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Failed to get video capture");

    // Initialize ASCII processor with desired target width/height
    let mut ascii_processor = AsciiProcessor::new(200, 90);

    // Infinite loop for frame capture and processing
    loop {
        // Capture a frame from the webcam
        cam.read(&mut ascii_processor.frame)
            .expect("Failed to capture frame");

        // Process and print the frame as ASCII art
        ascii_processor.process();

        // Adjust frame delay to balance speed (adjust as necessary)
        let frame_delay = time::Duration::from_millis(33); // Roughly 30 FPS
        sleep(frame_delay);
    }
}
