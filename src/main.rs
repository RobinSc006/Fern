extern crate image;
extern crate piston_window;

use image::*;
use piston_window::*;

use rand::distributions::{Distribution, Uniform};

fn main() {
    const WINDOW_WIDTH: u32 = 1050;
    const WINDOW_HEIGHT: u32 = 1050;

    const CLEAR_COLOR: [f32; 4] = [0.89; 4];
    //const FERN_COLOR: [f32; 4] = [0.05, 0.79, 0.1, 1.0];
    const FERN_COLOR: [u8; 4] = [40, 150, 114, 255];
    const FERN_COLOR_BACKGROUND: [u8; 4] = [54, 69, 71, 255];

    /*
     * Window setup
     */
    let mut window: PistonWindow = WindowSettings::new("Fern", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    /*
     * Fern render setup
     */
    let mut texture_settings: TextureSettings = TextureSettings::new();
    let mut fern_image = gen_fern(
        650000,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        FERN_COLOR,
        FERN_COLOR_BACKGROUND,
        false,
    );
    let fern_texture: G2dTexture = Texture::from_image(
        &mut window.create_texture_context(),
        &fern_image,
        &texture_settings,
    )
    .unwrap();

    /*
    * Main loop
    */
    while let Some(event) = window.next() {
        /*
        * Rendering
        */
        window.draw_2d(&event, |context, graphics, _device| {
            clear(CLEAR_COLOR, graphics);

            image(&fern_texture, context.transform, graphics);
        });
    }
}

fn gen_fern(
    iterations: u64,
    width: u32,
    height: u32,
    color: [u8; 4],
    background_color: [u8; 4],
    save_file: bool,
) -> RgbaImage {
    /*
     * Plot values setup
     */
    let mut img: RgbaImage = ImageBuffer::new(width, height);

    let mut plot_x: f64 = 0.0;
    let mut plot_y: f64 = 0.0;

    let mut cur_x: f64 = 0.0;
    let mut cur_y: f64 = 0.0;

    let mut next_x: f64 = 0.0;
    let mut next_y: f64 = 0.0;

    let mut fractal_stage: u8 = 0;

    let mut render_pixel_coords: Vec<[u32; 2]> = Vec::new();

    /*
     * Random setup
     */
    let mut random_gen = rand::thread_rng();
    let random_range = Uniform::from(1..1000000000);

    let mut random_num: f32 = 0.0;

    /*
     * Image background set
     */
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba(background_color);
    }

    /*
     * Generation
     */
    for i in 0..iterations {
        /*
         * Random number sample
         */
        random_num = (random_range.sample(&mut random_gen) as f32) * 0.000000001;

        /*
         * Magic
         */
        if random_num < 0.01 {
            next_x = 0.0;
            next_y = 0.16 * cur_y;
            fractal_stage = 0;
        } else if random_num < 0.86 {
            next_x = 0.85 * cur_x + 0.04 * cur_y;
            next_y = -0.04 * cur_x + 0.85 * cur_y + 1.6;
            fractal_stage = 1;
        } else if random_num < 0.93 {
            next_x = 0.20 * cur_x - 0.26 * cur_y;
            next_y = 0.23 * cur_x + 0.22 * cur_y + 1.6;
            fractal_stage = 2;
        } else {
            next_x = -0.15 * cur_x + 0.28 * cur_y;
            next_y = 0.26 * cur_x + 0.24 * cur_y + 0.44;
            fractal_stage = 3;
        }

        /*
         * Scaling, assignment
         */
        plot_x = width as f64 * (cur_x + 3.0) / 6.0;
        plot_y = height as f64 - height as f64 * ((cur_y + 2.0) / 14.0);

        cur_x = next_x;
        cur_y = next_y;

        /*
         * Save targeted pixel coords
         */
        render_pixel_coords.push([plot_x as u32, plot_y as u32]);
    }

    /*
     * Render targeted pixels
     */
    for pix_coord in render_pixel_coords.iter() {
        img.put_pixel(pix_coord[0], pix_coord[1], image::Rgba(color));
    }

    /*
     * Save copy of image
     */
    if save_file {
        img.save(format!("fern{}x{}_iter_{}.png", width, height, iterations))
            .unwrap();
    }

    return img;
}
