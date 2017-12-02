#ifndef EMU_H_
#define EMU_H_

#include <cmath>
#include <iostream>
#include "blockbuffer.h"
#include "constants.h"
#include "state.h"

BlockBuffer bb;

#ifdef COIN
int coinX = 0x1d;
int coinY = 0xa;
#endif

#ifdef COLLECT_POWERUP
//int powerupX = 0xa, powerupY = 0x7; // 1-2 powerup
int powerupX = 0x3b, powerupY = 0x8; // 1-3 powerup
//int powerupX = 0x1e, powerupY = 0x4; // 1-4 powerup
#endif

class Emu {
  State s;
  bool noCollisions;

  int joypad = 0;
  int joypadLR = 0;
  int joypadUD = 0;
  int maxSpeedLeft = 0;
  int maxSpeedRight = 0;
  int friction = 0;

  bool startedJump = false;
#ifdef SCREENPOS
  int xScroll = 0;
  bool sideCollision = false;
#endif

public:
  const static bool option_clearYPosFractional = false;
  const static bool option_ignoreVerticalPipes = false;

  Emu(State& s_) {
    s = s_;
    noCollisions = false;
  }
  Emu(State& s_, bool noCollisions_) {
    s = s_;
    noCollisions = noCollisions_;
  }

  State runStep(int input) {
    joypad = input;
    bool startedOnGround = s.isOnGround();

#ifdef RUNNINGTIMER
    if (s.runningTimer > 0) s.runningTimer -= 1;
#endif

    playerCtrlRoutine();

#ifdef SCREENPOS
    if (sideCollision) s.sideCollisionTimer = 0xf;
    else if (s.sideCollisionTimer) s.sideCollisionTimer--;
#endif

#ifdef SWIMMING
    if (startedJump) s.jumpSwimTimer = 0x1f;
    else if (s.jumpSwimTimer) s.jumpSwimTimer--;
#endif

    if (s.playerState != 1) s.vForceIdx = 0; // only needed for JumpSwim, set whenever entered
    if (option_clearYPosFractional && s.isOnGround()) s.yPos &= 0xff00, s.vForceDIdx = 0;
    if (option_clearYPosFractional && startedOnGround && s.playerState == 2) { // ran off edge with cleared fractional
      blockHit = true;
      cv = -oo; // hit impossible object to disqualify this state
    }
    return s;
  }

  void playerCtrlRoutine() {
    // assume GameEngineSubroutine != DEATH
    if (s.yPos < (0x10000 - yPosOffset) || s.yPos >= (0x1d000 - yPosOffset)) joypad = 0;
    joypadLR = joypad & 0x03;
    joypadUD = joypad & 0x0c;
    if ((joypad & D) != 0 && s.isOnGround() && joypadLR != 0) {
      joypadLR = 0;
      joypadUD = 0;
    }
    playerMovementSubs();
    if (s.xSpd < (0 >> 2)) s.movingDir = 2;
    else if (s.xSpd >= (0x100 >> 2)) s.movingDir = 1;

    scrollHandler();

    // PlayerBGCollision
    playerBgCollision();
  }

  void scrollHandler() {
#ifdef SCREENPOS
    int relXPos = ((s.xPos >> 4) - s.leftScreenEdgePos) & 0xff;
    if (relXPos < 0x50 || s.sideCollisionTimer || xScroll <= 0) return;
    if (relXPos < 0x70 && xScroll >= 2) xScroll--;
    s.leftScreenEdgePos += xScroll;
#endif
  }

  void playerMovementSubs() {
#ifdef BIG
    if (s.isOnGround())
      s.crouched = (joypadUD & D) != 0;
#endif

    playerPhysicsSub();

    // MoveSubs
    switch (s.playerState) {
    case 0:
      getPlayerAnimSpeed();
      if (joypadLR != 0) s.facingDir = joypadLR;
      imposeFriction();
      moveHorizontally();
      break;
    case 1:
      if (s.ySpd >= 0 || ((joypad & A) == 0 && !startedJump)) s.vForceIdx = s.vForceDIdx;
#ifdef SWIMMING
      getPlayerAnimSpeed();
      if (s.yPos < 0x11400 - yPosOffset) s.vForceIdx = 0;
      if (joypadLR != 0) s.facingDir = joypadLR;
#endif
      if (joypadLR != 0) imposeFriction();
      moveHorizontally();
      moveVertically();
      break;
    case 2:
      s.vForceIdx = s.vForceDIdx;
      if (joypadLR != 0) imposeFriction();
      moveHorizontally();
      moveVertically();
      break;
    default:
      throw "climbing unimplemented";
    }
  }

  void playerPhysicsSub() {
    // handle starting jumps
#ifdef SWIMMING
    if ((joypad & A) != 0 && (s.isOnGround() || s.jumpSwimTimer != 0 || s.ySpd >= 0)) {
#else
    if ((joypad & A) != 0 && (s.isOnGround())) {
#endif
      s.yPos &= 0xffff00; // clear fractional yPos
      startedJump = true;
      s.playerState = 1;
      if (s.xSpdAbsIdx >= 3) s.vForceIdx = 4, s.vForceDIdx = 7, s.ySpd = jumpVelocityFast;
      else if (s.xSpdAbsIdx >= 2) s.vForceIdx = 2, s.vForceDIdx = 5, s.ySpd = jumpVelocitySlow;
      else s.vForceIdx = 3, s.vForceDIdx = 6, s.ySpd = jumpVelocitySlow;
#ifdef SWIMMING
      s.vForceIdx = 8, s.vForceDIdx = 9, s.ySpd = jumpVelocitySwim;
      if (s.yPos < 0x11400 - yPosOffset) s.ySpd &= 0xff; // kill upward momentum if swimming too high
#endif
    }

    // X_Physics
#ifdef SWIMMING
    bool isRunning = false;
#else
    bool isRunning = s.isOnGround() && joypadLR == s.movingDir && (joypad & B) != 0;
#endif
#ifdef RUNNINGTIMER
    if (isRunning) s.runningTimer = 0xa;
#endif
    if (!s.isOnGround() && s.xSpdAbsIdx >= 3)
      maxSpeedRight = maxXSpeedRun >> 2, maxSpeedLeft = -(maxXSpeedRun >> 2), friction = friction0 >> 2;
    else if (!s.isOnGround() && s.runningSpeed)
      maxSpeedRight = maxXSpeedWalk >> 2, maxSpeedLeft = -(maxXSpeedWalk >> 2), friction = friction2 >> 2;
    else if (!s.isOnGround())
      maxSpeedRight = maxXSpeedWalk >> 2, maxSpeedLeft = -(maxXSpeedWalk >> 2), friction = friction1 >> 2;
#ifdef SWIMMING
    else if (!s.runningSpeed && s.xSpdAbsIdx < 5)
      maxSpeedRight = maxXSpeedSwim >> 2, maxSpeedLeft = -(maxXSpeedSwim >> 2), friction = friction1 >> 2;
    else if (true)
      maxSpeedRight = maxXSpeedSwim >> 2, maxSpeedLeft = -(maxXSpeedSwim >> 2), friction = friction2 >> 2;
#endif
#ifdef RUNNINGTIMER
    else if (joypadLR == s.movingDir && s.runningTimer > 0)
#else
    else if (joypadLR == s.movingDir && isRunning)
#endif
      maxSpeedRight = maxXSpeedRun >> 2, maxSpeedLeft = -(maxXSpeedRun >> 2), friction = friction0 >> 2;
    else if (!s.runningSpeed && s.xSpdAbsIdx < 5)
      maxSpeedRight = maxXSpeedWalk >> 2, maxSpeedLeft = -(maxXSpeedWalk >> 2), friction = friction1 >> 2;
    else
      maxSpeedRight = maxXSpeedWalk >> 2, maxSpeedLeft = -(maxXSpeedWalk >> 2), friction = friction2 >> 2;

    if (s.facingDir != s.movingDir) friction <<= 1;
  }

  void getPlayerAnimSpeed() {
    if (s.xSpdAbsIdx >= 4) s.runningSpeed = true;
    else if ((joypad & ~A) != 0 && (joypad & 0x03) == s.movingDir) s.runningSpeed = false;
    else if ((joypad & ~A) != 0 && s.xSpdAbsIdx < 1) {
      s.movingDir = s.facingDir;
      s.xSpd = 0;
    }
  }

  void imposeFriction() {
    int lrCollision = joypadLR & s.collisionBits;
    if ((lrCollision & R) != 0 || (lrCollision == 0 && s.xSpd < 0)) { // LeftFrict
      s.xSpd += friction;
      if (s.xSpd >= maxSpeedRight) s.xSpd = maxSpeedRight + (s.xSpd & (0xff >> 2));
    } else if (lrCollision != 0 || s.xSpd >= (0x100 >> 2)) { // RightFrict
      s.xSpd -= friction;
      if (s.xSpd <= maxSpeedLeft) s.xSpd = maxSpeedLeft + (s.xSpd & (0xff >> 2));
    }
    s.xSpdAbsIdx = getXSpdAbsIdx(abs(s.xSpd >> 6));
  }

  void moveHorizontally() {
#ifdef SCREENPOS
    int oldXPos = s.xPos;
#endif
    s.xPos += s.xSpd >> 6;
#ifdef SCREENPOS
    xScroll = (s.xPos >> 4) - (oldXPos >> 4);
#endif
  }

  void moveVertically() {
    s.yPos += s.ySpd;
    s.ySpd += vForceValues[s.vForceIdx];
    if (s.ySpd >= maxYSpeed && (s.ySpd & 0xff) >= 0x80) s.ySpd = maxYSpeed;
  }

  bool blockHit = false;
  int cx = 0, cy = 0, cv = 0;
  void blockBufferCollision(int blockBufferAdderOffset) {
    if (noCollisions) return;

#ifdef BIG
    blockBufferAdderOffset += s.crouched ? 0x0e : (IS_SWIMMING ? 0x07 : 0);
#else
    blockBufferAdderOffset += 0x0e;
#endif

    int bx = blockBufferXAdderData[blockBufferAdderOffset];
    int by = blockBufferYAdderData[blockBufferAdderOffset];
    cx = (s.xPos + bx) >> 8;
    cy = ((s.yPos + yPosOffset + by - 0x2000) >> 12) & 0x0f;

    cv = bb.getBlockAt(cx, cy);
#ifdef COIN
    if (!s.coinCollected && cx == coinX && cy == coinY) return;
#endif
#ifdef COLLECT_POWERUP
    if (s.powerupBlockHit && cx == powerupX && cy == powerupY) cv = 0xc4; // question block changed to solid block
#endif
    if (isCoin(cv)) cv = 0; // pretend coin has been collected already
  }

  void playerBgCollision() {
    if (s.playerState == 0 || s.playerState == 3) s.playerState = 2;
#ifdef SWIMMING
    s.playerState = 1;
#endif

    if (s.yPos < (0x10000 - yPosOffset) || s.yPos >= (0x20000 - yPosOffset)) return; // yPos out of bounds
    s.collisionBits = 0x3;
    if (s.yPos >= (0x1cf00 - yPosOffset)) return; // yPos out of bounds

#ifdef BIG
    if (s.yPos >= (!s.crouched ? 0x12000 : 0x11000) - yPosOffset) { // HeadChk
#else
    if (s.yPos >= 0x11000 - yPosOffset) { // HeadChk
#endif
      blockBufferCollision(0);
      if (isCoin(cv)) {
#ifdef COIN
        s.coinCollected = true;
        return; // exit (no feet or side checks)
#endif
      } else if (cv != 0 && s.ySpd < 0 && ((s.yPos >> 8) & 0x0f) >= 4) {
        if (isSolid(cv) || IS_SWIMMING)
          s.ySpd = (0x100) + (s.ySpd & 0xff); // hit solid block
        else {
          if (IS_BIG && !isQuestionBlock(cv))
            s.ySpd = -0x200 + (s.ySpd & 0xff); // shatter brick
          else {
#ifdef COLLECT_POWERUP
            if (isQuestionBlock(cv) && cx == powerupX && cy == powerupY)
              s.powerupBlockHit = true;
#endif
            s.ySpd &= 0xff; // bump block
          }
        }
      }
    }

    { // DoFootCheck
      blockBufferCollision(2);
      int rv = cv;
      blockBufferCollision(1);
      int lv = cv;
      if (cv == 0) blockBufferCollision(2);
      if (isCoin(cv)) {
#ifdef COIN
        s.coinCollected = true;
        return; // exit (no side checks)
#endif
      } else if (cv != 0 &&!isClimb(cv) && !isHiddenBlock(cv) && s.ySpd >= 0) {
        if (cv == 0xc5) { // axe hit
          blockHit = true;
          return; // state change
        } else if (((s.yPos >> 8) & 0x0f) >= blockLandingFractionalHeight) {
          impedePlayerMove(s.movingDir);
          return; // exit (no side checks)
        } else {
          s.yPos &= 0xfff0ff; // align height with block
          if (!option_ignoreVerticalPipes && joypadLR == 0 && lv == 0x10 && rv == 0x11) {
            blockHit = true; // vertical pipe entry
            return;
          }
          s.ySpd = 0; // kill vertical speed
          s.playerState = 0; // land
        }
      }
    }

    if (s.yPos >= (0x10800 - yPosOffset)) for (int i = 0; i < 2; i++) { // DoPlayerSideCheck
      if (s.yPos >= (0x12000 - yPosOffset)) {
        blockBufferCollision(3 + 2*i);
        if (cv != 0 && cv != 0x1c && cv != 0x6b && !isClimb(cv)) {
          checkSideMTiles(2-i);
          return;
        }
      }
      blockBufferCollision(4 + 2*i);
      if (cv != 0) {
        checkSideMTiles(2-i);
        return;
      }
    }
  }

  void checkSideMTiles(int movingDir) {
    if (isHiddenBlock(cv)) return;
    else if (isClimb(cv)) {
      if (((s.xPos >> 4) & 0x0f) >= 6 && ((s.xPos >> 4) & 0x0f) < 10) blockHit = true; // flag or vine collision
    }
    else if (isCoin(cv)) {
#ifdef COIN
      s.coinCollected = true;
#endif
      return; // grab coin
    } else if (!s.isOnGround() || s.facingDir != 1 || (cv != 0x6c && cv != 0x1f)) impedePlayerMove(movingDir);
    else blockHit = true; // sideways pipe entry
  }

  void impedePlayerMove(int movingDir) {
    if (movingDir == 1 && s.xSpd >= 0) {
      s.xSpd &= 0xff >> 2;
      s.xPos -= (0x100 >> 4);
#ifdef SCREENPOS
      sideCollision = true;
#endif
    } else if (movingDir != 1 && s.xSpd < (0x100 >> 2)) {
      s.xSpd &= 0xff >> 2;
      s.xPos += (0x100 >> 4);
#ifdef SCREENPOS
      sideCollision = true;
#endif
    }
    s.collisionBits &= ~movingDir;
  }

  inline bool isCoin(int& cv) {
    return cv == 0xc2 || cv == 0xc3;
  }
  inline bool isSolid(int& cv) {
    return (cv < 0x40 && cv >= 0x10)
        || (cv < 0x80 && cv >= 0x61)
        || (cv < 0xc0 && cv >= 0x88)
        || cv >= 0xc4;
  }
  inline bool isClimb(int& cv) {
    return (cv < 0x40 && cv >= 0x24)
        || (cv < 0x80 && cv >= 0x6d)
        || (cv < 0xc0 && cv >= 0x8a)
        || cv >= 0xc6;
  }
  inline bool isHiddenBlock(int& cv) {
    return cv == 0x5f || cv == 0x60;
  }
  inline bool isQuestionBlock(int& cv) {
    return cv == 0xc0 || cv == 0xc1 || (cv >= 0x55 && cv <= 0x60);
  }
};

State iterateEntrance(State s) {
  while (((s.yPos + yPosOffset) & 0xff00) < 0x3000)
    s = Emu(s, true).runStep(0);
  return s;
}

#endif /* EMU_H_ */
