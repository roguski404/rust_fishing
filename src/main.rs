use macroquad::color::{BLACK, GREEN};
use macroquad::prelude::{draw_rectangle, screen_width};
use macroquad::window::{clear_background, screen_height};
use macroquad::prelude::*;

mod game;
mod graphics;
use crate::graphics::*;
use crate::game::*;


use game::Game;

#[macroquad::main("Fishing")]
async fn main() {
    let mut game = Game::new().await;
//    draw_rectangle(400.0 ,200.0 ,screen_width() / 2.0, screen_height() / 2.0   , BLACK);


    loop{
     game.update();
     game.draw();

    next_frame().await;





}


}





// draw_rectangle(200.0, 150.0, 300.0, 200.0, BLACK);
// draw_rectangle(20.0, 30.0, 300.0, 200.0, PINK);
//