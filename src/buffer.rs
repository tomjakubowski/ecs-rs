
#![experimental]
#![doc(hidden)]

// This is a strange, wonderful, disgusting, and useful object.

use std::mem;
use std::ptr;
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

    pub fn clear(&mut self)
    {
        self.bytes = Vec::new();
    }

    pub unsafe fn set<T: Copy+'static>(&mut self, index: uint, val: &T)
    {
        if mem::size_of::<T>() != self.stride
        {
            panic!("Type has invalid size for buffer")
        }
        let offset = self.stride * index;
        while offset + self.stride > self.bytes.len()
        {
            self.bytes.grow(self.stride, 0u8);
        }

        let src = slice::ref_slice(val).as_ptr() as *const u8;
        let dst = self.bytes.slice_mut(offset, offset + self.stride).as_mut_ptr();
        ptr::copy_memory(dst, src, self.stride);
    }

    pub unsafe fn get<T: Copy+'static>(&self, index: uint) -> T
    {
        if mem::size_of::<T>() != self.stride
        {
            panic!("Type has invalid size for buffer")
        }
        let offset = self.stride * index;
        if offset >= self.bytes.len()
        {
            panic!("Index Out of Bounds")
        }
        else
        {
            let _slice = self.bytes.slice(offset, offset + self.stride);
            let oslice: &[T] = mem::transmute(Slice
            {
                data: _slice.as_ptr() as *const T,
                len: self.stride,
            });
            oslice[0]
        }
    }

    pub unsafe fn borrow<T: Copy+'static>(&mut self, index: uint) -> &mut T
    {
        if mem::size_of::<T>() != self.stride
        {
            panic!("Type has invalid size for buffer")
        }
        let offset = self.stride * index;
        if offset >= self.bytes.len()
        {
            panic!("Index Out of Bounds")
        }
        else
        {
            let _slice = self.bytes.slice(offset, offset + self.stride);
            let oslice: &mut [T] = mem::transmute(Slice
            {
                data: _slice.as_ptr() as *const T,
                len: self.stride,
            });
            &mut oslice[0]
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
