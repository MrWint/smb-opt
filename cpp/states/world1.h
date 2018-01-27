#ifndef STATES_WORLD1_H_
#define STATES_WORLD1_H_

State s11 = State {
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

State s11sub = iterateEntrance(State {
  .xPos = 0x1800 >> 4,
  .yPos = 0x12000 - yPosOffset,
  .xSpd = 0 >> 2,
  .vForceIdx = 1,
  .ySpd = 0x0,
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

State s11pipe = State {
  .xPos = 0xa3800 >> 4,
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

State s11flag = State {
  .xPos = 0xBB490 >> 4,
  .yPos = 0x13088 - yPosOffset,
  .xSpd = 0x2804 >> 2,
  .vForceIdx = 7,
  .ySpd = 0x0,
  .vForceDIdx = 7,
  .facingDir = 2,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x44,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 19,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s12 = iterateEntrance(State {
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

State s12powerup = State {
  .xPos = 0x7890 >> 4,
  .yPos = 0x1B028 - yPosOffset,
  .xSpd = 0x2888 >> 2,
  .vForceIdx = 7,
  .ySpd = 0x0,
  .vForceDIdx = 7,
  .facingDir = 2,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 0,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x18,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 11,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};

State s11anyPipe = State {
  xPos : 0xB3800 >> 4,
  yPos : 0x19000 - yPosOffset,
  xSpd : 0x0 >> 2,
  vForceIdx : 1,
  ySpd : 0x0,
  vForceDIdx : 1,
  facingDir : 1,
  movingDir : 0,
  playerState : 0,
  xSpdAbsIdx : 0,
  runningSpeed : 0,
  collisionBits : 0b11,
#ifdef SCREENPOS
  sideCollisionTimer : 0,
  leftScreenEdgePos : 0x0,
#endif
#ifdef SWIMMING
  jumpSwimTimer : 0,
#endif
#ifdef RUNNINGTIMER
  runningTimer : 0,
#endif
#ifdef BIG
  crouched : 0,
#endif
};

State s11anyPipeFlag = State {
  xPos : 0xBB090 >> 4,
  yPos : 0x13B00 - yPosOffset,
  xSpd : 0x2860 >> 2,
  vForceIdx : 4,
  ySpd : (short)0xFB28,
  vForceDIdx : 7,
  facingDir : 2,
  movingDir : 1,
  playerState : 1,
  xSpdAbsIdx : 5,
  runningSpeed : 0,
  collisionBits : 0b11,
#ifdef SCREENPOS
  sideCollisionTimer : 0,
  leftScreenEdgePos : 0x40,
#endif
#ifdef SWIMMING
  jumpSwimTimer : 31,
#endif
#ifdef RUNNINGTIMER
  runningTimer : 0,
#endif
#ifdef BIG
  crouched : 0,
#endif
};

State s13powerup = State {
  xPos : 0x39380 >> 4,
  yPos : 0x17DF0 - yPosOffset,
  xSpd : 0x2868 >> 2,
  vForceIdx : 7,
  ySpd : 0x3A0,
  vForceDIdx : 7,
  facingDir : 2,
  movingDir : 1,
  playerState : 1,
  xSpdAbsIdx : 5,
  runningSpeed : 1,
  collisionBits : 0b11,
#ifdef SCREENPOS
  sideCollisionTimer : 0,
  leftScreenEdgePos : 0x23,
#endif
#ifdef SWIMMING
  jumpSwimTimer : 8,
#endif
#ifdef RUNNINGTIMER
  runningTimer : 0,
#endif
#ifdef BIG
  crouched : 1,
#endif
};

State s13powerupAlt = State {
  xPos : 0x38C00 >> 4,
  yPos : 0x17330 - yPosOffset,
  xSpd : 0x2868 >> 2,
  vForceIdx : 7,
  ySpd : 0x188,
  vForceDIdx : 7,
  facingDir : 2,
  movingDir : 1,
  playerState : 1,
  xSpdAbsIdx : 5,
  runningSpeed : 1,
  collisionBits : 0b11,
#ifdef SCREENPOS
  sideCollisionTimer : 0,
  leftScreenEdgePos : 0x1C,
#endif
#ifdef SWIMMING
  jumpSwimTimer : 11,
#endif
#ifdef RUNNINGTIMER
  runningTimer : 0,
#endif
#ifdef BIG
  crouched : 1,
#endif
};

State s13powerupMov = State {
  xPos : 0x3A760 >> 4,
  yPos : 0x1B0D0 - yPosOffset,
  xSpd : 0xCD4 >> 2,
  vForceIdx : 7,
  ySpd : 0x0,
  vForceDIdx : 7,
  facingDir : 2,
  movingDir : 1,
  playerState : 0,
  xSpdAbsIdx : 1,
  runningSpeed : 1,
  collisionBits : 0b11,
#ifdef SCREENPOS
  sideCollisionTimer : 0,
  leftScreenEdgePos : 0x37,
#endif
#ifdef SWIMMING
  jumpSwimTimer : 0,
#endif
#ifdef RUNNINGTIMER
  runningTimer : 0,
#endif
#ifdef BIG
  crouched : 1,
#endif
};

State s13floor = State {
  xPos : 0x7D5F0 >> 4,
  yPos : 0x17092 - yPosOffset,
  xSpd : 0x98 >> 2,
  vForceIdx : 5,
  ySpd : 0x0,
  vForceDIdx : 5,
  facingDir : 3,
  movingDir : 1,
  playerState : 0,
  xSpdAbsIdx : 0,
  runningSpeed : 1,
  collisionBits : 0b11,
#ifdef SCREENPOS
  sideCollisionTimer : 0,
  leftScreenEdgePos : 0x6C,
#endif
#ifdef SWIMMING
  jumpSwimTimer : 0,
#endif
#ifdef RUNNINGTIMER
  runningTimer : 0,
#endif
#ifdef BIG
  crouched : 0,
#endif
};

State s14 = State {
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

State s14powerup = State {
  .xPos = 0x1C620 >> 4,
  .yPos = 0x180C0 - yPosOffset,
  .xSpd = 0x2874 >> 2,
  .vForceIdx = 7,
  .ySpd = 0x0,
  .vForceDIdx = 7,
  .facingDir = 1,
  .movingDir = 1,
  .playerState = 0,
  .xSpdAbsIdx = 5,
  .runningSpeed = 1,
  .collisionBits = 0b11,
#ifdef SCREENPOS
  .sideCollisionTimer = 0,
  .leftScreenEdgePos = 0x56,
#endif
#ifdef SWIMMING
  .jumpSwimTimer = 2,
#endif
#ifdef RUNNINGTIMER
  .runningTimer = 0,
#endif
#ifdef BIG
  .crouched = 0,
#endif
};



#endif /* STATES_WORLD1_H_ */
