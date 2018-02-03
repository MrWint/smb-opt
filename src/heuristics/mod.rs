pub mod xpos;
pub mod ypos;

use emu::EmuResult;
use options::{Options, Platform};
use state::{Dist, State};
use std::cmp::{max, min};
use std::marker::PhantomData;

pub trait SearchGoal {
  fn new() -> Self;
  fn distance_to_goal_heuristic(&self, s: &mut State, steps_already_taken: Dist) -> Option<Dist>;
  fn is_goal_state(&self, s: &State, emu_result: &EmuResult) -> bool;

  fn track_metric(&mut self, _: &State) -> () {}
  fn report_metrics(&self) -> () {}
}

#[allow(dead_code)]
pub struct ImpossibleSearchGoal;
impl SearchGoal for ImpossibleSearchGoal {
  fn new() -> Self { return Self {}; }
  fn distance_to_goal_heuristic(&self, _: &mut State, _: Dist) -> Option<Dist> { Some(0) }
  fn is_goal_state(&self, _: &State, _: &EmuResult) -> bool { false }
}

#[allow(dead_code)]
pub struct MaxXPosMetric<P: Platform> {
  max_x_pos: i32,
  _platform: PhantomData<P>,
}
impl<P: Platform> SearchGoal for MaxXPosMetric<P> {
  fn new() -> Self { MaxXPosMetric { max_x_pos: ::std::i32::MIN, _platform: PhantomData } }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(min_x_pos_heuristic::<P>(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, _: &State, _: &EmuResult) -> bool { false }
  fn track_metric(&mut self, s: &State) -> () {
    if self.max_x_pos < s.x_pos {
      self.max_x_pos = s.x_pos;
      println!("new best max_x_pos: {:x}", self.max_x_pos);
    }
  }
  fn report_metrics(&self) -> () {
    println!("best max_x_pos so far: {:x}", self.max_x_pos);
  }
}

#[allow(dead_code)]
pub fn min_x_speed_heuristic<P: Platform>(s: &State, target_x_spd: i16) -> Dist {
  if s.x_spd >= target_x_spd { 0 } else { ((target_x_spd - s.x_spd + 2 * P::FRICTION_RUN - 1) / 2 / P::FRICTION_RUN) as u16 }
}
pub fn min_x_pos_heuristic<P: Platform>(s: &State, target_x_pos: i32) -> Dist {
  let mut x_pos = s.x_pos;
  let mut x_spd = s.x_spd;
  let mut steps = 0;
  while target_x_pos > x_pos {
    steps += 1;
    x_spd = min(x_spd + 2 * P::FRICTION_RUN, P::MAX_X_SPD_RUN);
    x_pos += (x_spd as i32 >> 8) << 4;
  }
  steps
}

pub struct BoundsHeuristic {
  x_heuristic: xpos::XPosHeuristic,
  y_heuristic: ypos::YPosHeuristic,
}
impl BoundsHeuristic {
  pub fn new<O: Options>(initial_states: &Vec<State>) -> BoundsHeuristic {
    BoundsHeuristic {
      x_heuristic: xpos::XPosHeuristic::new::<O>(initial_states),
      y_heuristic: ypos::YPosHeuristic::new::<O>(initial_states),
    }
  }
  #[allow(dead_code)]
  pub fn get_steps_until_x_pos_at_least(&self, s: &State, target_x_pos: i32) -> Dist {
    self.x_heuristic.get_steps_until_x_pos_at_least(s, target_x_pos)
  }
  #[allow(dead_code)]
  pub fn get_steps_until_x_pos_at_most(&self, s: &State, target_x_pos: i32) -> Dist {
    self.x_heuristic.get_steps_until_x_pos_at_most(s, target_x_pos)
  }
  #[allow(dead_code)]
  pub fn get_steps_until_x_pos_between(&self, s: &State, min_x_pos: i32, max_x_pos: i32) -> Dist {
    self.x_heuristic.get_steps_until_x_pos_between(s, min_x_pos, max_x_pos)
  }
  #[allow(dead_code)]
  pub fn get_steps_until_y_pos_at_least(&self, s: &State, target_y_pos: i32) -> Dist {
    self.y_heuristic.get_steps_until_y_pos_at_least(s, target_y_pos)
  }
  #[allow(dead_code)]
  pub fn get_steps_until_y_pos_at_most(&self, s: &State, target_y_pos: i32) -> Dist {
    self.y_heuristic.get_steps_until_y_pos_at_most(s, target_y_pos)
  }
  #[allow(dead_code)]
  pub fn get_steps_until_y_pos_between(&self, s: &State, min_y_pos: i32, max_y_pos: i32) -> Dist {
    self.y_heuristic.get_steps_until_y_pos_between(s, min_y_pos, max_y_pos)
  }
  #[allow(dead_code)]
  pub fn get_steps_until_bounds_at_least(&self, s: &State, target_x_pos: i32, target_y_pos: i32) -> Dist {
    max(self.x_heuristic.get_steps_until_x_pos_at_least(s, target_x_pos), self.y_heuristic.get_steps_until_y_pos_at_least(s, target_y_pos))
  }
  #[allow(dead_code)]
  pub fn get_steps_until_bounds_at_most(&self, s: &State, target_x_pos: i32, target_y_pos: i32) -> Dist {
    max(self.x_heuristic.get_steps_until_x_pos_at_most(s, target_x_pos), self.y_heuristic.get_steps_until_y_pos_at_most(s, target_y_pos))
  }
  #[allow(dead_code)]
  pub fn get_steps_until_bounds_between(&self, s: &State, min_x_pos: i32, max_x_pos: i32, min_y_pos: i32, max_y_pos: i32) -> Dist {
    max(self.x_heuristic.get_steps_until_x_pos_between(s, min_x_pos, max_x_pos), self.y_heuristic.get_steps_until_y_pos_between(s, min_y_pos, max_y_pos))
  }
}
