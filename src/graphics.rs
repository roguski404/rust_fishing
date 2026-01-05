use macroquad::prelude::*;

const BASE_W: f32 = 1280.0;
const BASE_H: f32 = 720.0;

pub fn world_scale() -> f32 {
    let sx = screen_width() / BASE_W;
    let sy = screen_height() / BASE_H;
    sx.min(sy)
}

pub fn world_offset() -> Vec2 {
    let scale = world_scale();
    vec2(
        (screen_width() - BASE_W * scale) / 2.0,
        (screen_height() - BASE_H * scale) / 2.0,
    )
}

pub fn draw_bar(bar_texture: &Texture2D) {
    let scale = world_scale();
    let bar_w = 90.0 * scale;
    let bar_h = 605.0 * scale;

    let bar_x = screen_width() - bar_w - 120.0 * scale;
    let bar_y = (screen_height() - bar_h) / 2.0;

    draw_texture_ex(
        bar_texture,
        bar_x,
        bar_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(bar_w, bar_h)),
            ..Default::default()
        },
    );
}
    pub fn draw_background(lake_texture: &Texture2D) {
        let scale = world_scale();
        let offset = world_offset();

        draw_texture_ex(
            lake_texture,
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(BASE_W * scale, BASE_H * scale)),
                ..Default::default()
            },
        );
    }
pub fn draw_player_bar(player_y: &mut f32, bar_size: f32, bar_texture: &Texture2D) {
    let scale = world_scale();


    let frame_w = 90.0 * scale;
    let frame_h = 605.0 * scale;
    let frame_x = screen_width() - frame_w - 120.0 * scale;
    let frame_y = (screen_height() - frame_h) / 2.0;


    let base_player_w = 65.0 * scale;
    let base_player_h = 100.0 * scale;
    let player_h = base_player_h * bar_size;
    let player_w = base_player_w;

    let min_y = frame_y + 6.0;
    let max_y = frame_y + frame_h - player_h - 6.0;


    *player_y = player_y.clamp(min_y, max_y);


    let player_x = frame_x + (frame_w - player_w) / 2.0;
    draw_rectangle(player_x, *player_y, player_w, player_h, GREEN);
}

pub fn draw_fish(fish_texture: &Texture2D, fish_y: f32) {

    let scale = world_scale();
    let fish_w = 64.0 * scale;
    let fish_h = 64.0 * scale;

    let fish_x = screen_width() - fish_w - 135.0 * scale;
    


    draw_texture_ex(
        fish_texture,
        fish_x,
        fish_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(fish_w, fish_h)),
            ..Default::default()
        },
    );


}