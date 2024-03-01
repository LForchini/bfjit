use mmap::MapOption::*;
use mmap::*;
use std::marker::Tuple;
use std::mem;

pub struct AsmFunction<Args, Ret>
where
    Args: Tuple,
{
    function: unsafe extern "C" fn(Args) -> Ret,
    _mmap: MemoryMap,
}

impl<Args, Ret> AsmFunction<Args, Ret>
where
    Args: Tuple,
{
    pub unsafe fn new(buf: &[u8]) -> Self {
        let mmap = MemoryMap::new(buf.len(), &[MapReadable, MapWritable, MapExecutable]).unwrap();
        std::ptr::copy_nonoverlapping(buf.as_ptr(), mmap.data(), buf.len());

        let function = mem::transmute(mmap.data());

        Self {
            function,
            _mmap: mmap,
        }
    }
}

impl<Args, Ret> Fn<Args> for AsmFunction<Args, Ret>
where
    Args: Tuple,
{
    #[inline]
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        unsafe { (self.function)(args) }
    }
}

impl<Args, Ret> FnMut<Args> for AsmFunction<Args, Ret>
where
    Args: Tuple,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        self.call(args)
    }
}

impl<Args, Ret> FnOnce<Args> for AsmFunction<Args, Ret>
where
    Args: Tuple,
{
    type Output = Ret;

    #[inline]
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.call(args)
    }
}
