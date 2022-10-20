# AoC Oracle

It can answer any Advent of Code problem[^1] in an instant[^2]:


## Try it
Try it out by going to [apljungquist.github.io/aocoracle](https://apljungquist.github.io/aocoracle/), pasting an input of your choice and hitting the button.

![screenshot](screenshot.png)

Or install from crates.io and use e.g. like

```bash
echo '16,1,2,0,4,2,7,1,2,19\n' | aocoracle --part=1
42
```

[^1]: As long as it is from the first week of the 2021 event.
[^2]: Hopefully in less than 7.5M years, the goal is to keep the duration below 100ms for "official" inputs.
