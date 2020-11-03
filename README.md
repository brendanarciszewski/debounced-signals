# Bounced
A utility to debounce signals.

Debouncing is best thought of as a running average. It might also be thought of
as a hysteresis of an input: if the input changes, it needs to head towards the
new state consistently and for long enough.

Contains an integration-type debouncer.

Compared to [other](#other-projects) debouncer libraries, it is extensible with your own
debouncing algorithm.

## TODO
- shift-type
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
