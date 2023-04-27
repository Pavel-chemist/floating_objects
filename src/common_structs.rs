#[derive(Copy, Clone)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct RGBCanvas {
    pub width: f32,
    pub height: f32,
    pub data: Vec<u8>,
}

impl RGBCanvas {
    pub fn new(width: f32, height: f32) -> RGBCanvas {
        return RGBCanvas {
            width,
            height,
            data: vec![0; (width * height * 3.0) as usize],
        };
    }

    pub fn copy(&self) -> RGBCanvas {
        let mut copied_data: Vec<u8> = vec![0; self.data.len()];

        for i in 0..self.data.len() {
            copied_data[i] = self.data[i];
        }

        return RGBCanvas {
            width: self.width,
            height: self.height,
            data: copied_data,
        };
    }
}