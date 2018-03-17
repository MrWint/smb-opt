use blockbuffer::world6::*;
use emu::EmuResult;
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::{BoundsHeuristic, SearchGoal};
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};


/// Vine grab in 6-2
/// Input sequence: [3x NIL, 1x A, 7x A|L, 18x L, 3x B|L, 1x L|R, 9x A|L, 2x A, 1x A|L, 9x A|R, 4x R, 2x B|R, 1x L|R, 1x B|R, 2x L|R, 13x B|R] (len: 77)
#[allow(dead_code)]
pub struct W62Vine {
  max_x_pos: i32,
  h: BoundsHeuristic,
}
impl Options for W62Vine {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = SinglePowerupHandler<::typenum::U17, ::typenum::U3>; // (0x11, 0x3);
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W62Vine {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB62Vine;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0xfd10,
      y_pos: 0x180c8,
      x_spd: 0x183c,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LR,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x10,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 2,
      running_timer : 9,
      left_screen_edge_pos: 0x8d,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    println!("start state {}", s);
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
      vec![s]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 77;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W62Vine {
  fn new() -> Self { return Self { max_x_pos: 0, h: BoundsHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, steps_already_taken: Dist) -> Option<Dist> {
    let steps_already_taken = steps_already_taken.saturating_add(1);
    // let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    if steps_already_taken >= 15 && !s.powerup_block_hit { return None; } // hit vine block too late
    if s.x_pos < 0xdc00 { return None; } // went too far left
    if s.x_pos >= 0x11000 { return None; } // went too far right
    if s.y_pos >= 0x18100 { return None; } // went too low
    // if steps_already_taken >= 26 && s.y_pos >= 0x13500 && s.y_spd > 0 { return None; } // should go up
    if s.y_pos >= 0x14c00 && steps_already_taken >= 48 { return None; } // should be through gap already
    if s.y_pos >= 0x13500 && steps_already_taken >= 57 { return None; } // should be through gap already
    if steps_already_taken < 48 && self.h.get_steps_until_x_pos_at_most(s, 0xe300) + steps_already_taken >= 48 { return None; } // should be through gap already

    // let heuristic_steps = self.h.get_steps_until_x_pos_between(s, 0x10600, 0x119f0);
    let heuristic_steps = self.h.get_steps_until_bounds_at_least(s, self.max_x_pos + 0x10, 0x12800);
    Some(heuristic_steps)
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    _s.x_pos >= 0x10600 + 0x100 && _s.x_pos < 0x11a00 && _s.y_pos >= 0x12800 && _s.y_pos < 0x13500
    // false
  }
  fn track_metric(&mut self, s: &State) -> () {
    if s.y_pos >= 0x12800 && s.y_pos < 0x13500 && self.max_x_pos < s.x_pos {
      self.max_x_pos = s.x_pos;
      println!("new best max_x_pos: {:x}", self.max_x_pos);
    }
  }
  fn report_metrics(&self) -> () {
    println!("best max_x_pos so far: {:x}", self.max_x_pos);
  }
}


/// Speed-up after vine glitch in 6-2
/// Input sequence: [13x R, 2x NIL, 5x L, 2x NIL, 14x R, 1x B|R, 6x A|R, 8x R, 7x B|R] (len: 58)
#[allow(dead_code)]
pub struct W62VineSpeedup {
  h: XPosHeuristic,
}
impl Options for W62VineSpeedup {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W62VineSpeedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB62Vine;

  fn start_states() -> Vec<State> {
    vec![State {
      x_pos: 0x19ae0,
      y_pos: 0x13050,
      x_spd: 0x0,
      y_spd: 0x0,
      player_state: PlayerState::FALLING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::empty(),
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 6,
      left_screen_edge_pos: 0xad,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    }]
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 58;
  const SEARCH_SPACE_SIZE_HINT: usize = 100000000;
}
impl SearchGoal for W62VineSpeedup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x1f000))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x1f000
  }
}
