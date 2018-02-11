use state::State;
use std::marker::PhantomData;
use typenum::Unsigned;

enum Void {}

pub trait Options {
  type Platform: Platform;
  type CoinHandler: CoinHandler;
  type PowerupHandler: PowerupHandler;
  type PlayerSize: PlayerSize;
  type Swim: Swim;
  type RunningTimer: RunningTimer;
  type ScrollPos: ScrollPos;
  type Parity: Parity;
  type VerticalPipeHandler: VerticalPipeHandler;
  type YPosFractionalBehavior: YPosFractionalBehavior;
}
#[allow(dead_code)]
pub struct SmbOptions<Size: PlayerSize, Swi: Swim, RunT: RunningTimer, YPFB: YPosFractionalBehavior, ScrP: ScrollPos, Par: Parity, Plat: Platform, CoiH: CoinHandler, PowH: PowerupHandler, VerP: VerticalPipeHandler> {
  _player_size: PhantomData<Size>,
  _player_swimming: PhantomData<Swi>,
  _running_timer: PhantomData<RunT>,
  _ypfb: PhantomData<YPFB>,
  _scroll_pos: PhantomData<ScrP>,
  _parity: PhantomData<Par>,
  _platform: PhantomData<Plat>,
  _coin_handler: PhantomData<CoiH>,
  _powerup_handler: PhantomData<PowH>,
  _vertical_pipe_handler: PhantomData<VerP>,
  _void: Void,
}
impl<Size: PlayerSize, Swi: Swim, RunT: RunningTimer, YPFB: YPosFractionalBehavior, ScrP: ScrollPos, Par: Parity, Plat: Platform, CoiH: CoinHandler, PowH: PowerupHandler, VerP: VerticalPipeHandler> Options for SmbOptions<Size, Swi, RunT, YPFB, ScrP, Par, Plat, CoiH, PowH, VerP> {
  type Platform = Plat;
  type CoinHandler = CoiH;
  type PowerupHandler = PowH;
  type PlayerSize = Size;
  type Swim = Swi;
  type RunningTimer = RunT;
  type ScrollPos = ScrP;
  type Parity = Par;
  type VerticalPipeHandler = VerP;
  type YPosFractionalBehavior = YPFB;
}

pub trait PlayerSize {
  const CROUCH_BITS: usize;
  const MAY_BE_BIG: bool;
  fn is_big(&State) -> bool;
}
#[allow(dead_code)]
pub enum Small {}
impl PlayerSize for Small {
  const CROUCH_BITS: usize = 0;
  const MAY_BE_BIG: bool = false;
  fn is_big(_: &State) -> bool { false }
}
#[allow(dead_code)]
pub enum Big {}
impl PlayerSize for Big {
  const CROUCH_BITS: usize = 1;
  const MAY_BE_BIG: bool = true;
  fn is_big(_: &State) -> bool { true }
}
#[allow(dead_code)]
pub enum BigAfterPowerup {}
impl PlayerSize for BigAfterPowerup {
  const CROUCH_BITS: usize = 1;
  const MAY_BE_BIG: bool = true;
  fn is_big(s: &State) -> bool { s.powerup_collected }
}

pub trait CoinHandler {
  const COIN_HANDLER_BITS: usize;
  fn is_coin_collected(&State, usize, usize) -> bool;
  fn collect_coin(&mut State, usize, usize) -> ();
}
#[allow(dead_code)]
pub enum IgnoreCoins {}
impl CoinHandler for IgnoreCoins {
  const COIN_HANDLER_BITS: usize = 0;
  fn is_coin_collected(_: &State, _: usize, _:usize) -> bool { true }
  fn collect_coin(_: &mut State, _: usize, _: usize) -> () {}
}
pub trait SingleCoinHandler {
  const COIN_X: usize;
  const COIN_Y: usize;
}
impl<T: SingleCoinHandler> CoinHandler for T {
  const COIN_HANDLER_BITS: usize = 1;
  fn is_coin_collected(s: &State, cx: usize, cy: usize) -> bool {
    s.coin_collected || Self::COIN_X != cx || Self::COIN_Y != cy
  }
  fn collect_coin(s: &mut State, _: usize, _: usize) -> () {
    s.coin_collected = true;
  }
}

pub trait PowerupHandler {
  const POWERUP_HANDLER_BITS: usize;
  fn is_activated_powerup_block(s: &State, cx: usize, cy: usize) -> bool;
  fn activate_powerup_block(s: &mut State, cx: usize, cy: usize) -> ();
}
#[allow(dead_code)]
pub enum NoPowerups {}
impl PowerupHandler for NoPowerups {
  const POWERUP_HANDLER_BITS: usize = 0;
  fn is_activated_powerup_block(_: &State, _: usize, _: usize) -> bool { false }
  fn activate_powerup_block(_: &mut State, _: usize, _: usize) -> () {}
}
#[allow(dead_code)]
pub struct SinglePowerupHandler<X: Unsigned, Y: Unsigned> {
  _x: PhantomData<X>,
  _y: PhantomData<Y>,
  _void: Void,
}
impl<X: Unsigned, Y: Unsigned> PowerupHandler for SinglePowerupHandler<X, Y> {
  const POWERUP_HANDLER_BITS: usize = 2;
  fn is_activated_powerup_block(s: &State, cx: usize, cy: usize) -> bool {
    s.powerup_block_hit && cx == X::to_usize() && cy == Y::to_usize()
  }
  fn activate_powerup_block(s: &mut State, cx: usize, cy: usize) -> () {
    if cx == X::to_usize() && cy == Y::to_usize() { s.powerup_block_hit = true };
  }
}
#[allow(dead_code)]
pub enum ImaginaryPowerup {}
impl PowerupHandler for ImaginaryPowerup {
  const POWERUP_HANDLER_BITS: usize = 2;
  fn is_activated_powerup_block(_: &State, _: usize, _: usize) -> bool { false }
  fn activate_powerup_block(_: &mut State, _: usize, _: usize) -> () {}
}

pub trait VerticalPipeHandler {
  fn enter_vertical_pipe(cx: usize, cy: usize) -> bool;
}
#[allow(dead_code)]
pub enum IgnoreVerticalPipes {}
impl VerticalPipeHandler for IgnoreVerticalPipes {
  fn enter_vertical_pipe(_: usize, _: usize) -> bool { false }
}
#[allow(dead_code)]
pub struct EnterVerticalPipe<X: Unsigned, Y: Unsigned> {
  _x: PhantomData<X>,
  _y: PhantomData<Y>,
  _void: Void,
}
impl<X: Unsigned, Y: Unsigned> VerticalPipeHandler for EnterVerticalPipe<X, Y> {
  fn enter_vertical_pipe(cx: usize, cy: usize) -> bool {
    cx == X::to_usize() && cy == Y::to_usize()
  }
}

pub trait Swim {
  const IS_SWIMMING: bool;
  const SWIMMING_BITS: usize;
}
#[allow(dead_code)]
pub enum NotSwimming {}
impl Swim for NotSwimming {
  const IS_SWIMMING: bool = false;
  const SWIMMING_BITS: usize = 0;
}
#[allow(dead_code)]
pub enum Swimming {}
impl Swim for Swimming {
  const IS_SWIMMING: bool = true;
  const SWIMMING_BITS: usize = 5;
}

pub trait RunningTimer {
  const USE_RUNNING_TIMER: bool;
  const RUNNING_TIMER_BITS: usize;
}
#[allow(dead_code)]
pub enum NoRunningTimer {}
impl RunningTimer for NoRunningTimer {
  const USE_RUNNING_TIMER: bool = false;
  const RUNNING_TIMER_BITS: usize = 0;
}
#[allow(dead_code)]
pub enum WithRunningTimer {}
impl RunningTimer for WithRunningTimer {
  const USE_RUNNING_TIMER: bool = true;
  const RUNNING_TIMER_BITS: usize = 4;
}

pub trait YPosFractionalBehavior {
  const CLEAR_Y_POS_FRACTIONALS: bool;
}
#[allow(dead_code)]
pub enum ClearYPosFractionals {}
impl YPosFractionalBehavior for ClearYPosFractionals {
  const CLEAR_Y_POS_FRACTIONALS: bool = true;
}
#[allow(dead_code)]
pub enum KeepYPosFractionals {}
impl YPosFractionalBehavior for KeepYPosFractionals {
  const CLEAR_Y_POS_FRACTIONALS: bool = false;
}

pub trait ScrollPos {
  const TRACK_SCROLL_POS: bool;
  const SCROLL_POS_BITS: usize;
}
#[allow(dead_code)]
pub enum NoScrollPos {}
impl ScrollPos for NoScrollPos {
  const TRACK_SCROLL_POS: bool = false;
  const SCROLL_POS_BITS: usize = 0;
}
#[allow(dead_code)]
pub enum WithScrollPos {}
impl ScrollPos for WithScrollPos {
  const TRACK_SCROLL_POS: bool = true;
  const SCROLL_POS_BITS: usize = 12;
}

pub trait Parity {
  const PARITY: u8;
  const PARITY_BITS: usize;
}
#[allow(dead_code)]
pub enum NoParity {}
impl Parity for NoParity {
  const PARITY: u8 = 1;
  const PARITY_BITS: usize = 0;
}
#[allow(dead_code)]
pub enum Parity2 {}
impl Parity for Parity2 {
  const PARITY: u8 = 2;
  const PARITY_BITS: usize = 1;
}
#[allow(dead_code)]
pub enum Parity3 {}
impl Parity for Parity3 {
  const PARITY: u8 = 3;
  const PARITY_BITS: usize = 2;
}
#[allow(dead_code)]
pub enum Parity4 {}
impl Parity for Parity4 {
  const PARITY: u8 = 4;
  const PARITY_BITS: usize = 2;
}
#[allow(dead_code)]
pub enum Parity8 {}
impl Parity for Parity8 {
  const PARITY: u8 = 8;
  const PARITY_BITS: usize = 3;
}

pub trait Platform {
  const MAX_X_SPD_RUN: i16;
  const MAX_X_SPD_WALK: i16;
  const MAX_X_SPD_SWIM: i16;

  const MAX_Y_SPD: i16;
  const BLOCK_SURFACE_THICKNESS: i32;

  const FRICTION_RUN: i16;
  const FRICTION_WALK_FAST: i16;
  const FRICTION_WALK_SLOW: i16;

  const BLOCK_BUFFER_X_ADDER_DATA: [u16; 21] = [
      0x0800,  0x0300,  0x0c00, 0x0200,  0x0200, 0x0d00,  0x0d00,
      0x0800,  0x0300,  0x0c00, 0x0200,  0x0200, 0x0d00,  0x0d00,
      0x0800,  0x0300,  0x0c00, 0x0200,  0x0200, 0x0d00,  0x0d00];
  const BLOCK_BUFFER_Y_ADDER_DATA: [u16; 21] = [
      0x400, 0x2000, 0x2000,  0x800, 0x1800,  0x800, 0x1800,
      0x200, 0x2000, 0x2000,  0x800, 0x1800,  0x800, 0x1800,
      0x1200, 0x2000, 0x2000, 0x1800, 0x1800, 0x1800, 0x1800];

  const JUMP_VELOCITY_SLOW: i16;
  const JUMP_VELOCITY_FAST: i16;
  const JUMP_VELOCITY_SWIM: i16;

  const V_FORCE_SWIM_TOO_HIGH: u8 = 0x18;
  const V_FORCE_AREA_INIT: u8;
  const V_FORCE_JUMP_STANDING: u8;
  const V_FORCE_JUMP_WALKING: u8;
  const V_FORCE_JUMP_RUNNING: u8;
  const V_FORCE_JUMP_SWIMMING: u8 = 0x0d;
  const V_FORCE_FALL_STANDING: u8;
  const V_FORCE_FALL_WALKING: u8;
  const V_FORCE_FALL_RUNNING: u8;
  const V_FORCE_FALL_SWIMMING: u8 = 0x0a;

  const X_SPD_ABS_CUTOFFS: [u8; 6];
  fn get_x_spd_abs_cutoff(x_spd_abs: u8) -> usize {
    for i in (0..6).rev() {
      if x_spd_abs >= Self::X_SPD_ABS_CUTOFFS[i] {
        return i;
      }
    }
    0
  }
  fn get_v_force_index(v_force: u8) -> u32 {
    if v_force == Self::V_FORCE_SWIM_TOO_HIGH { 0 }
    else if v_force == Self::V_FORCE_AREA_INIT { 1 }
    else if v_force == Self::V_FORCE_JUMP_STANDING { 2 }
    else if v_force == Self::V_FORCE_JUMP_WALKING { 3 }
    else if v_force == Self::V_FORCE_JUMP_RUNNING { 4 }
    else if v_force == Self::V_FORCE_JUMP_SWIMMING { 5 }
    else if v_force == Self::V_FORCE_FALL_STANDING { 6 }
    else if v_force == Self::V_FORCE_FALL_WALKING { 7 }
    else if v_force == Self::V_FORCE_FALL_RUNNING { 8 }
    else if v_force == Self::V_FORCE_FALL_SWIMMING { 9 }
    else { panic!("unexpected v_force value {}", v_force); }
  }
}
#[allow(dead_code)]
pub enum NTSC {}
impl Platform for NTSC {
  const MAX_X_SPD_RUN: i16 = 0x2800;
  const MAX_X_SPD_WALK: i16 = 0x1800;
  const MAX_X_SPD_SWIM: i16 = 0x1000;

  const MAX_Y_SPD: i16 = 0x400;
  const BLOCK_SURFACE_THICKNESS: i32 = 0x500;

  const FRICTION_RUN: i16 = 0xe4;
  const FRICTION_WALK_FAST: i16 = 0xd0;
  const FRICTION_WALK_SLOW: i16 = 0x98;

  const JUMP_VELOCITY_SLOW: i16 = -0x400;
  const JUMP_VELOCITY_FAST: i16 = -0x500;
  const JUMP_VELOCITY_SWIM: i16 = -0x180;

  const V_FORCE_AREA_INIT: u8 = 0x28;
  const V_FORCE_JUMP_STANDING: u8 = 0x20;
  const V_FORCE_JUMP_WALKING: u8 = 0x1e;
  const V_FORCE_JUMP_RUNNING: u8 = 0x28;
  const V_FORCE_FALL_STANDING: u8 = 0x70;
  const V_FORCE_FALL_WALKING: u8 = 0x60;
  const V_FORCE_FALL_RUNNING: u8 = 0x90;

  const X_SPD_ABS_CUTOFFS: [u8; 6] = [0x00, 0x0b, 0x10, 0x19, 0x1c, 0x21];
}
#[allow(dead_code)]
pub enum PAL {}
impl Platform for PAL {
  const MAX_X_SPD_RUN: i16 = 0x3000;
  const MAX_X_SPD_WALK: i16 = 0x1c00;
  const MAX_X_SPD_SWIM: i16 = 0x1300;

  const MAX_Y_SPD: i16 = 0x500;
  const BLOCK_SURFACE_THICKNESS: i32 = 0x600;

  const FRICTION_RUN: i16 = 0x1c0;
  const FRICTION_WALK_FAST: i16 = 0x180;
  const FRICTION_WALK_SLOW: i16 = 0x100;

  const JUMP_VELOCITY_SLOW: i16 = -0x4cc;
  const JUMP_VELOCITY_FAST: i16 = -0x600;
  const JUMP_VELOCITY_SWIM: i16 = -0x180;

  const V_FORCE_AREA_INIT: u8 = 0x70;
  const V_FORCE_JUMP_STANDING: u8 = 0x30;
  const V_FORCE_JUMP_WALKING: u8 = 0x2d;
  const V_FORCE_JUMP_RUNNING: u8 = 0x38;
  const V_FORCE_FALL_STANDING: u8 = 0xa8;
  const V_FORCE_FALL_WALKING: u8 = 0x90;
  const V_FORCE_FALL_RUNNING: u8 = 0xd0;

  const X_SPD_ABS_CUTOFFS: [u8; 6] = [0x00, 0x0d, 0x12, 0x1d, 0x20, 0x27];
}
