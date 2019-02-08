//! Simple wheel timer with bounded interval.
//!
//! Relevant:
//! http://www.cs.columbia.edu/~nahum/w6998/papers/sosp87-timing-wheels.pdf

#![no_std]
extern crate heapless;

use heapless::Vec;
use heapless::consts::{U8, U4};

/// Fixed ring size
pub struct WheelTimer<T> {
    max_interval: usize,
    current_tick: usize,
    size: usize,
    ring: Vec<Vec<T, U4>, U8>,
}

/// Implementing Iterator trait for `WheelTimer`
impl<T> Iterator for WheelTimer<T>
{
    type Item = Vec<T, U4>;

    fn next(&mut self) -> Option<Self::Item> {
        let _size = self.size();

        // if size > 0 {
        //     Some(self.tick())
        // } else {
        //     None
        // }
        None
    }
}

impl<T> WheelTimer<T> {
    pub fn new() -> WheelTimer<T>
    {
        let mut ring: Vec<_, U8> = Vec::new();
        let max_interval = 8;

        for _ in 0..max_interval {
            ring.push(Vec::<T, U4>::new());
        }

        WheelTimer{
            max_interval,
            current_tick: 0,
            ring,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn tick(&mut self) -> &Vec<T, U4> {
        let node = &self.ring[self.current_tick];
        self.current_tick = (self.current_tick + 1) % self.max_interval;
        self.size = self.size - node.len();
        node
    }

    pub fn schedule(&mut self, ticks: usize, value: T) {
        let index = (self.current_tick + ticks) % self.max_interval;
        self.ring[index].push(value);
        self.size = self.size + 1;
    }
}

#[cfg(test)]
extern crate std;
extern crate rtfm;
mod tests {
    #[test]
    fn one_schedule_one_tick() {
        use crate::WheelTimer;

        let mut wt = WheelTimer::new();
        wt.schedule(0, 1);
        let to_run = wt.tick();
        assert_eq!(to_run[0], 1);
    }

    #[test]
    fn schedule_lambda() {
        use crate::WheelTimer;

        let mut wt = WheelTimer::new();
        let f = || { 2+2 };
        wt.schedule(0, f);
        assert_eq!(wt.tick()[0](), 4);
    }

    #[test]
    fn on_rtfm() {
        use crate::WheelTimer;

        let mut wt = WheelTimer::new();
        wt.schedule(0, rtfm::export::run(a));

        wt.tick()[0];
    }

    fn a(){
        assert_eq!(true, true);
    }
}
