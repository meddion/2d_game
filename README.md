# How to compile and run

Initially, you need to install a few dependencies for Amethyst (a game engine used in this project).
On Ubuntu:

```
sudo apt install gcc pkg-config openssl libasound2-dev cmake
build-essential python3 libfreetype6-dev libexpat1-dev
libxcb-composite0-dev libssl-dev
```

Amethyst requires you to have one of the graphical backends. I went with Vulkan:

```
sudo apt-get install libvulkan-dev mesa-vulkan-drivers
vulkan-­utils
```

Finally, to compile and run the game simply type:

```
cargo run --features vulkan
```
