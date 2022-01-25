# `wordle-suggest`

## Usage

Run without arguments to get a list of good starter words:

```shell
$ wordle-suggest
sanes
sales
sores
cares
bares
sates
tares
pares
sones
seres
```

Enter one into the puzzle and write the result to a file using the following
syntax:

- `^x`: `x` is in the correct position (green)
- `?x`: `x` is in the incorrect position (yellow)
- `x`: `x` is not in the word (none)

For example, this result:

<img width="337" alt="Screen Shot 2022-01-25 at 9 37 46 AM" src="https://user-images.githubusercontent.com/566993/151033991-a088eb62-5515-4ca4-bcb1-b83bd3f48f10.png">

Is represented like this:

```
^sc?r^ap
```

Assume this file is called `attempts.txt`. Re-run `wordle-suggest` and point it
to this file to get a new set of suggestions:

```shell
$ wordle-suggest -f ./attempts.txt
salar
sonar
solar
safar
sowar
sofar
sitar
segar
simar
sekar
```

Continue adding results to the attempts file and re-running `wordle-suggest`
until you've solved the puzzle!

See `wordle-suggest -h` for more usage options.

## Implementation

The words are stored in a priority queue (a `BinaryHeap` specifically), based
on the frequency of each letter in its specific position compared to the entire
word list ([`words.txt`](/words.txt)). So the returned words are sorted by
their probability of having their letters in the correct positions.
