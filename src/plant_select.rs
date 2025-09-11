use macroquad::prelude::*;

use crate::{constants::*, factory::plant_factory::PlantType};
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
        let mut hovered_description: Option<&str> = None; 

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

            let text = format!("{:?}", plant);
            let mut font_size = 20.0;
            let mut metrics = measure_text(&text, None, font_size as u16, 1.0);

            while metrics.width > w - 10.0 && font_size > 10.0 {
                font_size -= 1.0;
                metrics = measure_text(&text, None, font_size as u16, 1.0);
            }

            let text_x = x + (w - metrics.width) / 2.0;
            let text_y = y + h - 20.0;
            draw_text(&text, text_x, text_y, font_size, BLACK);

            if self.selected.contains(plant) {
                draw_rectangle_lines(x, y, w, h, 5.0, YELLOW);
            }

            let cost = plant.cost();
            let cost_text = format!("{}", cost);
            let cost_w = 40.0;
            let cost_h = 25.0;
            let cost_x = x + (w - cost_w) / 2.0;
            let cost_y = y + h + 5.0;

            draw_rectangle(cost_x, cost_y, cost_w, cost_h, GOLD);

            let metrics = measure_text(&cost_text, None, 18, 1.0);
            let text_x = cost_x + (cost_w - metrics.width) / 2.0;
            let text_y = cost_y + cost_h - 6.0;
            draw_text(&cost_text, text_x, text_y, 18.0, BLACK);

            // if hovered → remember description
            if hovered {
                hovered_description = Some(plant.description());
            }
        }

        // show selected plants at the bottom
        draw_text("Selected:", 100.0, 400.0, 30.0, BLACK);

        let mut x_offset = 120.0;
        for plant in &self.selected {
            let text = format!("{:?}", plant);
            let metrics = measure_text(&text, None, 25, 1.0);

            draw_text(&text, x_offset, 450.0, 25.0, DARKBLUE);
            x_offset += metrics.width + 30.0;
        }

        draw_text("Press ENTER to start!", 100.0, 500.0, 30.0, RED);

        if self.selected.len() >= MAX_SELECTED_PLANTS {
            draw_text("Max plants selected!", 100.0, 550.0, 25.0, RED);
        }

        if let Some(desc) = hovered_description {
            // build full text
            let label = "Description:";
            let label_metrics = measure_text(label, None, 20, 1.0);
            let desc_metrics = measure_text(desc, None, 20, 1.0);

            let padding = 10.0;
            let box_w = f32::max(label_metrics.width, desc_metrics.width) + padding * 2.0;
            let box_h = 60.0; // enough for 2 lines
            let x = (screen_width() - box_w) / 2.0;
            let y = screen_height() - 100.0;

            // background box
            draw_rectangle(x, y, box_w, box_h, WHITE);
            draw_rectangle_lines(x, y, box_w, box_h, 2.0, BLACK);

            // draw label
            let label_x = x + (box_w - label_metrics.width) / 2.0;
            let label_y = y + 22.0;
            draw_text(label, label_x, label_y, 20.0, DARKGREEN);

            // draw description text
            let desc_x = x + (box_w - desc_metrics.width) / 2.0;
            let desc_y = y + 45.0;
            draw_text(desc, desc_x, desc_y, 20.0, BLACK);
        }
    }
}
