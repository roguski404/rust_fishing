use macroquad::prelude::*;
use crate::graphics::*;





pub enum Faza {
    Playing,
    Upgrade,
    GameOver,
    Victory,
}

pub struct Game {
    pub stan: Faza,
    pub level: u32,

    pub  bar: f32,
    pub fish: f32,
    pub progress: f32,
    // powerups
    pub speed: f32,
    pub bar_size: f32,
    pub progress_speed: f32,
    // upgrade
    pub upgrade_selection: usize,

    //graphics
    pub lake_gr: Texture2D,
    pub bar_gr: Texture2D,
    pub fish1_gr: Texture2D,
    pub fish2_gr: Texture2D,
}


impl Game {
    pub async fn new() -> Self {
        let lake_gr = load_texture("graphics/tlo.png").await.unwrap();
        let bar_gr = load_texture("graphics/bar.png").await.unwrap();
        let fish1_gr = load_texture("graphics/ryba1.png").await.unwrap();
        let fish2_gr = load_texture("graphics/ryba2.png").await.unwrap();
        Self {
            stan: Faza::Playing,
            level: 2,
            bar: 200.0,
            fish: 250.0,
            progress: 0.5,
            //powerups
            speed: 1.0,
            bar_size: 1.0,
            progress_speed: 1.0,
            upgrade_selection: 0,
            //graphs
            lake_gr,
            bar_gr,
            fish1_gr,
            fish2_gr,
        }

    }

    pub fn update(&mut self) {
        match self.stan {
            Faza::Playing => self.update_playing(),
            Faza::Upgrade => self.update_upgrade(),
            Faza::GameOver => {},
            Faza::Victory => {},
        }
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);

        match self.stan {
            Faza::Playing => self.draw_playing(),
            Faza::Upgrade => self.draw_upgrade(),
            Faza::GameOver => self.draw_game_over(),
            Faza::Victory => self.draw_victory(),
        }
    }

    fn update_playing(&mut self) {

        let dt = get_frame_time();

        if is_key_down(KeyCode::Up) {
            self.bar -= 150.0 * self.speed * dt ;
        }

        if is_key_down(KeyCode::Down) {
            self.bar += 150.0 * self.speed * dt;
        }
if(self.level==1) {
    self.fish += (rand::gen_range(-4.0, 4.0)) * 100.0 * dt;
}
        else if(self.level==2) {
            self.fish += (rand::gen_range(-4.0, 4.0)) * 140.0 * dt;
        }
        if(self.fish<120.0){
        self.fish = 120.0;
    }else if(self.fish > 700.0){
        self.fish =700.0;
    }



    if ( (self.bar - self.fish).abs() < ( 30.0 * self.bar_size) ){
            self.progress += 0.09 * dt * self.progress_speed;
        }
else {
    self.progress -= 0.09 * dt ;
}


        if(self.progress <= 0.0){
            self.stan=Faza::GameOver;
        }

        if(self.progress >= 1.0){
            self.stan = Faza::Upgrade;
        }



    }

    fn draw_playing(&mut self) {
        draw_background(&self.lake_gr);
        draw_bar(&self.bar_gr);

        draw_player_bar(&mut self.bar, self.bar_size, &self.bar_gr);


        if(self.level==1) {
            draw_fish(&self.fish1_gr, self.fish, self.level);
        }else if(self.level==2){
            draw_fish(&self.fish2_gr, self.fish, self.level);
        }





    }






    fn update_upgrade(&mut self) {

        if( is_key_pressed(KeyCode::Up) &&  self.upgrade_selection > 0){
            self.upgrade_selection -= 1;
        }
        if is_key_pressed(KeyCode::Down) &&  self.upgrade_selection <2{
            self.upgrade_selection +=1;

        }
        // 0 - speed
        // 1 - bar size
        // 2 - progress speed

        if is_key_pressed(KeyCode::Space){
            if (self.upgrade_selection == 0){
                self.speed *= 1.25;
            }
            else if (self.upgrade_selection == 1){
                self.bar_size *= 1.15;
            }
            else if (self.upgrade_selection == 2){
                self.progress_speed *= 1.15;
            }
            //przejscie
            self.progress = 0.5;
            self.level += 1;
            if self.level > 2 {
                self.stan = Faza::Victory;
            } else {
                self.stan = Faza::Playing;
            }

        }






    }

    fn draw_upgrade(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        let panel_w = 500.0;
        let panel_h = 300.0;

        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = (screen_h - panel_h) / 2.0;

        // panel
        draw_rectangle(panel_x, panel_y, panel_w, panel_h, DARKBLUE);

        draw_text("CHOOSE POWER UP!!!", panel_x + 100.0, panel_y + 40.0, 30.0, WHITE);

        let y1 = panel_y + 100.0;
        let y2 = panel_y + 140.0;
        let y3 = panel_y + 180.0;

        // opcja 0
        if self.upgrade_selection == 0 {
            draw_text(">", panel_x + 40.0, y1, 30.0, YELLOW);
        }
        draw_text("Speed +25%", panel_x + 70.0, y1, 30.0, WHITE);

        // opcja 1
        if self.upgrade_selection == 1 {
            draw_text(">", panel_x + 40.0, y2, 30.0, YELLOW);
        }
        draw_text("Bar Size +15%", panel_x + 70.0, y2, 30.0, WHITE);

        // opcja 2
        if self.upgrade_selection == 2 {
            draw_text(">", panel_x + 40.0, y3, 30.0, YELLOW);
        }
        draw_text("Progress Speed +15%", panel_x + 70.0, y3, 30.0, WHITE);


    }
    fn draw_game_over(&self) {
        draw_text("GAME OVER", 200.0, 200.0, 40.0, RED);
    }

    fn draw_victory(&self) {
        draw_text(" YOU WIN LESSGO!", 200.0, 200.0, 40.0, GREEN);
    }

}