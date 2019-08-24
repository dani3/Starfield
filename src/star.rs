use rand::Rng;

pub struct Star {
    x: f64,
    y: f64,
    z: f64,
    r: usize,
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
            r: 1,
            max_x: width as f64,
            max_y: height as f64
        }
    }

    pub fn get_radius(&self) -> usize {
        self.r
    }

    pub fn update(&mut self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        self.z -= 1.0;

        let pz = 1.0 - map_range((0.0, self.max_x), (0.95, 0.995), self.z);

        if self.x > self.max_x || self.y > self.max_y || self.x < 0.0 || self.y < 0.0 {
            self.x = rng.gen_range(0, self.max_x as usize) as f64;
            self.y = rng.gen_range(0, self.max_y as usize) as f64;
            self.z = self.max_x;
        }

        self.x = (self.x - self.max_x / 2.0) * (1.0 + pz) + self.max_x / 2.0;
        self.y = (self.y - self.max_y / 2.0) * (1.0 + pz) + self.max_y / 2.0;

        (self.x as usize, self.y as usize)
    }
}
