# Iterz

Playing around with rust iterators. This is a function that takes an initial state and a transfer function and returns an iterator.

```rust
fn new(initial: S, transition: F) -> StateIter<S,I,F>
	where F: FnMut(S) -> (Option<S>,I)
```
