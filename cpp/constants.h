#ifndef CONSTANTS_H_
#define CONSTANTS_H_

const int A = 0x80;
const int B = 0x40;
const int SELECT = 0x20;
const int START = 0x10;
const int U = 0x08;
const int D = 0x04;
const int L = 0x02;
const int R = 0x01;

const int oo = 0x10101010;

#ifdef BIG
  const bool IS_BIG = true;
#else
  const bool IS_BIG = false;
#endif
#ifdef SWIMMING
  const bool IS_SWIMMING = true;
#else
  const bool IS_SWIMMING = false;
#endif

const int blockBufferXAdderData[21] = {
    0x080,  0x030,  0x0c0, 0x020,  0x020, 0x0d0,  0x0d0,
    0x080,  0x030,  0x0c0, 0x020,  0x020, 0x0d0,  0x0d0,
    0x080,  0x030,  0x0c0, 0x020,  0x020, 0x0d0,  0x0d0};
const int blockBufferYAdderData[21] = {
     0x400, 0x2000, 0x2000,  0x800, 0x1800,  0x800, 0x1800,
     0x200, 0x2000, 0x2000,  0x800, 0x1800,  0x800, 0x1800,
    0x1200, 0x2000, 0x2000, 0x1800, 0x1800, 0x1800, 0x1800};

const unsigned int yPosOffset = 0xd000;

#ifdef PAL
const short jumpVelocitySlow = -(0x4cc);
const short jumpVelocityFast = -(0x600);
const short jumpVelocitySwim = -(0x180);
const int maxXSpeedWalk = 0x1c00;
const int maxXSpeedRun = 0x3000;
const int maxXSpeedSwim = 0x1300;
const int maxYSpeed = 0x500;
const int friction0 = 0x1c0;
const int friction1 = 0x100;
const int friction2 = 0x180;
const int blockLandingFractionalHeight = 6;
#else
const short jumpVelocitySlow = -(0x400);
const short jumpVelocityFast = -(0x500);
const short jumpVelocitySwim = -(0x180);
const int maxXSpeedWalk = 0x1800;
const int maxXSpeedRun = 0x2800;
const int maxXSpeedSwim = 0x1000;
const int maxYSpeed = 0x400;
const int friction0 = 0xe4;
const int friction1 = 0x98;
const int friction2 = 0xd0;
const int blockLandingFractionalHeight = 5;
#endif /* PAL */

#endif /* CONSTANTS_H_ */
