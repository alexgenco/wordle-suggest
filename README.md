# `wordle-suggest`

![Crates.io](https://img.shields.io/crates/d/wordle-suggest?style=for-the-badge)

## Usage

Run without arguments to get a list of good starter words:

```shell
$ wordle-suggest
carey
casey
money
salem
marie
saver
karen
saved
safer
panel
```

Enter one into the puzzle and write the result to a file using the following
syntax:

| Syntax | Meaning                                   |
| ------ | ----------------------------------------- |
| `a^`   | `a` is in the correct position (green)    |
| `b?`   | `b` is in the incorrect position (yellow) |
| `c`    | `c` is not in the word (gray)             |

For example, this result:

<img width="337" alt="wordle-result" src="https://user-images.githubusercontent.com/566993/151033991-a088eb62-5515-4ca4-bcb1-b83bd3f48f10.png">

Is represented like this:

```
s^cr?a^p
```

Assume this file is called `hints.txt`. Re-run `wordle-suggest` and point it
to this file to get a new set of suggestions:

```shell
$ wordle-suggest -f ./hints.txt
solar
sugar
# ...
```

Continue adding results to the hints file and re-running `wordle-suggest`
until you've solved the puzzle!

See `wordle-suggest -h` for more usage options.

## Implementation

The words are stored in a priority queue (a `BinaryHeap` specifically), based
on the frequency of each letter in its specific position compared to the entire
word list ([`words/all.txt`](words/all.txt)), as well as its presence in a list
of common words ([`words/common.txt`](words/common.txt)). So the returned words
are sorted by their probability of having their letters in the correct
positions, with common words appearing first.
