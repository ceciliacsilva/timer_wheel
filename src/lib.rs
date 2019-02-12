//! Simple wheel timer with bounded interval.
//!
//! Relevant:
//! http://www.cs.columbia.edu/~nahum/w6998/papers/sosp87-timing-wheels.pdf

#![no_std]
extern crate heapless;

use heapless::Vec;
use heapless::ArrayLength;

/// Fixed ring size
pub struct TimerWheel<T, RINGLEN, TICKLEN>
where
    TICKLEN: ArrayLength<T>,
    RINGLEN: ArrayLength<Vec<T, TICKLEN>>,
{
    max_interval: usize,
    current_tick: usize,
    size: usize,
    ring: Vec<Vec<T, TICKLEN>, RINGLEN>,
}

impl<T, RINGLEN, TICKLEN> TimerWheel<T, RINGLEN, TICKLEN>
where
    TICKLEN: ArrayLength<T>,
    RINGLEN: ArrayLength<Vec<T, TICKLEN>>,
{
    pub fn new() -> TimerWheel<T, RINGLEN, TICKLEN>
    where
        TICKLEN: ArrayLength<T>,
        RINGLEN: ArrayLength<Vec<T, TICKLEN>>,
    {
        let mut ring: Vec<_, RINGLEN> = Vec::new();
        let max_interval = ring.capacity() - 1;

        for _ in 0..max_interval {
            let mut each_tick = Vec::<T, TICKLEN>::new();
            let _ = ring.push(each_tick);
        }

        TimerWheel{
            max_interval,
            current_tick: 0,
            ring,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn tick(&mut self) -> &mut Vec<T, TICKLEN>
    where
        TICKLEN: ArrayLength<T>,
    {
        let node = &mut self.ring[self.current_tick];
        self.current_tick = (self.current_tick + 1) % self.max_interval;
        self.size = self.size - node.len();
        node
    }

    pub fn clear_node(&mut self) {
        let previous_tick = if self.current_tick == 0 { self.max_interval } else { (self.current_tick - 1) };
        let node = &mut self.ring[previous_tick];
        node.clear();
    }

    pub fn schedule(&mut self, ticks: usize, value: T) -> Result<(), T> {
        let index = (self.current_tick + ticks) % self.max_interval;
        self.ring[index].push(value)?;
        self.size = self.size + 1;
        Ok(())
    }
}

#[cfg(test)]
extern crate std;
extern crate rtfm;
mod tests {
    #[test]
    fn one_schedule_one_tick() {
        use crate::TimerWheel;

        let mut wt = TimerWheel::<_, heapless::consts::U8, heapless::consts::U4>::new();
        wt.schedule(0, 1).unwrap();
        let to_run = wt.tick();
        assert_eq!(to_run[0], 1);
    }

    #[test]
    fn schedule_lambda() {
        use crate::TimerWheel;

        let mut wt = TimerWheel::<_, heapless::consts::U8, heapless::consts::U4>::new();
        let f = || { 2+2 };
        let _ = wt.schedule(0, f);
        assert_eq!(wt.tick()[0](), 4);
    }

    #[test]
    fn on_rtfm() {
        use crate::TimerWheel;

        let mut wt = TimerWheel::<_, heapless::consts::U8, heapless::consts::U4>::new();
        let _ = wt.schedule(0, rtfm::export::run(to_call));

        wt.tick()[0];
    }

    #[test]
    fn to_call(){
        assert_eq!(true, true);
    }

    #[test]
    fn tick_multiples() {
        use crate::TimerWheel;

        let mut wt = TimerWheel::<_, heapless::consts::U8, heapless::consts::U4>::new();

        wt.schedule(0, 1).unwrap();
        wt.schedule(3, 2).unwrap();

        {
            let _tick_1 = wt.tick();
        }

        {
            let _tick_2 = wt.tick();
        }

        {
            let _tick_3 = wt.tick();
        }

    }

    #[test]
    fn clean_schedule_tick() {
        use crate::TimerWheel;

        let mut wt = TimerWheel::<_, heapless::consts::U8, heapless::consts::U4>::new();

        wt.schedule(0, 1).unwrap();
        wt.schedule(0, 2).unwrap();

        {
            let _tick = wt.tick();
        }
        {
            wt.clear_node();
        }
    }

    #[test]
    fn tick_pop() {
        use crate::TimerWheel;

        let mut wt = TimerWheel::<_, heapless::consts::U8, heapless::consts::U4>::new();

        wt.schedule(0, 1).unwrap();
        wt.schedule(0, 2).unwrap();

        let tick = wt.tick();

        while let Some(e) = tick.pop() {
            let _ = e + 1;
        }
    }
}
