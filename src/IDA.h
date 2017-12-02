#ifndef IDA_H_
#define IDA_H_

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <unordered_set>
#include <unordered_map>
#include <vector>
#include "state.h"
#include "kbtree.h"

using namespace std;

struct StateDist {
  State s;
  unsigned short d;
};

//#define elem_cmp(a, b) memcmp(&a, &b, sizeof(State))
#define elem_cmp(a, b) compareState((a).s, (b).s)
KBTREE_INIT(str, StateDist, elem_cmp)

kbtree_t(str) *seen;
//set<State> seen;
//map<State, unsigned char> minDistance;
//map<State, unsigned char> minDistancePrev;

int distanceToGoalHeuristic(State& s, int stepsAlreadyTaken);
bool isGoalBlockHit(State& s, int cx, int cy, int cv);

#ifdef MAXXPOS
int maxXPos;
#endif

vector<int> validInputs(State& s) {
  vector<int> res;
  res.push_back(0);
  if (s.yPos < (0x10000 - yPosOffset) || s.yPos >= (0x1d000 - yPosOffset)) return res; // inputs are ignored
  res.clear();

  if (s.isOnGround() && s.movingDir != 0) res.push_back(B | s.movingDir);
  res.push_back(R);
  if (s.isOnGround()) res.push_back(L | R);
  res.push_back(0);
  res.push_back(L);
#ifdef BIG
  if (s.isOnGround()) res.push_back(D);
#endif

  if (IS_SWIMMING || (s.playerState == 1 && s.vForceIdx != s.vForceDIdx) || s.isOnGround()) res.push_back(A);
  if (IS_SWIMMING || (s.playerState == 1 && s.vForceIdx != s.vForceDIdx) || s.isOnGround()) res.push_back(A | R);
  if (IS_SWIMMING || (s.playerState == 1 && s.vForceIdx != s.vForceDIdx) || s.isOnGround()) res.push_back(A | L | R);
  if (IS_SWIMMING || (s.playerState == 1 && s.vForceIdx != s.vForceDIdx) || s.isOnGround()) res.push_back(A | L);
#ifdef BIG
  if (s.isOnGround()) res.push_back(D | A);
#endif

  return res;
}

vector<int> solutionInputs;
long visits = 0;

int acceptableCx = 0;
int acceptableCy = 0;
int acceptableCv = 0;

bool findSolution(State& s, int stepsAlreadyTaken, int maxAllowedSteps) {
  int heuristic = distanceToGoalHeuristic(s, stepsAlreadyTaken);
  if (stepsAlreadyTaken + heuristic > maxAllowedSteps) // out of steps
    return false;
#ifndef MAXXPOS
  if (heuristic <= 0) // found solution
    return true;
#endif
  if (s.yPos >= (0x1c500 - yPosOffset)) // too low (or underflow)
    return false;
//  if (minDistance.count(s) )//&& minDistance.at(s) <= stepsAlreadyTaken)
//    return false;
//  if (minDistancePrev.count(s) && minDistancePrev.at(s) < stepsAlreadyTaken)
//    return false;
//  if (seen.count(s))
//    return false;

#ifdef MAXXPOS
  if ((s.xPos << 4) > maxXPos) {
    maxXPos = s.xPos << 4;
    cout << "maxXPos " << hex << maxXPos << endl;
  }
#endif

  StateDist sd {s, (unsigned short)stepsAlreadyTaken};
  StateDist* sp = kb_getp_str(seen, &sd);
  if (sp && sp->d <= stepsAlreadyTaken) return false;

  visits += 1;
  if (visits % 1000000 == 0) {
//    cout << "distance " << dec << stepsAlreadyTaken << " heuristic " << heuristic << " seen " << minDistance.size() << endl;
//    cout << "distance " << dec << stepsAlreadyTaken << " heuristic " << heuristic << " seen " << seen.size() << endl;
    cout << "distance " << dec << stepsAlreadyTaken << " heuristic " << heuristic << " limit " << maxAllowedSteps << " seen " << kb_size(seen) << endl;
  }

//  minDistance[s] = stepsAlreadyTaken;
//  seen.insert(s);
  if (sp) sp->d = stepsAlreadyTaken;
  else kb_putp_str(seen, &sd);

  vector<int> inputs = validInputs(s);
  for (int& input : inputs) {
    Emu emu = Emu(s);
    State ns = emu.runStep(input);
    if (emu.blockHit) {
      if (isGoalBlockHit(ns, emu.cx, emu.cy, emu.cv)) {
        solutionInputs.push_back(input);
        if (heuristic > (int)solutionInputs.size())
          cerr << "WARNING: heuristic " << dec << heuristic << " larger than steps needed " << solutionInputs.size() << endl;
        cout << emu.blockHit << " " << hex << emu.cx << " " << emu.cy << " " << emu.cv << endl;
        ns.print();
        return true;
      }
      continue; // unacceptable block hit
    }
    if (findSolution(ns, stepsAlreadyTaken + 1, maxAllowedSteps)) {
      solutionInputs.push_back(input);
      cout << emu.blockHit << " " << hex << emu.cx << " " << emu.cy << " " << emu.cv << endl;
      ns.print();
      return true;
    }
  }
  return false;
}

vector<int> IDDFS(vector<State> ss, int maxAllowedSteps) {
  seen = kb_init(str, KB_DEFAULT_SIZE);

  while (true) {
    bool found = false;

    cout << "search max distance " << dec << maxAllowedSteps << endl;
    for (State s : ss)
      if (findSolution(s, 0, maxAllowedSteps)) {
        s.print();
        found = true;
        break;
      }
    if (found) break;

#ifdef MAXXPOS
    cout << "maxXPos " << hex << maxXPos << endl;
#endif

    maxAllowedSteps++;
//    minDistancePrev.swap(minDistance);
//    minDistance.clear();
//    seen.clear();

    kbitr_t itr;
    kb_itr_first(str, seen, &itr); // get an iterator pointing to the first
    for (; kb_itr_valid(&itr); kb_itr_next(str, seen, &itr)) { // move on
        kb_itr_key(StateDist, &itr).d++; // increase distance by one; only shortest paths to any known state will be considered next round
    }

//    kb_destroy(str, seen);
  }
  reverse(solutionInputs.begin(), solutionInputs.end());

  return solutionInputs;
}
vector<int> IDDFS(vector<State> ss) {
  return IDDFS(ss, 0);
}

#endif /* IDA_H_ */
