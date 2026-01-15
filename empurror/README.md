Empurror is simple a turn based grand strategy game where you control an empire of cats!
Gather resources and manage a feudal economy on your way to global domination (which shouldn't be too hard, because enemy ai is dumb).

## Implemented features:

- All basic functionality, excluding serialization/loading/saving current game state
- **Extra functionality**:
    - **population**
        - The gameplay loop is based managing the growing population by assigning free *pops* (population units) to provinces as workers,
        and expadning your lands to produce the grain necesary to sustain your workers.  
        - Each province can only be controlled by an empire, if it has it's population inhabit the province. 
        - For pops to live in a province, houses need to be built first. 
        - When a province is occupied (taken over) by a foreign empire, some part of province's population dies.
        - Armies are made out of soldiers that have homes in provinces with a castle. 
        - When a province with a castle is taken over by a foreign empire, the soldiers that have their homes in that province abandon thier posts and are removed from armies they fight in.
    - **economy**:
        - Each empire gathers 4 types of resorces (besides population): grain, lumber, stone and gold.
        - Each resource serves a different purpose, and they form a bit of a technology progression (grain -> lumber -> stone -> gold -> castles and armies that lead to conquest).
        - The ai's are programmed to follow this progression and generally keep their economies afloat and manage to defend themselves with armies (which they can support via their economy).
        - There are special buildings: farms, lumber mills, stone mines, gold mines and castles.
        - Each have different costs and functions. The first 4 improve resource extraction. Castles are used to recruit soldiers but also serve as points of command - if they are taken down, the coresponding soldiers flee from thier armies.
    There is a very rudimentary *'debt mechanic'* - when an empire goes in debt it collapses and it's population dies, leaving behind empty buildings.
    **There isn't a market for trading resources**
    -  **3d** - the game is 3d with some placeholder models and a (objectively) cool terrain generation

## Missing things

- I found a good set of models for buildings but they required culling in blender which I didn't have time for,
all resource buildings have the model of a windmill :D
- I didn't implement a proper game over screen because I got so fed up with writing ui in bevy. When the player goes in debt, the game is going to crash on a ```todo!()``` macro.