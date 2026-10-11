# guessing-game-rust

Een raadspel in Rust: de computer kiest een geheim nummer tussen 1 en 100 en jij raadt het.
Na elke gok krijg je te horen of je te hoog of te laag zit. Het spel stopt als je het goed hebt.

Gemaakt als oefening bij het hoofdstuk "Programming a Guessing Game" van
[The Rust Programming Language](https://doc.rust-lang.org/book/), met Nederlandse teksten.

## Wat erin zit

- `loop` om te blijven vragen tot de gok klopt
- invoer lezen met `std::io` en omzetten naar een getal (`parse`)
- een ongeldige invoer opvangen met `match`
- vergelijken met `cmp` en `Ordering`
- een willekeurig getal met de `rand`-bibliotheek

## Draaien

Je hebt Rust nodig (zie [rust-lang.org/learn](https://www.rust-lang.org/learn)). Dan:

```
cargo run
```
