# mon - a command line Pokédex

Uses art from [pokemon-colorscripts](https://gitlab.com/phoneybadger/pokemon-colorscripts) and Pokémon data from [PokéAPI](https://pokeapi.co).
___
### Building

Tested on:
```
> rustc --version
rustc 1.54.0 (a178d0322 2021-07-26)
```

A simple `cargo build --release` should be enough.

___
### Usage

`mon [name/ID]` - gives you the data for specified Pokémon\
`mon` - gives you a random Pokémon

#### There are some Pokémon that return a garbled mess of HTML response, please make an issue with the name or ID.
___

### Goals

- Add a synopsis of the Pokémon, like Pikachu's "When several of these Pokémon gather, their electricity could build and cause lightning storms." (Maybe choose a random one from each generation it appears in?)
- Mythic and Legendary Pokémon name highlighting

___

