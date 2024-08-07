## Example project using Rust sdl2


## Setup
To compile, put lib files below into rustup installation folder or any directory which rustc can refer to. 

The full path will be like this: `.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib` (usually in the rustup installation folder)

- `SDL2.lib`
- `SDL2main.lib`
- `SDL2test.lib`
- `SDL2_image.lib`
- `SDL2_mixer.lib`


To run, put several dll files to the project directory.
- `SDL2.dll`
- `SDL2_image.dll` 
- `SDL2_mixer.dll`

## Usage
To run, execute
```cargo run```


## Making sprite
I used **Aseprite** to generate sprite sheet and corresponding JSON.


To export sprite sheet, use `File->Export->Export sprite sheet`, in the `output` tab check export both `Output File` and `JSON Data`.