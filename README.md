# Arena2D
## A WIP survival-like game

![screenshot](https://i.imgur.com/EiwIFro.png)

## Running the game

Note: ```cargo run --release``` will compile the first time it is called, it can take anywhere from 5 to 60 minutes depending on your computer, subsequent
calls won't need to compile again

### NixOS
```sh
git clone --depth 1 https://github.com/a-panda-miner/arena2d
cd arena2d
nix-shell
cargo run --release
```

### Other Linux Systems
Follow the instructions at [bevy dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
```sh
git clone --depth 1 https://github.com/a-panda-miner/arena2d
cd arena2d
cargo run --release
```
### Planned features
- [ ] PowerUp system
- [ ] MetaUpgrades system
- [ ] Pet system
- [ ] Complex enemies
- [ ] Bosses
- [ ] Android build


### LICENSE 
All assets and **written** code are CC0
