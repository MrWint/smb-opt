use emu::{Emu,EmuResult,Input};
use heuristics::SearchGoal;
use options::{Options, Platform};
use state::{Dist,PlayerState, State};
use std::marker::PhantomData;
use store::StateStore;

pub trait InputFetcher {
  fn valid_next_inputs(s: &State) -> Vec<Input>;
}

#[allow(dead_code)]
pub struct SmbInputFetcher<O: Options> {
  _options: PhantomData<O>
}

impl<O: Options> InputFetcher for SmbInputFetcher<O> {
  fn valid_next_inputs(s: &State) -> Vec<Input> {
    if s.y_pos < 0x10000 || s.y_pos >= 0x1d000 { return vec![Input::empty()]; } // inputs are ignored

    let mut inputs = Vec::new();
    if !O::IS_SWIMMING && s.is_on_ground() {
      inputs.push(Input::B | Input::from_bits_truncate(s.moving_dir.bits()));
    }
    inputs.push(Input::RIGHT);
    if s.player_state == PlayerState::STANDING || O::IS_SWIMMING { // Changes the facing_dir
      inputs.push(Input::LEFT | Input::RIGHT);
    }
    inputs.push(Input::empty());
    inputs.push(Input::LEFT);

    if O::is_big(s) && s.is_on_ground() {
      inputs.push(Input::DOWN);
    }
    if s.is_on_ground() && s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[4] && (!s.moving_dir.is_empty() || !O::is_big(s)) {
      inputs.push(Input::DOWN | Input::from_bits_truncate(s.moving_dir.bits()));
    }
    if s.is_on_ground() && s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[1] && (s.moving_dir.is_empty() || !O::is_big(s)) {
      inputs.push(Input::DOWN | if s.moving_dir.is_empty() { Input::RIGHT } else { Input::empty() });
    }
    if s.is_on_ground() || (O::IS_SWIMMING && (s.jump_swim_timer != 0 || s.y_spd >= 0)) || (s.player_state == PlayerState::JUMPING && s.v_force != s.v_force_down) {
      inputs.push(Input::A);
      inputs.push(Input::A | Input::RIGHT);
      if O::IS_SWIMMING { // Changes the facing_dir
        inputs.push(Input::A | Input::LEFT | Input::RIGHT);
      }
      inputs.push(Input::A | Input::LEFT);
    }
    if O::is_big(s) && s.is_on_ground() {
      inputs.push(Input::A | Input::DOWN);
    }
    if O::IS_SWIMMING && s.is_on_ground() && s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[4] && (!s.moving_dir.is_empty() || !O::is_big(s)) {
      inputs.push(Input::A | Input::DOWN | Input::from_bits_truncate(s.moving_dir.bits()));
    }
    if O::IS_SWIMMING && s.is_on_ground() && s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[1] && (s.moving_dir.is_empty() || !O::is_big(s)) {
      inputs.push(Input::A | Input::DOWN | if s.moving_dir.is_empty() { Input::RIGHT } else { Input::empty() });
    }
    if (s.is_on_ground() || O::IS_SWIMMING) && s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[4] && s.moving_dir.is_empty() {
      inputs.push(Input::UP);
    }
    if (s.is_on_ground() || O::IS_SWIMMING) && s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[1] && !s.moving_dir.is_empty() {
      inputs.push(Input::UP);
    }
    if O::IS_SWIMMING
        && (s.is_on_ground() || s.jump_swim_timer != 0 || s.y_spd >= 0 || s.v_force != s.v_force_down)
        && s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[4]
        && s.moving_dir.is_empty() {
      inputs.push(Input::A | Input::UP);
    }
    if O::IS_SWIMMING
        && (s.is_on_ground() || s.jump_swim_timer != 0 || s.y_spd >= 0 || s.v_force != s.v_force_down)
        && s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[1]
        && !s.moving_dir.is_empty() {
      inputs.push(Input::A | Input::UP);
    }

    return inputs;
  }
}

#[allow(dead_code)]
pub struct AllInputs;
impl InputFetcher for AllInputs {
  fn valid_next_inputs(_: &State) -> Vec<Input> {
    let mut inputs = Vec::with_capacity(256);
    for i in (0..256).map(|byte| byte as u8) {
      inputs.push(Input::from_bits_truncate(i));
    }
    inputs
  }
}


pub enum SearchResult {
  NotFound,
  Found(Vec<State>, Vec<Input>)
}

pub trait Search {
  fn find_first_solution(start_states: Vec<State>, initial_max_allowed_steps: Dist, search_space_size_hint: usize) -> SearchResult;
}
pub struct IDA<S: StateStore, E: Emu, G: SearchGoal, I: InputFetcher> {
  visited_states: S,
  search_goal: G,
  _emu: PhantomData<E>,
  _input_fetcher: PhantomData<I>,
  num_visits: u64,
}
impl<S: StateStore, E: Emu, G: SearchGoal, I: InputFetcher> Search for IDA<S, E, G, I> {
  fn find_first_solution(mut start_states: Vec<State>, initial_max_allowed_steps: Dist, search_space_size_hint: usize) -> SearchResult {
    let emu = Self::new(search_space_size_hint);
    let initial_max_allowed_steps = ::std::cmp::max(initial_max_allowed_steps, start_states.iter_mut().filter_map(|mut s| emu.search_goal.distance_to_goal_heuristic(&mut s, 0)).min().unwrap());
    emu.find_first_solution(start_states, initial_max_allowed_steps)
  }
}
impl<S: StateStore, E: Emu, G: SearchGoal, I: InputFetcher> IDA<S, E, G, I> {
  fn new(search_space_size_hint: usize) -> IDA<S, E, G, I> {
    Self {
      visited_states: S::new(search_space_size_hint),
      search_goal: G::new(),
      _emu: PhantomData,
      _input_fetcher: PhantomData,
      num_visits: 0,
    }
  }
  fn find_first_solution_rec(&mut self, mut s: State, steps_already_taken: Dist, max_allowed_steps: Dist) -> SearchResult {
    let heuristic_distance_to_goal;
    if let Some(distance) = self.search_goal.distance_to_goal_heuristic(&mut s, steps_already_taken) {
      self.search_goal.track_metric(&s);
      heuristic_distance_to_goal = distance;
    } else {
      return SearchResult::NotFound;
    }
    if steps_already_taken >= max_allowed_steps
        || steps_already_taken + heuristic_distance_to_goal > max_allowed_steps // out of steps
        || s.y_pos >= 0x1c500 // too low
        || !self.visited_states.check_and_update_dist(&s, steps_already_taken) {
      return SearchResult::NotFound;
    }

    self.num_visits += 1;
    if self.num_visits % 1000000 == 0 {
      println!("distance: {}, heuristic: {}, limit: {}, seen: {}", steps_already_taken, heuristic_distance_to_goal, max_allowed_steps, self.visited_states.len());
    }

    for input in I::valid_next_inputs(&s) {
      let (new_state, emu_result) = E::run_step(s.clone(), input);
      if self.search_goal.is_goal_state(&new_state, &emu_result) {
        println!("Found goal state after {} seen states!", self.visited_states.len());
        if heuristic_distance_to_goal > 1 {
          println!("WARNING: heuristic ({}) larger than actual steps needed (1) for state {:?}", heuristic_distance_to_goal, s);
        }
        return SearchResult::Found(vec![new_state, s], vec![input]);
      }
      if let EmuResult::Success = emu_result {
        if let SearchResult::Found(mut states, mut inputs) = self.find_first_solution_rec(new_state, steps_already_taken + 1, max_allowed_steps) {
          inputs.push(input);
          if heuristic_distance_to_goal as usize > inputs.len() {
            println!("WARNING: heuristic ({}) larger than actual steps needed ({}) for state {:?}", heuristic_distance_to_goal, inputs.len(), s);
          }
          states.push(s);
          return SearchResult::Found(states, inputs);
        }
      }
    }
    SearchResult::NotFound
  }
  fn find_first_solution(mut self, start_states: Vec<State>, initial_max_allowed_steps: Dist) -> SearchResult {
    let mut max_allowed_steps = initial_max_allowed_steps;

    loop {
      println!("search max distance  {}", max_allowed_steps);
      for s in &start_states {
        if let SearchResult::Found(mut states, mut inputs) = self.find_first_solution_rec(s.clone(), 0, max_allowed_steps) {
          states.reverse();
          inputs.reverse();
          return SearchResult::Found(states, inputs);
        }
      }
      self.search_goal.report_metrics();

      max_allowed_steps += 1;
      self.visited_states.increment_all_dists(); // increase distance by one; only shortest paths to any known state will be considered next round
    }
  }
}
