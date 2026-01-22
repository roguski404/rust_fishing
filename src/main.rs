
use macroquad::prelude::*;

mod game;
mod graphics;

use crate::game::*;

use game::Game;


fn window_conf() -> Conf {
    Conf {
        window_title: "Fishing".to_owned(),
        fullscreen: false ,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {

        game.update();
        game.draw();
        next_frame().await;



    }
}




