#ifndef STATES_WORLD4_H_
#define STATES_WORLD4_H_

State s44 = State {
  .xPos = 0x2800 >> 4,
  .yPos = 0x15000 - yPosOffset,
  .xSpd = 0x0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0x0,
  .vForceDIdx = 1,
  .facingDir = 1,
  .movingDir = 0,
  .playerState = 0,
  .xSpdAbsIdx = 0,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x0,
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

State s44Int = State {
  .xPos = 0x5A3F0 >> 4,
  .yPos = 0x180B0 - yPosOffset,
  .xSpd = 0x2818 >> 2,
  .vForceIdx = 7,
  .ySpd = 0x0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b01,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x33,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 14,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 10,
#endif
};

#endif /* STATES_WORLD4_H_ */
