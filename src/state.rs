use bitpack::BitPack;
use options::{CoinHandler, Options, Platform, PlayerSize, PlayerSwimming, PowerupHandler, RunningTimer, ScrollPos};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub type Dist = u16;

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub enum PlayerState {
  STANDING,
  JUMPING,
  FALLING,
  CLIMBING,
}
impl PlayerState {
  fn bits(&self) -> u32 {
    match self {
      &PlayerState::STANDING => { 0 }
      &PlayerState::JUMPING => { 1 }
      &PlayerState::FALLING => { 2 }
      &PlayerState::CLIMBING => { 3 }
    }
  }
}

bitflags! {
  pub struct Dir: u8 {
    const LEFT   = 0b00000010;
    const RIGHT  = 0b00000001;
    const LR = Self::LEFT.bits | Self::RIGHT.bits;
  }
}

#[derive(Clone,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub struct State {
  pub x_pos: i32,
  pub y_pos: i32,
  pub x_spd: i16,
  pub y_spd: i16,
  pub player_state: PlayerState,
  pub moving_dir: Dir,
  pub facing_dir: Dir,
  pub v_force: u8,
  pub v_force_down: u8,
  pub x_spd_abs: u8,
  pub running_speed: bool,
  pub collision_bits: Dir,
  pub is_crouching: bool, // only when big
  pub jump_swim_timer: u8, // only when swimming
  pub running_timer: u8, // only when running timer enabled
  pub left_screen_edge_pos: u8, // only when scrolling is tracked
  pub side_collision_timer: u8, // only when scrolling is tracked
  pub coin_collected: bool, // only for single coin strategy
  pub powerup_block_hit: bool, // only for powerup collection
  pub powerup_collected: bool, // only for powerup collection
}
impl State {
  pub fn is_on_ground(&self) -> bool { self.player_state == PlayerState::STANDING }
  pub fn set_x_spd_abs<P: Platform>(&mut self, spd: u8) -> () {
    self.x_spd_abs = P::X_SPD_ABS_CUTOFFS[P::get_x_spd_abs_cutoff(spd)];
  }
}
impl ::std::fmt::Display for State {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    writeln!(f, "State {{")?;
    writeln!(f, "  x_pos: {:#x}", self.x_pos)?;
    writeln!(f, "  y_pos: {:#x}", self.y_pos)?;
    writeln!(f, "  x_spd: {:#x}", self.x_spd)?;
    writeln!(f, "  y_spd: {:#x}", self.y_spd)?;
    writeln!(f, "  player_state: {:?}", self.player_state)?;
    writeln!(f, "  moving_dir: {:?}", self.moving_dir)?;
    writeln!(f, "  facing_dir: {:?}", self.facing_dir)?;
    writeln!(f, "  v_force: {:#x}", self.v_force)?;
    writeln!(f, "  v_force_down: {:#x}", self.v_force_down)?;
    writeln!(f, "  x_spd_abs: {:#x}", self.x_spd_abs)?;
    writeln!(f, "  running_speed: {:?}", self.running_speed)?;
    writeln!(f, "  collision_bits: {:?}", self.collision_bits)?;
    writeln!(f, "  is_crouching: {:?}", self.is_crouching)?;
    writeln!(f, "  jump_swim_timer: {:?}", self.jump_swim_timer)?;
    writeln!(f, "  running_timer: {:?}", self.running_timer)?;
    writeln!(f, "  left_screen_edge_pos: {:#x}", self.left_screen_edge_pos)?;
    writeln!(f, "  side_collision_timer: {:?}", self.side_collision_timer)?;
    writeln!(f, "  coin_collected: {:?}", self.coin_collected)?;
    writeln!(f, "  powerup_block_hit: {:?}", self.powerup_block_hit)?;
    writeln!(f, "  powerup_collected: {:?}", self.powerup_collected)?;
    write!(f, "}}")
  }
}

pub trait StateCompressor {
  fn from_state(&State) -> Self;
}
impl StateCompressor for State {
  fn from_state(s: &State) -> State {
    s.clone()
  }
}

const COMPRESSED_STATE_BYTES: usize = 10;
pub struct CompressedState<O: Options> {
  buf: [u8; COMPRESSED_STATE_BYTES],
  _options: PhantomData<O>,
}
impl<O: Options> PartialEq for CompressedState<O> {
  fn eq(&self, other: &Self) -> bool { self.buf.eq(&other.buf) }
}
impl<O: Options> Eq for CompressedState<O> {}
impl<O: Options> Clone for CompressedState<O> {
  fn clone(&self) -> Self { Self { buf: self.buf.clone(), _options: PhantomData } }
}
impl<O: Options> Hash for CompressedState<O> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.buf.hash(state);
  }
}
impl<O: Options> ::store::VecHashKey for CompressedState<O> {
  fn is_valid(&self) -> bool {
    self.buf[7] != 0 // contains facing_dir, which is never 0
  }
  fn invalid() -> Self {
    CompressedState { buf: [0; COMPRESSED_STATE_BYTES], _options: PhantomData }
  }
}
impl<O: Options> StateCompressor for CompressedState<O> {
  fn from_state(s: &State) -> CompressedState<O> {
    const Y_POS_OFFSET: u32 = 0xd000;

    let bytes_needed = (16 + 16 + 13 + 12 + 2 + 2 + 2 + 4 + 4 + 3 + 1 + 2 + 7
        + O::PlayerSize::CROUCH_BITS + O::PlayerSwimming::SWIMMING_BITS
        + O::RunningTimer::RUNNING_TIMER_BITS
        + O::ScrollPos::SCROLL_POS_BITS
        + O::CoinHandler::COIN_HANDLER_BITS
        + O::PowerupHandler::POWERUP_HANDLER_BITS) >> 3;
    assert!(bytes_needed == COMPRESSED_STATE_BYTES, "bytes_needed {} != COMPRESSED_STATE_BYTES {}", bytes_needed, COMPRESSED_STATE_BYTES);

    let mut buf = [0; COMPRESSED_STATE_BYTES];
    {
      let mut bitpack = BitPack::<&mut [u8]>::new(&mut buf);
      bitpack.write(s.x_pos as u32 >> 4, 16).unwrap();
      bitpack.write(s.y_pos as u32 - Y_POS_OFFSET, 16).unwrap();
      bitpack.write(s.x_spd as u32 >> 2, 13).unwrap();
      bitpack.write(s.y_spd as u32, 12).unwrap();
      bitpack.write(s.player_state.bits(), 2).unwrap();
      bitpack.write(s.moving_dir.bits() as u32, 2).unwrap();
      bitpack.write(s.facing_dir.bits() as u32, 2).unwrap();
      bitpack.write(O::Platform::get_v_force_index(s.v_force), 4).unwrap();
      bitpack.write(O::Platform::get_v_force_index(s.v_force_down) as u32, 4).unwrap();
      bitpack.write(O::Platform::get_x_spd_abs_cutoff(s.x_spd_abs) as u32, 3).unwrap();
      bitpack.write(if s.running_speed { 1 } else { 0 }, 1).unwrap();
      bitpack.write(s.collision_bits.bits() as u32, 2).unwrap();

      if O::PlayerSize::CROUCH_BITS > 0 {
        bitpack.write(if s.is_crouching { 1 } else { 0 }, 1).unwrap();
      }
      if O::PlayerSwimming::SWIMMING_BITS > 0 {
        bitpack.write(s.jump_swim_timer as u32, 5).unwrap();
      }
      if O::RunningTimer::RUNNING_TIMER_BITS > 0 {
        bitpack.write(s.running_timer as u32, 4).unwrap();
      }
      if O::ScrollPos::SCROLL_POS_BITS > 0 {
        bitpack.write(s.left_screen_edge_pos as u32, 8).unwrap();
        bitpack.write(s.side_collision_timer as u32, 4).unwrap();
      }
      if O::CoinHandler::COIN_HANDLER_BITS > 0 {
        bitpack.write(if s.coin_collected { 1 } else { 0 }, 1).unwrap();
      }
      if O::PowerupHandler::POWERUP_HANDLER_BITS > 0 {
        bitpack.write(if s.powerup_block_hit { 1 } else { 0 }, 1).unwrap();
        bitpack.write(if s.powerup_collected { 1 } else { 0 }, 1).unwrap();
      }
    }
    CompressedState { buf, _options: PhantomData }
  }
}

pub trait StateDistCompressor {
  fn from_state_dist(&State, Dist) -> Self;
  fn dist(&self) -> Dist;
  fn increment_dist(&mut self) -> ();
}
pub struct StateDist<S: StateCompressor> {
  s: S,
  dist: Dist,
}
impl<S: StateCompressor + PartialEq> PartialEq for StateDist<S> {
  fn eq(&self, other: &Self) -> bool { self.s.eq(&other.s) }
}
impl<S: StateCompressor + Eq> Eq for StateDist<S> {}
impl<S: StateCompressor + Clone> Clone for StateDist<S> {
  fn clone(&self) -> Self { Self { s: self.s.clone(), dist: self.dist } }
}
impl<S: StateCompressor + Hash> Hash for StateDist<S> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.s.hash(state);
  }
}
impl<S: StateCompressor + ::store::VecHashKey> ::store::VecHashKey for StateDist<S> {
  fn is_valid(&self) -> bool {
    self.s.is_valid()
  }
  fn invalid() -> Self {
    StateDist { s: S::invalid(), dist: 0 }
  }
}
impl<S: StateCompressor> StateDistCompressor for StateDist<S> {
  fn from_state_dist(s: &State, d: Dist) -> Self {
    Self { s: S::from_state(s), dist: d }
  }
  fn dist(&self) -> Dist {
    self.dist
  }
  fn increment_dist(&mut self) -> () {
    self.dist += 1;
  }
}
