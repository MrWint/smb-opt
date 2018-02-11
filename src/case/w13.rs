use blockbuffer::world1::*;
use emu::{Emu, EmuResult};
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::{BoundsHeuristic, SearchGoal};
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};


/// Collect mushroom in 1-3 as small Mario
/// Input sequence: [1x A, 17x R, 1x L, 1x NIL, 1x L, 5x R, 1x A|R, 3x R, 2x NIL, 1x L, 9x R, 1x B|R, 1x A|L, 1x A|R, 1x A|L, 1x A, 2x R, 1x B|R, 16x R] (len: 66)
/// Input sequence: [2x A, 3x R, 3x NIL, 9x L, 1x R, 10x L, 2x A|L, 1x A, 1x A|R, 9x R, 1x L|R, 1x B|R, 1x A|L, 2x A, 1x A|R, 2x R, 1x B|R, 16x R] (len: 66)
#[allow(dead_code)]
pub struct W13PowerupSmall {
  max_x_pos: i32,
  h: BoundsHeuristic,
}
impl Options for W13PowerupSmall {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Small;
  type Swim = NotSwimming;
  type PowerupHandler = SinglePowerupHandler<::typenum::U59, ::typenum::U8>; // (0x3b, 0x8)
  type RunningTimer = NoRunningTimer;
  type ScrollPos = WithScrollPos;
  type Parity = Parity4;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W13PowerupSmall {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB13;

  fn start_states() -> Vec<State> {
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![State {
      x_pos: 0x37590,
      y_pos: 0x17e50,
      x_spd: 0x2800,
      y_spd: -0x320, // 0xfce0
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_JUMP_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      side_collision_timer: 0,
      left_screen_edge_pos: 0x5,
      jump_swim_timer: 20,
      running_timer : 0,
      is_crouching: false,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    }]
    // )), 10)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 66;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000; //79761384;
}
impl SearchGoal for W13PowerupSmall {
  fn new() -> Self { return Self { max_x_pos: 0x3de70, h: BoundsHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, steps_already_taken: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;
    if rel_x_pos > 0x70 { return None; }

    if steps_already_taken == 17 && s.y_pos >= 0x19100 && s.x_pos < 0x39e00 { return None; } // collided with moving platform
    if steps_already_taken == 19 && s.y_pos >= 0x19400 && s.x_pos < 0x39e00 { return None; } // collided with moving platform
    if steps_already_taken == 21 && s.y_pos >= 0x19700 && s.x_pos < 0x39e00 { return None; } // collided with moving platform

    const MAX_STEPS_TO_HIT: Dist = 29 + 4;
    if (steps_already_taken < MAX_STEPS_TO_HIT-3 && s.powerup_block_hit) || (steps_already_taken >= MAX_STEPS_TO_HIT && !s.powerup_block_hit) {
      return None; // not hit powerup block in valid timeframe
    }

    for i in 0..12 {
      if !s.powerup_collected
          && (steps_already_taken == MAX_STEPS_TO_HIT + 16 + 4*i || steps_already_taken == MAX_STEPS_TO_HIT + 18 + 4*i)
          && s.y_pos < 0x19500 - 0x100*i as i32
          && s.y_pos >= 0x17c00 - 0x100*i as i32
          && s.x_pos >= 0x3a500
          && s.x_pos < 0x3bc00 {
        s.powerup_collected = true;
        s.player_state = PlayerState::STANDING;
        break;
      }
    }

    if s.x_pos >= 0x3c000 && !s.powerup_collected { return None } // missed powerup collection
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x3de80))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x3de80
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

/// Collect fire flower in 1-3 as big Mario
/// Input sequence: [1x A, 9x R, 16x L, 1x NIL, 1x A|L, 2x NIL, 5x R, 4x L|R, 1x B|R, 1x L, 1x A|R, 1x L, 2x R, 1x A|R, 9x R] (len: 55)
#[allow(dead_code)]
pub struct W13PowerupBig {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W13PowerupBig {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = SinglePowerupHandler<::typenum::U59, ::typenum::U8>; // (0x3b, 0x8)
  type RunningTimer = NoRunningTimer;
  type ScrollPos = WithScrollPos;
  type Parity = Parity8;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W13PowerupBig {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB13;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x37590,
      y_pos: 0x17e50,
      x_spd: 0x2800,
      y_spd: -0x320, // 0xfce0
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_JUMP_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: true,
      jump_swim_timer: 20,
      running_timer : 0,
      left_screen_edge_pos: 0x5,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    println!("start state {}", s);
    // super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // ))
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 55;
  const SEARCH_SPACE_SIZE_HINT: usize = 59989102;
}
impl SearchGoal for W13PowerupBig {
  fn new() -> Self { return Self { max_x_pos: 0x3d3d0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, steps_already_taken: Dist) -> Option<Dist> {
    let steps_already_taken = steps_already_taken;

    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;
    if rel_x_pos > 0x70 { return None; }

    if steps_already_taken == 17 && s.y_pos >= 0x19100 && s.x_pos < 0x39f00 { return None; } // collided with moving platform
    if steps_already_taken == 19 && s.y_pos >= 0x19400 && s.x_pos < 0x39f00 { return None; } // collided with moving platform
    if steps_already_taken == 21 && s.y_pos >= 0x19700 && s.x_pos < 0x39f00 { return None; } // collided with moving platform

    const MAX_STEPS_TO_HIT: Dist = 29;
    if (steps_already_taken < MAX_STEPS_TO_HIT-3 && s.powerup_block_hit) || (steps_already_taken >= MAX_STEPS_TO_HIT && !s.powerup_block_hit) {
      return None; // not hit powerup block in valid timeframe
    }

    for i in 0..12 {
      if !s.powerup_collected
          && (steps_already_taken == MAX_STEPS_TO_HIT + 16 + 4*i || steps_already_taken == MAX_STEPS_TO_HIT + 18 + 4*i)
          && s.y_pos < 0x19500 + if s.is_crouching { 0 } else { 0xc00 } - 0x100*i as i32
          && s.y_pos >= 0x17c00 - 0x100*i as i32
          && s.x_pos >= 0x3a400
          && s.x_pos < 0x3bd00 {
        s.powerup_collected = true;
        s.player_state = PlayerState::STANDING;
        break;
      }
    }

    if s.x_pos >= 0x3bd00 && !s.powerup_collected { return None } // missed powerup collection
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x3d3e0))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x3d3e0
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

/// Floor clip using Red Koopa in 1-3 as big Mario
/// Input sequence: [20x R, 7x NIL, 2x L, 1x NIL, 8x L, 2x NIL, 10x R, 1x NIL, 1x L, 6x NIL] (len: 58)
// Possible jump heights: Standing jump for 15 or 22 frames (all other heights or speeds don't get far enough into the block for a successful clip) 
#[allow(dead_code)]
pub struct W13FloorClip {
  max_x_pos: i32,
  h: XPosHeuristic,
  // h: BoundsHeuristic,
}
impl Options for W13FloorClip {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = ImaginaryPowerup; // used to store whether Koopa was hit
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W13FloorClip {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB13;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x7d580,
      y_pos: 0x16c00,
      x_spd: 0x0,
      y_spd: -0x3e0, //0xfc20,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::LEFT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_JUMP_STANDING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_STANDING,
      x_spd_abs: 0x0,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 31,
      running_timer : 0,
      left_screen_edge_pos: 0x61,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    let s = Self::Emu::run_steps_nr(s, &[A; 14]); // 15xA
    let s = Self::Emu::run_steps_nr(s, &[NIL]);
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 58;
  const SEARCH_SPACE_SIZE_HINT: usize = 10;
}
impl SearchGoal for W13FloorClip {
  fn new() -> Self { return Self { max_x_pos: 0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    // Corresponds to Koopa position 0x80bxx
    if _steps_already_taken == 42 && !s.powerup_block_hit && s.y_pos < 0x1ba00 + if s.is_crouching { 0 } else { 0xc00 } && s.y_pos >= 0x1a100 && s.x_pos >= 0x7ff00 { // Koopa collision
      if s.y_spd < 0x100 { return None; } // Injured by Koopa
      s.powerup_block_hit = true;
      s.y_spd = -0x400 + (s.y_spd & 0xff);
    }
    if s.x_pos >= 0x80800 && s.y_pos < 0x1b500 { return None; } // no floor clip

    if s.x_pos < 0x7d580 { return None; } // going backwards

    // Some(self.h.get_steps_until_bounds_at_least(s, 0x80fe0, 0x1c000))
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x80fe0))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x80fe0 && s.y_pos >= 0x1c000
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


/// Speed up after Floor clip
/// Input sequence: [9x R, 1x L, 5x NIL, 1x R, 9x B|R, 2x L|R, 1x B|R, 1x L|R, 1x B|R, 2x L|R, 1x B|R, 1x L|R, 2x B|R, 1x L|R, 17x B|R] (len: 54)
#[allow(dead_code)]
pub struct W13FloorClipSpeedup {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W13FloorClipSpeedup {
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
impl super::SmbSearchCase for W13FloorClipSpeedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB13;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x7ff10,
      y_pos: 0x1c550,
      x_spd: 0xbfc,
      y_spd: -0x390, //0xfc70,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LR,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_STANDING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_STANDING,
      x_spd_abs: 0xb,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 0,
      left_screen_edge_pos: 0x8b,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_and_lr_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 0;
  const SEARCH_SPACE_SIZE_HINT: usize = 10;
}
impl SearchGoal for W13FloorClipSpeedup {
  fn new() -> Self { return Self { max_x_pos: 0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    if s.y_pos < 0x1b500 { return None; } // no floor clip
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x84470))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x84470 && s.y_pos >= 0x1c000
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


/// Full flag pole glitch in 1-3 from floor clip
/// Input sequence: [1x B|R, 2x A, 1x L, 2x NIL, 1x R] (len: 7)
#[allow(dead_code)]
pub struct W13FloorFlag {
  h: XPosHeuristic,
}
impl Options for W13FloorFlag {
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
impl super::SmbSearchCase for W13FloorFlag {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB13;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x84470,
      y_pos: 0x1c0d0,
      x_spd: 0x28e8,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_STANDING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_STANDING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::RIGHT,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 10,
      left_screen_edge_pos: 0xc6,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    let s = Self::Emu::run_steps_nr(s, &[B|R; 120]);
    //let s = Self::Emu::run_steps_nr(s, &[L; 8]);
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_and_lr_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 0;
  const SEARCH_SPACE_SIZE_HINT: usize = 10;
}
impl SearchGoal for W13FloorFlag {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x97600))
  }
  fn is_goal_state(&self, s: &State, emu_result: &EmuResult) -> bool {
    if let &EmuResult::StateChangeFlag(cx, cy) = emu_result {
      cx == 0x98 && cy == 9 && s.y_pos >= 0x1a500
    } else { false }
  }
}








#[allow(dead_code)]
pub enum EmulatorTesting {}
impl Options for EmulatorTesting {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = SinglePowerupHandler<::typenum::U59, ::typenum::U8>; // (0x3b, 0x8)
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbCase for EmulatorTesting {
  type BlockBuffer = BB13;

  fn run() -> () {
    let s = State {
      x_pos: 0x37590,
      y_pos: 0x17e50,
      x_spd: 0x28ac, //0x2800,
      y_spd: -0x320, // 0xfce0
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_JUMP_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: true,
      jump_swim_timer: 20,
      running_timer : 0,
      left_screen_edge_pos: 0x5,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    let h = ::heuristics::xpos::XPosHeuristic::new::<Self>(&vec![s.clone()]);

    println!("Old State: {}", s);
    let s = Self::Emu::run_steps_nr(s, &[A|R, R]);
    let s = Self::Emu::run_steps_nr(s, &[NIL; 8]);
    let s = Self::Emu::run_steps_nr(s, &[L; 10]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[A|L]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[NIL]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[NIL]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L|R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L|R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L|R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L|R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L|R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[B|R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[L]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[A]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[NIL]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[NIL]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let mut s = Self::Emu::run_steps_nr(s, &[R]);
    s.player_state = PlayerState::STANDING;
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[A|R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    let s = Self::Emu::run_steps_nr(s, &[R]);
    println!("h: {}", h.get_steps_until_x_pos_at_least(&s, 0x3d330));
    println!("New State: {}", s);
  }
}