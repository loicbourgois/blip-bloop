# Blip Bloop

In a small universe repeating on all directions

Things are floating, stuff can be too

There lives one special `Bloop`

They call it `Blip`

`↑`, `Z`, `W`

`←`, `Q`, `A`

`→`, `D`

##

```bash
# in .zshrc
bb () {
  cargo +nightly run --quiet --manifest-path $HOME/github.com/loicbourgois/blip-bloop/blip-bloop-cli/Cargo.toml -- $*
}
# in a terminal
bb help
```
