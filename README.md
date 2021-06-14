# battlesnake
Leisure project - Rust API for [battlesnake.com](https://play.battlesnake.com/)

Currently working on implementation solely for Royale mode due to ongoing [Summer Leage](https://play.battlesnake.com/league/summer-league-2021/) event (registered as _Go Giddy_ snake).

## Current state (~30% win ratio)
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
### ASAP
- [ ] rework astar cost heur to "health cost" (e.g. normal move < hazard move costs)
- [ ] disregard paths with cost > health
- [ ] fallback to longest available path outside of the hazard
- [ ] game logging
- [ ] batch test scripts
- [x] avoid _Hazard Sauce_ completely for now, unless no path found
- [x] add hazard fields around heads of longer snakes
- [ ] hazards around snakes heads vs food - which has prio? reconsider design

### IDEAS
- [ ] figure out how not to get _self-stuck_ (perhaps the GE paper could help)
- [ ] think of aggressive moves to block off oponents
- [ ] astar on enemy heads to check if they can reach an apple before me -> give up on the apple?
    - consider collecting paths for each apple before decision making -> no need to rediscover paths every time

