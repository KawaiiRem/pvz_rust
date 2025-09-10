use macroquad::prelude::*;

pub struct GameOver {
    pub restart: bool,
}

impl GameOver {
    pub fn new() -> Self {
        Self { restart: false }
    }

    pub fn update(&mut self) {
        // Handle input
        if is_key_pressed(KeyCode::Enter) {
            self.restart = true;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let btn_x = screen_width() / 2.0 - 80.0;
            let btn_y = screen_height() / 2.0 + 40.0;
            let btn_w = 160.0;
            let btn_h = 50.0;

            if mx >= btn_x && mx <= btn_x + btn_w && my >= btn_y && my <= btn_y + btn_h {
                self.restart = true;
            }
        }
    }

    pub fn draw(&self) {
        clear_background(DARKGRAY);

        // Title
        let text = "GAME OVER!";
        let metrics = measure_text(text, None, 60, 1.0);
        draw_text(
            text,
            screen_width() / 2.0 - metrics.width / 2.0,
            screen_height() / 2.0 - 40.0,
            60.0,
            RED,
        );

        // Button
        let btn_x = screen_width() / 2.0 - 80.0;
        let btn_y = screen_height() / 2.0 + 40.0;
        let btn_w = 160.0;
        let btn_h = 50.0;

        draw_rectangle(btn_x, btn_y, btn_w, btn_h, LIGHTGRAY);
        draw_rectangle_lines(btn_x, btn_y, btn_w, btn_h, 3.0, BLACK);

        let btn_text = "Restart";
        let metrics = measure_text(btn_text, None, 30, 1.0);
        draw_text(
            btn_text,
            btn_x + (btn_w - metrics.width) / 2.0,
            btn_y + btn_h / 2.0 + metrics.height / 2.0,
            30.0,
            BLACK,
        );

        draw_text("Press ENTER to Restart", 20.0, screen_height() - 40.0, 25.0, WHITE);
    }
}
