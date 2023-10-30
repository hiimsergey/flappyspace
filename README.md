<img src="assets/icon.png" width=20%/>

# Flappy Space
A lil' 2D game about a UFO and asteroids inspired by Flappy Bird.
Written with [Bevy](https://bevyengine.org) in [Rust](https://rust-lang.org).

## Gameplay
Press the spacebar to jump. Dodge the asteroids. That's it. Yippie.

## Setup
Under **Releases**, you can download the binary. As of now, there is only Linux.

If you have `cargo` installed, you can compile the game from source:

```
git clone https://github.com/hiimsergey/flappyspace
cd flappyspace
cargo run --release
```

I used the [bevy_embedded_assets](https://github.com/vleue/bevy_embedded_assets) plugin so that you don't need to download the assets folder alongside the executable.
However, if you want a window icon, you would need to download it. I implemented it through winit and not Bevy, since there is no official implementation for that yet.

If the program doesn't find an icon, it simply runs without it. But I think it's not too tragic.

## Assets
- font: [Pixelify Sans](https://fonts.google.com/specimen/Pixelify+Sans)
- sprites: my own (feel free to use them)
  - inspired by ["ASTEROID" by greenretroman](http://greenretroman.itch.io/asteroids)
- sounds: [Sci-Fi Sounds](https://kenney.nl/assets/sci-fi-sounds) and [Interface Sounds](https://kenney.nl/assets/interface-sounds) by [Kenney.nl](https://kenney.nl)
