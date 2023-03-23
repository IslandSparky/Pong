#Readme.md

Pong is not really a pong game.  But it is a bouncing ball variant that was developed as a Rust learning exercise, specifically to deal with structures and vectors containing structures. 

In the window there are four types of "balls" (actually they are squares):

Targets (Black) just move along and are attacked by Seekers and ,occasionally Cowards, and are affected by Stinkers.

Seekers (Red) each choose a Target and follow it, trying to intercept it. They are much slower than the Targets, but are smarter. 

Cowards (Yellow, of course) also choose a Target and follow it, but chicken out at the last moment of the interception. Except, they will attack a Target if it is paralysed under the influence of a stinker.

Stinkers(CYAN) move at their own pace and not influenced by other balls (except with a direct collision with another Stinker).  But they influence other ball types.  Other ball types try of avoid them, but if caught within the aura of a Stinker they become temporarily paralyzed under the influence. Stinkers do not influence other stinkers except by direct collisions.

#notes

Like other SDL2 applications under Windows, SDL2.dll must be located in the application directory or a suitable library path. 
