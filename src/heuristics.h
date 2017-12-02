#ifndef HEURISTICS_H_
#define HEURISTICS_H_

#include <cmath>
#include <iostream>
#include "constants.h"
#include "state.h"

int targetXSpeedHeuristic(State s, int xSpd) {
  return max((xSpd - (s.xSpd << 2) + (2*friction0 - 1)) / 2*friction0, 0);
}
int targetXPosHeuristic(State s, int xPos) {
  int pos = s.xPos << 4;
  int spd = s.xSpd << 2;
  int steps = 0;
  while (xPos > pos) {
    steps += 1;
    spd = min(spd + 2*friction0, maxXSpeedRun);
    pos += (spd >> 8) << 4;
  }
  return steps;
}

int getAirtimeFriction(int& xSpdAbs, int runningSpeed, int facingDir, int& movingDir) {
  int friction = friction1;
  if (xSpdAbs >= xSpdAbsValues[3]) friction = friction0;
  else if (runningSpeed) friction = friction2;
  if (facingDir != movingDir)
    return 2*friction;
  else
    return friction;
}
int getAirtimeMaxSpeed(int& xSpdAbs) {
  if (xSpdAbs >= xSpdAbsValues[3])
    return maxXSpeedRun;
  else
    return maxXSpeedWalk;
}

void iterateYPosAirtime(State& s, int& yPos, int& ySpd, int& vForce) {
  yPos += ySpd;
  ySpd += vForce;
  if (ySpd >= maxYSpeed && (ySpd & 0xff) >= 0x80) ySpd = maxYSpeed;
  if (ySpd >= 0) vForce = vForceValues[s.vForceDIdx];
}
void iterateMinYPosAny(State& s, int& yPos, int& ySpd, int& vForce) {
  yPos += jumpVelocityFast;
}
void iterateMaxYPosAny(State& s, int& yPos, int& ySpd, int& vForce) {
  vForce = vForceValues[7];
  yPos += ySpd;
  ySpd += vForce;
  if (ySpd >= maxYSpeed && (ySpd & 0xff) >= 0x80) ySpd = maxYSpeed + 0x7f;
}
void iterateMinXPosAirtime(State& s, int& xPos, int& xSpd, int& xSpdAbs, int& movingDir) {
  const int friction = getAirtimeFriction(xSpdAbs, s.runningSpeed, s.facingDir, movingDir);
  xSpd = max(xSpd - friction, -getAirtimeMaxSpeed(xSpdAbs));
  xPos += (xSpd >> 8) << 4;
  xSpdAbs = abs(xSpd >> 8);
  if (xSpd < 0) movingDir = 2;
  else if (xSpd >= 0x100) movingDir = 1;
}
void iterateMaxXPosAirtime(State& s, int& xPos, int& xSpd, int& xSpdAbs, int& movingDir) {
  const int friction = getAirtimeFriction(xSpdAbs, s.runningSpeed, s.facingDir, movingDir);
  xSpd = min(xSpd + friction, getAirtimeMaxSpeed(xSpdAbs));
  xPos += (xSpd >> 8) << 4;
  xSpdAbs = abs(xSpd >> 8);
  if (xSpd < 0) movingDir = 2;
  else if (xSpd >= 0x100) movingDir = 1;
}
void iterateMinXPosAny(State& s, int& xPos, int& xSpd, int& xSpdAbs, int& movingDir, int& facingDir) {
  if (xSpd > 0) xSpd &= 0xff; // wall hit to brake faster
  int friction;
  if (!(xSpdAbs > xSpdAbsValues[3])) {
    if (facingDir == oo && s.runningSpeed) friction = 2*friction2;
    else if (facingDir == oo && !s.runningSpeed) friction = 2*friction1;
    else if (movingDir == 2 && facingDir != 2) friction = 2*friction0;
    else if (s.runningSpeed) friction = 2*friction2;
    else friction = 2*friction1;
  } else {
    if (facingDir == oo) friction = 2*friction0;
    else if (movingDir == facingDir) friction = friction0;
    else friction = 2*friction0;
  }
  facingDir = oo;

  xSpd = max(xSpd - friction, -maxXSpeedRun);
  xPos += min((xSpd >> 8) << 4, -0x100 /* wall ejection */);
  xSpdAbs = abs(xSpd >> 8);
  if (xSpd < 0) movingDir = 2;
  else if (xSpd >= 0x100) movingDir = 1;
}
void iterateMaxXPosAny(State& s, int& xPos, int& xSpd, int& xSpdAbs, int& movingDir, int& facingDir) {
  if (xSpd < 0) xSpd &= 0xff; // wall hit to brake faster
  int friction;
  if (!(xSpdAbs > xSpdAbsValues[3])) {
    if (facingDir == oo && s.runningSpeed) friction = 2*friction2;
    else if (facingDir == oo && !s.runningSpeed) friction = 2*friction1;
    else if (movingDir == 1 && facingDir != 1) friction = 2*friction0;
    else if (s.runningSpeed) friction = 2*friction2;
    else friction = 2*friction1;
  } else {
    if (facingDir == oo) friction = 2*friction0;
    else if (movingDir == facingDir) friction = friction0;
    else friction = 2*friction0;
  }
  facingDir = oo;

  xSpd = min(xSpd + friction, maxXSpeedRun);
  xPos += max((xSpd >> 8) << 4, 0x100 /* wall ejection */);
  xSpdAbs = abs(xSpd >> 8);
  if (xSpd < 0) movingDir = 2;
  else if (xSpd >= 0x100) movingDir = 1;
}

bool anyNonCoinBlockInRect(int left, int up, int right, int down) {
  const int bbl = 0x0200, bbu = 0x1200, bbr = 0x0d00, bbd = 0x2000;
  const int minX = (left + bbl) >> 12, minY = max((up + bbu - 0x2000) >> 12, 0x10), maxX = (right + bbr) >> 12, maxY = min((down + bbd - 0x2000) >> 12, 0x1c);
  for (int xPos = minX; xPos <= maxX; xPos++) for (int yPos = minY; yPos <= maxY; yPos++) {
    int cv = bb.getBlockAt(xPos, yPos & 0x0f);
    if (cv != 0 && cv != 0xc2 && cv != 0xc3) return true;
  }
  return false;
}

int minStepsIntoBounds(State s, int boundLeft, int boundUp, int boundRight, int boundDown) {
  int lPos = s.xPos << 4, lSpd = s.xSpd << 2, lSpdAbs = xSpdAbsValues[s.xSpdAbsIdx], lMovingDir = s.movingDir;
  int rPos = s.xPos << 4, rSpd = s.xSpd << 2, rSpdAbs = xSpdAbsValues[s.xSpdAbsIdx], rMovingDir = s.movingDir;
  int uPos = s.yPos + yPosOffset, uSpd = s.ySpd, uForce = vForceValues[s.vForceIdx];
  int dPos = s.yPos + yPosOffset, dSpd = s.ySpd, dForce = max(vForceValues[s.vForceIdx], vForceValues[s.vForceDIdx]);
  int steps = 0;
//  cout << "right after " << dec << steps << ": " << hex << rPos << "," << rSpd << "," << rSpdAbs << "," << rMovingDir << endl;
//  cout << "down after " << dec << steps << ": " << hex << dPos << "," << dSpd << "," << dForce << endl;
  if (lPos <= boundRight && rPos >= boundLeft && uPos <= boundDown && dPos >= boundUp) return steps;
  while (!s.isOnGround() && !anyNonCoinBlockInRect(lPos, uPos, rPos, dPos)) {
    steps += 1;
    iterateMinXPosAirtime(s, lPos, lSpd, lSpdAbs, lMovingDir);
    iterateMaxXPosAirtime(s, rPos, rSpd, rSpdAbs, rMovingDir);
    iterateYPosAirtime(s, uPos, uSpd, uForce);
    iterateYPosAirtime(s, dPos, dSpd, dForce);
//    cout << "right after " << dec << steps << ": " << hex << rPos << "," << rSpd << "," << rSpdAbs << "," << rMovingDir << endl;
//    cout << "down after " << dec << steps << ": " << hex << dPos << "," << dSpd << "," << dForce << endl;
    if (lPos <= boundRight && rPos >= boundLeft && uPos <= boundDown && dPos >= boundUp) return steps;
  }
//  cout << "landed after " << dec << steps << endl;
  dPos |= 0xff;// redirect trajectory downwards
  dSpd = max(dSpd, 0x1ff);
  dForce = vForceValues[7];
  int lFacingDir = s.facingDir;
  int rFacingDir = s.facingDir;
  while (!(lPos <= boundRight && rPos >= boundLeft && uPos <= boundDown && dPos >= boundUp)) {
    steps += 1;
    iterateMinXPosAny(s, lPos, lSpd, lSpdAbs, lMovingDir, lFacingDir);
    iterateMaxXPosAny(s, rPos, rSpd, rSpdAbs, rMovingDir, rFacingDir);
    iterateMinYPosAny(s, uPos, uSpd, uForce);
    iterateMaxYPosAny(s, dPos, dSpd, dForce);
//    cout << "up after " << dec << steps << ": " << hex << uPos << "," << uSpd << endl;
//    cout << "right after " << dec << steps << ": " << hex << rPos << "," << rSpd << "," << rSpdAbs << "," << rMovingDir << endl;
  }
  return steps;
}


#endif /* HEURISTICS_H_ */
