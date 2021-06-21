# battlesnake
Leisure project - Rust API for [battlesnake.com](https://play.battlesnake.com/)

Currently working on implementation solely for Royale mode due to ongoing [Summer Leage](https://play.battlesnake.com/league/summer-league-2021/) event (registered as _Go Giddy_ snake).

## Current state

### Aggregate
On aggregate all version have achieved following results during battlesnake's Summer League Event 2021:
```
Jun 18

Games: 865
Wins: 199
Losses: 666
Win Rate: 23.01%
```

### v0.1.0 WIP
Despite there are a lot of possible improvements for v0.0.2 algo, I believe that minmax based snake has better chances on success.
So I'll attempt to create one and test it 1v1 with it's predecessors. Let see.

### v0.0.2 (~27% win ratio)
Additional DFS checking implemented to check whether a path has an escape path. This feature could use some refactoring in order to be more inpactful

Local tests of Self vs. Self royale games averaged on 77 turns in 10k games. The results reflect average survival rate of this version as it doesn't involve any _aggresive_ moves yet.

### v0.0.1 (~25% win ratio)
New heuristic _Battlesnake_ that considers move cost to be 1 + [15 if hazard] + [(snake hp + 1) if potential collision with other snake].

### v0.0.0 (~20% win ratio)
Simple manhattan A* forcing out of hazards paths with fallback on ignoring hazard. 
Marking surrounding cells of bigger snakes' heads as _hazard_.

## Used farmworks
```
Rust
 - actix
 - serde
```

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
```

## TO-DO
### Minimax Rework
- [ ] all eligible moves of all snakes at current board
- [ ] move A* to board

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

