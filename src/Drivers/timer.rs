use std::time::Instant;

pub struct Timer(u128);

impl Timer {
    pub fn new(interval_ms: u128) -> Timer {
        Timer(interval_ms)
    }

    pub fn run(&self, f: fn()) {
        
        let now = Instant::now();
        if (now.elapsed().as_millis() % self.0) == 0 {
            f(); 
        };
        self.run(f);
    }
}
