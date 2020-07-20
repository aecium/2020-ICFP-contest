# Free Tacos - ICFP Contest 2020

## Manifesto

Set the tacos free!

## Run the galaxy.txt Parser
```
cargo run --bin runner galaxy.txt ':1043'
```
or
```
cargo run --bin runner galaxy.txt 'galaxy'
```

## Graph Points to Console
cargo run --bin grapher <point-order:x|y> <ENHANCE!> <dest:ascii|image.png> <x|y> <y|x> [...<x|y> <y|x>]
```
cargo run --bin grapher x 2 ascii 1 0 2 0 3 0 4 0 5 0  0 1 6 1  0 2 1 2 2 2 3 2 4 2 5 2 6 2
..##########..
..##########..
##..........##
##..........##
##############
##############

cargo run --bin grapher y 1 ascii 1 0 2 0 3 0 4 0 5 0  0 1 6 1  0 2 1 2 2 2 3 2 4 2 5 2 6 2
.##
#.#
#.#
#.#
#.#
#.#
.##
```

## Graph Points to Image File
```
cargo run --bin grapher x 2 graph.png 1 0 2 0 3 0 4 0 5 0  0 1 6 1  0 2 1 2 2 2 3 2 4 2 5 2 6 2
```