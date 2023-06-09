# Changelog

## 2023-04-22

- [Nicer character movement](devblog/2023-04-22.md#nicer-movement)
- [Little changes](devblog/2023-04-22.md#little-improvements)
- Switched to the latest [Rust 1.69](https://blog.rust-lang.org/2023/04/20/Rust-1.69.0.html)

## 2023-04-21

- [I changed to event-based control](devblog/2023-04-21.md#event-based-control)
- [Potential mobile web play](devblog/2023-04-21.md#mobile-play)
- [Moving tile by tile](devblog/2023-04-21.md#moving-tile-by-tile)

## 2023-04-20

- [New level map data structure](#new-level-map-datastructure)
- [A bit more complex map](#a-slight-map-change)
- First unit tests - for the map data
- Reduced the size of the labyrinth
- Fixed the movements (still temporary fix)
- Put the exit to random places

### New level map data structure

This release doesn't contain many gameplay improvements, but I changed how the level
map works, which gives me much more flexibility.

The original level map was a two-dimensional array.
It had multiple issues:
- I had to reserve the memory upfront for the potential most extensive map.
- The non-map tiles were also in the memory
- The math shouldn't be too complicated, but I had to use offsets a lot.

The new data structure:
```rust
type TheMap = HashMap<i32, HashMap<i32, Vec<Elements>>>;
```
That might look worse than arrays, but I can easily access random points.
The storage only depends on the number of used coordinates.

### A slight map change

The better map data allow me to build an outskirts labyrinth as well.
You won't know if you are next to the furthest wall or not.
Ok, it is a bit evil, but hey, I want to make it challenging.

## 2023-04-19

- [Next.js or not Next.js](#the-rabbit-hole-of-nextjs)
- [Introducing component hierarchy](#component-hierarchy)
- Adding root level Makefile and putting the rust code into the `labyrinth/` subfolder
- [Laying down the foundation for bigger maps](#bigger-and-better-map)

Lastly, I have a more or less complete plan for how I want to build this game.
But no spoilers ;)

### The rabbit hole of Next.js

Next.js is good; I love it. So I thought part of the refactoring I use Next.js to generate the page.
Well, in theory, that should work.
But because I used WASM the first time, I wasn't able to make it work smoothly.
And let's be honest, using Next.js to render the following would be slightly overkill.

```html
<html>
  <body>
    <h1>Maze Game</h1>
    <canvas id="labyrinth" width="720" height="720"></canvas>
  </body>
  <script type="module">
    import init from './wasm32/labyrinth-game.js'
    init()
  </script>
</html>
```

So after a few hours of fight, I decided to give up and add some styling to the HTML.
Eventually, I'll turn this changelog into a changelog blog so that I might come back to it.

### Component hierarchy

I had a minor issue. The main character, the sprite, isn't centred.
After 5 mins of googling, I found a StackOverflow answer: I have to put the sprite into a parent and offset that.
The pseudo-code of the original:

```rust
commands.spawn((
    SpriteBundle {
        translation: Translation {x, y, 0}
    },
    Player {}
))
```
The pseudo-code of the proposed solution:

```rust
commands.spawn((
    Player { }
)).add_children(|parent| {
    parent.spawn(SpriteBundle {
        translation: Translation {x, y, 0}
    })
})
```
But it didn't show up.

I realised what was missing,
the [SpatialBundle](https://docs.rs/bevy/0.9.1/bevy/render/prelude/struct.SpatialBundle.html).

```rust
commands.spawn((
    SpatialBundle {
        translation: Translation {x, y, 0},
        ..default()
    }
    Player { }
)).add_children(|parent| {
    parent.spawn(SpriteBundle {
        translation: Translation {offset_x, offsey_y, 0},
        ..default()
    })
})
```

And it works now.

### Bigger and better map

I've introduced an abstract `LevelMap` struct.
That should hide away the internal array.
I also plan to offset the map, so the (0, 0) will be the centre,
not the bottom left corner.
I also started to work towards dynamic-sized map generation.
These "halfway-through" code changes will enable me to make smaller or larger maps and build outer skirts.

## 2023-04-18

- Collision checker is way better (it turns out I don't need to rewrite it, use it right)
- Zoom in; that makes it more difficult ;)

## 2023-04-17

- Build it for WASM
- Implement simple collider

### WASM
Yay, it's exciting. *It was just too simple ;)*

### Collider and keyboard movements
Still not great, but better.
I probably need to write a better collider which can tell the list of sides colliding.
I want to slide next to a wall even if I press against the wall.

## 2023-04-16 - Initial version

- Basic labyrinth generation
- Simple movement

The movement isn't great, but it works.
I use the .just_released() method so the sprite isn't moving too fast.


