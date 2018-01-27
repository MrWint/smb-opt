//#define COIN
//#define SCREENPOS
//#define MAXXPOS
//#define PAL
//#define SWIMMING
//#define RUNNINGTIMER
//#define BIG
//#define COLLECT_POWERUP

#include <cmath>
#include <iostream>
#include <vector>
#include "blockbuffer.h"
#include "emu.h"
#include "heuristics.h"
#include "state.h"
#include "IDA.h"

using namespace std;

void printRLE(vector<int>& inputs) {
  int curValue = inputs[0];
  int num = 0;
  cout << dec << inputs.size() << endl;
  for (int& input : inputs) {
    if (curValue != input) {
      cout << "(" << dec << num << "x " << hex << curValue << "), ";
      curValue = input;
      num = 1;
    } else num++;
  }
  cout << "(" << dec << num << "x " << hex << curValue << ")" << endl;
}

vector<State> withFacingDirs(vector<State> ss) {
  vector<State> ret;
  for (State s : ss) {
    s.facingDir = 1;
    ret.push_back(s);
    s.facingDir = 2;
    ret.push_back(s);
  }
  return ret;
}

vector<State> withSmallerXPos(vector<State> ss, int numSubpixels) {
  vector<State> ret;
  for (State s : ss) {
    int origXPos = s.xPos;
    for (int i = 0; i <= numSubpixels; i++) {
      s.xPos = origXPos - i;
#ifdef SCREENPOS
      s.leftScreenEdgePos -= (origXPos >> 4) - (s.xPos >> 4);
#endif
      ret.push_back(s);
    }
  }
  return ret;
}

vector<State> withAllXSpdSubpixels(vector<State> ss) {
  vector<State> ret;
  for (State s : ss) {
    for (int i = 0; i < 0x40; i++) {
      s.xSpd = (s.xSpd & 0x1fc0) + i;
      ret.push_back(s);
    }
  }
  return ret;
}

const int frameCounterOffset = 0xa6; // pal-4-2-int2
//const int frameCounterOffset = 0x9a; // pal-8-2-int2
//const int frameCounterOffset = 0xe4; // pal-1-2-int7d

int distanceToGoalHeuristic(State& s, int stepsAlreadyTaken) {
#ifdef SCREENPOS
  int relXPos = ((s.xPos >> 4) - s.leftScreenEdgePos) & 0xff;
#endif
//  if (relXPos > 0x7c) return oo;
//  if ((s.xPos << 4) >= 0x1f000 && relXPos < 0x84) return oo;

//  if (((stepsAlreadyTaken + frameCounterOffset) & 1) == 0 && (s.yPos + yPosOffset) >= 0x17400 && (s.xPos << 4) >= 0x47e00 && (s.xPos << 4) < 0x49300) return oo; // pal-4-2-int2 piranha hit
//  if (((stepsAlreadyTaken + frameCounterOffset) & 1) == 0 && (s.yPos + yPosOffset) >= 0x13400 && (s.xPos << 4) >= 0x4de00 && (s.xPos << 4) < 0x4f300) return oo; // pal-4-2-int2 piranha hit
//  if (((stepsAlreadyTaken + frameCounterOffset) & 1) == 0 && (s.yPos + yPosOffset) >= 0x16600 && (s.xPos << 4) >= 0x9be00 && (s.xPos << 4) < 0x9d300) return oo; // 8-2 piranha hit
//  if (stepsAlreadyTaken == 107-54 && (s.yPos + yPosOffset) >= (0x15100 - 0x1400) && (s.yPos + yPosOffset) < (0x15100 + 0x100) && (s.xPos << 4) >= 0x14500 && (s.xPos << 4) < 0x15a00) return oo; // 8-2 PAL spiny hit
//  if (stepsAlreadyTaken == 109-54 && (s.yPos + yPosOffset) >= (0x15a00 - 0x1400) && (s.yPos + yPosOffset) < (0x15a00 + 0x100) && (s.xPos << 4) >= 0x14500 && (s.xPos << 4) < 0x15a00) return oo; // 8-2 PAL spiny hit
//  if (stepsAlreadyTaken == 111-54 && (s.yPos + yPosOffset) >= (0x16200 - 0x1400) && (s.yPos + yPosOffset) < (0x16200 + 0x100) && (s.xPos << 4) >= 0x14500 && (s.xPos << 4) < 0x15a00) return oo; // 8-2 PAL spiny hit
//  if (stepsAlreadyTaken == 113-54 && (s.yPos + yPosOffset) >= (0x16a00 - 0x1400) && (s.yPos + yPosOffset) < (0x16a00 + 0x100) && (s.xPos << 4) >= 0x14500 && (s.xPos << 4) < 0x15a00) return oo; // 8-2 PAL spiny hit
//  if (stepsAlreadyTaken == 115-54 && (s.yPos + yPosOffset) >= (0x17200 - 0x1400) && (s.yPos + yPosOffset) < (0x17200 + 0x100) && (s.xPos << 4) >= 0x14500 && (s.xPos << 4) < 0x15a00) return oo; // 8-2 PAL spiny hit
//  if (((stepsAlreadyTaken + frameCounterOffset) & 1) == 0 && (s.yPos + yPosOffset) >= 0x16400 && (s.xPos << 4) >= 0x9be00 && (s.xPos << 4) < 0x9d300) return oo; // pal-8-2 piranha hit
//  if (stepsAlreadyTaken == 59 && (s.yPos + yPosOffset) >= 0x15000 && (s.xPos << 4) >= 0x13e00 && (s.xPos << 4) < 0x15300) return oo; // 8-4-sub2 PAL piranha hit
//  if (s.leftScreenEdgePos >= 0x10 && ((stepsAlreadyTaken + frameCounterOffset) & 1) == 0 && (s.yPos + yPosOffset) >= 0x17400 && (s.xPos << 4) >= 0xb1e00 && (s.xPos << 4) < 0xb3300) return oo; // pal-1-2 piranha hit
//  if (s.leftScreenEdgePos >= 0x10 && ((stepsAlreadyTaken + frameCounterOffset) & 1) == 0 && (s.yPos + yPosOffset) >= 0x17400 && (s.xPos << 4) >= 0xb5e00 && (s.xPos << 4) < 0xb7300) return oo; // pal-1-2 piranha hit


//  if ((s.xPos << 4) + (s.yPos + yPosOffset) >= 0x28000 + 0x17000) return oo;
//  if ((s.xPos << 4) + (s.yPos + yPosOffset) >= 0x93000 + 0x1b000) return oo; // 8-4-sub1-int pipe entry
//  if ((s.xPos << 4) >= 0x1de00 && (s.yPos + yPosOffset) < 0x17100) return oo;
//  if ((s.xPos << 4) < 0x39300 && s.yPos + yPosOffset < 0x12100) return oo; // pal-4-2-sub forbid upper path
//  if ((s.xPos << 4) < 0x6c200) return oo;
//  if ((s.yPos + yPosOffset) < 0x1b000) return oo;
//  if ((s.xPos << 4) >= 0xa8000 && (s.xPos << 4) < 0xa9000 && (s.yPos + yPosOffset) < 0x17100) return oo; // 1-1-pipe force lower path
//  if ((s.xPos << 4) >= 0xa2000 && (s.xPos << 4) < 0xa8000 && (s.yPos + yPosOffset) < 0x18100) return oo; // 1-2 force floor clip

//  if (s.leftScreenEdgePos < 0x11 || s.leftScreenEdgePos >= 0x40) return 34 + minStepsIntoBounds(s, 0x18100, -oo, oo, oo);

//  if (relXPos < 0x84 && (s.xPos << 4) >= 0x1ea00) return oo;

//  return minStepsIntoBounds(s, 0x9190, -oo, oo, oo); // 1-1 speed up
//  return minStepsIntoBounds(s, 0x9420, -oo, oo, oo); // 1-1 speed up PAL
//  return max(minStepsIntoBounds(s, 0x39400, -oo, oo, oo), 1); // 1-1 pipe entry
//  return minStepsIntoBounds(s, 0x91e0, -oo, oo, oo); // 1-1-sub speed up
//  return minStepsIntoBounds(s, 0x4050, 0x18800, oo, oo); // 1-1-sub clip test
  return max(minStepsIntoBounds(s, 0xc300, 0x1b000, oo, oo), 1); // 1-1-sub pipe entry
//  return minStepsIntoBounds(s, 0xbc110, -oo, oo, oo); // 1-1-pipe speed up
//  return minStepsIntoBounds(s, 0xbc060, -oo, oo, oo); // 1-1-pipe speed up PAL
//  return max(minStepsIntoBounds(s, 0xc5600, 0x1a200, oo, oo), 1); // 1-1 flag
//  return max(minStepsIntoBounds(s, 0xc5600, 0x1a600, oo, oo), 1); // 1-1 flag PAL
//  return minStepsIntoBounds(s, 0x9190, -oo, oo, oo); // 1-2 speed up
//  return minStepsIntoBounds(s, 0x92b0, -oo, oo, oo); // 1-2 speed up PAL
//  { // for 1-2-powerup collection
//    const int maxStepsToHit = 14;
//    if ((stepsAlreadyTaken < maxStepsToHit-3 && s.powerupBlockHit) || (stepsAlreadyTaken >= maxStepsToHit && !s.powerupBlockHit)) return oo; // not hit powerup block in valid timeframe
//
//    for (int i = 0; i < 12; i++)
//      if (!s.powerupCollected && (stepsAlreadyTaken == maxStepsToHit + 16 + 4*i || stepsAlreadyTaken == maxStepsToHit + 18 + 4*i) && (s.yPos + yPosOffset) < 0x18500 - 0x100*i && (s.xPos << 4) >= 0x9500 && (s.xPos << 4) < 0xac00) s.powerupCollected = true, s.playerState = 0;
//
//    if ((s.xPos << 4) >= 0xa000 && !s.powerupCollected) return oo; // missed powerup collection
//    return minStepsIntoBounds(s, 0xbe60, -oo, oo, oo);
////    return minStepsIntoBounds(s, maxXPos + 0x10, -oo, oo, oo);
//  }
//  return minStepsIntoBounds(s, 0xa5ee0, 0x16800, oo, oo); // 1-2 pipe clip
//  return minStepsIntoBounds(s, 0xa8180, 0x16800, oo, oo); // 1-2 pipe clip speed up
//  return minStepsIntoBounds(s, 0xa93b0, 0x16800, oo, oo); // 1-2 pipe clip speed up 2
//  return minStepsIntoBounds(s, maxXPos + 0x10, 0x16800, oo, oo); // 1-2 pipe clip speed up max
//  return minStepsIntoBounds(s, 0xa8080, -oo, oo, oo); // 1-2 wall clip
//  return minStepsIntoBounds(s, 0xaf230, -oo, oo, oo); // 1-2 wall clip speed up
//  return minStepsIntoBounds(s, 0xae260, -oo, oo, oo); // 1-2 wall clip speed up PAL
//  return minStepsIntoBounds(s, 0xa5e70, -oo, oo, oo); // 1-2 floor clip PAL
//  return minStepsIntoBounds(s, maxXPos + 0x10, -oo, oo, oo); // 1-2 floor clip PAL max
//  { // for pal-1-2 pipe entry relXPos 7d/7f
//    if (s.leftScreenEdgePos >= 0x10 && s.xSpd < -0x300) return oo; // turned too early
//    if (s.leftScreenEdgePos >= 0x10 && (s.xPos << 4) < 0xb7400) // warp zone not activated yet, waiting for overflow to 0x00
//      return 40 + minStepsIntoBounds(s, 0xb8000, -oo, oo, oo); // run to warp activation point first
//    return max(minStepsIntoBounds(s, -oo, 0x18000, 0xb2cf0, oo), 1); // 1-2-tmp pipe entry
//  }
//  { // for 1-2 pipe entry relXPos 7c
//    if (s.leftScreenEdgePos >= 0x10 && s.xSpd < 0) return oo; // turned too early
//    if (s.leftScreenEdgePos >= 0x10 && (s.xPos << 4) < 0xb7400) // warp zone not activated yet, waiting for overflow to 0x00
//      return 48 + minStepsIntoBounds(s, 0xb8000, -oo, oo, oo); // run to warp activation point first
//    return max(minStepsIntoBounds(s, -oo, 0x18000, 0xb2cf0, oo), 1); // 1-2-tmp pipe entry
//  }
//  return minStepsIntoBounds(s, 0xBB090, -oo, oo, oo); // 1-1-anyPipe speed up
//  { // for 1-3-powerup collection
//    const int maxStepsToHit = 21+3;
//    if ((stepsAlreadyTaken < maxStepsToHit-3 && s.powerupBlockHit) || (stepsAlreadyTaken >= maxStepsToHit && !s.powerupBlockHit)) return oo; // not hit powerup block in valid timeframe
//
//    for (int i = 0; i < 12; i++)
////      if (!s.powerupCollected && (stepsAlreadyTaken == maxStepsToHit + 16 + 4*i || stepsAlreadyTaken == maxStepsToHit + 18 + 4*i) && (s.yPos + yPosOffset) < 0x1a100 - 0x100*i - (s.crouched ? 0xc00 : 0) && (s.xPos << 4) >= 0x3a400 && (s.xPos << 4) < 0x3bd00) s.powerupCollected = true, s.playerState = 0;
//      if (!s.powerupCollected && (stepsAlreadyTaken == maxStepsToHit + 16 + 4*i || stepsAlreadyTaken == maxStepsToHit + 18 + 4*i) && (s.yPos + yPosOffset) < 0x19500 - 0x100*i && (s.xPos << 4) >= 0x3a500 && (s.xPos << 4) < 0x3bc00) s.powerupCollected = true, s.playerState = 0;
//
//    if ((s.xPos << 4) >= 0x3c000 && !s.powerupCollected) return oo; // missed powerup collection
////    return minStepsIntoBounds(s, 0x3e0a0, -oo, oo, oo); // small
//    return minStepsIntoBounds(s, 0x3ed70, -oo, oo, oo); // small alt
////    return minStepsIntoBounds(s, 0x3ed20, -oo, oo, oo); // big TODO
////    return minStepsIntoBounds(s, 0x3fe60, -oo, oo, oo); // big alt
////    return minStepsIntoBounds(s, maxXPos + 0x10, -oo, oo, oo);
//  }
//  { // for 1-3-floor clip
//    if ((s.xPos << 4) >= 0x80400 && s.yPos + yPosOffset < 0x1b100) return oo; // not in ground
//    const int stepsOffset = stepsAlreadyTaken + 0;
//    const int xOffset = (stepsOffset / 2) * 0x100;
//    if (!s.powerupCollected && stepsOffset % 2 == 0 && (s.xPos << 4) >= 0x81a00 - xOffset && (s.xPos << 4) < 0x83300 - xOffset && (s.yPos + yPosOffset) >= 0x1A100 && (s.yPos + yPosOffset) < 0x1C600)  s.powerupCollected = true, s.playerState = 0, s.ySpd = (-0x400) + (s.ySpd & 0xff);
//    return minStepsIntoBounds(s, 0x81000, -oo, oo, oo);
////    return minStepsIntoBounds(s, maxXPos + 0x10, -oo, oo, oo);
//  }
//  return minStepsIntoBounds(s, 0x92a0, -oo, oo, oo); // 1-4 speed up
//  { // for 1-4-int powerup collection TODO re-optimize
//    const int maxStepsToHit = 10 - 2;
//    if ((stepsAlreadyTaken < maxStepsToHit-3 && s.powerupBlockHit) || (stepsAlreadyTaken >= maxStepsToHit && !s.powerupBlockHit)) return oo; // not hit powerup block in valid timeframe
//
//    for (int i = 0; i < 12; i++)
//      if (!s.powerupCollected && (stepsAlreadyTaken == maxStepsToHit + 16 + 4*i || stepsAlreadyTaken == maxStepsToHit + 18 + 4*i) && (s.yPos + yPosOffset) < 0x16100 - 0x100*i - (s.crouched ? 0xc00 : 0) && (s.xPos << 4) >= 0x1d400 && (s.xPos << 4) < 0x1ed00) s.powerupCollected = true, s.playerState = 0;
//
//    if ((s.xPos << 4) >= 0x1f000 && !s.powerupCollected) return oo; // missed powerup collection
////    return minStepsIntoBounds(s, 0x22970, -oo, oo, oo);
//    return minStepsIntoBounds(s, 0x22ba0, -oo, oo, oo);
////    return minStepsIntoBounds(s, maxXPos + 0x10, -oo, oo, oo);
//  }
//  return minStepsIntoBounds(s, 0x9190, -oo, oo, oo); // 4-1 speed up
//  return minStepsIntoBounds(s, 0x9420, -oo, oo, oo); // 4-1 speed up PAL
//  return max(minStepsIntoBounds(s, 0xe0600, 0x1a600, oo, oo), 1); // 4-1 flag PAL
//  return minStepsIntoBounds(s, 0x18190, -oo, oo, oo); // 4-2 speed up
//  return minStepsIntoBounds(s, 0x1de00, 0x17800, oo, oo); // 4-2-int wall clip
//  return minStepsIntoBounds(s, 0x1fd70, 0x1a800, oo, oo); // 4-2-int wall clip speed up
//  return minStepsIntoBounds(s, maxXPos + 0x10, 0x1a800, oo, oo); // 4-2 wall clip speed up max
//  return minStepsIntoBounds(s, 0x181b0, -oo, oo, oo); // 4-2 speed up PAL
//  return minStepsIntoBounds(s, 0x1ea00, -oo, oo, oo); // 4-2-int screen scroll
//  return minStepsIntoBounds(s, 0x1f270, -oo, oo, oo); // 4-2-int-pal screen scroll
//  return minStepsIntoBounds(s, maxXPos + 0x10, -oo, oo, oo); // 4-2-int-pal screen scroll max
//  return max(minStepsIntoBounds(s, 0x54400, -oo, oo, oo), 1); // pal-4-2-int2 pipe entry
//  return minStepsIntoBounds(s, 0x8420, -oo, oo, oo); // 4-2-sub speed up PAL
//  return max(minStepsIntoBounds(s, -oo, 0x18000, 0x32cf0, oo), 1); // 4-2-sub pipe entry
//  return minStepsIntoBounds(s, 0x94f0, -oo, oo, oo); // 4-4 speed up
//  { // for 4-4 wall clip
//    if ((s.xPos << 4) < 0x5c400) return oo; // don't go back
//    if (relXPos > 0x92) return oo; // screen scrolled too far
//    if ((s.yPos + yPosOffset) < 0x16100) return oo;
//    if ((s.xPos << 4) >= 0x5f400 && (s.yPos + yPosOffset) < 0x18100) return oo;
////      return minStepsIntoBounds(s, 0x601f0, -oo, oo, oo);
////    return minStepsIntoBounds(s, 0x60e80, -oo, oo, oo);
//    return minStepsIntoBounds(s, 0x60d80, -oo, oo, oo);
////    return minStepsIntoBounds(s, maxXPos + 0x10, -oo, oo, oo);
//  }
//  return max(minStepsIntoBounds(s, 0x77600, 0x1a600, oo, oo), 1); // 8-1 flag PAL
//  return minStepsIntoBounds(s, 0x8520, -oo, oo, oo); // 8-2 speed up PAL
//  return minStepsIntoBounds(s, 0x183f0, -oo, oo, oo); // 8-2 spiny avoidance PAL
//  return minStepsIntoBounds(s, 0x9edf0, -oo, oo, oo); // 8-2 piranha avoidance PAL
//  return max(minStepsIntoBounds(s, 0xd7600, 0x1a600, oo, oo), 1); // 8-2 flag PAL
//  return max(minStepsIntoBounds(s, 0xd5600, 0x1a200, oo, oo), 1); // 8-3 flag
//  return max(minStepsIntoBounds(s, 0xd5600, 0x1a600, oo, oo), 1); // 8-3 flag PAL
//  return minStepsIntoBounds(s, 0x9390, -oo, oo, oo); // 8-4 speed up
//  return minStepsIntoBounds(s, 0x9580, -oo, oo, oo); // 8-4 speed up PAL
//  return minStepsIntoBounds(s, 0x9510, -oo, oo, oo); // 8-4-sub1 speed up
//  return max(minStepsIntoBounds(s, 0x28400, 0x14000, 0x29cf0, 0x14800), 1); // 8-4-sub1 pipe entry
//  return minStepsIntoBounds(s, 0x28410, 0x14000, oo, 0x14800); // 8-4-sub1 pipe entry testing
//  return minStepsIntoBounds(s, 0x28610, 0x14000, oo, 0x14800); // 8-4-sub1 pipe entry testing PAL
//  return minStepsIntoBounds(s, 0x94c0, -oo, oo, oo); // 8-4-sub2 speed up
//  return minStepsIntoBounds(s, 0x94b0, -oo, oo, oo); // pal-8-4-sub2 speed up
//  { // for 8-4-sub2 pipe entry
//    if ((s.leftScreenEdgePos < 0x11 || s.leftScreenEdgePos >= 0x40) && s.xSpd < -0x1c8) return oo; // turned too early
//    if ((s.leftScreenEdgePos < 0x11 || s.leftScreenEdgePos >= 0x40) && (s.xPos << 4) < 0x17000) // pipe not activated yet, waiting for screen scroll
//      return 33 + minStepsIntoBounds(s, 0x18100, -oo, oo, oo); // run to pipe activation point first
////    return max(minStepsIntoBounds(s, -oo, -oo, 0x14cf0, oo), 1); // 8-4-sub2 pipe entry
//    return minStepsIntoBounds(s, -oo, -oo, 0x14e70, oo); // 8-4-sub2 pipe entry
//  }
//  { // for pal-8-4-sub2 pipe entry
//    if ((s.leftScreenEdgePos < 0x11 || s.leftScreenEdgePos >= 0x40) && s.xSpd < -0x300) return oo; // turned too early
//    if ((s.leftScreenEdgePos < 0x11 || s.leftScreenEdgePos >= 0x40) && (s.xPos << 4) < 0x17000) // pipe not activated yet, waiting for screen scroll
//      return 23 + minStepsIntoBounds(s, 0x18100, -oo, oo, oo); // run to pipe activation point first
//    return max(minStepsIntoBounds(s, -oo, -oo, 0x14cf0, oo), 1); // 8-4-sub2 pipe entry
////    return minStepsIntoBounds(s, -oo, -oo, 0x14e70, oo); // 8-4-sub2 pipe entry
//  }
//  return minStepsIntoBounds(s, 0x6250, -oo, oo, oo); // 8-4-sub3 speed up
//  return minStepsIntoBounds(s, 0x63f0, -oo, oo, oo); // 8-4-sub3 speed up PAL
//  return minStepsIntoBounds(s, 0x2f4d0, -oo, oo, oo); // 8-4-sub1 pipe entry testing
//  return minStepsIntoBounds(s, 0x2a000, 0x18800, oo, oo); // testing
//  return minStepsIntoBounds(s, 0x94c0, -oo, oo, oo); // testing
//  return minStepsIntoBounds(s, maxXPos + 0x10, -oo, oo, oo); // testing 1-2 wall clip speed up PAL
}
bool isGoalBlockHit(State& s, int cx, int cy, int cv) {
//  if (s.leftScreenEdgePos < 0x11 || s.leftScreenEdgePos >= 0x40) return false;

//  { // 4-2-int2 pipe entry scroll prevention
//    if (s.leftScreenEdgePos > 0xc0) return false; // scrolled too far
//    int curXPixelPos = s.xPos >> 4;
//    int lastXPixelPos = (s.xPos - (s.xSpd >> 6)) >> 4;
//    if (curXPixelPos != lastXPixelPos) return false; // screen will scroll during pipe entry
//  }

//  return cx == 0x39 && cy == 7 && cv == 0x10; // 1-1 vert pipe
  return cx == 13 && cy == 10 && cv == 0x1f; // 1-1-sub side pipe
//  return cx == 0xc6 && cy == 9 && cv == 0x25 && s.yPos >= (0x1a200 - yPosOffset) && (s.xPos & 0xf) >= 0x3; // 1-1 flag pole
//  return cx == 0xc6 && cy == 9 && cv == 0x25 && s.yPos >= (0x1a600 - yPosOffset); // 1-1 flag pole PAL
//  return s.leftScreenEdgePos < 0x10 && cx == 0xb2 && cy == 8 && cv == 0x10; // 1-2 vert pipe
//  return cx == 0xe1 && cy == 9 && cv == 0x25 && s.yPos >= (0x1a600 - yPosOffset); // 4-1 flag pole PAL
//  return cx == 0x54 && cy == 8 && cv == 0x10; // pal-4-2-int2 vert pipe
//  return cx == 0x32 && cy == 8 && cv == 0x10; // 4-2-sub warp pipe
//  return cx == 0x78 && cy == 9 && cv == 0x25 && s.yPos >= (0x1a600 - yPosOffset); // 8-1 flag pole PAL
//  return cx == 0xd8 && cy == 9 && cv == 0x25 && s.yPos >= (0x1a600 - yPosOffset); // 8-2 flag pole PAL
//  return cx == 0xd6 && cy == 9 && cv == 0x25 && s.yPos >= (0x1a600 - yPosOffset); // 8-3 flag pole PAL
//  return cx == 0xd6 && cy == 9 && cv == 0x25 && s.yPos >= (0x1a200 - yPosOffset) && (s.xPos & 0xf) >= 0x7; // 8-3 flag pole
//  return cx == 0x28 && cy == 4 && cv == 0x10; // 8-4-sub1 vert pipe
//  return s.leftScreenEdgePos >= 0x11 && s.leftScreenEdgePos < 0x40 && cx == 0x14 && cy == 5 && cv == 0x10; // 8-4-sub2 vert pipe
  return false;
}

void demoStep(State& s, int input) {
  Emu emu(s);
  s = emu.runStep(input);
  cout << emu.blockHit << " " << hex << emu.cx << " " << emu.cy << " " << emu.cv << endl;
  s.print();
}

int main() {
#ifdef MAXXPOS
  maxXPos = 0x3c000;
#endif

  cout << "State size: " << sizeof(State) << endl;
  cout << "StateDist size: " << sizeof(StateDist) << endl;

  bb = bb11sub;
  bb.print();
	State s = s11sub;
//	s.yPos = 0x1c0a8 - yPosOffset;
//  {
//    s.yPos -= 0x100;
//    s.xPos += 0x300 >> 4;
//    s.xSpd = 0x3040 >> 2;
//  }

//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(L | A);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(L);
//  for (int i = 0; i < 2; i++) s = Emu(s).runStep(0);
//  for (int i = 0; i < 4; i++) s = Emu(s).runStep(R);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(R | A);
//	s = Emu(s).runStep(L);
//  { // for pal-8-4-sub1 pipe entry
//    for (int i = 0; i < 3; i++) s = Emu(s).runStep(B | R);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(L);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(B | R);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(L);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(B | R);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(L);
//    for (int i = 0; i < 20; i++) s = Emu(s).runStep(R | A);
//  }
//  for (int i = 0; i < 14; i++) s = Emu(s).runStep(B | R);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(D | A);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(L | A);
//  for (int i = 0; i < 2; i++) s = Emu(s).runStep(L);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(0);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(L);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(L | R);

  //  for (int i = 0; i < 14; i++) s = Emu(s).runStep(0);
//  for (int i = 0; i < 17; i++) s = Emu(s).runStep(R | A);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(R);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(0);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(L);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(0);
//  for (int i = 0; i < 2; i++) s = Emu(s).runStep(L);
//  for (int i = 0; i < 10; i++) s = Emu(s).runStep(L | R);
//  s.facingDir = 3;
//	{ // for 1-2-int-pal
//	  for (int i = 0; i < 23; i++) s = Emu(s).runStep(B | R);
//    for (int i = 0; i < 6; i++) s = Emu(s).runStep(R | A);
//
//    for (int i = 0; i < 2; i++) s = Emu(s).runStep(R | A);
//    for (int i = 0; i < 5; i++) s = Emu(s).runStep(L | A);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(R | A);
//    for (int i = 0; i < 3; i++) s = Emu(s).runStep(R);
//	}
//	{ // for 1-2-int pipe clip
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(L);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(R | B);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(L);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(R | B);
//    for (int i = 0; i < 10; i++) s = Emu(s).runStep(R | B);
//
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(R | B);
//    for (int i = 0; i < 2; i++) s = Emu(s).runStep(R | L);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(R | A);
//	}
//	{ // for 4-2-int wall clip
//    for (int i = 0; i < 3; i++) s = Emu(s).runStep(0);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(B | R);
//    for (int i = 0; i < 2; i++) s = Emu(s).runStep(0);
//    for (int i = 0; i < 2; i++) s = Emu(s).runStep(B | R);
//    for (int i = 0; i < 1; i++) s = Emu(s).runStep(0);
//    for (int i = 0; i < 30; i++) s = Emu(s).runStep(B | R);
//    for (int i = 0; i < 4; i++) s = Emu(s).runStep(B | R);
//
//    for (int i = 0; i < 2; i++) s = Emu(s).runStep(B | R);
////    for (int i = 0; i < 1; i++) s = Emu(s).runStep(R | A);
//	}
//  for (int i = 0; i < 13; i++) s = Emu(s).runStep(A);
//  for (int i = 0; i < 15; i++) s = Emu(s).runStep(0);
//  for (int i = 0; i < 11; i++) s = Emu(s).runStep(L);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(L | R);
//  for (int i = 0; i < 5; i++) s = Emu(s).runStep(L | A);
//  for (int i = 0; i < 24; i++) s = Emu(s).runStep(L);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(L);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(R | A);
//  for (int i = 0; i < 28; i++) s = Emu(s).runStep(R);
//  for (int i = 0; i < 18; i++) s = Emu(s).runStep(R | A);
//  for (int i = 0; i < 3; i++) s = Emu(s).runStep(B | R);
//  for (int i = 0; i < 1; i++) s = Emu(s).runStep(R | A);
//  for (int i = 0; i < 28; i++) s = Emu(s).runStep(R);
//  for (int i = 0; i < 18; i++) s = Emu(s).runStep(R | A);
//  s.leftScreenEdgePos -= 2;
//  for (int i = 0; i < 41; i++) s = Emu(s).runStep(R);
//  s = Emu(s).runStep(R | B);
//  s = Emu(s).runStep(L);

//	cout << dec << distanceToGoalHeuristic(s) << endl;

	vector<State> ss;
	ss.push_back(s);
//  ss = withAllXSpdSubpixels(ss);
//  ss = withSmallerXPos(ss, 7);
//  ss = withFacingDirs(ss);
  cout << "Initial state size: " << dec << ss.size() << endl;
	vector<int> inputs = IDDFS(ss);

	printRLE(inputs);

//  {
//    for (int i = 0; i < 3; i++) demoStep(s, 0);
//    for (int i = 0; i < 1; i++) demoStep(s, B | R);
//    for (int i = 0; i < 2; i++) demoStep(s, 0);
//    for (int i = 0; i < 2; i++) demoStep(s, B | R);
//    for (int i = 0; i < 1; i++) demoStep(s, 0);
//    for (int i = 0; i < 34; i++) demoStep(s, B | R);
//
//    for (int i = 0; i < 1; i++) demoStep(s, B | R);
//    for (int i = 0; i < 10; i++) demoStep(s, R | A);
//    for (int i = 0; i < 4; i++) demoStep(s, 0);
//    for (int i = 0; i < 1; i++) demoStep(s, L);
//    for (int i = 0; i < 2; i++) demoStep(s, 0);
//  }
//  for (int i = 0; i < 1; i++) demoStep(s, L | A);
//  for (int i = 0; i < 1; i++) demoStep(s, L);
//  for (int i = 0; i < 2; i++) demoStep(s, 0);
//  for (int i = 0; i < 4; i++) demoStep(s, R);
//  for (int i = 0; i < 1; i++) demoStep(s, R | A);
//  for (int i = 0; i < 4; i++) demoStep(s, R);
//  for (int i = 0; i < 1; i++) demoStep(s, 0);
//  for (int i = 0; i < 1; i++) demoStep(s, R | B);
//  for (int i = 0; i < 1; i++) demoStep(s, A);
//  for (int i = 0; i < 1; i++) demoStep(s, 0);
//  for (int i = 0; i < 2; i++) demoStep(s, R);
//  for (int i = 0; i < 1; i++) demoStep(s, L | R | A);
//  for (int i = 0; i < 1; i++) demoStep(s, R);
//  for (int i = 0; i < 2; i++) demoStep(s, 0);
//  for (int i = 0; i < 1; i++) demoStep(s, L);
//  for (int i = 0; i < 2; i++) demoStep(s, 0);
//  for (int i = 0; i < 3; i++) demoStep(s, R);
//  for (int i = 0; i < 1; i++) demoStep(s, L);
//
//	s.print();
//	return 0;
}
