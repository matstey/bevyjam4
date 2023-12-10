# Santa F**ked Up

My submission for [bevyjam4](https://itch.io/jam/bevy-jam-4)

## Idea

Santa ripped his present sack on some space junk and now the presents are in an orbit around earth. Launch rockets to collect and bring them back to help save Christmas.

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