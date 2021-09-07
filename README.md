# mon - a command line Pokédex

Uses art from [pokemon-colorscripts](https://gitlab.com/phoneybadger/pokemon-colorscripts) and Pokémon data from [PokéAPI](https://pokeapi.co).

Tested on:
```
> rustc --version
rustc 1.54.0 (a178d0322 2021-07-26)
```

A simple `cargo build --release` should be enough, you can place the built binary somewhere in your path and just run `mon [pokemon name or id]`

___

*Goals*

- Add a synopsis of the Pokémon, like Pikachu's "When several of these Pokémon gather, their electricity could build and cause lightning storms." (Maybe choose a random one from each generation it appears in?)
- Add error catching for things like not passing in arguments or a request failing
- Get rid of spaghetti code, and a lot of it
- Mythic and Legendary Pokémon name highlighting

___

![Screenshot 2021-09-07 161442](https://user-images.githubusercontent.com/54457902/132411889-7cd97cbd-b4d0-4942-8d70-470966f82b14.png)

