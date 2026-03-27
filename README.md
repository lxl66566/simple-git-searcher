just a toy & test.

```sh
❯ hyperfine --warmup 3 'rg -F -e "114514" -e "114515" -e "114516" -e "114517" -e "114518" -e "114519"' 'simple-git-searcher 114514 114515 114516 114517 114518 114519' -i
Benchmark 1: rg -F -e "114514" -e "114515" -e "114516" -e "114517" -e "114518" -e "114519"
  Time (mean ± σ):      86.3 ms ±   3.1 ms    [User: 229.5 ms, System: 291.4 ms]
  Range (min … max):    82.2 ms …  93.9 ms    33 runs

  Warning: Ignoring non-zero exit code.

Benchmark 2: simple-git-searcher 114514 114515 114516 114517 114518 114519
  Time (mean ± σ):     115.4 ms ±   6.0 ms    [User: 176.8 ms, System: 249.9 ms]
  Range (min … max):   103.1 ms … 123.3 ms    23 runs

  Warning: Ignoring non-zero exit code.

Summary
  rg -F -e "114514" -e "114515" -e "114516" -e "114517" -e "114518" -e "114519" ran
    1.34 ± 0.08 times faster than simple-git-searcher 114514 114515 114516 114517 114518 114519
```
