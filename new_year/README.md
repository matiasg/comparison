This was a sequel of [this tweet](https://twitter.com/fermatslibrary/status/1476946554930012160)

In short, it states that
(10 + 9) * 8 * (7 + 6) + (5 + 4) * (3 + 2) + 1 = 2022
(it was tweeted on Dec 31, 2021)

1. The exercise consists on trying all different ways for associating numbers
from 10 to 1 and all different ways to put `+`, `*` and `-` between them
and writing how many positive numbers are obtained this way.

2. Generalization: do that for any N, not only for N = 10.

3. Nice to have, write different ways to obtain the following years,
let's say from 2023 to 2030, if they can be obtained.

The brute-force algorithm is to get
- the list of all ways of associating N numbers.
  This is one of the ways in which Catalan numbers show up
  (see [Wikipedia's Catalan number article](https://en.wikipedia.org/wiki/Catalan_number)).
  The number of ways is `C(2(N-1), N-1) / N`, where `C` stands for combinatorial number
  (notice that `N` here is `n+1` in Wikipedia). For `N=10`, the original problem, this is
  4862 ways of doing it.
- the list of all ways of writing the three operators between the N numbers.
  This is `3^(N-1)`, which in the original problem is 19,683.

Therefore, the number of combinations to try is
```
3^(N-1) * C(2(N-1), N-1) / N
```
Assymptotically, this grows as `O(3^N * 4^N / N^(3/2)) = O(12^N / N^(3/2))`.
For the original problem `N=10`, this is 95,698,746 combinations.

One might do better by not considering some combinations that are known in
advance to give the same results as others already explored. But for the
purposes of this repository, I'll keep it simple and stick to the brute-force
way.
