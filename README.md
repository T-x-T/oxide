The primary source lives on my own forgejo instance at [git.thetxt.io](https://git.thetxt.io/thetxt/oxide). Please also submit issues there. Registration also works through your GitHub or Discord account, so you don't have to remember yet another password.

# Oxide
The Oxide Minecraft Server aims to implement a fully functional Minecraft Server, compatible with the latest version, from scratch entirely in Rust.

You can try it right now by joining play.oxide.thetxt.io

This repository contains the Oxide Minecraft Server, as well as all auxillary parts described below.
Each of these parts live in their own subdirectory.

The main purpose of this repository is for me to learn and share my gained knowledge through a series of [YouTube videos](https://www.youtube.com/playlist?list=PLht_DnAZ_Av4UZwQGhz7aNDMH9pfI0ein).

**Please don't run it with any worlds you aren't prepared to lose.**

# Features
## Currently implemented
This list is non-exhaustive, but covers the most important parts.
- Multiple players with basic interactions (see item in hand or armor and their position)
- Chat (without encryption) and some basic commands
- Placing and breaking blocks (very few block state rules implemented)
- Loading and saving vanilla worlds
- Creative mode
- Block entities (chests, furnaces (only smelting raw iron with coal) and signs for now)

## Missing
These will be implemented in the nearish future
- Entities with basic AI
- More commands
- Survival mode
- Nether and End dimensions
- Protocol encryption / online mode
- Packet compression
- More block state rules
- Rules on which blocks can be placed on which other blocks

# Running
## For testing
You can run the server by just executing `cargo run` in the server subdirectory.

## For "production"
Remember, this is very alpha software that _will_ at some point delete your worlds or damage them otherwise!

I recommend running through docker using the provided `docker-compose.yml` file. There is also a `docker-compose.build.yaml` file that builds a fresh image locally, instead of using the pre-built image from the gibthub container registry.

# Configuration

Configuration is handled through environment variables.
| Key | Default value | Format | Notes |
| ------------------- | ------------- | ------------- | ----------------------------------------------------------------------------------------------------------------------- |
| OXIDE_LISTEN_ON | 0.0.0.0:25565 | [ip]:[port] | |
| OXIDE_WORLD_PATH    | ./world       | any path      | ./ must be explicitly stated                                                                                            |
| OXIDE_SAVE_SECONDS | 60            | Whole seconds |                                                                                                                         |
| OXIDE_MOTD          | Hello oxide!  | String        | Quick and dirty hack, Freaks out on windows if string is contained in quotes or includes spaces and i don't know why... |
# Repository contents
## server
This is the actual Minecraft server.

## proxy
This is a very basic proxy to intercept communication between the minecraft client and server.
It listens on localhost port 35565 and connects to a server running on localhost port 25565, without a way to configure these at the moment.
It has access to all the packet deserialization/serialization code which makes this proxy useful for testing their implementations against the official minecraft server.

## lib
A library containing common functionality that is shared between the server and proxy. This mostly includes the logic to deserialize and serialize packets and some common types.

## data_generator_tools
Extremely rough program to generate the data library crate from the official servers generated json files.

## data
Another library that contains data on biomes, blocks and items. It is mostly generated using the data_generator_tools crate.

# Contributing
At the moment I'm not looking for major source code contributions. If you encounter a minor bug and decide to fix yourself, then feel free to open a PR and I will take a look.

Instead, the best way for you to help is to spread the word about this project and to test it yourself. If you encounter any bugs or have ideas for things that could be improved, then please open an issue or get in touch otherwise.

The last option is to [support me financially](https://buymeacoffee.com/thetxt), which would make it possible for me to invest more time into Oxide. Please only do this if you can afford it.

# Dependencies
This project aims to minimally rely on third party dependecies. Everything related to Minecraft is fully custom.
The only third party dependency is flate2 for compression and decompression. The jzon crate is also used in the `data_gnerator_tools` utility for working with the generated json files from the official Minecraft server.

# Credit
This project wouldn't be possible without the contributors of the [Minecraft wiki](https://minecraft.wiki/w/Java_Edition_protocol).
