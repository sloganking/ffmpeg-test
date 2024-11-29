use image::RgbImage;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    // Video parameters
    let width = 640;
    let height = 480;
    let framerate = 30;
    let total_frames = 300; // For a 10-second video at 30 fps

    // Start the FFmpeg process
    let mut ffmpeg = Command::new("ffmpeg")
        .args(&[
            "-y", // Overwrite output files without asking
            "-f",
            "rawvideo",
            "-pixel_format",
            "rgb24",
            "-video_size",
            &format!("{}x{}", width, height),
            "-framerate",
            &framerate.to_string(),
            "-i",
            "-",          // Read input from stdin
            "output.mp4", // Output file
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start FFmpeg process");

    {
        // Obtain a handle to FFmpeg's stdin
        let stdin = ffmpeg.stdin.as_mut().expect("Failed to open FFmpeg stdin");

        // Generate and send frames to FFmpeg
        for frame_number in 0..total_frames {
            // Create a new RGB image
            let mut img = RgbImage::new(width, height);

            // Generate frame content (customize this part as needed)
            for (x, y, pixel) in img.enumerate_pixels_mut() {
                let r = ((x + frame_number) % 256) as u8;
                let g = ((y + frame_number) % 256) as u8;
                let b = ((frame_number) % 256) as u8;
                *pixel = image::Rgb([r, g, b]);
            }

            // Convert the image to raw RGB data
            let raw_data = img.into_raw();

            // Write the raw data to FFmpeg's stdin
            stdin.write_all(&raw_data)?;
        }
    } // Closing the stdin to signal FFmpeg that we're done sending data

    // Wait for FFmpeg to finish processing
    ffmpeg.wait()?;

    Ok(())
}
