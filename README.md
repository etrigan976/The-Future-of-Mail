# The-Future-of-Mail

## Author: Ryan Matthews

## Project: The Future of Mail

## Project Description

"The Future of Mail" is a partially 3D game written in the Bevy Game Engine where you play as a mail delivery robot. The goal is to deliver mail from 1 dynamically appearing mail sender to another dynamically appearing mail checkpoint cube. the main environment is a grassy cityscape with buildings and a ring wall around the city. your goal is to obtain as many points from fulfilling deliveries as you can before accidentally crashing into a building or wall, but beware the gaps can be a bit narrow. Its intended function is to be a relatively simple game showcasing what the Bevy engine and the Rust Programming Language can do.

## How to Run

### To Normally Run the Game

cargo run --release

### To Build the Game, then Run it

cargo build --release\
cargo run --release

## Controls

W - move forward\
S - move backward\
A - move left\
D - move right\
Up Arrow - camera angle down\
Down Arrow - camera angle up\
Left Arrow - camera angle right\
Right Arrow - camera angle left\
Escape Key - return to main menu

## Testing reasoning

For testing the game, i couldnt really do any kind of automatic testing due to not necessarily having experience with the engine in terms of implementing a #[test] attribute for the main aspects. i mainly tested it by going through and making sure every new feature or fix worked when going through the game cycle. so i can assure it has been tested, just manually.

## Developer Feedback

### What worked?

### What didn't work?

### Satisfaction

### Future Improvement

## Tools Used to make this

### Modeling: Blender

## Licensing

[LICENSE](LICENSE)
