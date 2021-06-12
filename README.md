# battlesnake
Leisure project - Rust API for battlesnake.com

# Used farmworks
```
Rust
 - rocket
 - serde
```

# AWS EC deployment
## Deployment custom script
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

# TO-DO
## ASAP
- [ ] game logging
- [x] avoid _Hazard Sauce_ completely for now, unless no path found
- [x] add hazard fields around heads of longer snakes
- [ ] fix fallback logic to get out of a hazard area asap


## IDEAS
- [ ] consider logic for picking apples from _Hazard Sauce_ (might not be even needed with current fallback on ignoring hazards)
- [ ] think of aggressive moves to block off oponents
- [ ] astar on enemy heads to check if they can reach an apple before me -> give up on the apple?
    - consider collecting paths for each apple before decision making -> no need to rediscover paths every time

