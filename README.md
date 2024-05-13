# mon - A Command Line Pokédex

mon is a command line Pokédex tool that provides information and art for Pokémon using data from [PokéAPI](https://pokeapi.co) and art from [pokemon-colorscripts](https://gitlab.com/phoneybadger/pokemon-colorscripts). It allows users to fetch data for specific Pokémon, view random Pokémon, and even generate shiny variants. Additionally, mon provides a synopsis for each Pokémon, making the experience more immersive.

## Building and Installation

### Prerequisites

- Rust compiler (`rustc`) version 1.78.0 or later.

### Building

To build mon, simply run the following command:

```bash
cargo build --release
```

### Further Setup (optional)

For convenient access, you can add the build path to your PATH environment variable. Additionally, you can configure your terminal to greet you with a random Pokémon every time you launch it.

- **PowerShell 5:** Append `-NoExit mon` to your PowerShell shortcut's target.
- **PowerShell 7:** Append `-NoExit -Command "mon"` to your PowerShell shortcut's target.
- **Command Prompt:** Append `-cmd /K mon` to your Command Prompt shortcut's target.

## Usage

### Commands

- `mon`: Displays information and art for a random Pokémon.
- `mon [name/ID]`: Fetches data for the specified Pokémon by name or ID.
- `-s` or `--shiny`: Generates the shiny variant of the Pokémon.
- `-g` or `--gen`: Allows users to specify the generation (1 to 8) of the Pokémon to be shown.

### Known Issues

Some Pokémon may return garbled HTML responses. If you encounter such issues, please open an issue with the Pokémon's name or ID, and it will be addressed promptly.

## Goals

- [x] Add shiny variants.
- [x] Provide a synopsis for each Pokémon.
- [ ] Highlight mythic and legendary Pokémon names.
- [ ] Integrate animated sprites.

mon aims to enhance your Pokémon experience by providing convenient access to Pokémon data and artwork directly from your terminal. If you encounter any issues or have suggestions for improvements, feel free to open an issue or pull request. Enjoy exploring the world of Pokémon with mon!
