// Circle struct with all its implementations

use crate::common_structs::{RGBColor, RGBCanvas};

#[derive(Clone)]
pub struct Circle {
    pub name: String,
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_vel: f32,
    pub y_vel: f32,
    pub radius: f32,
    border_width: f32,
    body_color: RGBColor,
    border_color: RGBColor,
    pub mass: f32,
    is_selected: bool,
}

impl Circle {
    pub fn new(
        name: String,
        x_pos: f32,
        y_pos: f32,
        x_vel: f32,
        y_vel: f32,
        radius: f32,
        mut border_width: f32,
        mass: f32,
        body_color: RGBColor,
        border_color: RGBColor,
    ) -> Circle {
        if border_width > radius {
            border_width = radius;
        }

        return Circle {
            name,
            x_pos,
            y_pos,
            x_vel,
            y_vel,
            radius,
            border_width,
            body_color,
            border_color,
            mass,
            is_selected: false,
        }
    }

    pub fn clone(&self) -> Circle {
        return Circle {
            name: String::from(&self.name),
            x_pos: self.x_pos,
            y_pos: self.y_pos,
            x_vel: self.x_vel,
            y_vel: self.y_vel,
            radius: self.radius,
            border_width: self.border_width,
            body_color: self.body_color,
            border_color: self.border_color,
            mass: self.mass,
            is_selected: false,
        }
    }

    pub fn move_circle(&mut self, x_range_start: f32, x_range_end: f32, y_range_start: f32, y_range_end: f32) {
        
        if self.x_pos + self.x_vel <= x_range_start + self.radius {

            // when touching start of range, calculate reflection    
            self.x_pos = 2.0 * self.radius - self.x_pos - self.x_vel;
            self.x_vel = -self.x_vel;

        } else if self.x_pos + self.x_vel >= x_range_end - self.radius {

            // when touching end of range, calculate reflection
            self.x_pos = 2.0 * x_range_end - 2.0 * self.radius - self.x_pos - self.x_vel;
            self.x_vel = -self.x_vel;

        } else {
            
            // when in range, just proceed as always
            self.x_pos += self.x_vel;
        }

    
        if self.y_pos + self.y_vel <= y_range_start + self.radius {

            // when touching start of range, calculate reflection    
            self.y_pos = 2.0 * self.radius - self.y_pos - self.y_vel;
            self.y_vel = -self.y_vel;

        } else if self.y_pos + self.y_vel >= y_range_end - self.radius {

            // when touching end of range, calculate reflection
            self.y_pos = 2.0 * y_range_end - 2.0 * self.radius - self.y_pos - self.y_vel;
            self.y_vel = -self.y_vel;

        } else {
            
            // when in range, just proceed as always
            self.y_pos += self.y_vel;
        }
    }

    pub fn collide_with_other_circles(&mut self, circles_array: &Vec<Circle>, own_index: usize) {
        let mut distance_squared: f32;
        let mut sum_radii_squared: f32;

        // let mut self_momentum: f32 = self.mass * f32::sqrt(self.x_vel * self.x_vel + self.y_vel * self.y_vel);
        // let mut collider_momentum: f32;
        // let mut collision_point_x: f32;
        // let mut collision_point_y: f32;
        let mut rel_v_x: f32;
        let mut rel_v_y: f32;
        // let mut abs

        for i in 0..circles_array.len() {
            if i != own_index {
                distance_squared = (circles_array[i].x_pos - self.x_pos) * (circles_array[i].x_pos - self.x_pos) + (circles_array[i].y_pos - self.y_pos) * (circles_array[i].y_pos - self.y_pos);

                sum_radii_squared = (circles_array[i].radius +  self.radius) * (circles_array[i].radius +  self.radius);
    
                if distance_squared < sum_radii_squared {
                    // collider_momentum = circles_array[i].mass *  f32::sqrt(circles_array[i].x_vel * circles_array[i].x_vel + circles_array[i].y_vel * circles_array[i].y_vel);

                // these are relative velocity of collider  
                    rel_v_x = circles_array[i].x_vel - self.x_vel;
                    rel_v_y = circles_array[i].y_vel - self.y_vel;

                    self.x_vel += rel_v_x * circles_array[i].mass / self.mass;
                    self.y_vel += rel_v_y * circles_array[i].mass / self.mass;
                }
            }
        }
    }



    pub fn check_on_top(circle: &Circle, circles_array: &Vec<Circle>, own_index: usize) -> bool {
        let mut is_on_top: bool = false;
        let mut distance_squared: f32;
        let mut sum_radii_squared: f32;

        for i in 0..circles_array.len() {
            if i != own_index {
                distance_squared = (circles_array[i].x_pos - circle.x_pos) * (circles_array[i].x_pos - circle.x_pos) + (circles_array[i].y_pos - circle.y_pos) * (circles_array[i].y_pos - circle.y_pos);

                sum_radii_squared = (circles_array[i].radius +  circle.radius) * (circles_array[i].radius +  circle.radius);
    
                if distance_squared < sum_radii_squared {
                    is_on_top = true;
    
                    break;
                }
            }
        }

        return is_on_top;
    }

    pub fn accelerate_to_position(&mut self, new_x: f32, new_y: f32) {
        self.x_vel = new_x - self.x_pos;
        self.y_vel = new_y - self.y_pos;

        self.x_pos = new_x;
        self.y_pos = new_y;
    }

    /* pub fn put_on_canvas(&self, canvas: &mut RGBCanvas) {
        if self.x_pos >= 0.0 - self.radius
        && self.x_pos < canvas.width + self.radius
        && self.y_pos >= 0.0 - self.radius
        && self.y_pos < canvas.height + self.radius
        {
            let width: i32 = canvas.width as i32;
            let x: i32 = self.x_pos as i32;
            let y: i32 = self.y_pos as i32;
            let box_lx: i32 = if self.x_pos > self.radius {(self.x_pos - self.radius) as i32} else {0};
            let box_hx: i32 = (self.x_pos + self.radius) as i32;
            let box_ly: i32 = if self.y_pos > self.radius {(self.y_pos - self.radius) as i32} else {0};
            let box_hy: i32 = (self.y_pos + self.radius) as i32;
            let squared_outer_radius: i32 = (self.radius * self.radius) as i32;
            let squared_inner_radius: i32 = ((self.radius - self.border_width) * (self.radius - self.border_width)) as i32;
            let mut squared_distance: i32;

            for j in box_ly..box_hy {
                for i in box_lx..box_hx {
                    if i >= 0 && i < width && j >= 0 && j < canvas.height as i32 {
                        squared_distance = (x - i) * (x - i) + (y - j) * (y - j);

                        if squared_distance <= squared_inner_radius {
                            canvas.data[(width * j * 3 + i * 3 + 0) as usize] = self.body_color.r;
                            canvas.data[(width * j * 3 + i * 3 + 1) as usize] = self.body_color.g;
                            canvas.data[(width * j * 3 + i * 3 + 2) as usize] = self.body_color.b;
                        } else if squared_distance <= squared_outer_radius {
                            canvas.data[(width * j * 3 + i * 3 + 0) as usize] = self.border_color.r;
                            canvas.data[(width * j * 3 + i * 3 + 1) as usize] = self.border_color.g;
                            canvas.data[(width * j * 3 + i * 3 + 2) as usize] = self.border_color.b;
                        }
                    }
                }
            }
        }
    } */

    pub fn put_on_canvas_smoothed(&self, canvas: &mut RGBCanvas) {
        if self.x_pos >= 0.0 - self.radius
        && self.x_pos < canvas.width + self.radius
        && self.y_pos >= 0.0 - self.radius
        && self.y_pos < canvas.height + self.radius
        {
            let width: i32 = canvas.width as i32;
            // let x: i32 = self.x_pos as i32;
            // let y: i32 = self.y_pos as i32;
            let box_lx: i32 = if self.x_pos > self.radius {(self.x_pos - self.radius) as i32} else {0};
            let box_hx: i32 = (self.x_pos + self.radius + 2.0) as i32;
            let box_ly: i32 = if self.y_pos > self.radius {(self.y_pos - self.radius) as i32} else {0};
            let box_hy: i32 = (self.y_pos + self.radius + 2.0) as i32;

            let mut distance: f32;

            let inner_radius: f32 = self.radius - self.border_width;
            
            let mut x_f: f32;
            let mut y_f: f32;

            let mut d: f32; //distance from the edge, must be between 0.0 and 1.0

            for j in box_ly..box_hy {
                for i in box_lx..box_hx {
                    if i >= 0 && i < canvas.width as i32&& j >= 0 && j < canvas.height as i32 {
                        x_f = i as f32;
                        y_f = j as f32;

                        distance = f32::sqrt((self.x_pos - x_f) * (self.x_pos - x_f) + (self.y_pos - y_f) * (self.y_pos - y_f));

                        if distance <= inner_radius {
                            canvas.data[(width * j * 3 + i * 3 + 0) as usize] = self.body_color.r;
                            canvas.data[(width * j * 3 + i * 3 + 1) as usize] = self.body_color.g;
                            canvas.data[(width * j * 3 + i * 3 + 2) as usize] = self.body_color.b;
                        } else if distance <= inner_radius + 1.0 {
                            d = distance - inner_radius;
                            canvas.data[(width * j * 3 + i * 3 + 0) as usize] = ((self.body_color.r as f32) * (1.0 - d) + (self.border_color.r as f32) * d) as u8;
                            canvas.data[(width * j * 3 + i * 3 + 1) as usize] = ((self.body_color.g as f32) * (1.0 - d) + (self.border_color.g as f32) * d) as u8;
                            canvas.data[(width * j * 3 + i * 3 + 2) as usize] = ((self.body_color.b as f32) * (1.0 - d) + (self.border_color.b as f32) * d) as u8;
                        } else if distance <= self.radius {
                            canvas.data[(width * j * 3 + i * 3 + 0) as usize] = self.border_color.r;
                            canvas.data[(width * j * 3 + i * 3 + 1) as usize] = self.border_color.g;
                            canvas.data[(width * j * 3 + i * 3 + 2) as usize] = self.border_color.b;
                        } else if distance <= self.radius + 1.0 {
                            d = distance - self.radius;
                            canvas.data[(width * j * 3 + i * 3 + 0) as usize] = ((self.border_color.r as f32) * (1.0 - d) + (canvas.data[(width * j * 3 + i * 3 + 0) as usize] as f32) * d) as u8;
                            canvas.data[(width * j * 3 + i * 3 + 1) as usize] = ((self.border_color.g as f32) * (1.0 - d) + (canvas.data[(width * j * 3 + i * 3 + 1) as usize] as f32) * d) as u8;
                            canvas.data[(width * j * 3 + i * 3 + 2) as usize] = ((self.border_color.b as f32) * (1.0 - d) + (canvas.data[(width * j * 3 + i * 3 + 2) as usize] as f32) * d) as u8;
                        }
                    }
                }
            }
        }
    }
}