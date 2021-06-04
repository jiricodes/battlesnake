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
