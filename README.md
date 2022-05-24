# mon - a command line Pokédex

Uses art from [pokemon-colorscripts](https://gitlab.com/phoneybadger/pokemon-colorscripts) and Pokémon data from [PokéAPI](https://pokeapi.co).
___
### Building

Tested on:
```
> rustc --version
rustc 1.61.0 (fe5b13d68 2022-05-18)
```

A simple `cargo build --release` should be enough.

___
## Further Setup (optional)
- Add the build path to your PATH environment variable
- in Terminal, appending `-NoExit mon` to your Powershell Command line, or  `-cmd /K mon` to your command prompt Command line settings will greet you with a new random mon every time you launch either PowerShell or the command prompt using the terminal

___
### Usage

`mon [name/ID]` - gives you the data for specified Pokémon\
`mon` - gives you a random Pokémon

#### There are some Pokémon that return a garbled mess of HTML response, please make an issue with the name or ID.
___

### Goals

- [x] Add shiny variants
- [ ] Add a synopsis of the Pokémon, like Pikachu's "When several of these Pokémon gather, their electricity could build and cause lightning storms." (Maybe choose a random one from each generation it appears in?)
- [ ] Mythic and Legendary Pokémon name highlighting
- [ ] Add animated sprites

