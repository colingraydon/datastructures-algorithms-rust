use std::alloc::{self, Layout};
use std::ptr::NonNull;
use std::{fmt, mem, ptr};

pub struct Vector<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "TODO - deal with size 0");
        Vector {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_cap) }
        };

        //deal with a failed alloc
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.cap = new_cap;
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }

        self.len += 1;
    }
}

impl<T: fmt::Debug> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", unsafe { self.ptr.as_ptr().add(i).read() })?;
        }
        write!(f, "]")
    }
}
