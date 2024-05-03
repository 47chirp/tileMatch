# tileMatch

The premise of this game is to recreate the feeling of the childhood arcade game Stacker.

The mission of the game was recreate the feeling of the grid like system, which led us to break the game board down into a board made up of squares of the same size. This was one of the easier components of the game to create. After trial and error with some differing grid sizes, a finalized version of the grid background was formed. From here the rest of the game logic was created.

Similarly to the game Stacker itself, there exists a slow jump in movement speed for each round with an additional level system that determines the width of the blocks themselves.

The first level features a moving 3x1 block, the second a 2x1 block, and finally a 1x1 block.

There is a constant speed attached to the blocks movement that is multiplied by a scalar when each block is dropped. This creates an increasingly faster movement speed.

To make the game feel more realistic, hitting the walls additionally leds to a bouncing mechanic.

This was set up with the intention of letting users get a better feel for the pace of the game so that they can take their time in placing the blocks.

Some of the biggest struggles with the game were ways in which to have the blocks interact with one another.

The decided upon solution was to shift the blocks upwards by manipulating the respective y values of the blocks, which were divided up into frozen and active blocks.

Given more time, additional features such as more validity checks and potentially shaved blocks depedent on placement could have been used but the foundation of the game is structured to be built upon further.