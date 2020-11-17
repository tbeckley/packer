# Hello!
Welcome to a bin packing library for rust. I wrote this because I couldn't find one that used anything other than NF. Maybe I just missed an obvious one. Who knows. I probably just missed an obvious one.

#### This library is not stable and ~~will~~ may have breaking changes. Here's what is going to change if I ever do it:
1. The signature of the packing functions (especially online_nf) will go from taking Vec<T> to U<T>, where U: Iter. Since this is meant to be an online lazy-load function, I'd like to allow nay iterator.
2. I'll figure out why Modified FFD doesn't outperform FFD in any test I could think up. If you know, please PR this. It's driving me insane.

#### Also to do (non breaking):
3. Write an "exact" algorithm ([see here](https://en.wikipedia.org/wiki/Bin_packing_problem#Exact_algorithm)). This is way over my head. Again PR welcome.
4. Write more tests. I wrote some basic ones but this is far from sound.

### Currently implemented packing functions ([see here](https://en.wikipedia.org/wiki/Bin_packing_problem#Exact_algorithm))
1. Next Fit (NF). This should be online but isn't currently (see note 1)
2. First Fit Decreasing: Probably the go-to for most situations.
3. Modified First Fit Decreasing: This SHOULD outperform FFD in most cases with large data but in practice doesn't. If you find a case that benefits from it, let me know!

