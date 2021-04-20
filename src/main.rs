extern crate piston_window;

use piston_window::*;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

fn main() {
    
    const WINDOW_WIDTH: u32 = 1000;
    const WINDOW_HEIGHT: u32 = 1000;

    const CLEAR_COLOR: [f32; 4] = [0.89; 4];
    const FERN_COLOR: [f32; 4] = [0.05, 0.79, 0.1, 1.0];

    /*
    * Window setup
    */

    let mut window: PistonWindow = WindowSettings::new("Fern", [WINDOW_WIDTH, WINDOW_HEIGHT])
    .exit_on_esc(true)
    .build()
    .unwrap();

    /*
    * Plot values
    */
    let mut plot_x: f64 = 0.0;
    let mut plot_y: f64 = 0.0;

    let mut cur_x: f64 = 0.0;
    let mut cur_y: f64 = 0.0;

    let mut next_x: f64 = 0.0;
    let mut next_y: f64 = 0.0;

    let mut random_gen = rand::thread_rng();
    let random_range = Uniform::from(1..1000000000);
    

    while let Some(event) = window.next() {
        /*
        * Random number sample
        */
        let random_num: f32 = (random_range.sample(&mut random_gen) as f32) * 0.000000001; 

        /*
        * Magic
        */
        if random_num < 0.01 {
            next_x =  0.0;
            next_y =  0.16 * cur_y;
        } else if random_num < 0.86 {
            next_x =  0.85 * cur_x + 0.04 * cur_y;
            next_y = -0.04 * cur_x + 0.85 * cur_y + 1.6;
        } else if random_num < 0.93 {
            next_x =  0.20 * cur_x - 0.26 * cur_y;
            next_y =  0.23 * cur_x + 0.22 * cur_y + 1.6;
        } else {
            next_x = -0.15 * cur_x + 0.28 * cur_y;
            next_y =  0.26 * cur_x + 0.24 * cur_y + 0.44;
        }

        /*
        * Scaling, assignment
        */
        plot_x = WINDOW_WIDTH as f64 * (cur_x + 3.0) / 6.0;
        plot_y = WINDOW_HEIGHT as f64 - WINDOW_HEIGHT as f64 * ((cur_y + 2.0) / 14.0);

        cur_x = next_x;
        cur_y = next_y;


        /*
        * Rendering
        */
        window.draw_2d(&event, |context, graphics, _device| {

            circle_arc(FERN_COLOR, 1.0, 0.0, f64::_360() as f64 * 1.2, [plot_x, plot_y, 0.1, 0.1],
            context.transform, graphics);
            
        });
    }
}