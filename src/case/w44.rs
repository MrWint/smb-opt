use blockbuffer::world4::*;
use emu::EmuResult;
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::SearchGoal;
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};


/// Initial speed-up starting in 4-4
/// Input sequence: [1x L|R, 1x A|R, 5x R, 1x A|R, 9x R, 1x B|R, 1x L|R, 1x B|R, 1x L|R, 1x B|R, 1x L, 1x A|R, 14x R, 3x NIL, 1x L, 2x NIL, 1x A|R, 14x R] (len: 59)
#[allow(dead_code)]
pub struct W44Speedup {
  h: XPosHeuristic,
}
impl Options for W44Speedup {
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
impl super::SmbSearchCase for W44Speedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB44;

  fn start_states() -> Vec<State> {
    vec![State {
      x_pos: 0x2800,
      y_pos: 0x15000,
      x_spd: 0x0,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::empty(),
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      v_force_down: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      x_spd_abs: 0x0,
      running_speed: false,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 0,
      left_screen_edge_pos: 0x0,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    }]
  }
  const SEARCH_SPACE_SIZE_HINT: usize = 0;
}
impl SearchGoal for W44Speedup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x9270))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x9270
  }
}


/// Wall clip shortcut in 4-4
/// Input sequence: [1x L, 1x L|R, 18x NIL, 1x A | DOWN, 1x A, 4x NIL, 2x R, 1x NIL, 13x R, 1x B|R, 1x L, 1x A|R, 3x R, 1x A|R, 3x R] (len: 52)
#[allow(dead_code)]
pub struct W44Clip {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W44Clip {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = WithScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W44Clip {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB44;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x5d2c0,
      y_pos: 0x170e8,
      x_spd: 0x2800,
      y_spd: 0x318,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: true,
      jump_swim_timer: 24,
      running_timer : 1,
      left_screen_edge_pos: 0x62,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    // State {
    //   x_pos: 0x5a3f0 - 0x30,
    //   y_pos: 0x18098,
    //   x_spd: 0x2818,
    //   y_spd: 0x0,
    //   player_state: PlayerState::STANDING,
    //   moving_dir: Dir::RIGHT,
    //   facing_dir: Dir::LEFT,
    //   v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
    //   v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
    //   x_spd_abs: 0x21,
    //   running_speed: true,
    //   collision_bits: Dir::RIGHT,
    //   is_crouching: false,
    //   jump_swim_timer: 16,
    //   running_timer : 10,
    //   left_screen_edge_pos: 0x33,
    //   side_collision_timer: 0,
    //   collected_coins: 0,
    //   powerup_block_hit: false,
    //   powerup_collected: false,
    //   parity: 0,
    // };
    // let s = Self::Emu::run_steps_nr(s, &[B; 3]);
    // let s = Self::Emu::run_steps_nr(s, &[B|R; 3+5]);
    // let s = Self::Emu::run_steps_nr(s, &[A|B|D; 1]);
    // let s = Self::Emu::run_steps_nr(s, &[B; 7]);
    println!("start state {}", s);
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
      vec![s]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 52;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W44Clip {
  fn new() -> Self { return Self { max_x_pos: 0x60d00, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    if rel_x_pos > 0x92 { return None; } // scrolled too far

    if s.y_pos < 0x16100 { return None; }
    if s.x_pos >= 0x5f400 && s.y_pos < 0x18100 { return None; }

    // const GOAL_SCROLL_POS: i32 = 0x70;

    // if s.x_pos >= 0x5da00 || s.side_collision_timer > 0 {
    //   if s.side_collision_timer < 15 {
    //     let max_x_pos = self.h.get_max_x_pos_after_steps(s, s.side_collision_timer as usize);
    //     let max_scroll = max((max_x_pos >> 8) - (s.x_pos >> 8), 0);
    //     if rel_x_pos + max_scroll < GOAL_SCROLL_POS { return None; }
    //   }
    // }
    let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, 0x60f30 - 0x50 - 0x160);
    // let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10);
    Some(heuristic_steps + if s.x_pos < 0x5da00 && s.side_collision_timer == 0 { 15 } else { 0 }) // adjust for speed loss when colliding
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    _s.x_pos >= 0x60f30 - 0x50 - 0x160
    // false
  }
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
