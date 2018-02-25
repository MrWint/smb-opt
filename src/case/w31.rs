use blockbuffer::world3::*;
#[allow(unused_imports)] use emu::{Emu, EmuResult};
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::SearchGoal;
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};
use std::cmp::max;


/// Input sequence: [10x A, 3x R, 1x L, 1x NIL, 2x L, 1x R, 5x NIL, 1x R, 4x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 7x R] (len: 57)
#[allow(dead_code)]
pub struct W31ScreenScrollBlock3PipeClip {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W31ScreenScrollBlock3PipeClip {
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
impl super::SmbSearchCase for W31ScreenScrollBlock3PipeClip {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB31;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x63ff0,
      y_pos: 0x1b060,
      x_spd: 0x2808,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 31,
      running_timer : 8,
      left_screen_edge_pos: 0xcf,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    let s = Self::Emu::run_steps_nr(s, &[A; 1]);
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // )), 0)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 57;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W31ScreenScrollBlock3PipeClip {
  fn new() -> Self { return Self { max_x_pos: 0x67000, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    const GOAL_SCROLL_POS: i32 = 0x8c;
    if rel_x_pos > GOAL_SCROLL_POS { return None; }

    if s.x_pos >= 0x66a00 || s.side_collision_timer > 0 {
      if s.side_collision_timer < 15 {
        let max_x_pos = self.h.get_max_x_pos_after_steps(s, s.side_collision_timer as usize);
        let max_scroll = max((max_x_pos >> 8) - (s.x_pos >> 8), 0);
        if rel_x_pos + max_scroll < GOAL_SCROLL_POS { return None; }
      }
    }
    let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, 0x698e0);
    // let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10);
    Some(heuristic_steps + if s.x_pos < 0x66a00 && s.side_collision_timer == 0 { 15 } else { 0 }) // adjust for speed loss when colliding
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x698e0
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
