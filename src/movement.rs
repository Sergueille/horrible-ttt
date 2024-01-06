
use crate::state::State;
use core::ops::{*};
use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub enum EaseType {
    Linear, Ease
}

#[derive(Copy, Clone)]
pub struct Movement<T> 
    where T: Sub<T, Output = T> + Add<T, Output = T> + Mul<f32, Output = T> + Copy {
    pub start: T,
    pub end: T,
    pub duration: f32,
    pub ease_type: EaseType,
    pub callback: fn(&T, &mut State),

    start_time: f32,
    active: bool,
}

impl<T> Movement<T>
    where T: Sub<T, Output = T> + Add<T, Output = T> + Mul<f32, Output = T> + Copy {
    pub fn new(start: T, end: T, duration: f32, ease_type: EaseType, callback: fn(&T, &mut State)) -> Movement<T> {
        return Movement {
            start, end, duration, ease_type,
            active: false,
            start_time: 0.0,
            callback,
        };
    }

    pub fn restart(&mut self, time: &crate::time::Time) {
        self.start_time = time.time;
        self.active = true;
    }

    pub fn update(mut self, state: &mut State) -> Movement<T>
        where T: Sub<T, Output = T> + Add<T, Output = T> + Mul<f32, Output = T> + Copy {
        if !self.active { return self; }

        let mut t = (state.time.time - self.start_time) / self.duration;

        if t >= 1.0 {
            self.active = false;
            t = 1.0;
        }

        let value = (self.end - self.start) * get_eased_value(t, self.ease_type) + self.start;
        (self.callback)(&value, state);

        return self;
    }

    pub fn stop(&mut self){
        self.active = false;
    }
}

pub fn get_eased_value(t: f32, ease_type: EaseType) -> f32 {
    match ease_type {
        EaseType::Linear => return t,
        EaseType::Ease => return (PI * t - PI / 2.0).sin() / 2.0 + 0.5,
    };
}


