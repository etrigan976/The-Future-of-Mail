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
Escape Key - return to main menu / pause

## Testing reasoning

For testing the game, i couldnt really do any kind of automatic testing due to not necessarily having experience with the engine in terms of implementing a #[test] attribute for the main aspects. i mainly tested it by going through and making sure every new feature or fix worked when going through the game cycle. so i can assure it has been tested, just manually.

## Developer Feedback

### What worked?

I think the concept worked. Mail right now is very much dependent on automatically controlled robots and this game resembles the concept pretty well. i think also the geometry designs of the people, city, buildings and robot worked well, except for the fact i had to speed learn blender for textures. this rush definitely shows in a view of models.

### What didn't work?

i couldnt fit in any special mechanics in the game because time would not allow it unfortunately. i was gonna have some extra obstacles like a person model that would come around the course and cause the player to lose but i couldnt determine how to have a kind of AI for it for pathfinding. i also didn't have enough to make music for the game, even though i have never used a audio production application before.

### Satisfaction

I am pretty satisfied with the result at least given how much time i had. I think it turned out fine as a prototype game showcasing the engine.

### Future Improvement

I would like to improve the composition of the textures because i was not anticipating how raw the texture painting is for blender, so it got sloppy. Additionally, i would like to have it where for the people models to have a bunch of different textures so that i can have an array of paths and have the game randomly select the textures to use.

## Tools Used to make this

### Modeling: Blender

### Audio: Audacity

### Engine: Bevy

### Programming Language: Rust

## Licensing

[LICENSE](LICENSE)
