use options::{Options, Platform, Swim};
use state::{Dir, Dist, State};
use std::cmp::{max,min};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::marker::PhantomData;

pub struct XPosHeuristic {
  max_distance: HashMap<XPosState, Vec<i32>>,
  min_distance: HashMap<XPosState, Vec<i32>>,
  max_x_distance: i32,
}
impl XPosHeuristic {
  pub fn new<O: Options>(initial_states: &Vec<State>) -> XPosHeuristic {
    let mut h = XPosHeuristic {
      max_distance: HashMap::<XPosState, Vec<i32>>::new(),
      min_distance: HashMap::<XPosState, Vec<i32>>::new(),
      max_x_distance: ((if O::Swim::IS_SWIMMING { O::Platform::MAX_X_SPD_WALK } else { O::Platform::MAX_X_SPD_RUN } >> 8) << 4) as i32,
    };
    let state_map = Self::build_state_map::<O>(initial_states.iter().map(|s| Self::to_x_pos_state(s)).collect());
    println!("total number of XPosState: {}; total size of state map: {}", state_map.len(), state_map.values().map(|v| v.len()).sum::<usize>());
    h.precompute_distances(state_map);
    h
  }
  fn build_state_map<O: Options>(initial_states: Vec<XPosState>) -> HashMap<XPosState, Vec<(XPosState, i32)>> {
    let mut state_map: HashMap<XPosState, Vec<(XPosState, i32)>> = HashMap::new();

    let mut stack = initial_states;
    while let Some(s) = stack.pop() {
      if state_map.contains_key(&s) { continue; }
      let mut next_states = XPosEmu::<O>::run_step(vec![s.clone()], ALLOW_SIDE_COLLISIONS);
      state_map.insert(s.clone(), next_states.clone().into_iter().map(|mut next_state| {
        let dist = next_state.x_pos - s.x_pos;
        next_state.x_pos = 0;
        (next_state, dist)
      }).collect());
      if state_map.len() % 100000 == 0 { println!("{}", state_map.len()); }
      stack.append(&mut XPosEmu::<O>::run_step(vec![s.clone()], true).into_iter().map(|mut s| { s.x_pos = 0; s }).collect());
    }

    state_map
  }
  fn to_x_pos_state(s: &State) -> XPosState {
    XPosState {
      x_pos: 0,
      x_spd: s.x_spd,
      x_spd_abs: s.x_spd_abs,
      moving_dir: s.moving_dir,
      facing_dir: s.facing_dir,
      is_on_ground: s.is_on_ground(),
      running_speed: s.running_speed,
    }
  }
  pub fn get_steps_until_x_pos_at_least(&self, s: &State, target_x_pos: i32) -> Dist {
    if target_x_pos <= s.x_pos { return 0; }
    let distance = target_x_pos - s.x_pos;
    let s = Self::to_x_pos_state(s);
    if let Some(max_dists) = self.max_distance.get(&s) {
      let len = max_dists.len();
      if len == 0 { ((distance + self.max_x_distance - 1) / self.max_x_distance) as Dist }
      else if max_dists[len-1] < distance { (len as i32 + (distance - max_dists[len-1] + self.max_x_distance - 1) / self.max_x_distance) as Dist }
      else { max_dists.iter().position(|d| *d >= distance).unwrap() as Dist + 1 }
    } else { panic!("trying to determine XPos heuristic for unknown state {}", s); }
  }
  pub fn get_steps_until_x_pos_at_most(&self, s: &State, target_x_pos: i32) -> Dist {
    if target_x_pos >= s.x_pos { return 0; }
    let distance = target_x_pos - s.x_pos;
    let s = Self::to_x_pos_state(s);
    if let Some(min_dists) = self.min_distance.get(&s) {
      let len = min_dists.len();
      if len == 0 { ((-distance + self.max_x_distance - 1) / self.max_x_distance) as Dist }
      else if min_dists[len-1] > distance { (len as i32 + (min_dists[len-1] - distance + self.max_x_distance - 1) / self.max_x_distance) as Dist }
      else { min_dists.iter().position(|d| *d <= distance).unwrap() as Dist + 1 }
    } else { panic!("trying to determine XPos heuristic for unknown state {}", s); }
  }
  pub fn get_steps_until_x_pos_between(&self, s: &State, min_x_pos: i32, max_x_pos: i32) -> Dist {
    if s.x_pos < min_x_pos { self.get_steps_until_x_pos_at_least(s, min_x_pos) }
    else { self.get_steps_until_x_pos_at_most(s, max_x_pos) }
  }
  fn precompute_distances(&mut self, state_map: HashMap<XPosState, Vec<(XPosState, i32)>>) {
    for s in state_map.keys() { self.max_distance.insert(s.clone(), vec![]); }
    for s in state_map.keys() { self.min_distance.insert(s.clone(), vec![]); }
    let mut steps = 0;
    let mut changed_cache = true;
    print!("XPos precompute_distances:");
    ::std::io::stdout().flush().ok();
    while changed_cache {
      steps += 1;
      print!(" {},", steps);
      ::std::io::stdout().flush().ok();
      changed_cache = false;
      for (s, next_states) in state_map.iter() {
        if self.max_distance.get(s).unwrap().len() == steps - 1 {
          let mut max_dist = ::std::i32::MIN;
          for &(ref next_state, x_dist) in next_states {
            max_dist = max(max_dist, self.get_max_distance(next_state, steps-1) + x_dist);
          }
          if max_dist - self.get_max_distance(s, steps-1) < self.max_x_distance {
            self.max_distance.get_mut(s).unwrap().push(max_dist);
            changed_cache = true;
          }
        }
        if self.min_distance.get(s).unwrap().len() == steps - 1 {
          let mut min_dist = ::std::i32::MAX;
          for &(ref next_state, x_dist) in next_states {
            min_dist = min(min_dist, self.get_min_distance(next_state, steps-1) + x_dist);
          }
          if min_dist - self.get_min_distance(s, steps-1) > -self.max_x_distance {
            self.min_distance.get_mut(s).unwrap().push(min_dist);
            changed_cache = true;
          }
        }
      }
    }
    println!();
  }
  pub fn get_max_x_pos_after_steps(&self, s: &State, steps: usize) -> i32 {
    s.x_pos + self.get_max_distance(&Self::to_x_pos_state(s), steps)
  }
  fn get_max_distance(&self, s: &XPosState, steps: usize) -> i32 {
    let max_dists = self.max_distance.get(s).unwrap();
    let len = max_dists.len();
    if steps == 0 { 0 }
    else if len == 0 { self.max_x_distance * steps as i32 }
    else if len >= steps { max_dists[steps-1] }
    else { max_dists[len-1] + self.max_x_distance * (steps - len) as i32 }
  }
  fn get_min_distance(&self, s: &XPosState, steps: usize) -> i32 {
    let min_dists = self.min_distance.get(s).unwrap();
    let len = min_dists.len();
    if steps == 0 { 0 }
    else if len == 0 { -self.max_x_distance * steps as i32 }
    else if len >= steps { min_dists[steps-1] }
    else { min_dists[len-1] - self.max_x_distance * (steps - len) as i32 }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct XPosState {
  x_pos: i32,
  x_spd: i16,
  x_spd_abs: u8,
  moving_dir: Dir,
  facing_dir: Dir,
  is_on_ground: bool,
  running_speed: bool,
}
impl ::std::fmt::Display for XPosState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    writeln!(f, "XPosState {{")?;
    writeln!(f, "  x_pos: {:#x}", self.x_pos)?;
    writeln!(f, "  x_spd: {:#x}", self.x_spd)?;
    writeln!(f, "  x_spd_abs: {}", self.x_spd_abs)?;
    writeln!(f, "  moving_dir: {:?}", self.moving_dir)?;
    writeln!(f, "  facing_dir: {:?}", self.facing_dir)?;
    writeln!(f, "  is_on_ground: {:?}", self.is_on_ground)?;
    writeln!(f, "  running_speed: {:?}", self.running_speed)?;
    write!(f, "}}")
  }
}

const ALLOW_SIDE_COLLISIONS: bool = true;
enum Void {}
pub struct XPosEmu<O: Options> {
  _options: PhantomData<O>,
  _void: Void,
}
impl<O: Options> XPosEmu<O> {
  fn run_step(states: Vec<XPosState>, allow_side_collisions: bool) -> HashSet<XPosState> {
    let states = Self::player_movement_subs(states);

    let states: HashSet<XPosState> = states.into_iter().map(|mut s| {
      if s.x_spd < 0  { s.moving_dir = Dir::LEFT; }
      else if s.x_spd >= 0x100 { s.moving_dir = Dir::RIGHT; }
      s
    }).collect();

    Self::player_bg_collision(states, allow_side_collisions)
  }
  fn player_movement_subs(states: Vec<XPosState>) -> HashSet<XPosState> {
    let states = Self::maybe_start_jumping(states);

    let states: HashSet<XPosState> = states.into_iter().flat_map(|s| {
      let mut result: Vec<XPosState> = Vec::new();
      for &(lr, joypad_lr) in &[(Dir::empty(), Dir::empty()), (Dir::LEFT, Dir::empty()), (Dir::RIGHT, Dir::empty()), (Dir::LR, Dir::empty()), (Dir::LEFT, Dir::LEFT), (Dir::RIGHT, Dir::RIGHT), (Dir::LR, Dir::LR)] {
        for (friction, max_speed) in Self::player_physics_sub(&s, joypad_lr) {
          result.append(&mut Self::move_subs(&s, lr, joypad_lr, friction, max_speed));
        }
      }
      result
    }).collect();

    Self::move_horizontally(states)
  }
  fn maybe_start_jumping(states: Vec<XPosState>) -> HashSet<XPosState> {
    states.into_iter().flat_map(|s| {
      if s.is_on_ground {
        let mut jumping = s.clone();
        jumping.is_on_ground = false;
        vec![s, jumping]
      } else {
        vec![s]
      }
    }).collect()
  }
  fn player_physics_sub(s: &XPosState, joypad_lr: Dir) -> Vec<(i16, i16)> {

    let mut result = if !s.is_on_ground && s.x_spd_abs >= O::Platform::X_SPD_ABS_CUTOFFS[3] {
      vec![(O::Platform::FRICTION_RUN, O::Platform::MAX_X_SPD_RUN)]
    } else if !s.is_on_ground && s.running_speed {
      vec![(O::Platform::FRICTION_WALK_FAST, O::Platform::MAX_X_SPD_WALK)]
    } else if !s.is_on_ground {
      vec![(O::Platform::FRICTION_WALK_SLOW, O::Platform::MAX_X_SPD_WALK)]
    } else if O::Swim::IS_SWIMMING && !s.running_speed && s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[5] {
      vec![(O::Platform::FRICTION_WALK_SLOW, O::Platform::MAX_X_SPD_SWIM)]
    } else if O::Swim::IS_SWIMMING {
      vec![(O::Platform::FRICTION_WALK_FAST, O::Platform::MAX_X_SPD_SWIM)]
    } else {
      let mut inner_result = Vec::new();
      if joypad_lr == s.moving_dir {
        inner_result.push((O::Platform::FRICTION_RUN, O::Platform::MAX_X_SPD_RUN));
      }
      if !s.running_speed && s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[5] {
        inner_result.push((O::Platform::FRICTION_WALK_SLOW, O::Platform::MAX_X_SPD_WALK));
      } else {
        inner_result.push((O::Platform::FRICTION_WALK_FAST, O::Platform::MAX_X_SPD_WALK));
      }
      inner_result
    };

    if s.facing_dir != s.moving_dir {
      for &mut (ref mut friction, _) in &mut result { *friction <<= 1; }
    }
    result
  }
  fn move_subs(s: &XPosState, lr: Dir, joypad_lr: Dir, friction: i16, max_speed: i16) -> Vec<XPosState> {
    let mut result: Vec<XPosState> = vec![s.clone()];
    if s.is_on_ground || O::Swim::IS_SWIMMING {
      result = Self::get_player_anim_speed(s.clone(), lr);
      if !joypad_lr.is_empty() {
        for s in &mut result { s.facing_dir = joypad_lr; }
      }
    }
    if s.is_on_ground || !joypad_lr.is_empty() {
      result = result.into_iter().flat_map(|s| { Self::impose_friction(s, joypad_lr, friction, max_speed) }).collect();
    }
    result.sort();
    result.dedup();
    result
  }
  fn get_player_anim_speed(mut s: XPosState, lr: Dir) -> Vec<XPosState> {
    if s.x_spd_abs >= O::Platform::X_SPD_ABS_CUTOFFS[4] {
      s.running_speed = true;
      vec![s]
    } else {
      let mut result = Vec::new();
      if lr == s.moving_dir {
        let mut stop_running = s.clone();
        stop_running.running_speed = false;
        result.push(stop_running);
        if !lr.is_empty() { return result; }
      }
      if s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[1] {
        let mut stopping = s.clone();
        stopping.moving_dir = stopping.facing_dir;
        stopping.x_spd = 0;
        result.push(stopping);
        if !lr.is_empty() { return result; }
      }
      result.push(s);

      result
    }
  }
  fn impose_friction(mut s: XPosState, joypad_lr: Dir, friction: i16, max_speed: i16) -> Vec<XPosState> {
    let mut result = Vec::new();
    if joypad_lr.contains(Dir::RIGHT) || s.x_spd < 0 {
      let mut left_frict = s.clone();
      left_frict.x_spd += friction;
      if left_frict.x_spd >= max_speed { left_frict.x_spd = max_speed + (left_frict.x_spd & 0xff); }
      left_frict.x_spd_abs = O::Platform::X_SPD_ABS_CUTOFFS[O::Platform::get_x_spd_abs_cutoff((left_frict.x_spd >> 8).abs() as u8)];
      result.push(left_frict);
      if Dir::RIGHT.contains(joypad_lr) && s.x_spd < 0 { return result; }
    }
    if joypad_lr.contains(Dir::LEFT) || s.x_spd >= 0x100 {
      let mut right_frict = s.clone();
      right_frict.x_spd -= friction;
      if right_frict.x_spd <= (-max_speed) { right_frict.x_spd = (-max_speed) + (right_frict.x_spd & 0xff); }
      right_frict.x_spd_abs = O::Platform::X_SPD_ABS_CUTOFFS[O::Platform::get_x_spd_abs_cutoff((right_frict.x_spd >> 8).abs() as u8)];
      result.push(right_frict);
      if s.x_spd < 0 || s.x_spd >= 0x100 { return result; }
    }
    s.x_spd_abs = O::Platform::X_SPD_ABS_CUTOFFS[O::Platform::get_x_spd_abs_cutoff((s.x_spd >> 8).abs() as u8)];
    result.push(s);
    result
  }
  fn move_horizontally(states: HashSet<XPosState>) -> HashSet<XPosState> {
    states.into_iter().map(|mut s| { s.x_pos += (s.x_spd as i32 >> 8) << 4; s }).collect()
  }
  fn player_bg_collision(states: HashSet<XPosState>, allow_side_collisions: bool) -> HashSet<XPosState> {
    let states: HashSet<XPosState> = states.into_iter().flat_map(|s| {
      let mut landing = s.clone();
      landing.is_on_ground = true;
      let mut falling = s.clone();
      falling.is_on_ground = false;
      vec![landing, falling]
    }).collect();

    states.into_iter().flat_map(|s| {
      let mut result: Vec<XPosState> = vec![s.clone()];
      if allow_side_collisions && s.x_spd >= 0 {
        let mut collision_right = s.clone();
        collision_right.x_spd &= 0xff;
        collision_right.x_pos -= 0x100;
        result.push(collision_right);
      }
      if allow_side_collisions && s.x_spd < 0x100 {
        let mut collision_left = s.clone();
        collision_left.x_spd &= 0xff;
        collision_left.x_pos += 0x100;
        result.push(collision_left);
      }
      result
    }).collect()
  }
}
