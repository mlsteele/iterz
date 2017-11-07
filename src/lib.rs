#![feature(conservative_impl_trait)]

pub struct StateIter<S,I,F>
    where F: FnMut(S) -> (Option<S>,I)
{
    // None if completed.
    state: Option<S>,
    transition: F,
}

impl<S,I,F> StateIter<S,I,F>
    where F: FnMut(S) -> (Option<S>,I) + Sized
{
    fn new(initial: S, transition: F) -> Self {
        Self{
            state: Some(initial),
            transition: transition,
        }
    }
}

impl<S,I,F> Iterator for StateIter<S,I,F>
    where F: FnMut(S) -> (Option<S>,I)
{
    type Item = I;

    fn next(&mut self) -> Option<I> {
        let state = self.state.take();
        match state {
            None => None,
            Some(state) => {
                let (state2, item) = (self.transition)(state);
                self.state = state2;
                Some(item)
            }
        }
    }
}

pub mod examples {
    use super::*;
    use std::collections::VecDeque;

    pub fn new_infinite_counter() -> impl Iterator<Item=i32> {
        StateIter::new(0, |x| {
            let y = x + 1;
            (Some(y), x)
        })
    }

    pub fn new_finite_counter(last: i32) -> impl Iterator<Item=i32> {
        StateIter::new((0, last), |(x, last)| {
            if x >= last {
                (None, x)
            } else {
                (Some((x+1, last)), x)
            }
        })
    }

    pub fn new_marquee<T>(start: VecDeque<T>) -> impl Iterator<Item=VecDeque<T>>
        where T: Clone,
    {
        StateIter::new(start, |mut xs| {
            let ys = xs.clone();
            if let Some(x) = xs.pop_back() {
                xs.push_front(x);
            }
            (Some(xs), ys)
        })
    }
}

#[cfg(test)]
mod tests {
    use examples::*;

    #[test]
    fn infinite_counter() {
        let iter = new_infinite_counter();
        let v: Vec<_> = iter.take(5).collect();
        assert_eq!(v, vec![0,1,2,3,4]);
    }

    #[test]
    fn finite_counter() {
        let iter = new_finite_counter(3);
        let v: Vec<_> = iter.collect();
        assert_eq!(v, vec![0,1,2,3]);
    }

    #[test]
    fn marquee() {
        let iter = new_marquee(vec![0,1,2].into());
        let v: Vec<_> = iter.take(4).collect();
        assert_eq!(v[0], vec![0,1,2]);
        assert_eq!(v[1], vec![2,0,1]);
        assert_eq!(v[2], vec![1,2,0]);
        assert_eq!(v[3], vec![0,1,2]);
    }

}
