use image::{GenericImage, RgbImage};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

// grid
fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
    let width = img.width();
    let height = img.height();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let shift = frame_number as u32;
        let r = (((x + shift) % 256) as u8);
        let g = (((y + shift) % 256) as u8);
        let b = 128;
        *pixel = image::Rgb([r, g, b]);
    }
}

// bouncing ball
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     use image::Pixel;

//     let width = img.width() as i32;
//     let height = img.height() as i32;

//     // Ball properties
//     let ball_radius = 100;
//     let speed = 15;

//     // Calculate ball position
//     let mut x = (frame_number as i32 * speed) % (2 * width);
//     let mut y = (frame_number as i32 * speed) % (2 * height);

//     if x >= width {
//         x = 2 * width - x;
//     }
//     if y >= height {
//         y = 2 * height - y;
//     }

//     // Clear the image (black background)
//     for pixel in img.pixels_mut() {
//         *pixel = image::Rgb([0, 0, 0]);
//     }

//     // Draw the ball (simple circle)
//     for (img_x, img_y, pixel) in img.enumerate_pixels_mut() {
//         let dx = img_x as i32 - x;
//         let dy = img_y as i32 - y;
//         if dx * dx + dy * dy <= ball_radius * ball_radius {
//             *pixel = image::Rgb([255, 0, 0]); // Red ball
//         }
//     }
// }

// rainbow
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     use palette::{FromColor, Hsv, Srgb};

//     let hue_shift = (frame_number as f32) % 360.0;

//     for (x, y, pixel) in img.enumerate_pixels_mut() {
//         let hue = ((x + y) as f32 + hue_shift) % 360.0;
//         let hsv = Hsv::new(hue, 1.0, 1.0);

//         // Convert Hsv to Srgb
//         let rgb: Srgb = Srgb::from_color(hsv);

//         // Convert to u8 format
//         let rgb_u8 = rgb.into_format::<u8>();

//         // Extract the RGB components
//         let (r, g, b) = rgb_u8.into_components();

//         // Set the pixel value
//         *pixel = image::Rgb([r, g, b]);
//     }
// }

// Mandelbrot
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     let width = img.width() as f64;
//     let height = img.height() as f64;

//     // Zoom parameters
//     let zoom = 0.01 + frame_number as f64 * 0.0005;
//     let move_x = -0.5;
//     let move_y = 0.0;
//     let max_iter = 256;

//     for (x, y, pixel) in img.enumerate_pixels_mut() {
//         let x0 = (x as f64 / width - 0.5) / zoom + move_x;
//         let y0 = (y as f64 / height - 0.5) / zoom + move_y;

//         let mut a = 0.0;
//         let mut b = 0.0;
//         let mut iter = 0;

//         while a * a + b * b <= 4.0 && iter < max_iter {
//             let temp = a * a - b * b + x0;
//             b = 2.0 * a * b + y0;
//             a = temp;
//             iter += 1;
//         }

//         let color = if iter < max_iter {
//             let c = (iter * 255 / max_iter) as u8;
//             [c, c, c]
//         } else {
//             [0, 0, 0]
//         };
//         *pixel = image::Rgb(color);
//     }
// }

// rotating square
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     use image::Pixel;
//     use std::f32::consts::PI;

//     let width = img.width() as i32;
//     let height = img.height() as i32;

//     // Square properties
//     let square_size = 200.0;
//     let angle = (frame_number as f32 * 0.05) % (2.0 * PI);

//     // Center of the image
//     let cx = width as f32 / 2.0;
//     let cy = height as f32 / 2.0;

//     // Clear the image (black background)
//     for pixel in img.pixels_mut() {
//         *pixel = image::Rgb([0, 0, 0]);
//     }

//     // Draw the rotating square
//     for (img_x, img_y, pixel) in img.enumerate_pixels_mut() {
//         let x = img_x as f32 - cx;
//         let y = img_y as f32 - cy;

//         // Rotate coordinates
//         let x_rot = x * angle.cos() - y * angle.sin();
//         let y_rot = x * angle.sin() + y * angle.cos();

//         if x_rot.abs() < square_size / 2.0 && y_rot.abs() < square_size / 2.0 {
//             *pixel = image::Rgb([0, 255, 0]); // Green square
//         }
//     }
// }

// Image slideshow
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     use image::{GenericImageView, Pixel};

//     // List of image paths
//     let images = [
//         "c:\\Users\\Brioche Elm\\Pictures\\2023-10-21_23-03.png",
//         "c:\\Users\\Brioche Elm\\Pictures\\blinding_lights.jpg",
//         "c:\\Users\\Brioche Elm\\Pictures\\crown.jpg",
//     ];
//     let frames_per_image = 60; // Display each image for 2 seconds at 30 fps

//     // Determine which image to display
//     let image_index = (frame_number / frames_per_image) % images.len();
//     let image_path = images[image_index];

//     // Load the image
//     let loaded_img = image::open(image_path).expect("Failed to load image");

//     // Resize the image to match the frame size
//     let resized = loaded_img.resize_exact(
//         img.width(),
//         img.height(),
//         image::imageops::FilterType::Lanczos3,
//     );

//     // Convert the resized image to an RgbImage
//     let rgb_image = resized.to_rgb8();

//     // Copy the pixels into the frame buffer
//     img.copy_from(&rgb_image, 0, 0)
//         .expect("Failed to copy image");
// }

struct VideoRenderer {
    width: u32,
    height: u32,
    framerate: u32,
    frame_count: u32,
    ffmpeg: std::process::Child,
    // stdin: std::process::ChildStdin,
}

impl VideoRenderer {
    fn new(width: u32, height: u32, framerate: u32, output_file_path: &Path) -> Self {
        // Start the FFmpeg process
        let ffmpeg = Command::new("ffmpeg")
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
                "-",                                // Read input from stdin
                output_file_path.to_str().unwrap(), // Output file
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to start FFmpeg process");

        Self {
            width,
            height,
            framerate,
            frame_count: 0,
            ffmpeg,
        }
    }

    fn append_frame(&mut self, img: RgbImage) -> std::io::Result<u32> {
        let stdin = self
            .ffmpeg
            .stdin
            .as_mut()
            .expect("Failed to open FFmpeg stdin");

        // Convert the image to raw RGB data
        let raw_data = img.into_raw();

        // Write the raw data to FFmpeg's stdin
        stdin.write_all(&raw_data)?;

        self.frame_count += 1;

        Ok(self.frame_count)
    }

    fn finish(mut self) -> std::io::Result<()> {
        // Close the stdin to signal FFmpeg that we're done sending data
        drop(self.ffmpeg.stdin.take());

        // Wait for FFmpeg to finish processing
        self.ffmpeg.wait()?;

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    // Video parameters
    let width = 1920;
    let height = 1080;
    let framerate = 60;
    let total_frames = 900; // For a 10-second video at 30 fps
                            // Initialize the VideoRenderer

    // path from str
    let out_video_path = Path::new("output.mp4");
    let mut renderer = VideoRenderer::new(width, height, framerate, out_video_path);

    // Append frames
    for frame_number in 0..total_frames {
        let mut img = RgbImage::new(width, height);
        generate_frame_content(&mut img, frame_number.try_into().unwrap());
        let _frames_encoded = renderer.append_frame(img)?;
    }

    // Finalize the video rendering
    renderer.finish()?;

    Ok(())
}
