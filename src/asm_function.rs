use std::mem;

use mmap::MapOption;
use mmap::MemoryMap;

pub struct AsmFunction<Args, Ret> {
    function: unsafe extern "C" fn(Args) -> Ret,
    _mmap: MemoryMap,
}

impl<Args, Ret> AsmFunction<Args, Ret> {
    /// # Panics
    /// Panics when given an empty slice
    #[must_use]
    pub fn new(buf: &[u8]) -> Self {
        let mmap = MemoryMap::new(
            buf.len(),
            &[
                MapOption::MapReadable,
                MapOption::MapWritable,
                MapOption::MapExecutable,
            ],
        )
        .unwrap();

        // this is fine since it's copying into newly mapped memory
        unsafe {
            std::ptr::copy_nonoverlapping(buf.as_ptr(), mmap.data(), buf.len());
        }

        // to use the function requires unsafe and if it's never used no harm no foul
        let function = unsafe { mem::transmute(mmap.data()) };

        Self {
            function,
            _mmap: mmap,
        }
    }

    /// # Safety
    /// This calls arbitrary, potentially runtime generated, machine code.
    /// Good luck
    pub unsafe fn run(&self, args: Args) -> Ret {
        (self.function)(args)
    }
}
