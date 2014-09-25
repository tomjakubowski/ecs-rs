
#![experimental]

//! Store objects as bytes for types that need to avoid generics, with partial safety.

// This is a strange, wonderful, disgusting, and useful object.
// Apart from `Phantom`, this is a freestanding backend for the component lists.

use std::mem;
use std::slice;
use std::raw::Slice;

pub struct Buffer
{
    bytes: Vec<u8>,
    stride: uint,
}

impl Buffer
{
    pub fn new(stride: uint) -> Buffer
    {
        Buffer
        {
            bytes: Vec::new(),
            stride: stride,
        }
    }

    pub unsafe fn set<T: Copy+'static>(&mut self, index: uint, val: &T)
    {
        if mem::size_of::<T>() != self.stride
        {
            fail!("Type has invalid size for buffer")
        }
        let offset = self.stride * index;
        while offset + self.stride > self.bytes.len()
        {
            self.bytes.grow(self.stride, 0u8);
        }
        let _slice = self.bytes.slice_mut(offset, offset + self.stride);

        let vslice: &[u8] = mem::transmute(Slice
        {
            data: slice::ref_slice(val).as_ptr() as *const u8,
            len: self.stride,
        });
        _slice.copy_memory(vslice);
    }

    pub unsafe fn get<T: Copy+'static>(&self, index: uint) -> Option<T>
    {
        if mem::size_of::<T>() != self.stride
        {
            fail!("Type has invalid size for buffer")
        }
        let offset = self.stride * index;
        if offset >= self.bytes.len()
        {
            None
        }
        else
        {
            let _slice = self.bytes.slice(offset, offset + self.stride);
            let oslice: &[T] = mem::transmute(Slice
            {
                data: _slice.as_ptr() as *const T,
                len: self.stride,
            });
            Some(oslice[0])
        }
    }

    pub unsafe fn borrow<T: Copy+'static>(&self, index: uint) -> Option<&T>
    {
        if mem::size_of::<T>() != self.stride
        {
            fail!("Type has invalid size for buffer")
        }
        let offset = self.stride * index;
        if offset >= self.bytes.len()
        {
            None
        }
        else
        {
            let _slice = self.bytes.slice(offset, offset + self.stride);
            let oslice: &[T] = mem::transmute(Slice
            {
                data: _slice.as_ptr() as *const T,
                len: self.stride,
            });
            Some(&oslice[0])
        }
    }

    pub unsafe fn borrow_mut<T: Copy+'static>(&mut self, index: uint) -> Option<&mut T>
    {
        if mem::size_of::<T>() != self.stride
        {
            fail!("Type has invalid size for buffer")
        }
        let offset = self.stride * index;
        if offset >= self.bytes.len()
        {
            None
        }
        else
        {
            let _slice = self.bytes.slice(offset, offset + self.stride);
            let oslice: &mut [T] = mem::transmute(Slice
            {
                data: _slice.as_ptr() as *const T,
                len: self.stride,
            });
            Some(&mut oslice[0])
        }
    }

    pub fn len(&self) -> uint
    {
        self.bytes.len() / self.stride
    }

    pub fn bytes_len(&self) -> uint
    {
        self.bytes.len()
    }

    pub fn stride(&self) -> uint
    {
        self.stride
    }

    pub fn as_bytes(&self) -> &Vec<u8>
    {
        &self.bytes
    }
}
