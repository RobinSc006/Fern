extern crate find_folder;
extern crate image;
extern crate piston_window;

use image::*;
use piston_window::*;

use rand::distributions::{Distribution, Uniform};
use std::time::Instant;

fn main() {
    const WINDOW_WIDTH: u32 = 1050;
    const WINDOW_HEIGHT: u32 = 1050;

    const CLEAR_COLOR: [f32; 4] = [1.0; 4];

    const FERN_COLOR_OPT_ONE: [u8; 4] = [51, 153, 137, 255];
    const FERN_COLOR_OPT_TWO: [u8; 4] = [0, 108, 103, 255]; 
    const FERN_COLOR_OPT_THREE: [u8; 4] = [40, 150, 114, 255]; 

    const FERN_BACKGROUND_COLOR_OPT_ONE: [u8; 4] = [43, 44, 40, 255];
    const FERN_BACKGROUND_COLOR_OPT_TWO: [u8; 4] = [19, 21, 21, 255];
    const FERN_BACKGROUND_COLOR_OPT_THREE: [u8; 4] = [54, 69, 71, 255];

    const FONT_COLOR: [f32; 4] = [0.92, 0.92, 0.92, 1.0];

    /*
     * Window setup
     */
    let mut window: PistonWindow = WindowSettings::new("Fern", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    /*
     * Font loading
     */
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("font/RobotoCondensed-Light.ttf"))
        .unwrap();

    glyphs.preload_printable_ascii(15).unwrap();

    /*
     * Text variables setup
     */
    let mut average_frame_time_string: String;
    let mut max_frame_time_string: String;
    let mut iterated_count_text: String;

    /*
     * Fern values setup
     */
    let temp_fern_image: RgbaImage = ImageBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let temp_fern_coords: Vec<[u32; 2]> = Vec::new();
    let temp_fern_iterated_count: u128 = 0;

    let temp_fern_values: FernValues = FernValues {
        current_img: temp_fern_image,
        current_x: 0.0,
        current_y: 0.0,
        current_calculated_coords: temp_fern_coords,
    };
    let mut fern: Fern = Fern {
        values: temp_fern_values,
        background_drawn: false,
        iterated_count: temp_fern_iterated_count,
    };

    /*
     * Fern render setup
     */
    let window_texture_context = &mut window.create_texture_context();

    let fern_texture_settings: TextureSettings = TextureSettings::new();
    let mut fern_texture = fern.get_render_target(window_texture_context, fern_texture_settings);

    let mut current_fern_color: [u8; 4] = FERN_COLOR_OPT_ONE;
    let mut current_fern_background_color: [u8; 4] = FERN_BACKGROUND_COLOR_OPT_ONE;

    fern.gen_fern(
        100,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        current_fern_color,
        current_fern_background_color,
        false,
    );

    /*
     * Timing system setup
     */
    const TICK_TIME: u128 = 10;
    const MAX_SAVED_FRAME_TIMES: u64 = 30;

    let mut tick_clock = Instant::now();

    let mut time_stamp_frame_start: Instant;
    let mut _time_stamp_frame_end: Instant = Instant::now();

    let mut current_frame_time_us: u128 = 0;
    let mut last_frame_times_us: Vec<u128> = Vec::new();
    let mut average_frame_time: u128 = 0;
    let mut max_frame_time: u128 = 0;

    let mut generation_paused: bool = false;

    /*
     * Main loop
     */
    while let Some(event) = window.next() {
        time_stamp_frame_start = Instant::now();

        /*
         * Input
         */
        if let Some(button) = event.release_args() {
            match button {
                Button::Keyboard(key) => {
                    if key == Key::Space {
                        if generation_paused {
                            generation_paused = false;
                        } else {
                            generation_paused = true;
                        }
                    }
                    else if key == Key::R {
                        fern.reset();
                    }
                    else if key == Key::D1 {
                        fern.reset();
                        current_fern_color = FERN_COLOR_OPT_ONE;
                        current_fern_background_color = FERN_BACKGROUND_COLOR_OPT_ONE;
                    }
                    else if key == Key::D2 {
                        fern.reset();
                        current_fern_color = FERN_COLOR_OPT_TWO;
                        current_fern_background_color = FERN_BACKGROUND_COLOR_OPT_TWO;
                    }
                    else if key == Key::D3 {
                        fern.reset();
                        current_fern_color = FERN_COLOR_OPT_THREE;
                        current_fern_background_color = FERN_BACKGROUND_COLOR_OPT_THREE;
                    }
                }
                Button::Mouse(_) => {}
                Button::Controller(_) => {}
                Button::Hat(_) => {}
            }
        }

        /*
         * Fern update
         */
        if tick_clock.elapsed().as_millis() >= TICK_TIME && !generation_paused {
            fern.gen_fern(
                1000,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                current_fern_color,
                current_fern_background_color,
                false,
            );

            tick_clock = Instant::now();
        }

        /*
         * Max frame time calculation
         */
        if average_frame_time > 0
            && last_frame_times_us.len() > 1
            && last_frame_times_us[last_frame_times_us.len() - 1] < current_frame_time_us
            && current_frame_time_us > max_frame_time
        {
            max_frame_time = current_frame_time_us;
        }

        /*
         * Average frame time calculation
         */
        if last_frame_times_us.len() as u64 > MAX_SAVED_FRAME_TIMES {
            let mut combined_frame_times: u128 = 0;

            for time_ms in last_frame_times_us.iter() {
                combined_frame_times += time_ms
            }

            average_frame_time = combined_frame_times / MAX_SAVED_FRAME_TIMES as u128;

            last_frame_times_us.clear();
        } else {
            last_frame_times_us.push(current_frame_time_us);
        }

        /*
         * Text variables update
         */
        average_frame_time_string =
            "Average frame time: ".to_string() + &format!("{:.4}", (f64::from(average_frame_time as u32) * 0.001).to_string().to_owned()) + "ms";
        max_frame_time_string =
            "Max frame time: ".to_string() + &format!("{:.4}", (f64::from(max_frame_time as u32) * 0.001).to_string().to_owned()) + "ms";
        iterated_count_text =
            "Iterations: ".to_string() + &fern.iterated_count.to_string().to_owned();

        /*
         * Rendering
         */
        window.draw_2d(&event, |context, graphics, _device| {
            clear(CLEAR_COLOR, graphics);
            /*
             * Fern image render
             */
            fern_texture = fern.get_render_target(window_texture_context, fern_texture_settings);
            image(&fern_texture, context.transform, graphics);

            /*
             * Text render
             */
            glyphs.factory.encoder.flush(_device);

            /*
            ? Caption
            */
            text::Text::new_color(FONT_COLOR, 22)
                .draw(
                    "Barnsley Fern",
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(5.0, 23.0),
                    graphics,
                )
                .unwrap();

            /*
            ? Average frame time
            */
            text::Text::new_color(FONT_COLOR, 15)
                .draw(
                    &average_frame_time_string,
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(8.0, 55.0),
                    graphics,
                )
                .unwrap();

            /*
            ? Max frame time
            */
            text::Text::new_color(FONT_COLOR, 15)
                .draw(
                    &max_frame_time_string,
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(8.0, 80.0),
                    graphics,
                )
                .unwrap();

            /*
            ? Total iterations
            */
            text::Text::new_color(FONT_COLOR, 15)
                .draw(
                    &iterated_count_text,
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(8.0, 105.0),
                    graphics,
                )
                .unwrap();
        });

        /*
         * Frame time calculation
         */
        _time_stamp_frame_end = Instant::now();

        current_frame_time_us = _time_stamp_frame_end
            .saturating_duration_since(time_stamp_frame_start)
            .as_micros();
    }
}

struct Fern {
    pub values: FernValues,
    pub background_drawn: bool,
    pub iterated_count: u128,
}
impl Fern {
    pub fn gen_fern(
        &mut self,
        iterations: u64,
        width: u32,
        height: u32,
        color: [u8; 4],
        background_color: [u8; 4],
        save_file: bool,
    ) {
        /*
         * Plot values setup
         */
        let mut plot_x: f64;
        let mut plot_y: f64;

        let mut next_x: f64;
        let mut next_y: f64;

        self.values.current_calculated_coords.clear();

        /*
         * Random setup
         */
        let mut random_gen = rand::thread_rng();
        let random_range = Uniform::from(1..1000000000);

        let mut random_num: f32;

        /*
         * Image background set
         */
        if !self.background_drawn {
            for (_, _, pixel) in self.values.current_img.enumerate_pixels_mut() {
                *pixel = image::Rgba(background_color);
            }
            self.background_drawn = true;
        }

        /*
         * Generation
         */
        self.iterated_count += iterations as u128;

        for _ in 0..iterations {
            /*
             * Random number sample
             */
            random_num = (random_range.sample(&mut random_gen) as f32) * 0.000000001;

            /*
             * Magic
             */
            if random_num < 0.01 {
                next_x = 0.0;
                next_y = 0.16 * self.values.current_y;
            } else if random_num < 0.86 {
                next_x = 0.85 * self.values.current_x + 0.04 * self.values.current_y;
                next_y = -0.04 * self.values.current_x + 0.85 * self.values.current_y + 1.6;
            } else if random_num < 0.93 {
                next_x = 0.20 * self.values.current_x - 0.26 * self.values.current_y;
                next_y = 0.23 * self.values.current_x + 0.22 * self.values.current_y + 1.6;
            } else {
                next_x = -0.15 * self.values.current_x + 0.28 * self.values.current_y;
                next_y = 0.26 * self.values.current_x + 0.24 * self.values.current_y + 0.44;
            }

            /*
             * Scaling, assignment
             */
            plot_x = width as f64 * (self.values.current_x + 3.0) / 6.0;
            plot_y = height as f64 - height as f64 * ((self.values.current_y + 2.0) / 14.0);

            self.values.current_x = next_x;
            self.values.current_y = next_y;

            /*
             * Save targeted pixel coords
             */
            self.values
                .current_calculated_coords
                .push([plot_x as u32, plot_y as u32]);
        }

        /*
         * Write targeted pixels
         */
        for pix_coord in self.values.current_calculated_coords.iter() {
            self.values
                .current_img
                .put_pixel(pix_coord[0], pix_coord[1], image::Rgba(color));
        }

        /*
         * Save copy of image
         */
        if save_file {
            self.values
                .current_img
                .save(format!("fern{}x{}_iter_{}.png", width, height, iterations))
                .unwrap();
        }
    }

    fn get_render_target(
        &self,
        texture_context: &mut G2dTextureContext,
        texture_settings: TextureSettings,
    ) -> G2dTexture {
        let fern_texture: G2dTexture =
            Texture::from_image(texture_context, &self.values.current_img, &texture_settings)
                .unwrap();
        return fern_texture;
    }

    fn reset(&mut self ) {
        self.background_drawn = false;
        self.iterated_count = 0;

        self.values.current_calculated_coords.clear();
        self.values.current_x = 0.0;
        self.values.current_y = 0.0;
    }
}

struct FernValues {
    pub current_img: RgbaImage,
    pub current_calculated_coords: Vec<[u32; 2]>,
    pub current_x: f64,
    pub current_y: f64,
}