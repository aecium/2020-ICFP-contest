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
```
cargo run --bin grapher ascii 1 0 2 0 3 0 4 0 5 0  0 1 6 1  0 2 1 2 2 2 3 2 4 2 5 2 6 2
.#####.
#.....#
#######
```

## Graph Points to Image File
```
cargo run --bin grapher graph.png 1 0 2 0 3 0 4 0 5 0  0 1 6 1  0 2 1 2 2 2 3 2 4 2 5 2 6 2
```