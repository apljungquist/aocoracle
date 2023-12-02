# AoC Oracle

It can answer any Advent of Code problem[^1] in an instant[^2]:


## Try it
Try it out by going to [apljungquist.github.io/aocoracle](https://apljungquist.github.io/aocoracle/), pasting an input of your choice and hitting the button.

Or install from crates.io and use e.g. like

```bash
echo '16,1,2,0,4,2,7,1,2,19\n' | aocoracle --part=1
42
```

[^1]: As long as it is from the first week of the 2021 event.
[^2]: Hopefully in less than 7.5M years, the goal is to keep the duration below 100ms for "official" inputs.


## Contribute

For Linux like environments the ambition is that setting up a development environment should be as easy as

```bash
source ./init_env.sh
make install_deps_py
```

Important workflows are documented in the [Makefile](./Makefile) and can be listed with `make help`.

### Prerequisites

- Rust e.g. by following these [instructions](https://www.rust-lang.org/tools/install)
- Python e.g. by
  1. installing pyenv following these [instructions](https://github.com/pyenv/pyenv#installation), and
  2. installing the targeted python version(s) like `cat .python-version | xargs -n 1 pyenv install`

