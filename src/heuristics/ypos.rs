use options::{Options, Platform, PlayerSize};
use state::{Dist, State};
use std::cmp::{max,min};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::marker::PhantomData;

pub struct YPosHeuristic {
  max_distance: HashMap<YPosState, Vec<i32>>,
  min_distance: HashMap<YPosState, Vec<i32>>,
}
const MAX_Y_DIST: i32 = 0x10000;
impl YPosHeuristic {
  pub fn new<O: Options>(initial_states: &Vec<State>) -> YPosHeuristic {
    let mut h = YPosHeuristic {
      max_distance: HashMap::<YPosState, Vec<i32>>::new(),
      min_distance: HashMap::<YPosState, Vec<i32>>::new()
    };
    let state_map = Self::build_state_map::<O>(initial_states.iter().map(|s| Self::to_y_pos_state(s)).collect());
    println!("total number of YPosStates: {}; total size of state map: {}", state_map.len(), state_map.values().map(|v| v.len()).sum::<usize>());
    h.precompute_distances(state_map);
    h
  }
  fn build_state_map<O: Options>(initial_states: Vec<YPosState>) -> HashMap<YPosState, Vec<(YPosState, i32)>> {
    let mut state_map: HashMap<YPosState, Vec<(YPosState, i32)>> = HashMap::new();

    let mut stack = initial_states;
    while let Some(s) = stack.pop() {
      if state_map.contains_key(&s) { continue; }
      let mut next_states = YPosEmu::<O>::run_step(vec![s.clone()]);
      state_map.insert(s.clone(), next_states.clone().into_iter().map(|mut next_state| {
        let dist = next_state.y_pos - s.y_pos;
        next_state.y_pos &= 0xfff;
        (next_state, dist)
      }).collect());
      if state_map.len() % 100000 == 0 { println!("{}", state_map.len()); }
      stack.append(&mut next_states.into_iter().map(|mut s| { s.y_pos &= 0xfff; s }).collect());
    }

    state_map
  }
  fn to_y_pos_state(s: &State) -> YPosState {
    YPosState {
      y_pos: s.y_pos & 0xfff,
      y_spd: s.y_spd,
      is_on_ground: s.is_on_ground(),
      v_force: s.v_force,
      v_force_down: s.v_force_down,
    }
  }
  pub fn get_steps_until_y_pos_at_least(&self, s: &State, target_y_pos: i32) -> Dist {
    if target_y_pos <= s.y_pos { return 0; }
    let distance = target_y_pos - s.y_pos;
    assert!(distance < MAX_Y_DIST);
    let s = Self::to_y_pos_state(s);
    if let Some(max_dists) = self.max_distance.get(&s) {
      let len = max_dists.len();
      if len == 0 || max_dists[len-1] < distance { len as Dist + 1 }
      else { max_dists.iter().position(|d| *d >= distance).unwrap() as Dist + 1 }
    } else { panic!("trying to determine YPos heuristic for unknown state {}", s); }
  }
  pub fn get_steps_until_y_pos_at_most(&self, s: &State, target_y_pos: i32) -> Dist {
    if target_y_pos >= s.y_pos { return 0; }
    let distance = target_y_pos - s.y_pos;
    assert!(distance > -MAX_Y_DIST);
    let s = Self::to_y_pos_state(s);
    if let Some(min_dists) = self.min_distance.get(&s) {
      let len = min_dists.len();
      if len == 0 || min_dists[len-1] > distance { len as Dist + 1 }
      else { min_dists.iter().position(|d| *d <= distance).unwrap() as Dist + 1 }
    } else { panic!("trying to determine YPos heuristic for unknown state {}", s); }
  }
  pub fn get_steps_until_y_pos_between(&self, s: &State, min_y_pos: i32, max_y_pos: i32) -> Dist {
    if s.y_pos < min_y_pos { self.get_steps_until_y_pos_at_least(s, min_y_pos) }
    else { self.get_steps_until_y_pos_at_most(s, max_y_pos) }
  }
  fn precompute_distances(&mut self, state_map: HashMap<YPosState, Vec<(YPosState, i32)>>) {
    for s in state_map.keys() { self.max_distance.insert(s.clone(), vec![]); }
    for s in state_map.keys() { self.min_distance.insert(s.clone(), vec![]); }
    let mut steps = 0;
    let mut changed_cache = true;
    print!("YPos precompute_distances:");
    ::std::io::stdout().flush().ok();
    while changed_cache {
      steps += 1;
      print!(" {},", steps);
      ::std::io::stdout().flush().ok();
      changed_cache = false;
      for (s, next_states) in state_map.iter() {
        if self.max_distance.get(s).unwrap().len() == steps - 1 {
          let mut max_dist = ::std::i32::MIN;
          for &(ref next_state, y_dist) in next_states {
            max_dist = max(max_dist, self.get_max_distance(next_state, steps-1) + y_dist);
          }
          if max_dist  < MAX_Y_DIST {
            self.max_distance.get_mut(s).unwrap().push(max_dist);
            changed_cache = true;
          }
        }
        if self.min_distance.get(s).unwrap().len() == steps - 1 {
          let mut min_dist = ::std::i32::MAX;
          for &(ref next_state, y_dist) in next_states {
            min_dist = min(min_dist, self.get_min_distance(next_state, steps-1) + y_dist);
          }
          if min_dist > -MAX_Y_DIST {
            self.min_distance.get_mut(s).unwrap().push(min_dist);
            changed_cache = true;
          }
        }
      }
    }
    println!();
  }
  fn get_max_distance(&self, s: &YPosState, steps: usize) -> i32 {
    let max_dists = self.max_distance.get(s).unwrap();
    if steps == 0 { 0 }
    else if max_dists.len() < steps { MAX_Y_DIST * 2 } // overestimate distance
    else { max_dists[steps-1] }
  }
  fn get_min_distance(&self, s: &YPosState, steps: usize) -> i32 {
    let min_dists = self.min_distance.get(s).unwrap();
    if steps == 0 { 0 }
    else if min_dists.len() < steps { -MAX_Y_DIST * 2 } // overestimate distance
    else { min_dists[steps-1] }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct YPosState {
  pub y_pos: i32,
  pub y_spd: i16,
  pub is_on_ground: bool,
  pub v_force: u8,
  pub v_force_down: u8,
}
impl ::std::fmt::Display for YPosState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    writeln!(f, "YPosState {{")?;
    writeln!(f, "  y_pos: {:#x}", self.y_pos)?;
    writeln!(f, "  y_spd: {:#x}", self.y_spd)?;
    writeln!(f, "  is_on_ground: {:?}", self.is_on_ground)?;
    writeln!(f, "  v_force: {:#x}", self.v_force)?;
    writeln!(f, "  v_force_down: {:#x}", self.v_force_down)?;
    write!(f, "}}")
  }
}

const WITH_ENEMY_COLLISIONS: bool = false;
enum Void {}
pub struct YPosEmu<O: Options> {
  _options: PhantomData<O>,
  _void: Void,
}
impl<O: Options> YPosEmu<O> {
  fn run_step(states: Vec<YPosState>) -> HashSet<YPosState> {
    let states = Self::player_movement_subs(states);
    let states = Self::player_bg_collision(states);
    let states = Self::player_enemy_collision(states);
    states.into_iter().map(|mut s| {
      if O::CLEAR_Y_POS_FRACTIONALS && s.is_on_ground { s.y_pos &= 0xffff00; s.v_force_down = O::Platform::V_FORCE_AREA_INIT; }
      if s.is_on_ground { s.v_force = s.v_force_down; } // only needed for JumpSwim, set whenever entered
      s
    }).collect()
  }
  fn player_movement_subs(states: Vec<YPosState>) -> HashSet<YPosState> {
    let states: HashSet<YPosState> = states.into_iter().flat_map(|s| {
      let mut result: HashSet<YPosState> = HashSet::new();
      if s.is_on_ground || (s.v_force != s.v_force_down && s.y_spd < 0) { result.insert(s.clone()); }
      if !s.is_on_ground {
        let mut stop_jump = s.clone();
        stop_jump.v_force = stop_jump.v_force_down;
        result.insert(stop_jump);
      }

      // handle starting jumps
      if s.is_on_ground || O::IS_SWIMMING {
        let mut start_jump = s.clone();
        start_jump.y_pos &= 0xffff00; // clear fractional yPos
        start_jump.is_on_ground = false;
        if O::IS_SWIMMING {
          let mut start_jump_swim = start_jump.clone();
          start_jump_swim.v_force = O::Platform::V_FORCE_JUMP_SWIMMING;
          start_jump_swim.v_force_down = O::Platform::V_FORCE_FALL_SWIMMING;
          start_jump_swim.y_spd = O::Platform::JUMP_VELOCITY_SWIM;
          result.insert(start_jump_swim);
          let mut start_jump_swim_too_high = start_jump.clone();
          start_jump_swim_too_high.v_force = O::Platform::V_FORCE_FALL_SWIMMING;
          start_jump_swim_too_high.v_force_down = O::Platform::V_FORCE_FALL_SWIMMING;
          start_jump_swim_too_high.y_spd = O::Platform::JUMP_VELOCITY_SWIM & 0xff; // kill upward momentum if swimming too high
          result.insert(start_jump_swim_too_high);
        } else {
          let mut start_jump_run = start_jump.clone();
          start_jump_run.v_force = O::Platform::V_FORCE_JUMP_RUNNING;
          start_jump_run.v_force_down = O::Platform::V_FORCE_FALL_RUNNING;
          start_jump_run.y_spd = O::Platform::JUMP_VELOCITY_FAST;
          result.insert(start_jump_run);
          let mut start_jump_walk = start_jump.clone();
          start_jump_walk.v_force = O::Platform::V_FORCE_JUMP_WALKING;
          start_jump_walk.v_force_down = O::Platform::V_FORCE_FALL_WALKING;
          start_jump_walk.y_spd = O::Platform::JUMP_VELOCITY_SLOW;
          result.insert(start_jump_walk);
          let mut start_jump_standing = start_jump.clone();
          start_jump_standing.v_force = O::Platform::V_FORCE_JUMP_STANDING;
          start_jump_standing.v_force_down = O::Platform::V_FORCE_FALL_STANDING;
          start_jump_standing.y_spd = O::Platform::JUMP_VELOCITY_SLOW;
          result.insert(start_jump_standing);
        }
      }

      result
    }).collect();

    // MoveSubs
    let states: HashSet<YPosState> = states.into_iter().flat_map(|s| {
      if !s.is_on_ground && O::IS_SWIMMING {
        let mut swim_too_high = s.clone();
        swim_too_high.v_force = O::Platform::V_FORCE_SWIM_TOO_HIGH;
        vec![s, swim_too_high]
      } else { vec![s] }
    }).collect();
    states.into_iter().map(|mut s| { // MoveVertically
      if !s.is_on_ground {
        s.y_pos += s.y_spd as i32;
        s.y_spd += s.v_force as i16;
        if s.y_spd >= O::Platform::MAX_Y_SPD && (s.y_spd & 0xff) >= 0x80 { s.y_spd = O::Platform::MAX_Y_SPD; }
      }
      s
    }).collect()
  }
  fn player_bg_collision(states: HashSet<YPosState>) -> HashSet<YPosState> {
    // Get into falling state if fractionals are not cleared
    let states: HashSet<YPosState> = states.into_iter().map(|mut s| { if !O::CLEAR_Y_POS_FRACTIONALS { s.is_on_ground = false; } s }).collect();

    // HeadChk
    let states: HashSet<YPosState> = states.into_iter().flat_map(|s| {
      if s.y_spd >= 0 || (s.y_pos & 0x0f00) < 0x400 { return vec![s]; }

      let mut hit_solid_block = s.clone();
      hit_solid_block.y_spd = 0x100 + (hit_solid_block.y_spd & 0xff); // hit solid block
      if O::IS_SWIMMING { return vec![s, hit_solid_block]; }

      let mut bump_block = s.clone();
      bump_block.y_spd &= 0xff; // bump block
      if !O::PlayerSize::MAY_BE_BIG || (s.y_spd >= -0x200 && s.y_spd < -0x100) { return vec![s, hit_solid_block, bump_block]; }

      let mut shatter_brick = s.clone();
      shatter_brick.y_spd = -0x200 + (shatter_brick.y_spd & 0xff); // shatter brick
      vec![s, hit_solid_block, shatter_brick, bump_block]
    }).collect();

    // DoFootCheck
    states.into_iter().flat_map(|s| {
      if s.y_spd >= 0 && (s.y_pos & 0x0f00) < O::Platform::BLOCK_SURFACE_THICKNESS {
        let mut land = s.clone();
        land.y_pos &= 0xfff0ff; // align height with block
        land.y_spd = 0; // kill vertical speed
        land.is_on_ground = true; // land
        vec![s, land]
      } else { vec![s] }
    }).collect()
  }

  fn player_enemy_collision(states: HashSet<YPosState>) -> HashSet<YPosState> {
    if !WITH_ENEMY_COLLISIONS { return states; }
    // bounce on enemy
    let states: HashSet<YPosState> = states.into_iter().flat_map(|s| {
      if s.y_spd >= 0x100 {
        let mut bounce_enemy = s.clone();
        bounce_enemy.y_spd = -0x300 + (bounce_enemy.y_spd & 0xff); // bounce on enemy
        vec![s, bounce_enemy]
      } else { vec![s] }
    }).collect();

    // bounce on shell
    states.into_iter().flat_map(|s| {
      if s.y_spd >= 0x100 {
        let mut bounce_shell = s.clone();
        bounce_shell.y_spd = -0x400 + (bounce_shell.y_spd & 0xff); // bounce on shell
        vec![s, bounce_shell]
      } else { vec![s] }
    }).collect()
  }
}
