// here, the object describing the state

use crate::{circle::Circle, common_structs::RGBCanvas};


pub struct State {
    pub width: f32, //world width
    pub height: f32, //world height
    background: Vec<u8>, //array containing rgb values for background image
    rendered_scene: RGBCanvas,
    pub circles: Vec<Circle>,
    pub selected_circle_index: usize,
    pub has_selected_circle: bool,
}

impl State {
    pub fn new(width: i32, height: i32) -> State {
        return State {
            width: width as f32,
            height: height as f32,
            background: State::create_background(width, height),
            rendered_scene: RGBCanvas::new(width as f32, height as f32),
            circles: Vec::<Circle>::new(),
            selected_circle_index: 0,
            has_selected_circle: false,
        };
    }

    fn render(&mut self) {
        self.rendered_scene = RGBCanvas::new(self.width, self.height);
        
        for i in 0..self.background.len() {
            self.rendered_scene.data[i] = self.background[i];
        }

        for i in 0..self.circles.len() {
            self.circles[i].put_on_canvas(&mut self.rendered_scene);
        }
    }

    pub fn get_rendered_view(&self) -> RGBCanvas {
        return self.rendered_scene.copy();
    }

/////////////////////////////////////////////////////////
    
    pub fn add_circle(&mut self, circle: Circle) {
        self.circles.push(circle);
        self.has_selected_circle = false;

        self.render();
    }

    pub fn select_circle(&mut self, x: i32, y: i32) {
        let x_pos: f32 = x as f32;
        let y_pos: f32 = y as f32;

        let mut is_selected: bool = false;
        let mut selected_index: usize = 0;
        // let
        
        for (index, circle) in self.circles.iter().enumerate().rev() {
            if (x_pos - circle.x_pos) * (x_pos - circle.x_pos) < circle.radius * circle.radius
            && (y_pos - circle.y_pos) * (y_pos - circle.y_pos) < circle.radius * circle.radius
            {
                selected_index = index;
                is_selected = true;

                println!("Circle with index {} is selected", index);
                break;
            }
        }

        if is_selected {
            println!("Circle was selected");
            let selected_circle = self.circles.remove(selected_index);
            self.circles.push(selected_circle);
            self.selected_circle_index = self.circles.len() - 1;
        } else {
            println!("No selection");
        }
        
        self.has_selected_circle = is_selected;

        self.render();
    }

    pub fn remove_circle(&mut self) {
        let mut new_circles_array: Vec<Circle> = Vec::new();

        if self.has_selected_circle {
            for i in 0..self.circles.len() {
                if i != self.selected_circle_index {
                    new_circles_array.push(self.circles[i].clone());
                }
            }

            self.circles = new_circles_array;
        }
        
        self.has_selected_circle = false;
        self.render();
    }

    pub fn replace_background(&mut self, new_background: Vec<u8>) {
        if new_background.len() == self.background.len() {
            self.background = new_background;
        } else {
            panic!("Error replacing background: The new background should be the same size as old one!");
        }
    }

    pub fn progress_one_step(&mut self) {
        for i in 0..self.circles.len() {
            self.circles[i].move_circle();
        }

        self.render();
    }

    fn get_background(&self) -> &Vec<u8> {
        return &self.background;
    }

    fn create_background(width: i32, height: i32) -> Vec<u8>{
        let num_pix: usize = (width * height) as usize;
        let data_array: Vec<u8>;
    
        data_array = vec![0; num_pix * 3];
    
        return data_array;
    }
}