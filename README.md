# Isometric Minecraft
minecraft but 2d or 3d? both?
a very early prototype even more than early alpha versions of minecraft
[icon](https://cdn.discordapp.com/attachments/695874384899866692/1287155653965647882/1BD8BFC2-BE65-45C7-ADB7-D3D776EC4D49.png?ex=66f0849e&is=66ef331e&hm=5a9c79d3ed398c77bc693d4ab8a1eb037f815f8811ee58f45ab9066d65552bc4&)
## Testing 
### WASM
```sh
# pre
rustup target add wasm32-unknown-unknown
cargo install basic-http-server # or any other way of serving
#
cargo build --release --target wasm32-unkown-unkown
basic-http-server . # or any other way of serving

# the template index.html already has a refrence to your outputed .wasm file
# note that for github deploying we dont use this index.html as of now
```
### Native
same as above without the target part, just note that this game for reasons i do not understand (bugs i am too lazy to fix) is not able to run in debug mode and should only be ran in optimized --release mode this means that debugging is not really possible :( until i fix the stack overflow that happens in non optimized builds.
