# battlesnake
Leisure project - Rust API for [battlesnake.com](https://play.battlesnake.com/)

Currently working on implementation solely for Royale mode due to ongoing [Summer Leage](https://play.battlesnake.com/league/summer-league-2021/) event (registered as _Go Giddy_ snake).

## Current state

### Aggregate
On aggregate all version have achieved following results during battlesnake's Summer League Event 2021:
```
Jul 1

Games: 2,447
Wins: 752
Losses: 1,695
Win Rate: 30.73%
```

### v0.1.1
```
Jun 30

Games: 239
Wins: 110
Losses: 129
Win Rate: 46.03%
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

## AWS EC deployment
### Deployment custom script
Installs aws-codedeploy and rust when instance launched

```
#!/bin/bash
sudo apt-get -y update
sudo apt-get -y install ruby
sudo apt-get -y install wget
sudo apt-get -y install curl
cd /home/ubuntu
wget https://aws-codedeploy-us-east-1.s3.amazonaws.com/latest/install
sudo chmod +x ./install
sudo ./install auto
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
```

## TO-DO
### Minimax Rework
- [x] add launch options for port and time budget at least
- [x] all eligible moves of all snakes at current board
- [x] move A* to board
- heuristics
    - [ ] floodfill
    - [ ] A*
    - [ ] aggression
    - [ ] consider Evolutionary algo

### ASAP
- [x] rework astar cost heur to "health cost" (e.g. normal move < hazard move costs)
- [x] disregard paths with cost > health
- [ ] save path to closest empty in astar
- [x] dfs fallback experiment
- [ ] check if dfs path is survivable
- [ ] fallback to survival mode
- [ ] game logging
- [x] batch test scripts
- [x] avoid _Hazard Sauce_ completely for now, unless no path found
- [x] add hazard fields around heads of longer snakes
- [ ] hazards around snakes heads vs food - which has prio? reconsider design

### A* to-do list
- Heuristics
    - [ ] floodfill for space eval (discord suggested high boost in performance)
    - [ ] implement aggressive move eval

### IDEAS
- [ ] figure out how not to get _self-stuck_ (perhaps the GE paper could help)
- [ ] think of aggressive moves to block off oponents
- [ ] astar on enemy heads to check if they can reach an apple before me -> give up on the apple?
    - consider collecting paths for each apple before decision making -> no need to rediscover paths every time

