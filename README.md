# pingman
Pingman is a very fast and concurrent http pinging cli. It supports proxies and very verbose configurations.

## Note
pingman is very new and has almost no actual testing so its probably a buggy mess atm just let that be known, if you spot a bug please either create an issue or a pull request :)

## Installation
There is 2 ways to install pingman, the first is compiling it yourself from source with cargo and the second is downloading from github releases.

### Build from source with `git` and `cargo`
```bash
git clone https://github.com/steele123/pingman.git
cd pingman
cargo build --release
```

### Install from github releases
Releases are added by GitHub actions for Mac, Linux and Windows each time I create a new release tag.
[Latest Release](https://github.com/steele123/pingman/releases/latest)

After you download the release simply extract it with WinRar then you can run pingman!

## Basic Usage
View current pingman version
```bash
pingman -V
```
View the help command for more verbose commands
```bash
pingman -h
```
Using a file filled with proxies to ping amazon
```bash
# By default the site will be google.com this will use your proxies.txt to ping all of them
pingman proxy -f ./proxies.txt -s https://amazon.com 
```
Basic http ping
```bash
# This will ping amazon 10 times and give you the analytics
pingman ping -s https://amazon.com
```
Saving the proxy results to a json file
```bash
pingman proxy -f ./proxies.txt -s https://amazon.com -o ./output.json
```