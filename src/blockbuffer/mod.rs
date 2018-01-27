macro_rules! blockbuf {
  (
    $name:ident, $width:expr, $buf:expr
  ) => {
    #[allow(dead_code)]
    pub enum $name {}
    impl ::blockbuffer::BlockBuffer for $name {
      fn get_block_at(block_x: usize, block_y: usize) -> u8 {
        const BUF: [[u8; $width]; 13] = $buf;

        if block_x < $width && block_y < 13 { BUF[block_y][block_x] } else { 0 }
      }
    }
  }
}

pub mod world1;

pub mod util {
  pub fn is_coin(cv: u8) -> bool {
    cv == 0xc2 || cv == 0xc3
  }
  pub fn is_solid(cv: u8) -> bool {
    (cv < 0x40 && cv >= 0x10)
        || (cv < 0x80 && cv >= 0x61)
        || (cv < 0xc0 && cv >= 0x88)
        || cv >= 0xc4
  }
  pub fn is_climb(cv: u8) -> bool {
    (cv < 0x40 && cv >= 0x24)
        || (cv < 0x80 && cv >= 0x6d)
        || (cv < 0xc0 && cv >= 0x8a)
        || cv >= 0xc6
  }
  pub fn is_hidden_block(cv: u8) -> bool {
    cv == 0x5f || cv == 0x60
  }
  pub fn is_question_block(cv: u8) -> bool {
    cv == 0xc0 || cv == 0xc1 || (cv >= 0x55 && cv <= 0x60)
  }
}

pub trait BlockBuffer {
  fn get_block_at(block_x: usize, block_y: usize) -> u8;
}

#[allow(dead_code)]
pub enum NoCollisions {}
impl BlockBuffer for NoCollisions {
  fn get_block_at(_: usize, _: usize) -> u8 { 0 }
}

blockbuf!(Test, 1, [[0],[0],[0],[0],[0],[0],[0],[0],[0],[0],[0],[0],[0],]);

