mod cornell_box;
mod image;
mod maths;
mod primitives;
mod worker;

use cornell_box::create_cornell_box;
use image::{save_canvas, Canvas};
use maths::{Matrix4x4, Point, Vector};
use minifb::{Key, Window, WindowOptions};
use primitives::Camera;
use rand::{seq::SliceRandom, thread_rng};
use worker::{Worker, WorkerState};

fn resample_buffer(buffer: &mut Vec<u32>, canvas: &Canvas, screen_width: i32, screen_height: i32) {
    let downsample = canvas.downsample(screen_width, screen_height);

    for y in 0..downsample.height() {
        for x in 0..downsample.width() {
            let color = downsample.pixel_at(x, y);
            buffer[(x + y * downsample.width()) as usize] = color.to_u32();
        }
    }
}

fn main() {
    let world = create_cornell_box().generate();

    // quality 1 == 128 * 128
    // quality 4 == 1024 * 1024
    let quality_multiplier = 3;
    let (width, height) = (
        128 * 2i32.pow(quality_multiplier),
        128 * 2i32.pow(quality_multiplier),
    );

    let (screen_width, screen_height) = (512, 512);

    // Create camera
    let view_transform = Matrix4x4::view(
        Point::new(0.0, 2.8, -10.0),
        Point::new(0.0, 2.0, 0.0),
        Vector::up(),
    );
    let camera = Camera::new(width, height, std::f64::consts::FRAC_PI_4, view_transform);

    // Generate all the points to render
    let mut points: Vec<(i32, i32)> = (0..height)
        .map(|row| (0..width).map(move |col| (row, col)))
        .flatten()
        .collect();

    // Shuffle all the points
    let mut rng = thread_rng();
    points.shuffle(&mut rng);

    // Create the window
    let mut window = Window::new(
        "RTracer - ESC to exit",
        screen_width as usize,
        screen_height as usize,
        WindowOptions::default(),
    )
    .expect("Failed to create window");
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Start rendering all the colors in another thread
    let worker = Worker::new(
        points,
        move |(row, col)| {
            let ray = camera.ray_for_pixel(col, row);
            ((row, col), world.color_at(ray))
        },
        width as usize,
    );

    // Create the buffers where the pixels will go once rendered
    let mut buffer = vec![0u32; (screen_width * screen_height) as usize];
    let mut canvas = Canvas::new(width, height);

    let mut saved = false; // Make sure we only save once

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Pull the new pixels fr
        match worker.fetch() {
            WorkerState::Values(vals) => vals.into_iter().for_each(|((row, col), color)| {
                canvas.write_pixel(col, row, color);
            }),
            WorkerState::Complete => {
                if !saved {
                    // Update the window
                    window.set_title("RTracer - ESC to exit -- Finished");

                    // Save the buffer to a canvas
                    save_canvas(&canvas, "out.png".to_owned()).unwrap();
                    saved = true;
                }
            }
        }

        // Resample the buffer in the location which has now changed
        resample_buffer(&mut buffer, &canvas, screen_width, screen_height);

        window
            .update_with_buffer(&buffer, screen_width as usize, screen_height as usize)
            .expect("Failed to update the window with buffer");
    }

    worker.finish();
}
