use macroquad::prelude::*;
use crate::graphics::*;

pub enum Faza { Playing, Upgrade, GameOver, Victory, Start }

pub struct Game {
    pub stan: Faza,
    pub level: u32,
    pub bar: f32,
    pub fish: f32,
    pub progress: f32,
    pub fish_target_y: f32,
    pub fish_speed: f32,
    pub fish_wait_timer: f32,
    pub fish_is_waiting: bool,
    pub speed: f32,
    pub bar_size: f32,
    pub progress_speed: f32,
    pub upgrade_selection: usize,
    pub fake_fish_y: f32,
    pub fake_fish_visible: bool,
    pub fake_fish_alpha: f32,
    pub fake_fish_speed: f32,
    pub fake_fish_target_y: f32,
    pub fish_visible: bool,
    pub fish_trail: Vec<f32>,
    pub fish_trail_timer: f32,
    pub lake_gr: Texture2D,
    pub bar_gr: Texture2D,
    pub fish1_gr: Texture2D,
    pub fish2_gr: Texture2D,
    pub fish3_gr: Texture2D,
    pub fish4_gr: Texture2D,
    pub health_bar_gr: Texture2D,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            stan: Faza::Start,
            level: 1,
            bar: 200.0,
            fish: 250.0,
            progress: 0.5,
            fish_target_y: 250.0,
            fish_speed: 0.0,
            fish_wait_timer: 0.0,
            fish_is_waiting: false,
            speed: 1.5,
            bar_size: 1.0,
            progress_speed: 1.0,
            upgrade_selection: 0,
            fake_fish_y: 300.0,
            fake_fish_visible: false,
            fake_fish_alpha: 0.8,
            fake_fish_speed: 0.0,
            fake_fish_target_y: 300.0,
            fish_visible: true,
            fish_trail: Vec::new(),
            fish_trail_timer: 0.0,
            lake_gr: load_texture("graphics/tlo.png").await.unwrap(),
            bar_gr: load_texture("graphics/bar.png").await.unwrap(),
            fish1_gr: load_texture("graphics/ryba1.png").await.unwrap(),
            fish2_gr: load_texture("graphics/ryba2.png").await.unwrap(),
            fish3_gr: load_texture("graphics/ryba3.png").await.unwrap(),
            fish4_gr: load_texture("graphics/ryba4.png").await.unwrap(),
            health_bar_gr: load_texture("graphics/health_bar.png").await.unwrap(),
        }
    }

    pub fn update(&mut self) {
        match self.stan {
            Faza::Start => self.update_start(),
            Faza::Playing => self.update_playing(),
            Faza::Upgrade => self.update_upgrade(),
            _ => {}
        }
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);
        match self.stan {
            Faza::Start => self.draw_start(),
            Faza::Playing => self.draw_playing(),
            Faza::Upgrade => self.draw_upgrade(),
            Faza::GameOver => self.draw_game_over(),
            Faza::Victory => self.draw_victory(),
        }
    }

    fn update_playing(&mut self) {
        let dt = get_frame_time();

        if is_key_down(KeyCode::Up) { self.bar -= 150.0 * self.speed * dt; }
        if is_key_down(KeyCode::Down) { self.bar += 150.0 * self.speed * dt; }

        match self.level {
            1 => self.fish += rand::gen_range(-4.0, 4.0) * 100.0 * dt,
            2 => self.fish += rand::gen_range(-4.0, 4.0) * 210.0 * dt,
            3 | 4 => {
                if self.fish_is_waiting {
                    self.fish_wait_timer -= dt;
                    if self.fish_wait_timer <= 0.0 { self.pick_new_fish_target(); }
                } else {
                    let dir = (self.fish_target_y - self.fish).signum();
                    self.fish += dir * self.fish_speed * dt;
                    if (self.fish - self.fish_target_y).abs() < 3.0 {
                        self.fish = self.fish_target_y;
                        self.fish_is_waiting = true;
                    }
                }
                if self.level == 3 {
                    if self.progress >= 0.7 && !self.fake_fish_visible {
                        self.fake_fish_visible = true;
                        self.fake_fish_y = self.fish;
                        self.fake_fish_target_y = rand::gen_range(120.0, 700.0);
                        self.fake_fish_speed = 120.0;
                    }
                    if self.fake_fish_visible {
                        let dir = (self.fake_fish_target_y - self.fake_fish_y).signum();
                        self.fake_fish_y += dir * self.fake_fish_speed * dt;
                        if (self.fake_fish_y - self.fake_fish_target_y).abs() < 5.0 {
                            self.fake_fish_target_y = rand::gen_range(120.0, 700.0);
                        }
                    }
                } else if self.level == 4 {
                    self.fish_visible = self.progress <= 0.7;
                    if !self.fish_visible {
                        self.fish_trail_timer -= dt;
                        if self.fish_trail_timer <= 0.0 {
                            self.fish_trail.push(self.fish);
                            self.fish_trail_timer = 0.05;
                        }
                        if self.fish_trail.len() > 15 { self.fish_trail.remove(0); }
                    } else { self.fish_trail.clear(); }
                }
            }
            _ => {}
        }

        self.fish = self.fish.clamp(120.0, 700.0);

        if (self.bar - self.fish).abs() < 80.0 * self.bar_size {
            self.progress += 0.09 * dt * self.progress_speed * (1.0 - 0.05 * self.level as f32);
        } else {
            self.progress -= 0.16 * dt;
        }

        if self.progress <= 0.0 { self.stan = Faza::GameOver; }
        else if self.progress >= 1.0 {
            if self.level == 4 { self.stan = Faza::Victory; }
            else { self.stan = Faza::Upgrade; }
        }
    }

    fn draw_playing(&mut self) {
        draw_background(&self.lake_gr);
        let scale = world_scale();
        let x = (screen_width() - 640.0 * scale) / 2.0;
        let y = screen_height() - (50.0 * scale) - 20.0 * scale;

        draw_health_bar(&self.health_bar_gr, x, y, scale, self.progress);
        draw_bar(&self.bar_gr);
        draw_player_bar(&mut self.bar, self.bar_size, &self.bar_gr);

        match self.level {
            1 => draw_fish(&self.fish1_gr, self.fish, self.level),
            2 => draw_fish(&self.fish2_gr, self.fish, self.level),
            3 => {
                draw_fish(&self.fish3_gr, self.fish, self.level);
                if self.fake_fish_visible {
                    let fs = world_scale();
                    let fx = screen_width() - 64.0 * fs - 135.0 * fs;
                    draw_texture_ex(&self.fish3_gr, fx, self.fake_fish_y, Color::new(1.0, 1.0, 1.0, self.fake_fish_alpha),
                                    DrawTextureParams { dest_size: Some(vec2(64.0 * fs, 64.0 * fs)), ..Default::default() });
                }
            }
            4 => {
                if self.fish_visible { draw_fish(&self.fish4_gr, self.fish, self.level); }
                else {
                    let fx = screen_width() - 64.0 * world_scale() - 135.0 * world_scale();
                    for (i, ty) in self.fish_trail.iter().enumerate() {
                        let alpha = i as f32 / self.fish_trail.len() as f32;
                        draw_circle(fx + 20.0 * world_scale(), *ty + 32.0 * world_scale(), 4.0 * world_scale(), Color::new(1.0, 1.0, 1.0, alpha));
                    }
                }
                draw_text("FINAL BOSS !!!! SEA DRAGON", 10.0, 100.0, 80.0 * scale, WHITE);
            }
            _ => {}
        }
    }

    fn pick_new_fish_target(&mut self) {
        self.fish_target_y = rand::gen_range(120.0, 700.0);
        self.fish_speed = if self.level == 4 { rand::gen_range(200.0, 390.0) } else { rand::gen_range(160.0, 320.0) };
        self.fish_wait_timer = if self.level == 3 { rand::gen_range(0.0, 0.5) } else { rand::gen_range(0.0, 0.4) };
        self.fish_is_waiting = false;
    }

    fn update_upgrade(&mut self) {
        if is_key_pressed(KeyCode::Up) && self.upgrade_selection > 0 { self.upgrade_selection -= 1; }
        if is_key_pressed(KeyCode::Down) && self.upgrade_selection < 2 { self.upgrade_selection += 1; }

        if is_key_pressed(KeyCode::Space) {
            match self.upgrade_selection {
                0 => self.speed *= 1.25,
                1 => self.bar_size *= 1.15,
                2 => self.progress_speed *= 1.15,
                _ => {}
            }
            self.progress = 0.5;
            self.level += 1;
            if self.level >= 3 { self.pick_new_fish_target(); }
            self.stan = Faza::Playing;
        }
    }

    fn draw_upgrade(&self) {
        let panel_x = (screen_width() - 500.0) / 2.0;
        let panel_y = (screen_height() - 300.0) / 2.0;
        draw_rectangle(panel_x, panel_y, 500.0, 300.0, DARKBLUE);
        draw_text("CHOOSE POWER UP!!!", panel_x + 100.0, panel_y + 40.0, 30.0, WHITE);

        let opts = ["Speed +25%", "Bar Size +15%", "Progress Speed +15%"];
        for (i, opt) in opts.iter().enumerate() {
            let y = panel_y + 100.0 + (i as f32 * 40.0);
            if self.upgrade_selection == i { draw_text(">", panel_x + 40.0, y, 30.0, YELLOW); }
            draw_text(opt, panel_x + 70.0, y, 30.0, WHITE);
        }
    }

    fn draw_start(&self) {
        let sw = screen_width();
        let sh = screen_height();
        draw_text("FISHING GAME", sw / 2.0 - 150.0, sh / 2.0 - 40.0, 50.0, GREEN);
        draw_text("PRESS SPACE TO START", sw / 2.0 - 160.0, sh / 2.0 + 10.0, 30.0, WHITE);
        draw_text("Use UP/DOWN arrows to control the bar", sw / 2.0 - 160.0, sh / 2.0 + 50.0, 20.0, WHITE);
    }

    fn update_start(&mut self) {
        if is_key_pressed(KeyCode::Space) { self.stan = Faza::Playing; }
    }

    fn draw_game_over(&self) { draw_text("GAME OVER", 200.0, 200.0, 40.0, RED); }
    fn draw_victory(&self) { draw_text("YOU WIN LESSGO!", 200.0, 200.0, 40.0, GREEN); }
}