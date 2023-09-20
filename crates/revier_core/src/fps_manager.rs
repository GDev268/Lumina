use std::time::{Instant,UNIX_EPOCH,Duration}; 

pub struct FPS{
    pub _fps:u32,
    max_milliseconds:i32,
    pub fps_limit:Duration,
    pub frame_count:f64,
    pub frame_elapsed:f64,
    elapsed_start:Instant,
}

impl FPS {
    pub fn new() -> Self {
        return Self{
            _fps: 0,
            max_milliseconds: 0,
            fps_limit: Duration::new(0,0),
            frame_count:0f64,
            frame_elapsed:0f64,
            elapsed_start: Instant::now(),
        };
    }

    pub fn update(&mut self){
        self.frame_count += 1f64;
        self.frame_elapsed = self.elapsed_start.elapsed().as_secs() as f64 +
        (self.elapsed_start.elapsed().subsec_nanos() as f64 / 1000000000f64);
    }
}
