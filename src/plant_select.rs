use crate::{constants::*, factory::plant_factory::PlantType};
use macroquad::prelude::*;
use strum::IntoEnumIterator;

pub struct PlantSelect {
    pub available: Vec<PlantType>,
    pub selected: Vec<PlantType>,
    pub warning_timer: f32,
    pub scroll_y: f32,
}

impl PlantSelect {
    pub fn new() -> Self {
        Self {
            available: PlantType::iter().collect(),
            selected: Vec::new(),
            warning_timer: 0.0,
            scroll_y: 0.0,
        }
    }

    pub fn update(&mut self) -> Option<Vec<PlantType>> {
        // screen mouse
        let (mx, my) = mouse_position();

        // --- viewport & layout constants (must match draw) ---
        let viewport_x = 80.0;
        let viewport_y = 120.0;
        let viewport_w = SCREEN_WIDTH - 160.0;
        let viewport_h = SCREEN_HEIGHT - 300.0;

        let padding_x = 20.0;
        let padding_y = 10.0;
        let cell_w = 100.0;
        let cell_h = 100.0;
        let spacing_x = 120.0;
        let spacing_y = 160.0;

        // handle scroll wheel
        let wheel = mouse_wheel().1;
        if wheel.abs() > f32::EPSILON {
            self.scroll_y -= wheel * 30.0;
        }

        // compute layout & limits
        let cols = ((viewport_w - padding_x * 2.0) / spacing_x).floor() as usize;
        let cols = std::cmp::max(1, cols);
        let rows = (self.available.len() + cols - 1) / cols;
        let total_content_height = padding_y * 2.0 + rows as f32 * spacing_y;
        let max_scroll = (total_content_height - viewport_h).max(0.0);
        self.scroll_y = self.scroll_y.clamp(0.0, max_scroll);

        // convert to world mouse
        let world_mouse = vec2(mx, my + self.scroll_y);

        // handle mouse click
        if is_mouse_button_pressed(MouseButton::Left) {
            // only if inside viewport on screen
            if mx >= viewport_x
                && mx <= viewport_x + viewport_w
                && my >= viewport_y
                && my <= viewport_y + viewport_h
            {
                for (i, plant) in self.available.iter().enumerate() {
                    let row = i / cols;
                    let col = i % cols;

                    let x = viewport_x + padding_x + col as f32 * spacing_x;
                    let y = viewport_y + padding_y + row as f32 * spacing_y;
                    let w = cell_w;
                    let h = cell_h;

                    if world_mouse.x >= x
                        && world_mouse.x <= x + w
                        && world_mouse.y >= y
                        && world_mouse.y <= y + h
                    {
                        if self.selected.contains(plant) {
                            self.selected.retain(|p| p != plant);
                        } else if self.selected.len() < MAX_SELECTED_PLANTS {
                            self.selected.push(*plant);
                        }
                        break;
                    }
                }
            }
        }

        // countdown warning
        if self.warning_timer > 0.0 {
            self.warning_timer -= get_frame_time();
        }

        // Enter pressed
        if is_key_pressed(KeyCode::Enter) {
            if !self.selected.is_empty() {
                return Some(self.selected.clone());
            } else {
                self.warning_timer = 2.0;
            }
        }

        None
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);

        draw_text("Select Your Plants", 250.0, 80.0, 40.0, DARKGREEN);

        let (mx, my) = mouse_position();
        let mut hovered_description: Option<&str> = None;

        // --- viewport & layout constants ---
        let viewport_x = 80.0;
        let viewport_y = 120.0;
        let viewport_w = SCREEN_WIDTH - 160.0;
        let viewport_h = SCREEN_HEIGHT - 300.0;

        let padding_x = 20.0;
        let padding_y = 10.0;
        let cell_w = 100.0;
        let cell_h = 100.0;
        let spacing_x = 120.0;
        let spacing_y = 160.0;

        // draw viewport background
        draw_rectangle(
            viewport_x,
            viewport_y,
            viewport_w,
            viewport_h,
            Color::new(0.95, 0.95, 0.95, 1.0),
        );

        // compute layout
        let cols = ((viewport_w - padding_x * 2.0) / spacing_x).floor() as usize;
        let cols = std::cmp::max(1, cols);
        let rows = (self.available.len() + cols - 1) / cols;
        let total_content_height = padding_y * 2.0 + rows as f32 * spacing_y;
        let max_scroll = (total_content_height - viewport_h).max(0.0);

        // camera
        let cam = Camera2D {
            target: vec2(
                viewport_x + viewport_w / 2.0,
                viewport_y + viewport_h / 2.0 + self.scroll_y,
            ),
            zoom: vec2(2.0 / viewport_w, 2.0 / viewport_h),
            viewport: Some((
                viewport_x as i32,
                (screen_height() - (viewport_y + viewport_h)) as i32,
                viewport_w as i32,
                viewport_h as i32,
            )),
            ..Default::default()
        };
        set_camera(&cam);

        // convert to world coords
        let world_mouse = vec2(mx, my + self.scroll_y);

        // draw plants
        for (i, plant) in self.available.iter().enumerate() {
            let row = i / cols;
            let col = i % cols;

            let x = viewport_x + padding_x + col as f32 * spacing_x;
            let y = viewport_y + padding_y + row as f32 * spacing_y;
            let w = cell_w;
            let h = cell_h;

            // hover test (must be inside viewport as well!)
            let hovered = world_mouse.x >= x
                && world_mouse.x <= x + w
                && world_mouse.y >= y
                && world_mouse.y <= y + h
                && mx >= viewport_x
                && mx <= viewport_x + viewport_w
                && my >= viewport_y
                && my <= viewport_y + viewport_h;

            draw_rectangle(x, y, w, h, if hovered { LIGHTGRAY } else { GRAY });
            plant.draw_preview(x + w / 2.0, y + 35.0);

            // name
            let text = format!("{:?}", plant);
            let mut font_size = 18.0;
            let mut metrics = measure_text(&text, None, font_size as u16, 1.0);
            while metrics.width > w - 10.0 && font_size > 8.0 {
                font_size -= 1.0;
                metrics = measure_text(&text, None, font_size as u16, 1.0);
            }
            let text_x = x + (w - metrics.width) / 2.0;
            let text_y = y + h - 20.0;
            draw_text(&text, text_x, text_y, font_size, BLACK);

            if self.selected.contains(plant) {
                draw_rectangle_lines(x, y, w, h, 4.0, YELLOW);
            }

            // cost
            let cost = plant.cost();
            let cost_text = format!("{}", cost);
            let cost_w = 40.0;
            let cost_h = 22.0;
            let cost_x = x + (w - cost_w) / 2.0;
            let cost_y = y + h + 6.0;
            draw_rectangle(cost_x, cost_y, cost_w, cost_h, GOLD);
            let metrics = measure_text(&cost_text, None, 16, 1.0);
            let tx = cost_x + (cost_w - metrics.width) / 2.0;
            let ty = cost_y + cost_h - 6.0;
            draw_text(&cost_text, tx, ty, 16.0, BLACK);

            if hovered {
                hovered_description = Some(plant.description());
            }
        }

        // back to default camera
        set_default_camera();

        // border
        draw_rectangle_lines(viewport_x, viewport_y, viewport_w, viewport_h, 3.0, BLACK);

        // scrollbar
        if max_scroll > 0.0 {
            let scrollbar_w = 12.0;
            let scrollbar_x = viewport_x + viewport_w - scrollbar_w;
            draw_rectangle(
                scrollbar_x,
                viewport_y,
                scrollbar_w,
                viewport_h,
                Color::new(0.85, 0.85, 0.85, 1.0),
            );

            let thumb_h = (viewport_h / total_content_height) * viewport_h;
            let thumb_h = thumb_h.clamp(16.0, viewport_h);

            let scroll_ratio = if max_scroll > 0.0 {
                self.scroll_y / max_scroll
            } else {
                0.0
            };
            let thumb_y = viewport_y + scroll_ratio * (viewport_h - thumb_h);

            draw_rectangle(
                scrollbar_x + 2.0,
                thumb_y,
                scrollbar_w - 4.0,
                thumb_h,
                DARKGRAY,
            );
        }

        // bottom UI
        let selected_y = viewport_y + viewport_h + 40.0;
        draw_text("Selected:", viewport_x, selected_y, 30.0, BLACK);

        let preview_size = 60.0;
        let gap = 16.0;
        let mut x_offset = viewport_x + 120.0;
        for plant in &self.selected {
            if x_offset + preview_size > SCREEN_WIDTH - 40.0 {
                break;
            }

            let cx = x_offset + preview_size / 2.0;
            let cy = selected_y + 30.0 + preview_size / 2.0;
            plant.draw_preview(cx, cy);

            draw_rectangle_lines(
                x_offset,
                selected_y + 10.0,
                preview_size,
                preview_size,
                2.0,
                BLACK,
            );
            x_offset += preview_size + gap;
        }

        draw_text(
            "Press ENTER to start!",
            viewport_x,
            selected_y + 120.0,
            28.0,
            RED,
        );
        if self.selected.len() == MAX_SELECTED_PLANTS {
            draw_text(
                "Max plants selected!",
                viewport_x,
                selected_y + 160.0,
                24.0,
                RED,
            );
        }

        if let Some(desc) = hovered_description {
            let label = "Description:";
            let label_metrics = measure_text(label, None, 20, 1.0);
            let desc_metrics = measure_text(desc, None, 18, 1.0);
            let padding = 12.0;
            let box_w = f32::max(label_metrics.width, desc_metrics.width) + padding * 2.0;
            let box_h = 66.0;
            let box_x = (screen_width() - box_w) / 2.0;
            let box_y = screen_height() - 110.0;

            draw_rectangle(box_x, box_y, box_w, box_h, WHITE);
            draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, BLACK);

            let label_x = box_x + (box_w - label_metrics.width) / 2.0;
            let label_y = box_y + 22.0;
            draw_text(label, label_x, label_y, 20.0, DARKGREEN);

            let desc_x = box_x + (box_w - desc_metrics.width) / 2.0;
            let desc_y = box_y + 45.0;
            draw_text(desc, desc_x, desc_y, 18.0, BLACK);
        }

        if self.warning_timer > 0.0 {
            draw_text(
                "Please select at least one plant!",
                viewport_x,
                selected_y + 200.0,
                24.0,
                RED,
            );
        }
    }
}
