use blockbuffer::world3::*;
use emu::EmuResult;
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::SearchGoal;
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};


/// Speed up after Floor clip
#[allow(dead_code)]
pub struct W32FloorClipSpeedup {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W32FloorClipSpeedup {
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
impl super::SmbSearchCase for W32FloorClipSpeedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB32;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x80c00 + 0x50,
      y_pos: 0x1c5c8,
      x_spd: 0x19d0, //0x1974,
      y_spd: -0x400, //0xfc00,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x19,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 0,
      left_screen_edge_pos: 0xa5,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(//super::with_all_x_spd_subpixels(
    vec![s]
    // ), 15)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 52;
  const SEARCH_SPACE_SIZE_HINT: usize = 10;
}
impl SearchGoal for W32FloorClipSpeedup {
  fn new() -> Self { return Self { max_x_pos: 0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    if s.y_pos < 0x1b500 { return None; } // no floor clip
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x852f0 - 0xa0))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    _s.x_pos >= 0x852f0 - 0xa0 && _s.y_pos >= 0x1c000
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
