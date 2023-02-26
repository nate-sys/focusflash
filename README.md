# Focusflash
[Flashfocus](https://github.com/fennerm/flashfocus), but in Rust.

![Demo](assets/demo.mp4.mov)

## Building from Source
```sh
cargo install --git https://github.com/nate-sys/focusflash
```

## How it works
`focusflash` is a program only does 2 things:
    - Gets the active/focused Xorg window 
    - Overlays a new, blank window on top of it for 100ms

In order to achieve the intended functionality you see in the demo gif above, you must set it up to be invoked when you switch windows. 

Here are some examples of how to setup focusflash

### SXHKD
``` sh
# in sxhkdrc

super + {_,shift + }{h,j,k,l}
	bspc node -{f,s} {west,south,north,east} && focusflash

# ...
```
### Other setups
`todo!("provide examples")`


## Why?
- Flashfocus didn't work on my system
- Wanted to write something similar in rust anyway
- Using it to learn X11

## Disclaimer
I am __not__ an x11 expert.
The code is 80% copied and slightly modified code from the internet.
It may or may not work on your system, though I don't see any reason why it shouldn't.

