#ifndef STATE_H_
#define STATE_H_

#include <functional>
#include <iostream>
#include "kbtree.h"

using namespace std;

// Swimming too high, Area init, jump x3, fall x3, swimming jump/fall
#ifdef PAL
const int vForceValues[] = {0x18, 0x70, 0x2d, 0x30, 0x38, 0x90, 0xa8, 0xd0, 0x0d, 0x0a};
const int xSpdAbsValues[] = {0x00, 0x0d, 0x12, 0x1d, 0x20, 0x27};
#else
const int vForceValues[] = {0x18, 0x28, 0x1e, 0x20, 0x28, 0x60, 0x70, 0x90, 0x0d, 0x0a};
const int xSpdAbsValues[] = {0x00, 0x0b, 0x10, 0x19, 0x1c, 0x21};
#endif /* PAL */

inline int getXSpdAbsIdx(int xSpdAbs) {
  if (xSpdAbs >= xSpdAbsValues[5]) return 5;
  if (xSpdAbs >= xSpdAbsValues[4]) return 4;
  if (xSpdAbs >= xSpdAbsValues[3]) return 3;
  if (xSpdAbs >= xSpdAbsValues[2]) return 2;
  if (xSpdAbs >= xSpdAbsValues[1]) return 1;
  return 0;
}

struct State {
  unsigned short xPos:16; // >>4
  unsigned short yPos:16; // - yPosOffset
  short xSpd:13; // >>2
  unsigned short vForceIdx:4;
  short ySpd:12;
  unsigned short vForceDIdx:4;
  unsigned short facingDir:2;
  unsigned short movingDir:2, playerState:2, xSpdAbsIdx:3, runningSpeed:1;
  unsigned short collisionBits:2;
#ifdef SCREENPOS
  unsigned short sideCollisionTimer:4;
  unsigned short leftScreenEdgePos:8;
#endif
#ifdef SWIMMING
  unsigned short jumpSwimTimer:5;
#endif
#ifdef RUNNINGTIMER
  unsigned short runningTimer:4;
#endif
#ifdef BIG
  unsigned short crouched:1;
#endif
#ifdef COIN
  unsigned short coinCollected:1;
#endif
#ifdef COLLECT_POWERUP
  unsigned short powerupBlockHit:1;
  unsigned short powerupCollected:1;
#endif

  inline bool isOnGround() {
    return !playerState;
  }

  void print() {
    cout << "xPos: " << hex << ((int)xPos << 4) << endl;
    cout << "xSpd: " << hex << (((int)xSpd) << 2) << endl;
    cout << "yPos: " << hex << ((int)yPos + yPosOffset) << endl;
    cout << "ySpd: " << hex << (int)ySpd << endl;
    cout << "vForce: " << hex << vForceValues[vForceIdx] << endl;
    cout << "vForceD: " << hex << vForceValues[vForceDIdx] << endl;
    cout << "facingDir: " << hex << (int)facingDir << endl;
    cout << "movingDir: " << hex << (int)movingDir << endl;
    cout << "playerState: " << hex << (int)playerState << endl;
    cout << "collisionBits: " << hex << (int)collisionBits << endl;
    cout << "xSpdAbsIdx: " << hex << (int)xSpdAbsIdx << endl;
    cout << "runningSpeed: " << hex << (int)runningSpeed << endl;
#ifdef SCREENPOS
    cout << "sideCollisionTimer: " << hex << (int)sideCollisionTimer << endl;
    cout << "leftScreenEdgePos: " << hex << (int)leftScreenEdgePos << endl;
#endif
#ifdef SWIMMING
    cout << "jumpSwimTimer: " << hex << (int)jumpSwimTimer << endl;
#endif
#ifdef RUNNINGTIMER
    cout << "runningTimer: " << hex << (int)runningTimer << endl;
#endif
#ifdef BIG
    cout << "crouched: " << hex << (int)crouched << endl;
#endif
#ifdef COIN
    cout << "coinCollected: " << hex << (int)coinCollected << endl;
#endif
#ifdef COLLECT_POWERUP
    cout << "powerupBlockHit: " << hex << (int)powerupBlockHit << endl;
    cout << "powerupCollected: " << hex << (int)powerupCollected << endl;
#endif
    cout << endl;
  }
};

State iterateEntrance(State s);

inline int compareState(const State& a, const State& b) {
  if (a.xPos != b.xPos) return kb_generic_cmp(a.xPos, b.xPos);
  if (a.xSpd != b.xSpd) return kb_generic_cmp(a.xSpd, b.xSpd);
  if (a.yPos != b.yPos) return kb_generic_cmp(a.yPos, b.yPos);
  if (a.ySpd != b.ySpd) return kb_generic_cmp(a.ySpd, b.ySpd);
  if (a.vForceIdx != b.vForceIdx) return kb_generic_cmp(a.vForceIdx, b.vForceIdx);
  if (a.vForceDIdx != b.vForceDIdx) return kb_generic_cmp(a.vForceDIdx, b.vForceDIdx);
  if (a.facingDir != b.facingDir) return kb_generic_cmp(a.facingDir, b.facingDir);
  if (a.movingDir != b.movingDir) return kb_generic_cmp(a.movingDir, b.movingDir);
  if (a.playerState != b.playerState) return kb_generic_cmp(a.playerState, b.playerState);
  if (a.collisionBits != b.collisionBits) return kb_generic_cmp(a.collisionBits, b.collisionBits);
  if (a.xSpdAbsIdx != b.xSpdAbsIdx) return kb_generic_cmp(a.xSpdAbsIdx, b.xSpdAbsIdx);
#ifdef SCREENPOS
  if (a.sideCollisionTimer != b.sideCollisionTimer) return kb_generic_cmp(a.sideCollisionTimer, b.sideCollisionTimer);
  if (a.leftScreenEdgePos != b.leftScreenEdgePos) return kb_generic_cmp(a.leftScreenEdgePos, b.leftScreenEdgePos);
#endif
#ifdef SWIMMING
  if (a.jumpSwimTimer != b.jumpSwimTimer) return kb_generic_cmp(a.jumpSwimTimer, b.jumpSwimTimer);
#endif
#ifdef RUNNINGTIMER
  if (a.runningTimer != b.runningTimer) return kb_generic_cmp(a.runningTimer, b.runningTimer);
#endif
#ifdef BIG
  if (a.crouched != b.crouched) return kb_generic_cmp(a.crouched, b.crouched);
#endif
#ifdef COIN
  if (a.coinCollected != b.coinCollected) return kb_generic_cmp(a.coinCollected, b.coinCollected);
#endif
#ifdef COLLECT_POWERUP
  if (a.powerupBlockHit != b.powerupBlockHit) return kb_generic_cmp(a.powerupBlockHit, b.powerupBlockHit);
  if (a.powerupCollected != b.powerupCollected) return kb_generic_cmp(a.powerupCollected, b.powerupCollected);
#endif
  return kb_generic_cmp(a.runningSpeed, b.runningSpeed);
}

#include "states/world1.h"
#include "states/world4.h"


State s11pipeIntPal = State {
  .xPos = 0xb9420 >> 4,
  .yPos = 0x15038 - yPosOffset,
  .xSpd = 0x30c0 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 2,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x24,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s12Int = State {
  .xPos = 0xa1e90 >> 4,
  .yPos = 0x18000 - yPosOffset,
  .xSpd = 0x285c >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0xae - 2, // warp zone 1 frame scroll lock incoming
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s12IntPal = State {
  .xPos = 0x9b8b0 >> 4,
  .yPos = 0x1ae00 - yPosOffset,
  .xSpd = 0x3080 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x48,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s12Int7dPal = State {
  .xPos = 0xaffe0 >> 4,
  .yPos = 0x19000 - yPosOffset,
  .xSpd = 0x30c0 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b01,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x82,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s12Int7fPal = State {
  .xPos = 0xb0370 >> 4,
  .yPos = 0x19000 - yPosOffset,
  .xSpd = 0x3000 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b01,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x84,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s12Int15 = State {
  .xPos = 0xa72f0 >> 4,
  .yPos = 0x13b20 - yPosOffset,
  .xSpd = 0x27a0 >> 2,
  .vForceIdx = 7,
  .ySpd =(-0x100),
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 1,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s12Int2 = State {
  .xPos = 0xa7980 >> 4,
  .yPos = 0x140e0 - yPosOffset,
  .xSpd = 0x40 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 1,
  .collisionBits = 0b10,
#ifdef SCREENPOS
  .sideCollisionTimer = 0xf,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s12IntPipe = State {
  .xPos = 0xaf530 >> 4,
  .yPos = 0x1705a - yPosOffset,
  .xSpd = 0x28bc >> 2,
  .vForceIdx = 5,
  .ySpd = 0,
  .vForceDIdx = 5,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b01,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x75,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s12Tmp = State {
  .xPos = 0xb6780 >> 4,
  .yPos = 0x18028 - yPosOffset,
  .xSpd = 0x2820 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 2,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0xeb,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s41 = State {
  .xPos = 0x2800 >> 4,
  .yPos = 0x1b000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s41Int = State {
  .xPos = 0xb8190 >> 4,
  .yPos = 0x1b030 - yPosOffset,
  .xSpd = 0x2864 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x11,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s41IntPal = State {
  .xPos = 0xd6c20 >> 4,
  .yPos = 0x13080 - yPosOffset,
  .xSpd = 0x3000 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x11,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s42 = iterateEntrance(State {
  .xPos = 0x2800 >> 4,
  .yPos = 0x12000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 2,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
});

State s42Int = iterateEntrance(State {
  .xPos = 0x14590 >> 4,
  .yPos = 0x1b0d8 - yPosOffset,
  .xSpd = 0x2874 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0xd5,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
});

State s42IntPal = iterateEntrance(State {
  .xPos = 0x199b0 >> 4,
  .yPos = 0x1b000 - yPosOffset,
  .xSpd = 0x3000 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x29,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
});

State s42Int2Pal = iterateEntrance(State {
  .xPos = 0x442f0 >> 4,
  .yPos = 0x1b0f0 - yPosOffset,
  .xSpd = 0x3000 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0xbb,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
});

State s42sub = State {
  .xPos = 0x1800 >> 4,
  .yPos = 0x1b000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s42subInt = State {
  .xPos = 0x24d90 >> 4,
  .yPos = 0x1b040 - yPosOffset,
  .xSpd = 0x28d4 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0xdd,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s42subIntPal = State {
  .xPos = 0x39320 >> 4,
  .yPos = 0x12078 - yPosOffset,
  .xSpd = 0x3080 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x02,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s81 = State {
  .xPos = 0x2800 >> 4,
  .yPos = 0x1b000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s81Int = State {
  .xPos = 0x62b90 >> 4,
  .yPos = 0x19038 - yPosOffset,
  .xSpd = 0x28c0 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0xbb,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s81IntPal = State {
  .xPos = 0x6d820 >> 4,
  .yPos = 0x13050 - yPosOffset,
  .xSpd = 0x3000 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x68,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s82 = State {
  .xPos = 0x2800 >> 4,
  .yPos = 0x1b000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s82IntPal = State {
  .xPos = 0xa920 >> 4,
  .yPos = 0x1b000 - yPosOffset,
  .xSpd = 0x3080 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x3a,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s82Int2Pal = State {
  .xPos = 0x924f0 >> 4,
  .yPos = 0x1b070 - yPosOffset,
  .xSpd = 0x3040 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0xb4,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s82Int3Pal = State {
  .xPos = 0xcd5f0 >> 4,
  .yPos = 0x130f8 - yPosOffset,
  .xSpd = 0x3040 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0xb4,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s82Int = State {
  .xPos = 0x8b190 >> 4,
  .yPos = 0x1b078 - yPosOffset,
  .xSpd = 0x28b0 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x41,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s82Int2 = State {
  .xPos = 0xca970 >> 4,
  .yPos = 0x16000 - yPosOffset,
  .xSpd = 0x283c >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x39,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s83 = State {
  .xPos = 0x2800 >> 4,
  .yPos = 0x1b000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s83IntPal = State {
  .xPos = 0xcb820 >> 4,
  .yPos = 0x130c0 - yPosOffset,
  .xSpd = 0x3000 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x48,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s83int = State {
  .xPos = 0xcb510 >> 4,
  .yPos = 0x130e0 - yPosOffset,
  .xSpd = 0x28f0 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84 = State {
  .xPos = 0x2800 >> 4,
  .yPos = 0x15000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84sub1 = State {
  .xPos = (0x73800 - 0x70000) >> 4,
  .yPos = 0x19000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84sub1IntPal = State {
  .xPos = (0x8e520 - 0x70000) >> 4,
  .yPos = 0x18088 - yPosOffset,
  .xSpd = 0x30c0 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x75,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84sub1Int05 = State {
  .xPos = (0x8f090 - 0x70000) >> 4,
  .yPos = 0x18050 - yPosOffset,
  .xSpd = 0x281c >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x80,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84sub1Int = State {
  .xPos = (0x92510 - 0x70000) >> 4,
  .yPos = 0x1b000 - yPosOffset,
  .xSpd = 0x28d0 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0xb5,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84sub2 = State {
  .xPos = (0xc3800 - 0xc0000) >> 4,
  .yPos = 0x19000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84sub2IntPal = State {
  .xPos = (0xcfb20 - 0xc0000) >> 4,
  .yPos = 0x180b8 - yPosOffset,
  .xSpd = 0x3080 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x8b,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84sub2Int = State {
  .xPos = (0xcf140 - 0xc0000) >> 4,
  .yPos = 0x180c8 - yPosOffset,
  .xSpd = 0x28d4 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x81,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84sub2Int2 = State {
  .xPos = (0xd8100 - 0xc0000) >> 4,
  .yPos = 0x18000 - yPosOffset,
  .xSpd = 0x70 >> 2,
  .vForceIdx = 7,
  .ySpd = 0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x11,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84sub3 = State {
  .xPos = 0x3800 >> 4,
  .yPos = 0x19000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s84sub4 = State {
  .xPos = 0x3800 >> 4,
  .yPos = 0x19000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 0,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};


#endif /* STATE_H_ */
