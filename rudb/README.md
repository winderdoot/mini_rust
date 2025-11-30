Rudb is a hastily written, bare bones, in-memory SQL like database without foreign keys and relation/constraint support.  
However it's written in rust, which automatically makes it 10x better than most modern ORM implementations.

![Hell yeah](data/ferris-rust.gif)

## Project structure

Rudb is split into two modules:
- **cli** contains logic related to parsing sql commands written in the terminal
- **core** contains the database implementation, based on the ```BTreeMap``` from std

Command parsing is done by hand, which is one of the main reasons this project is so underdeveloped and took me so long.
I was trying to do this quickly and didn't expect that using a library like pest would save me much time.
I swear I will never write a parser like this by hand ever again, it was a waste of time. And it isn't even that good either, 
as each command needs to be in a single line and string literals can't have spaces.

## Implemented modules

None as I didn't have time and I need to move on to other projects 😢

## Favourite module

My favourite module was studying for TAJF
