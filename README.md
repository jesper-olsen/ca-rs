ca-rs
==============

One dimensional cellular autometa - the kind classified by Wolfram [1]. 
You can specify the rule to use as well as the initial state.
The CA's are assumed to be constrained to a width of 64 - this creates boundary effects that an infinite CA does not have. 


References
----------
[1] Stephen Wolfram - A New Kind Of Science

Run
-----

```
% cargo run -- -h
Usage: ca-rs [OPTIONS]

Options:
  -r, --rule <R>   rule (0..255) [default: 30]
  -s, --state <S>  input (u64) - use binary (0b), octal (0o), hex (0x) or decimal notation [default: 0x80000000]
  -h, --help       Print help
  -V, --version    Print version
```

```
% cargo run -- --rule 30 
...............................O................................
..............................OOO...............................
.............................OO..O..............................
............................OO.OOOO.............................
...........................OO..O...O............................
..........................OO.OOOO.OOO...........................
.........................OO..O....O..O..........................
........................OO.OOOO..OOOOOO.........................
.......................OO..O...OOO.....O........................
......................OO.OOOO.OO..O...OOO.......................
.....................OO..O....O.OOOO.OO..O......................
....................OO.OOOO..OO.O....O.OOOO.....................
...................OO..O...OOO..OO..OO.O...O....................
..................OO.OOOO.OO..OOO.OOO..OO.OOO...................
.................OO..O....O.OOO...O..OOO..O..O..................
................OO.OOOO..OO.O..O.OOOOO..OOOOOOO.................
...............OO..O...OOO..OOOO.O....OOO......O................
..............OO.OOOO.OO..OOO....OO..OO..O....OOO...............
.............OO..O....O.OOO..O..OO.OOO.OOOO..OO..O..............
............OO.OOOO..OO.O..OOOOOO..O...O...OOO.OOOO.............
...........OO..O...OOO..OOOO.....OOOO.OOO.OO...O...O............
..........OO.OOOO.OO..OOO...O...OO....O...O.O.OOO.OOO...........
.........OO..O....O.OOO..O.OOO.OO.O..OOO.OO.O.O...O..O..........
........OO.OOOO..OO.O..OOO.O...O..OOOO...O..O.OO.OOOOOO.........
.......OO..O...OOO..OOOO...OO.OOOOO...O.OOOOO.O..O.....O........
......OO.OOOO.OO..OOO...O.OO..O....O.OO.O.....OOOOO...OOO.......
.....OO..O....O.OOO..O.OO.O.OOOO..OO.O..OO...OO....O.OO..O......
....OO.OOOO..OO.O..OOO.O..O.O...OOO..OOOO.O.OO.O..OO.O.OOOO.....
...OO..O...OOO..OOOO...OOOO.OO.OO..OOO....O.O..OOOO..O.O...O....
..OO.OOOO.OO..OOO...O.OO....O..O.OOO..O..OO.OOOO...OOO.OO.OOO...
.OO..O....O.OOO..O.OO.O.O..OOOOO.O..OOOOOO..O...O.OO...O..O..O..
OO.OOOO..OO.O..OOO.O..O.OOOO.....OOOO.....OOOO.OO.O.O.OOOOOOOOO.
```

```
% cargo run -- --rule 90
...............................O................................
..............................O.O...............................
.............................O...O..............................
............................O.O.O.O.............................
...........................O.......O............................
..........................O.O.....O.O...........................
.........................O...O...O...O..........................
........................O.O.O.O.O.O.O.O.........................
.......................O...............O........................
......................O.O.............O.O.......................
.....................O...O...........O...O......................
....................O.O.O.O.........O.O.O.O.....................
...................O.......O.......O.......O....................
..................O.O.....O.O.....O.O.....O.O...................
.................O...O...O...O...O...O...O...O..................
................O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.................
...............O...............................O................
..............O.O.............................O.O...............
.............O...O...........................O...O..............
............O.O.O.O.........................O.O.O.O.............
...........O.......O.......................O.......O............
..........O.O.....O.O.....................O.O.....O.O...........
.........O...O...O...O...................O...O...O...O..........
........O.O.O.O.O.O.O.O.................O.O.O.O.O.O.O.O.........
.......O...............O...............O...............O........
......O.O.............O.O.............O.O.............O.O.......
.....O...O...........O...O...........O...O...........O...O......
....O.O.O.O.........O.O.O.O.........O.O.O.O.........O.O.O.O.....
...O.......O.......O.......O.......O.......O.......O.......O....
..O.O.....O.O.....O.O.....O.O.....O.O.....O.O.....O.O.....O.O...
.O...O...O...O...O...O...O...O...O...O...O...O...O...O...O...O..
O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.O.
```

```
% cargo run -- --rule 90 --state 0b111111000000000000000000000000000
...........................OOOOOO...............................
..........................OO....OO..............................
.........................OOOO..OOOO.............................
........................OO..OOOO..OO............................
.......................OOOOOO..OOOOOO...........................
......................OO....OOOO....OO..........................
.....................OOOO..OO..OO..OOOO.........................
....................OO..OOOOOOOOOOOO..OO........................
...................OOOOOO..........OOOOOO.......................
..................OO....OO........OO....OO......................
.................OOOO..OOOO......OOOO..OOOO.....................
................OO..OOOO..OO....OO..OOOO..OO....................
...............OOOOOO..OOOOOO..OOOOOO..OOOOOO...................
..............OO....OOOO....OOOO....OOOO....OO..................
.............OOOO..OO..OO..OO..OO..OO..OO..OOOO.................
............OO..OOOOOOOOOOOOOOOOOOOOOOOOOOOO..OO................
...........OOOOOO..........................OOOOOO...............
..........OO....OO........................OO....OO..............
.........OOOO..OOOO......................OOOO..OOOO.............
........OO..OOOO..OO....................OO..OOOO..OO............
.......OOOOOO..OOOOOO..................OOOOOO..OOOOOO...........
......OO....OOOO....OO................OO....OOOO....OO..........
.....OOOO..OO..OO..OOOO..............OOOO..OO..OO..OOOO.........
....OO..OOOOOOOOOOOO..OO............OO..OOOOOOOOOOOO..OO........
...OOOOOO..........OOOOOO..........OOOOOO..........OOOOOO.......
..OO....OO........OO....OO........OO....OO........OO....OO......
.OOOO..OOOO......OOOO..OOOO......OOOO..OOOO......OOOO..OOOO.....
OO..OOOO..OO....OO..OOOO..OO....OO..OOOO..OO....OO..OOOO..OO....
OOOOO..OOOOOO..OOOOOO..OOOOOO..OOOOOO..OOOOOO..OOOOOO..OOOOOO...
O...OOOO....OOOO....OOOO....OOOO....OOOO....OOOO....OOOO....OO..
.O.OO..OO..OO..OO..OO..OO..OO..OO..OO..OO..OO..OO..OO..OO..OOOO.
O..OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO..OO
```

```
% cargo run -- --rule 110
...............................O................................
..............................OO................................
.............................OOO................................
............................OO.O................................
...........................OOOOO................................
..........................OO...O................................
.........................OOO..OO................................
........................OO.O.OOO................................
.......................OOOOOOO.O................................
......................OO.....OOO................................
.....................OOO....OO.O................................
....................OO.O...OOOOO................................
...................OOOOO..OO...O................................
..................OO...O.OOO..OO................................
.................OOO..OOOO.O.OOO................................
................OO.O.OO..OOOOO.O................................
...............OOOOOOOO.OO...OOO................................
..............OO......OOOO..OO.O................................
.............OOO.....OO..O.OOOOO................................
............OO.O....OOO.OOOO...O................................
...........OOOOO...OO.OOO..O..OO................................
..........OO...O..OOOOO.O.OO.OOO................................
.........OOO..OO.OO...OOOOOOOO.O................................
........OO.O.OOOOOO..OO......OOO................................
.......OOOOOOO....O.OOO.....OO.O................................
......OO.....O...OOOO.O....OOOOO................................
.....OOO....OO..OO..OOO...OO...O................................
....OO.O...OOO.OOO.OO.O..OOO..OO................................
...OOOOO..OO.OOO.OOOOOO.OO.O.OOO................................
..OO...O.OOOOO.OOO....OOOOOOOO.O................................
.OOO..OOOO...OOO.O...OO......OOO................................
OO.O.OO..O..OO.OOO..OOO.....OO.O................................
```

