use std::{fmt::Debug, num::NonZero};

use crate::error::{EmptyCollectionError, FullCollectionError};

pub struct RingBuffer<T, const SIZE: usize> {
    head: usize,
    len: usize,
    buffer: [Option<T>; SIZE],
}

impl<T, const SIZE: usize> RingBuffer<T, SIZE> {
    /// Creates an empty ring buffer.
    ///
    /// # Examples
    /// ```
    /// use fixed_collections::RingBuffer;
    ///
    /// let ring: RingBuffer<16, u32> = RingBuffer::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            head: 0,
            len: 0,
            buffer: [const { None }; SIZE],
        }
    }

    /// Returns number of elements in the ring buffer.
    /// 
    /// # Examples
    /// ```
    /// use fixed_collections::RingBuffer;
    /// 
    /// let ring: RingBuffer<20, i16> = RingBuffer::new();
    /// assert_eq!(ring.len(), 0);
    /// 
    /// todo!("Create example demonstrating the length increasing when pushing front and back.")
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the ring buffer is empty.
    ///
    /// # Examples
    /// ```
    /// use fixed_collections::RingBuffer;
    ///
    /// let ring: RingBuffer<16, u32> = RingBuffer::new();
    /// assert!(ring.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` if the ring buffer's length has reached its capacity.
    /// 
    /// # Examples
    /// ```
    /// use fixed_collections::RingBuffer;
    /// 
    /// todo!("Create an example")
    /// ```
    pub fn is_full(&self) -> bool {
        self.len == SIZE
    }

    /// Appends an element to the back of the ring buffer.
    /// Returns `Err(FullCollectionError)` if the ring buffer is full.
    /// Returns the new length otherwise.
    /// 
    /// # Examples
    /// ```
    /// todo!("Create an example")
    /// ```
    pub fn push_back(&mut self, value: T) -> Result<NonZero<usize>, FullCollectionError> {
        if self.is_full() {
            return Err(FullCollectionError);
        }
        let next_index: usize = (self.head + self.len) % SIZE; 
        self.buffer[next_index] = Some(value);
        self.len += 1;
        // SAFETY: self.len must be > 0.
        // We return Result<NonZero<usize, FullCollectionError> which should
        // be size of usize (FullCollectionError get optimised to 0).
        Ok(NonZero::new(self.len).unwrap())
    }

    /// Prepends an element to the front of the ring buffer.
    /// Returns `Err(FullCollectionError)` if the ring buffer is full.
    /// Returns the new length otherwise.
    /// 
    /// # Examples
    /// ```
    /// todo!("Create an example")
    /// ```
    pub fn push_front(&mut self, value: T) -> Result<NonZero<usize>, FullCollectionError> {
        if self.is_full() {
            return Err(FullCollectionError);
        }
        let next_index: usize = if self.head == 0 {
            SIZE - 1
        } else { 
            self.head - 1
        };
        self.buffer[next_index] = Some(value);
        self.len += 1;
        // SAFETY: self.len must be > 0.
        // We return Result<NonZero<usize, FullCollectionError> which should
        // be size of usize (FullCollectionError gets optimised to 0).
        Ok(NonZero::new(self.len).unwrap())
    }

    /// Removes first element and returns it.
    /// If the ring buffer is empty, we return Err(EmptyCollectionError)
    /// 
    /// # Examples
    /// ```
    /// todo!("Create an example")
    /// ```
    pub fn pop_front(&mut self) -> Result<T, EmptyCollectionError> {
        if self.is_empty() {
            return Err(EmptyCollectionError);
        }
        let value: T = self.buffer[self.head].take().unwrap();
        self.head = (self.head + 1) % SIZE;
        self.len -= 1;
        Ok(value)
    }

    /// Removes last element and returns it.
    /// If the ring buffer is empty, we return Err(EmptyCollectionError)
    /// 
    /// # Examples
    /// ```
    /// todo!("Create an example")
    /// ```    
    pub fn pop_back(&mut self) -> Result<T, EmptyCollectionError> {
        if self.is_empty() {
            return Err(EmptyCollectionError)
        }
        let index: usize = (self.head + self.len) % SIZE;
        let value: T = self.buffer[index].take().unwrap();
        self.len -= 1;
        Ok(value)
    }

    pub fn clear(&mut self) {
        todo!("This should drop every element.");
    }

    /// Moves all elements of `other` into `self`, leaving `other` empty.
    /// If the size of `self` is   
    pub fn append<const OTHER_SIZE: usize>(&mut self, other: &mut RingBuffer<T, OTHER_SIZE>)
        -> Result<usize, ()> {
        todo!("Iterate and take all into this ring buffer. Also create an error.")
    }
}

impl<T> Default for RingBuffer<T, 16> {
    /// Creates an empty RingBuffer of size 16.
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const SIZE: usize> Debug for RingBuffer<T, SIZE> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RingBuffer")
            .field("head", &self.head)
            .field("len", &self.len)
            .field("buffer", &self.buffer)
            .finish()
    }
}

// Note: This prevents double Option wraps.
impl<T, const SIZE: usize> From<[Option<T>; SIZE]> for RingBuffer<T, SIZE> {
    fn from(buffer: [Option<T>; SIZE]) -> Self {
        Self { 
            head: 0, 
            len: SIZE, 
            buffer 
        }
    }
}

impl<T, const SIZE: usize> From<[T; SIZE]> for RingBuffer<T, SIZE> {
    fn from(buffer: [T; SIZE]) -> Self {
        Self {
            head: 0,
            len: SIZE,
            buffer: buffer.map(|val| { Some(val) }),
        }
    }
}

// impl<T, const SIZE: usize> IntoIterator for RingBuffer<T, SIZE> {
//     type Item = T;

       // TODO: Create a ring_buffer::IntoIter type
//     type IntoIter;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
