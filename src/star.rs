use rand::Rng;

const MAX_BRIGHTNESS : u8 = 255;
const MAX_RADIUS     : u8 = 2;

pub struct Star {
    x: f64,
    y: f64,
    z: f64,
    max_x: f64,
    max_y: f64,
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

impl Star {
    pub fn new(height: usize, width: usize) -> Self {
        let mut rng = rand::thread_rng();
        Star {
            x: rng.gen_range(0, width) as f64,
            y: rng.gen_range(0, height) as f64,
            z: width as f64,
            max_x: width as f64,
            max_y: height as f64
        }
    }

    pub fn update(&mut self, pos: i32) -> (usize, usize, u8, u8) {
        let mut rng = rand::thread_rng();

        let step = map_range((0.0, self.max_x), (0.0, 40.0), pos as f64);

        if self.z >= step {
            self.z -= step;
        }

        let min_speed = map_range((0.0, self.max_x), (1.0, 1.04), pos as f64);
        let max_speed = map_range((0.0, self.max_x), (1.0, 1.08), pos as f64);

        if self.x >= self.max_x || self.y >= self.max_y || self.x <= 0.0 || self.y <= 0.0 {
            self.x = rng.gen_range(0, self.max_x as usize) as f64;
            self.y = rng.gen_range(0, self.max_y as usize) as f64;
            self.z = self.max_x;
        }

        let pz = map_range((0.0, self.max_x), (max_speed, min_speed), self.z);
        let brightness =
            MAX_BRIGHTNESS - map_range((0.0, self.max_x), (0.0, MAX_BRIGHTNESS as f64), self.z) as u8;
        let radius =
            MAX_RADIUS - map_range((0.0, self.max_x), (0.0, MAX_RADIUS as f64), self.z) as u8;

        self.x = (self.x - self.max_x / 2.0) * pz + self.max_x / 2.0;
        self.y = (self.y - self.max_y / 2.0) * pz + self.max_y / 2.0;

        (self.x as usize, self.y as usize, brightness, radius)
    }
}
