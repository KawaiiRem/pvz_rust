use macroquad::prelude::*;

use crate::plant_factory::PlantType;
use crate::constants::*;
use strum::IntoEnumIterator;

pub struct PlantSelect {
    pub available: Vec<PlantType>,
    pub selected: Vec<PlantType>,
}

impl PlantSelect {
    pub fn new() -> Self {
        Self {
            available: PlantType::iter().collect(),
            selected: Vec::new(),
        }
    }

    /// Update logic – returns Some(selected_plants) if Enter pressed, otherwise None
    pub fn update(&mut self) -> Option<Vec<PlantType>> {
        let (mx, my) = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, plant) in self.available.iter().enumerate() {
                let x = 100.0 + i as f32 * 120.0;
                let y = 150.0;
                let w = 100.0;
                let h = 100.0;

                if mx >= x && mx <= x + w && my >= y && my <= y + h {
                    if self.selected.contains(plant) {
                        // deselect if already chosen
                        self.selected.retain(|p| p != plant);
                    } else if self.selected.len() < MAX_SELECTED_PLANTS {
                        // add if under limit
                        self.selected.push(*plant);
                    }
                }
            }
        }

        // if Enter pressed and at least 1 plant selected → return selection
        if is_key_pressed(KeyCode::Enter) && !self.selected.is_empty() {
            return Some(self.selected.clone());
        }

        None
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);

        draw_text("Select Your Plants", 250.0, 80.0, 40.0, DARKGREEN);

        let (mx, my) = mouse_position();

        for (i, plant) in self.available.iter().enumerate() {
            let x = 100.0 + i as f32 * 120.0;
            let y = 150.0;
            let w = 100.0;
            let h = 100.0;

            let hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;

            // draw base box
            draw_rectangle(
                x,
                y,
                w,
                h,
                if hovered { LIGHTGRAY } else { GRAY },
            );

            // draw plant preview above text (centered horizontally)
            plant.draw_preview(x + w / 2.0, y + 35.0);

            // ==== centered + auto-scaled name ====
            let text = format!("{:?}", plant);
            let mut font_size = 20.0;
            let mut metrics = measure_text(&text, None, font_size as u16, 1.0);

            // shrink text until it fits inside the box width
            while metrics.width > w - 10.0 && font_size > 10.0 {
                font_size -= 1.0;
                metrics = measure_text(&text, None, font_size as u16, 1.0);
            }

            // center horizontally in the box
            let text_x = x + (w - metrics.width) / 2.0;
            let text_y = y + h - 20.0;
            draw_text(&text, text_x, text_y, font_size, BLACK);

            // highlight if selected
            if self.selected.contains(plant) {
                draw_rectangle_lines(x, y, w, h, 5.0, YELLOW);
            }

            // ==== draw cost box below ====
            let cost = plant.cost();
            let cost_text = format!("{}", cost);
            let cost_w = 40.0;
            let cost_h = 25.0;
            let cost_x = x + (w - cost_w) / 2.0;
            let cost_y = y + h + 5.0;

            // background box
            draw_rectangle(cost_x, cost_y, cost_w, cost_h, GOLD);

            // center text inside
            let metrics = measure_text(&cost_text, None, 18, 1.0);
            let text_x = cost_x + (cost_w - metrics.width) / 2.0;
            let text_y = cost_y + cost_h - 6.0;
            draw_text(&cost_text, text_x, text_y, 18.0, BLACK);
        }

        // show selected plants at the bottom
        draw_text("Selected:", 100.0, 400.0, 30.0, BLACK);
        for (i, plant) in self.selected.iter().enumerate() {
            draw_text(
                &format!("{:?}", plant),
                120.0 + i as f32 * 150.0,
                450.0,
                25.0,
                DARKBLUE,
            );
        }

        // instructions
        draw_text("Press ENTER to start!", 100.0, 500.0, 30.0, RED);

        // max selection warning
        if self.selected.len() >= MAX_SELECTED_PLANTS {
            draw_text("Max plants selected!", 100.0, 550.0, 25.0, RED);
        }
    }
}
