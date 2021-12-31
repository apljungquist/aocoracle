# AoC Oracle

It can answer any Advent of Code problem[^1] in an instant[^2]:

```bash
echo '16,1,2,0,4,2,7,1,2,19\n' | aocoracle --part=1 2>/dev/null
42
```

[^1]: As long as it is from the first week of the 2021 event.
[^2]: Hopefully in less than 7.5M years, the goal is to keep the duration below 100ms for "official" inputs.
