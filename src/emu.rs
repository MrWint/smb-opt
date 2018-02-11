use blockbuffer::BlockBuffer;
use blockbuffer::util::*;
use state::{Dir,PlayerState,State};
use std::marker::PhantomData;
use options::*;


bitflags! {
  pub struct Input: u8 {
    const A      = 0b10000000;
    const B      = 0b01000000;
    const SELECT = 0b00100000;
    const START  = 0b00010000;
    const UP     = 0b00001000;
    const DOWN   = 0b00000100;
    const LEFT   = 0b00000010;
    const RIGHT  = 0b00000001;
  }
}
impl ::std::fmt::Display for Input {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      use self::inputs::*;

      if *self == A|L|R { write!(f, "A|L|R") }
      else if *self == L|R { write!(f, "L|R") }
      else if *self == A|L { write!(f, "A|L") }
      else if *self == A|R { write!(f, "A|R") }
      else if *self == L { write!(f, "L") }
      else if *self == R { write!(f, "R") }
      else if *self == B|L|R { write!(f, "B|L|R") }
      else if *self == B|L { write!(f, "B|L") }
      else if *self == B|R { write!(f, "B|R") }
      else if *self == U { write!(f, "U") }
      else if *self == D { write!(f, "D") }
      else if *self == NIL { write!(f, "NIL") }
      else { write!(f, "{:?}", self) }
    }
}
pub mod inputs {
  use super::Input;

  pub const A: Input      = Input::A;
  pub const B: Input      = Input::B;
  pub const U: Input      = Input::UP;
  pub const D: Input      = Input::DOWN;
  pub const L: Input      = Input::LEFT;
  pub const R: Input      = Input::RIGHT;
  pub const NIL: Input    = Input { bits: 0b00000000 };
}
pub fn print_rle<I: Iterator>(mut inputs: I) -> () where I::Item: PartialEq + ::std::fmt::Display {
  let mut len = 0;
  print!("[");
  if let Some(mut cur_input) = inputs.next() {
    len += 1;
    let mut count = 1;
    while let Some(next_input) = inputs.next() {
      len += 1;
      if next_input == cur_input {
        count += 1;
      } else {
        print!("{}x {}, ", count, cur_input);
        cur_input = next_input;
        count = 1;
      }
    }
    print!("{}x {}", count, cur_input);
  }
  println!("] (len: {})", len);
}

pub trait Emu {
  fn run_step(State, Input) -> (State, EmuResult);
  fn run_step_nr(s: State, input: Input) -> State {
    let (new_state, emu_result) = Self::run_step(s, input);
    assert!(emu_result == EmuResult::Success);
    new_state
  }
  fn run_steps_nr(mut s: State, inputs: &[Input]) -> State {
    for input in inputs { s = Self::run_step_nr(s, *input); }
    s
  }
  fn iterate_entrance(mut s: State) -> State {
    while (s.y_pos & 0xff00) < 0x3000 { s = Self::run_step_nr(s, inputs::NIL); }
    s
  }
}

pub struct SmbEmu<O: Options, B: BlockBuffer> {
  s: State,
  joypad: Input,
  joypad_lr: Dir,
  joypad_ud: Input,
  max_speed_left: i16,
  max_speed_right: i16,
  friction: i16,
  started_jump: bool,
  started_on_ground: bool,
  x_scroll: i8, // only for scroll
  side_collision: bool, // only for scroll
  options: PhantomData<O>,
  block_buffer: PhantomData<B>,
}
impl<O: Options, B: BlockBuffer> SmbEmu<O, B> {
  fn new(s: State, joypad: Input) -> Self {
    SmbEmu {
      s,
      joypad,
      joypad_lr: Dir::empty(),
      joypad_ud: Input::empty(),
      max_speed_left: 0,
      max_speed_right: 0,
      friction: 0,
      started_jump: false,
      started_on_ground: false,
      x_scroll: 0,
      side_collision: false,
      options: PhantomData,
      block_buffer: PhantomData,
    }
  }
  fn run_step(mut self) -> (State, EmuResult) {
    self.started_on_ground = self.s.is_on_ground();
    if O::RunningTimer::USE_RUNNING_TIMER && self.s.running_timer > 0 { self.s.running_timer -= 1; }

    let result = self.player_ctrl_routine();

    if O::ScrollPos::TRACK_SCROLL_POS && self.side_collision { self.s.side_collision_timer = 0xf; }
    else if O::ScrollPos::TRACK_SCROLL_POS && self.s.side_collision_timer > 0 { self.s.side_collision_timer -= 1; }

    if O::Swim::IS_SWIMMING && self.started_jump { self.s.jump_swim_timer = 0x1f; }
    else if O::Swim::IS_SWIMMING && self.s.jump_swim_timer > 0 { self.s.jump_swim_timer -= 1; }

    if O::YPosFractionalBehavior::CLEAR_Y_POS_FRACTIONALS && self.s.is_on_ground() { self.s.y_pos &= 0xffff00; self.s.v_force_down = O::Platform::V_FORCE_AREA_INIT; }
    if self.s.player_state != PlayerState::JUMPING { self.s.v_force = self.s.v_force_down; } // only needed for JumpSwim, set whenever entered
    if O::YPosFractionalBehavior::CLEAR_Y_POS_FRACTIONALS && self.started_on_ground && self.s.player_state == PlayerState::FALLING { // ran off edge with cleared fractional
      return (self.s, EmuResult::InvalidStateFallingWithClearedYposFractionals);
    }
    self.s.parity = (self.s.parity + 1) % O::Parity::PARITY;

    (self.s, result)
  }
  fn player_ctrl_routine(&mut self) -> EmuResult {
    if self.s.y_pos < 0x10000 || self.s.y_pos >= 0x1d000 { self.joypad = Input::empty(); };
    self.joypad_lr = Dir::from_bits_truncate(self.joypad.bits());
    self.joypad_ud = self.joypad & (inputs::U | inputs::D);
    if self.joypad.contains(Input::DOWN) && self.s.is_on_ground() && !self.joypad_lr.is_empty() {
      self.joypad_lr = Dir::empty();
      self.joypad_ud = Input::empty();
    }
    self.player_movement_subs();
    if self.s.x_spd < 0  { self.s.moving_dir = Dir::LEFT; }
    else if self.s.x_spd >= 0x100 { self.s.moving_dir = Dir::RIGHT; }

    self.scroll_handler();

    self.player_bg_collision()
  }
  fn player_movement_subs(&mut self) -> () {
    if O::PlayerSize::is_big(&self.s) && self.s.is_on_ground() {
      self.s.is_crouching = self.joypad_ud.contains(Input::DOWN);
    }

    self.player_physics_sub();

    // MoveSubs
    match self.s.player_state {
      PlayerState::STANDING => {
        self.get_player_anim_speed();
        if !self.joypad_lr.is_empty() { self.s.facing_dir = self.joypad_lr; }
        self.impose_friction();
        self.move_horizontally();
      }
      PlayerState::JUMPING => {
        if self.s.y_spd >= 0 || (!self.joypad.contains(Input::A) && !self.started_jump) { self.s.v_force = self.s.v_force_down; }
        if O::Swim::IS_SWIMMING {
          self.get_player_anim_speed();
          if self.s.y_pos < 0x11400 { self.s.v_force = O::Platform::V_FORCE_SWIM_TOO_HIGH; }
          if !self.joypad_lr.is_empty() { self.s.facing_dir = self.joypad_lr; }
        }
        if !self.joypad_lr.is_empty() { self.impose_friction(); }
        self.move_horizontally();
        self.move_vertically();
      }
      PlayerState::FALLING => {
        self.s.v_force = self.s.v_force_down;
        if !self.joypad_lr.is_empty() { self.impose_friction(); }
        self.move_horizontally();
        self.move_vertically();
      }
      PlayerState::CLIMBING => {
        unimplemented!();
      }
    }
  }
  fn player_physics_sub(&mut self) -> () {
    // handle starting jumps
    if self.joypad.contains(Input::A) && (self.s.is_on_ground() || (O::Swim::IS_SWIMMING && (self.s.jump_swim_timer != 0 || self.s.y_spd >= 0))) {
      self.s.y_pos &= 0xffff00; // clear fractional yPos
      self.started_jump = true;
      self.s.player_state = PlayerState::JUMPING;
      if O::Swim::IS_SWIMMING {
        self.s.v_force = O::Platform::V_FORCE_JUMP_SWIMMING;
        self.s.v_force_down = O::Platform::V_FORCE_FALL_SWIMMING;
        self.s.y_spd = O::Platform::JUMP_VELOCITY_SWIM;
        if self.s.y_pos < 0x11400 { self.s.y_spd &= 0xff; } // kill upward momentum if swimming too high
      } else if self.s.x_spd_abs >= O::Platform::X_SPD_ABS_CUTOFFS[3] { // running speed
        self.s.v_force = O::Platform::V_FORCE_JUMP_RUNNING;
        self.s.v_force_down = O::Platform::V_FORCE_FALL_RUNNING;
        self.s.y_spd = O::Platform::JUMP_VELOCITY_FAST;
        } else if self.s.x_spd_abs >= O::Platform::X_SPD_ABS_CUTOFFS[2] { // walking speed
        self.s.v_force = O::Platform::V_FORCE_JUMP_WALKING;
        self.s.v_force_down = O::Platform::V_FORCE_FALL_WALKING;
        self.s.y_spd = O::Platform::JUMP_VELOCITY_SLOW;
      } else {
        self.s.v_force = O::Platform::V_FORCE_JUMP_STANDING;
        self.s.v_force_down = O::Platform::V_FORCE_FALL_STANDING;
        self.s.y_spd = O::Platform::JUMP_VELOCITY_SLOW;
      }
    }

    // X_Physics
    let is_running: bool = !O::Swim::IS_SWIMMING && self.s.is_on_ground() && self.joypad_lr == self.s.moving_dir && self.joypad.contains(Input::B);
    if O::RunningTimer::USE_RUNNING_TIMER && is_running { self.s.running_timer = 0xa; }

    if !self.s.is_on_ground() && self.s.x_spd_abs >= O::Platform::X_SPD_ABS_CUTOFFS[3] {
      self.max_speed_right = O::Platform::MAX_X_SPD_RUN;
      self.max_speed_left = -O::Platform::MAX_X_SPD_RUN;
      self.friction = O::Platform::FRICTION_RUN;
    } else if !self.s.is_on_ground() && self.s.running_speed {
      self.max_speed_right = O::Platform::MAX_X_SPD_WALK;
      self.max_speed_left = -O::Platform::MAX_X_SPD_WALK;
      self.friction = O::Platform::FRICTION_WALK_FAST;
    } else if !self.s.is_on_ground() {
      self.max_speed_right = O::Platform::MAX_X_SPD_WALK;
      self.max_speed_left = -O::Platform::MAX_X_SPD_WALK;
      self.friction = O::Platform::FRICTION_WALK_SLOW;
    } else if O::Swim::IS_SWIMMING && !self.s.running_speed && self.s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[5] {
      self.max_speed_right = O::Platform::MAX_X_SPD_SWIM;
      self.max_speed_left = -O::Platform::MAX_X_SPD_SWIM;
      self.friction = O::Platform::FRICTION_WALK_SLOW;
    } else if O::Swim::IS_SWIMMING {
      self.max_speed_right = O::Platform::MAX_X_SPD_SWIM;
      self.max_speed_left = -O::Platform::MAX_X_SPD_SWIM;
      self.friction = O::Platform::FRICTION_WALK_FAST;
    } else if self.joypad_lr == self.s.moving_dir && ((O::RunningTimer::USE_RUNNING_TIMER && self.s.running_timer > 0) || (!O::RunningTimer::USE_RUNNING_TIMER && is_running)) {
      self.max_speed_right = O::Platform::MAX_X_SPD_RUN;
      self.max_speed_left = -O::Platform::MAX_X_SPD_RUN;
      self.friction = O::Platform::FRICTION_RUN;
    } else if !self.s.running_speed && self.s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[5] {
      self.max_speed_right = O::Platform::MAX_X_SPD_WALK;
      self.max_speed_left = -O::Platform::MAX_X_SPD_WALK;
      self.friction = O::Platform::FRICTION_WALK_SLOW;
    } else {
      self.max_speed_right = O::Platform::MAX_X_SPD_WALK;
      self.max_speed_left = -O::Platform::MAX_X_SPD_WALK;
      self.friction = O::Platform::FRICTION_WALK_FAST;
    }

    if self.s.facing_dir != self.s.moving_dir { self.friction <<= 1; }
  }
  fn get_player_anim_speed(&mut self) -> () {
    if self.s.x_spd_abs >= O::Platform::X_SPD_ABS_CUTOFFS[4] { self.s.running_speed = true; }
    else if !Input::A.contains(self.joypad) && Dir::from_bits_truncate(self.joypad.bits()) == self.s.moving_dir { self.s.running_speed = false; }
    else if !Input::A.contains(self.joypad) && self.s.x_spd_abs < O::Platform::X_SPD_ABS_CUTOFFS[1] {
      self.s.moving_dir = self.s.facing_dir;
      self.s.x_spd = 0;
    }
  }
  fn impose_friction(&mut self) -> () {
    let lr_collision: Dir = self.joypad_lr & self.s.collision_bits;
    if lr_collision.contains(Dir::RIGHT) || (lr_collision.is_empty() && self.s.x_spd < 0) { // LeftFrict
      self.s.x_spd += self.friction;
      if self.s.x_spd >= self.max_speed_right { self.s.x_spd = self.max_speed_right + (self.s.x_spd & 0xff); }
    } else if !lr_collision.is_empty() || self.s.x_spd >= 0x100 { // RightFrict
      self.s.x_spd -= self.friction;
      if self.s.x_spd <= self.max_speed_left { self.s.x_spd = self.max_speed_left + (self.s.x_spd & 0xff); }
    }
    let x_spd_abs: u8 = (self.s.x_spd >> 8).abs() as u8;
    self.s.set_x_spd_abs::<O::Platform>(x_spd_abs);
  }
  fn move_horizontally(&mut self) -> () {
    let old_x_pos = self.s.x_pos;
    self.s.x_pos = self.s.x_pos + ((self.s.x_spd as i32 >> 8) << 4);
    if O::ScrollPos::TRACK_SCROLL_POS {
      self.x_scroll = ((self.s.x_pos >> 8) - (old_x_pos >> 8)) as i8;
    }
  }
  fn move_vertically(&mut self) -> () {
    self.s.y_pos += self.s.y_spd as i32;
    self.s.y_spd += self.s.v_force as i16;
    if self.s.y_spd >= O::Platform::MAX_Y_SPD && (self.s.y_spd & 0xff) >= 0x80 { self.s.y_spd = O::Platform::MAX_Y_SPD; }
  }
  fn scroll_handler(&mut self) -> () {
    if O::ScrollPos::TRACK_SCROLL_POS {
      let rel_x_pos = ((self.s.x_pos >> 8) - self.s.left_screen_edge_pos as i32) & 0xff;
      if rel_x_pos < 0x50 || self.s.side_collision_timer > 0 || self.x_scroll <= 0 { return; }
      if rel_x_pos < 0x70 && self.x_scroll >= 2 { self.x_scroll -= 1; }
      self.s.left_screen_edge_pos = self.s.left_screen_edge_pos.wrapping_add(self.x_scroll as u8);
    }
  }
  fn block_buffer_collision(&self, block_buffer_adder_offset: usize) -> CollisionResult {
    let block_buffer_adder_offset = block_buffer_adder_offset + if !O::PlayerSize::is_big(&self.s) || self.s.is_crouching { 0x0e } else if O::Swim::IS_SWIMMING { 0x07 } else { 0 };

    let bx = O::Platform::BLOCK_BUFFER_X_ADDER_DATA[block_buffer_adder_offset] as usize;
    let by = O::Platform::BLOCK_BUFFER_Y_ADDER_DATA[block_buffer_adder_offset] as usize;
    let cx: usize = (self.s.x_pos as usize + bx) >> 12;
    let cy: usize = ((self.s.y_pos as usize + by - 0x2000) >> 12) & 0x0f;

    let mut cv = B::get_block_at(cx, cy);

    if is_coin(cv) && O::CoinHandler::is_coin_collected(&self.s, cx, cy) { cv = 0; } // ignore collected coins
    if is_question_block(cv) && O::PowerupHandler::is_activated_powerup_block(&self.s, cx, cy) { cv = 0xc4; } // question block changed to solid block
    if cv == 0 { CollisionResult::NoCollision } else { CollisionResult::Collision(cv, cx, cy) }
  }
  fn player_bg_collision(&mut self) -> EmuResult {
    if O::Swim::IS_SWIMMING { self.s.player_state = PlayerState::JUMPING }
    else if self.s.player_state == PlayerState::STANDING || self.s.player_state == PlayerState::CLIMBING { self.s.player_state = PlayerState::FALLING; }

    if self.s.y_pos < 0x10000 || self.s.y_pos >= 0x20000 { return EmuResult::Success; } // yPos out of bounds
    self.s.collision_bits = Dir::LR;
    if self.s.y_pos >= 0x1cf00 { return EmuResult::Success; } // yPos out of bounds

    if self.s.y_pos >= (if O::PlayerSize::is_big(&self.s) && !self.s.is_crouching { 0x12000 } else { 0x11000 }) { // HeadChk
      if let CollisionResult::Collision(cv, cx, cy) = self.block_buffer_collision(0) {
        if is_coin(cv) {
          O::CoinHandler::collect_coin(&mut self.s, cx, cy);
          return EmuResult::Success; // exit (no feet or side checks)
        } else if self.s.y_spd < 0 && (self.s.y_pos & 0x0f00) >= 0x400 {
          if is_solid(cv) || O::Swim::IS_SWIMMING {
            self.s.y_spd = 0x100 + (self.s.y_spd & 0xff); // hit solid block
          } else if O::PlayerSize::is_big(&self.s) && !is_question_block(cv) {
            self.s.y_spd = -0x200 + (self.s.y_spd & 0xff); // shatter brick
          } else {
            if is_question_block(cv) { O::PowerupHandler::activate_powerup_block(&mut self.s, cx, cy); }
            self.s.y_spd &= 0xff; // bump block
          }
        }
      }
    }

    { // DoFootCheck
      let right_foot_on_vert_pipe;
      let foot_collision;
      let left_foot_collision = self.block_buffer_collision(1);
      let right_foot_collision = self.block_buffer_collision(2);
      if let CollisionResult::NoCollision = left_foot_collision {
        right_foot_on_vert_pipe = false;
        foot_collision = right_foot_collision;
      } else {
        right_foot_on_vert_pipe = if let CollisionResult::Collision(cv, _, _) = right_foot_collision { cv == 0x11 } else { false };
        foot_collision = left_foot_collision;
      }

      if let CollisionResult::Collision(cv, cx, cy) = foot_collision {
        if is_coin(cv) {
          O::CoinHandler::collect_coin(&mut self.s, cx, cy);
          return EmuResult::Success; // exit (no side checks)
        } else if !is_climb(cv) && !is_hidden_block(cv) && self.s.y_spd >= 0 {
          if cv == 0xc5 { // axe hit
            return EmuResult::StateChangeAxe(cx, cy);
          } else if (self.s.y_pos & 0x0f00) >= O::Platform::BLOCK_SURFACE_THICKNESS {
            let moving_dir = self.s.moving_dir;
            self.impede_player_move(moving_dir);
            return EmuResult::Success; // exit (no side checks)
          } else {
            self.s.y_pos &= 0xfff0ff; // align height with block
            if (!self.started_on_ground || self.joypad_lr.is_empty()) && cv == 0x10 && right_foot_on_vert_pipe && O::VerticalPipeHandler::enter_vertical_pipe(cx, cy) {
              return EmuResult::StateChangeVerticalPipe(cx, cy); // vertical pipe entry
            }
            self.s.y_spd = 0; // kill vertical speed
            self.s.player_state = PlayerState::STANDING; // land
          }
        }
      }
    }

    if self.s.y_pos >= 0x10800 {
      for dir in [Dir::LEFT, Dir::RIGHT].iter() { // DoPlayerSideCheck
        if self.s.y_pos >= 0x12000 {
          if let CollisionResult::Collision(cv, cx, cy) = self.block_buffer_collision(3 + 2 * (2 - dir.bits() as usize)) {
            if cv != 0x1c && cv != 0x6b && !is_climb(cv) {
              return self.check_side_mtiles(*dir, cv, cx, cy);
            }
          }
        }
        if let CollisionResult::Collision(cv, cx, cy) = self.block_buffer_collision(4 + 2 * (2 - dir.bits() as usize)) {
          return self.check_side_mtiles(*dir, cv, cx, cy);
        }
      }
    }
    EmuResult::Success
  }

  fn check_side_mtiles(&mut self, moving_dir: Dir, cv: u8, cx: usize, cy: usize) -> EmuResult {
    if is_hidden_block(cv) {
      return EmuResult::Success;
    } else if is_climb(cv) {
      if (self.s.x_pos & 0x0f00) >= 0x600 && (self.s.x_pos & 0x0f00) < 0xa00 {
        return if cv == 0x24 || cv == 0x25 { // Hit flag
          self.s.facing_dir = Dir::RIGHT;
          self.put_player_on_vine(cx);
          EmuResult::StateChangeFlag(cx, cy)
        } else { // Hit vine
          self.put_player_on_vine(cx);
          if cv == 0x26 && (self.s.y_pos & 0xff00) < 0x2000 { EmuResult::StateChangeVineAutoclimb(cx, cy) } else { EmuResult::HitVine(cx, cy) }
        }
      }
    } else if is_coin(cv) {
      O::CoinHandler::collect_coin(&mut self.s, cx, cy);
      return EmuResult::Success; // grab coin
    } else if self.s.is_on_ground() && self.s.facing_dir == Dir::RIGHT && (cv == 0x6c || cv == 0x1f) {
      return EmuResult::StateChangeSidePipe(cx, cy); // sideways pipe entry
    } else {
      self.impede_player_move(moving_dir);
    }
    EmuResult::Success
  }
  fn put_player_on_vine(&mut self, cx: usize) {
    const CLIMB_X_POS_ADDER: [i32; 4] = [0x8a, 0xf9, 0x7, 0xff];
    const CLIMB_PAGE_LOC_ADDER: [i32; 4] = [0x7, 0xff, 0x0, 0x18];

    self.s.player_state = PlayerState::CLIMBING;
    self.s.x_spd = 0;
    if O::ScrollPos::TRACK_SCROLL_POS && ((self.s.x_pos >> 8) - self.s.left_screen_edge_pos as i32) & 0xff < 16 { self.s.facing_dir = Dir::LEFT; }
    let x_pos = (((cx as i32) << 4) + CLIMB_X_POS_ADDER[self.s.facing_dir.bits() as usize]) & 0xff;
    let x_page_loc = if cx & 0xf != 0 { self.s.x_pos >> 16 } else {
      let left_screen_edge_pos: u8 = if O::ScrollPos::TRACK_SCROLL_POS { self.s.left_screen_edge_pos } else { (((self.s.x_pos >> 8) - 0x70) & 0xff) as u8 }; // assume middle of screen if scroll is not tracked
      let x_pos_byte: u8 = ((self.s.x_pos >> 8) & 0xff) as u8;
      let left_screen_page_loc = if left_screen_edge_pos <= x_pos_byte { self.s.x_pos >> 16 } else { (self.s.x_pos >> 16) - 1 };
      let right_screen_page_loc = if left_screen_edge_pos == 0 { left_screen_page_loc } else { left_screen_page_loc + 1 };
      (right_screen_page_loc + CLIMB_PAGE_LOC_ADDER[self.s.facing_dir.bits() as usize]) & 0xff
    };
    self.s.x_pos = (self.s.x_pos & 0xff) | (x_pos << 8) | (x_page_loc << 16);
  }
  fn impede_player_move(&mut self, moving_dir: Dir) -> () {
    if moving_dir == Dir::RIGHT && self.s.x_spd >= 0 {
      self.s.x_spd &= 0xff;
      self.s.x_pos -= 0x100;
      if O::ScrollPos::TRACK_SCROLL_POS { self.side_collision = true; }
    } else if moving_dir != Dir::RIGHT && self.s.x_spd < 0x100 {
      self.s.x_spd &= 0xff;
      self.s.x_pos += 0x100;
      if O::ScrollPos::TRACK_SCROLL_POS { self.side_collision = true; }
    }
    self.s.collision_bits -= moving_dir;
  }
}
impl<O: Options, B: BlockBuffer> Emu for SmbEmu<O, B> {
    fn run_step(s: State, input: Input) -> (State, EmuResult) {
        SmbEmu::<O, B>::new(s, input).run_step()
    }
}

enum CollisionResult {
    NoCollision,
    Collision(u8, usize, usize)
}

#[derive(Debug, Eq, PartialEq)]
pub enum EmuResult {
  Success,
  StateChangeAxe(usize, usize),
  StateChangeVerticalPipe(usize, usize),
  StateChangeSidePipe(usize, usize),
  StateChangeFlag(usize, usize),
  StateChangeVineAutoclimb(usize, usize),
  HitVine(usize, usize),
  InvalidStateFallingWithClearedYposFractionals,
}