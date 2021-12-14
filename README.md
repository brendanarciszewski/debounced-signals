# Bounced
[![Crates.io Latest Version][crates-io-shield]][crates-io]
![License](https://img.shields.io/crates/l/bounced)
[![codecov](https://codecov.io/gh/brendanarciszewski/debounced-signals/branch/main/graph/badge.svg)](https://codecov.io/gh/brendanarciszewski/debounced-signals)
[![coveralls](https://coveralls.io/repos/github/brendanarciszewski/debounced-signals/badge.svg)](https://coveralls.io/github/brendanarciszewski/debounced-signals)

A utility to debounce signals.

It has no dependencies (except for libcore) and no `unsafe`.

Debouncing is best thought of as a running average. It might also be thought of
as a hysteresis of an input: if the input changes, it needs to head towards the
new state consistently and for long enough.

Contains integration-type and shift-type (also with a specialization for
integration-type) debouncers.

Compared to [other](#other-projects) debouncer libraries, it is extensible with
your own debouncing algorithm.

## TODO
- random-noise type (for testing)

## Other Uses
This library is probably generic enough so that if you have some input that upon
meeting some desired history, the output could be some tri-state value (on, off,
and `None`).

## Other Projects
Projects similar to this include [`debouncr`] and [`debounced-pin`].

Other inspirations include [`debounce.c`].

[`debouncr`]: https://crates.io/crates/debouncr
[`debounced-pin`]: https://crates.io/crates/debounced-pin
[`debounce.c`]: http://www.kennethkuhn.com/electronics/debounce.c
[crates-io-shield]: https://img.shields.io/crates/v/bounced?cacheSeconds=86400
[crates-io]: https://crates.io/crates/bounced
