# Santa F**ked Up

My submission for [bevyjam4](https://itch.io/jam/bevy-jam-4). This is mostly so I can get a better understanding of Bevy. If I have a game at the end then thats a even bigger win.

## Idea

Santa ripped his present sack on some space junk and now the presents are in an orbit around earth. Launch rockets to collect and bring them back to help save Christmas.

### How to play

Use your mouse to spin the earth around and scroll to zoom in and out. Click the presents to tell the rockets to come collect it. You have 2min and 40 seconds to collect them all. 

## Building

* Desktop
    `cargo run`
* Web
    * `cargo run --target wasm32-unknown-unknown`
* Editor
    * `cargo run --features=editor`

## CI

* Github actions pulled from [bevy_github_ci_template](https://github.com/bevyengine/bevy_github_ci_template)
* Release action will run on new tags