use rand::Rng;

pub struct Star {
    x: usize,
    y: usize,
    z: usize,
    r: usize,
    max_x: usize,
    max_y: usize,
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

impl Star {
    pub fn new(height: usize, width: usize) -> Self {
        let mut rng = rand::thread_rng();
        Star {
            x: rng.gen_range(0, width),
            y: rng.gen_range(0, height),
            z: rng.gen_range(0, width),
            r: 1,
            max_x: width,
            max_y: height
        }
    }

    fn init(&mut self) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(1, self.max_x);
        self.y = rng.gen_range(1, self.max_y);
        self.z = rng.gen_range(1, self.max_x);
    }

    pub fn get_radius(&self) -> usize {
        self.r
    }

    pub fn update(&mut self) -> (usize, usize) {
        let mut rng = rand::thread_rng();

        self.z -= 1;
        if self.z < 1 {
            self.z = rng.gen_range(0, self.max_x)
        }

        let sx = map_range((0 as f64, 1 as f64), (0 as f64, self.max_x as f64), self.x as f64 / self.z as f64);
        let sy = map_range((0 as f64, 1 as f64), (0 as f64, self.max_y as f64), self.y as f64 / self.z as f64);

        (sx as usize, sy as usize)
    }
}
