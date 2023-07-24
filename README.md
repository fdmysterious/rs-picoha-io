Rust rp-pico template
=====================

- Florian Dupeyron
- June 2023

This minimal template is inspired by https://github.com/rp-rs/rp2040-project-template.

The following elements are added:

- dockerized environment for easy installation
- just is used for various shortcuts commands

# Dependencies

- docker
- just (`cargo install just`)

# Building

Just as simple as:

```c
just build
```

# TODO

- [ ] Debugging using `picoprobe`
- [ ] Add option to `build` target to choose build type (release or debug)
- [ ] Various improvements to the `just` file
- [ ] Add vscode config files using devcontainer, _etc._
- [x] Separate the application and platform initialization in two separate crates (using the `member` configuration field of the `Cargo.toml` file, see https://github.com/emilk/egui/blob/master/Cargo.toml for example)
