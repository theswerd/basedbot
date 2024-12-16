# PoseBot

Real-time human pose mirroring for the Zeroth robot. PoseBot captures human poses through a camera and replicates the movements on a physical robot.

## Overview

PoseBot uses computer vision to detect and track human poses in real-time, then translates these movements into robot joint commands. This creates an interactive experience where the robot mirrors human movements as they happen.

## Features

- Real-time human pose detection using computer vision
- Smooth translation of human movements to robot joint positions
- Built with Rust for high performance and reliability
- Generic `humanoid` interface supports different robotics platforms

## Requirements

- Rust (latest stable version)
- Compatible robot hardware (servo motors, actuators)
- USB camera or webcam
- Linux/macOS/Windows operating system

## Installation

1. Clone the repository:

```bash
git clone https://github.com/theswerd/basedbot.git
cd posebot
```

## Usage

1. Connect your camera and robot hardware
2. Setup `.env` with the correct IP for the server
3. Run the PoseBot service:

```bash
cargo run --release
```

3. Stand in front of the camera within the designated area
4. Perform movements and watch the robot mirror your poses

## Safety

- The robot has built-in movement constraints to prevent damage
- Please maintain a safe distance from the robot during operation

## Architecture

- `pose_mappings` crate runs pose estimation, and sends POST requests with data to the server.
- `humanoid` crate contains the generic humanoid robot interface and related types.
- `zeroth` crate includes the RPC client for the Zeroth robot.
- `kbot` crate contains the RPC client for the KBot.
- `bot` contains the controller server implementation.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to [K-Scale](https://github.com/kscalelabs)!

