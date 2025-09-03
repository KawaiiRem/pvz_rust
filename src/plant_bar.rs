use macroquad::prelude::*;
use crate::constants::*;
use crate::plant::PlantType;

pub struct UISlot {
    pub plant: PlantType,
    pub x: f32,
    pub y: f32,
    pub selected: bool,
    pub cooldown: f32,
}

pub struct UIBar {
    pub slots: Vec<UISlot>,
    pub selected: Option<PlantType>,
    pub sun_box_width: f32,
}

impl UIBar {
    pub fn new() -> Self {
        let mut slots = Vec::new();
        let sun_box_width = 120.0; 

        let plants = [PlantType::Sunflower, PlantType::Peashooter, PlantType::SlowPeashooter];
        for (i, plant) in plants.iter().enumerate() {
            let x = sun_box_width + SLOT_PADDING + i as f32 * (SLOT_SIZE + SLOT_PADDING);
            let y = (UI_BAR_HEIGHT - SLOT_SIZE) / 2.0;
            slots.push(UISlot {
                plant: *plant,
                x,
                y,
                selected: false,
                cooldown: 0.0,
            });
        }

        UIBar { slots, selected: None, sun_box_width }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        // reduce cooldown timers
        for slot in &mut self.slots {
            if slot.cooldown > 0.0 {
                slot.cooldown -= dt;
                if slot.cooldown < 0.0 {
                    slot.cooldown = 0.0;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse: Vec2 = mouse_position().into();

            // Step 1: find clicked slot (immutable iteration)
            let mut clicked: Option<usize> = None;
            for (i, slot) in self.slots.iter().enumerate() {
                if mouse.x >= slot.x && mouse.x < slot.x + SLOT_SIZE &&
                   mouse.y >= slot.y && mouse.y < slot.y + SLOT_SIZE {
                    clicked = Some(i);
                    break;
                }
            }

            // Step 2: apply selection logic (mutable iteration)
            if let Some(clicked_index) = clicked {
                for (i, slot) in self.slots.iter_mut().enumerate() {
                    if i == clicked_index {
                        slot.selected = true;
                        self.selected = Some(slot.plant);
                    } else {
                        slot.selected = false;
                    }
                }
            }
        }
    }

    pub fn draw(&self, sun_points: i32) {
        // Draw UI bar background
        draw_rectangle(0.0, 0.0, SCREEN_WIDTH, UI_BAR_HEIGHT, GRAY);

        let sun_box_width = self.sun_box_width;
        let sun_box_height = UI_BAR_HEIGHT;

        draw_rectangle(0.0, 0.0, sun_box_width, sun_box_height, BROWN);

        let sun_x = 30.0;
        let sun_y = sun_box_height / 2.0;

        draw_circle(sun_x, sun_y, 18.0, YELLOW);
        draw_circle(sun_x, sun_y, 12.0, ORANGE);

        draw_text(
            &format!("{}", sun_points),
            sun_x + 30.0,
            sun_y + 10.0,
            32.0,
            BLACK,
        );

        for slot in &self.slots {
            let color = if slot.selected { YELLOW } else { WHITE };
            draw_rectangle_lines(slot.x, slot.y, SLOT_SIZE, SLOT_SIZE, 3.0, color);

            let center_x = slot.x + SLOT_SIZE / 2.0;
            let center_y = slot.y + SLOT_SIZE / 2.0 - 6.0; 

            match slot.plant {
                PlantType::Peashooter => {
                    draw_circle(center_x, center_y, 12.0, GREEN);
                    draw_circle(center_x + 12.0, center_y, 6.0, DARKGREEN);
                }
                PlantType::SlowPeashooter => {
                    draw_circle(center_x, center_y, 12.0, BLUE);
                    draw_circle(center_x + 12.0, center_y, 6.0, DARKBLUE);
                }
                PlantType::Sunflower => {
                    draw_circle(center_x, center_y, 10.0, YELLOW);
                    draw_circle(center_x, center_y, 6.0, ORANGE);
                }
            }

            let cost_text = format!("{}", slot.plant.cost());
            let text_dim = measure_text(&cost_text, None, 20, 1.0);
            draw_text(
                &cost_text,
                center_x - text_dim.width / 2.0,
                slot.y + SLOT_SIZE - 4.0,
                20.0,
                BLACK,
            );

            // overlay cooldown bar
            if slot.cooldown > 0.0 {
                let ratio = slot.cooldown / slot.plant.planting_cooldown();
                let fill_h = SLOT_SIZE * ratio;
                draw_rectangle(
                    slot.x,
                    slot.y + (SLOT_SIZE - fill_h),
                    SLOT_SIZE,
                    fill_h,
                    Color::new(0.0, 0.0, 0.0, 0.5),
                );
            }
        }
    }
}
