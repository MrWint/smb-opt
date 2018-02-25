use blockbuffer::NoCollisions;
use emu::EmuResult;
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::SearchGoal;
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};


/// Initial speed-up starting in 2-2
/// Input sequence: [1x L|R, 16x R, 9x L|R, 1x R, 1x L|R, 2x R] (len: 30)
#[allow(dead_code)]
pub struct W22Speedup {
  h: XPosHeuristic,
}
impl Options for W22Speedup {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = Swimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W22Speedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 11]>, Dist>;

  type BlockBuffer = NoCollisions;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x2800,
      y_pos: 0x13150,
      x_spd: 0x0,
      y_spd: 0x3a0,
      player_state: PlayerState::JUMPING,
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
    };
    vec![s]
  }
  const SEARCH_SPACE_SIZE_HINT: usize = 80127;
}
impl SearchGoal for W22Speedup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x3d50))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x3d50
  }
}
