use std::fmt::Debug;
use std::ops::{Add, Sub};

#[allow(clippy::manual_non_exhaustive)] // to prevent instantiation within this crate
#[derive(Debug)]
pub struct Rectangle<T: Debug> {
    pub left: T,
    pub top: T,
    // TODO: Consider constraining width and height to non-negative types
    // Since constructors are only implemented for unsigned types this is
    // not yet a problem.
    pub width: T,
    pub height: T,
    _prevent_instantiation: (),
}

impl<T: Add<Output = T> + Copy + Debug + Ord + Sub<Output = T>> Rectangle<T> {
    fn right(&self) -> T {
        self.left + self.width
    }

    fn bottom(&self) -> T {
        self.top + self.height
    }

    pub fn intersection(&self, other: &Rectangle<T>) -> Option<Self> {
        let top = self.top.max(other.top);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());
        let left = self.left.max(other.left);
        if right <= left || bottom <= top {
            return None;
        }
        Some(Self {
            left,
            top,
            width: right - left,
            height: bottom - top,
            _prevent_instantiation: (),
        })
    }
}

macro_rules! unsigned_integer_rectangle {
    ($($a:ty)*) => ($(
        impl Rectangle<$a> {
            pub fn new(left: $a, top: $a, width: $a, height: $a) -> Self {
                Self {
                    left,
                    top,
                    width,
                    height,
                    _prevent_instantiation: (),
                }
            }

            fn area(&self) -> $a {
                self.width * self.height
            }

            pub fn tiles(&self) -> impl Iterator<Item = ($a, $a)> + '_ {
                (0..self.area()).map(|i| (self.left + i % self.width, self.top + i / self.width))
            }
        }
    )*);
}

unsigned_integer_rectangle! { u32 usize }
