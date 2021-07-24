
# Maze generator - A rust crate to generate mazes

## Overview
This crate provide functions to generate mazes with several algorithms, in an efficient way. Once a maze is generated, you can print it with walls as lines or as blocks (hey Minecrafters) or export it as jpg.

## Public API
```Rust
pub fn generate(w: usize, h: usize, generator: Generator) -> Maze;
pub fn print_linewise(&self);
pub fn print_blockwise(&self, free_char: char, wall_char: char);
pub fn export(&self, filename: &str, w: u32, h: u32, walls_width: u32);
```

## Compilation & Installation

Compile from source:
```
git clone https://github.com/lfalkau/maze-generator
cd maze-generator
cargo build
```
Import from crates.io
```
NOT YET EXPORTED
```

## Generation algorithms

 - [x] [Recursive backtracker](https://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking)
 - [ ] [Eller’s algorithm](https://weblog.jamisbuck.org/2010/12/29/maze-generation-eller-s-algorithm)
 - [ ]  [Kruskal’s algorithm](https://weblog.jamisbuck.org/2011/1/3/maze-generation-kruskal-s-algorithm)
 - [ ] [Prim’s algorithm](https://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm)
 - [ ] [Recursive division algorithm](https://weblog.jamisbuck.org/2011/1/12/maze-generation-recursive-division-algorithm)
 - [ ] [Aldous-Broder algorithm](https://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm)
 - [ ] [Wilson’s algorithm](https://weblog.jamisbuck.org/2011/1/20/maze-generation-wilson-s-algorithm)
 - [ ] [Hunt-and-Kill algorithm](https://weblog.jamisbuck.org/2011/1/24/maze-generation-hunt-and-kill-algorithm)
 - [ ] [Growing Tree algorithm](https://weblog.jamisbuck.org/2011/1/27/maze-generation-growing-tree-algorithm)
 - [ ] [Binary Tree algorithm](https://weblog.jamisbuck.org/2011/2/1/maze-generation-binary-tree-algorithm)
 - [ ] [Sidewinder algorithm](https://weblog.jamisbuck.org/2011/2/3/maze-generation-sidewinder-algorithm)

## Ressources

[Mazes for programmers](https://weblog.jamisbuck.org/2011/2/7/maze-generation-algorithm-recap)
