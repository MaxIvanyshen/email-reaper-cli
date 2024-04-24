#!/bin/sh

##### INSTALL CHROMEDRIVER AND UPDATE GOOGLE CHROME #####
#get latest version
version=`curl http://chromedriver.storage.googleapis.com/LATEST_RELEASE)`;
echo 'Currently LATEST_RELEASE:' $version;
#download the latest version chrome driver available as per the above line
wget -N http://chromedriver.storage.googleapis.com/${version}/chromedriver_linux64.zip
unzip chromedriver_linux64.zip -d /usr/local/bin
chmod a+x /usr/local/bin/chromedriver
echo 'Chromedriver installed'

##### INSTALL RUST #####
echo 'Installing Rust...'
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo 'Building Reaper...'   
cargo build --release
chmod +x ./target/release/reaper
sudo mkdir -p /opt/reaper
mv ./target/release/reaper /opt/reaper/
export PATH=$PATH:/opt/reaper

echo 'Installed'   
