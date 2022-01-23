# Versions and their brief descriptions

## WIP 0.2.0
Refactoring, restructuring, reinventing for new season. Breaks everything below

## Summer League 2021
This was my first ever battlesnake. Started with A* only and developed into somewhat working minmax.

### Aggregate
On aggregate all version have achieved following results during battlesnake's Summer League Event 2021:
```
Jun 24

Games: 1,632
Wins: 447
Losses: 1,185
Win Rate: 27.39%
```

### v0.1.1
```
Jun 29

Games: 0
Wins: 0
Losses: 0
Win Rate: 0
```
Slightly updated heuristics. WIP

### v0.1.0
Stats at the end of life:
```
Jun 29

Games: 576
Wins: 195
Losses: 381
Win Rate: 33.85%
```
Minimal minimax with astar as eval function only deployed on Jun 24 on Linode for better latency.

### v0.0.2 (~27% win ratio)
Stats at the end of life:
```
Jun 24

Games: 1,632
Wins: 447
Losses: 1,185
Win Rate: 27.39%
```
Additional DFS checking implemented to check whether a path has an escape path. This feature could use some refactoring in order to be more inpactful

Local tests of Self vs. Self royale games averaged on 77 turns in 10k games. The results reflect average survival rate of this version as it doesn't involve any _aggresive_ moves yet.

### v0.0.1 (~25% win ratio)
New heuristic _Battlesnake_ that considers move cost to be 1 + [15 if hazard] + [(snake hp + 1) if potential collision with other snake].

### v0.0.0 (~20% win ratio)
Simple manhattan A* forcing out of hazards paths with fallback on ignoring hazard. 
Marking surrounding cells of bigger snakes' heads as _hazard_.