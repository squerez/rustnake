# Rustnake

[![License: Apache](https://img.shields.io/badge/licence-apache-blue.svg)](LICENSE)

## Introduction

Rustnake is a customizable snake game implemented in Rust, designed to showcase the power and flexibility of the language. 
The game provides a simple yet engaging experience where players control a snake that grows in length as it consumes food items while avoiding collisions with walls and itself.

## Features

- **Customizable Game Setup**: The `main` branch of this repository introduces a flexible game setup, allowing developers to customize some aspects of the game, including window size, pixel size, and snake speed.
- **Responsive Controls**: The game offers smooth and responsive controls, allowing players to guide the snake effortlessly using arrow keys or other input mechanisms.
- **Collision Detection**: Rustnake incorporates robust collision detection algorithms to ensure accurate handling of snake-wall and snake-self collisions, resulting in a fair and challenging gameplay experience.
- **Score Tracking**: The game keeps track of the player's score, reflecting the number of food items consumed. The goal is to achieve the highest score possible while avoiding collisions.

## Installation

To run Rustnake locally, please ensure you have the following prerequisites:

- Rust programming language (version 1.55.0 or higher)
- Cargo (Rust's package manager)

Once you have the prerequisites set up, follow these steps:

1. Clone this repository:

```shell
git clone https://github.com/squerez/rustnake.git
```
2. Navigate to the project directory:
```
cd rustnake
```
3. Build the project:
```
cargo build --release
```
4. Run the game:
```
cargo run --release
```

## Contributions and Support

Contributions to Rustnake are welcome! 
If you find a bug, have a feature suggestion, or would like to contribute in any way, please feel free to open an issue or submit a pull request. 
Your contributions will be greatly appreciated.

If you encounter any problems while using Rustnake or have any questions, please don't hesitate to reach out. 
You can contact the project maintainer via the email provided in the repository.

## License
Rustnake is distributed under the Apache License. See the [LICENSE](LICENSE) file for more information.
