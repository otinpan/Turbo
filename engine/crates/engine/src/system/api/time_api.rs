use crate::{Time};

pub trait TimeApi {
    fn time(&self) -> &Time;

    fn delta_seconds(&self) -> f32 {
        self.time().delta_seconds()
    }

    fn elapsed_seconds(&self) -> f32 {
        self.time().elapsed_seconds()
    }
}