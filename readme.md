# rust-bgfx-experiment
First rust project, experimenting with bgfx as a graphics solution.

## Architecture
The architecture of this project is very straight forward and borders on being a hacky solution.

### main.rs
The main entrypoint for the game. This is where the application is initialized.

### platform
Platform contains the platform related code such as creating an SDL window with a vulkan context, the bgfx renderer, and the logging system. This can be loosely thought of as the "engine" of the game.

### application
Application contains the game related code, it will receive a "platform" struct that will contain the globals (renderer, window, event_bus, ecs, etc.) that will be used to actually build the game.
