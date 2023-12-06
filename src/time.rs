
use std::time::SystemTime;

pub struct Time {
    pub time: f32,
    pub delta_time: f32,
    start_time: SystemTime,
}

pub fn init_time() -> Time {
    return Time {time: 0.0, delta_time: 0.0, start_time: SystemTime::now()};
}

impl Time {
    pub fn update(self: &mut Time) {
        let new_time = SystemTime::now()
            .duration_since(self.start_time)
            .expect("Uuuh?")
            .as_secs_f32();
    
        self.delta_time = new_time - self.time;
        self.time = new_time;
    }
}

