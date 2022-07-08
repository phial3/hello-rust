//! Memory-related implementations for R0VM
use super::*;
use static_assertions as sa;
use std::alloc::Layout;

/// A piece of managed memory owned by the virtual machine
pub struct ManagedMemory {
    ptr: *mut u8,
    layout: Layout,
    is_const: bool,
}

#[allow(clippy::len_without_is_empty)]
impl ManagedMemory {
    /// Allocate a piece of managed memory using global allocator
    pub fn alloc(layout: Layout) -> Result<ManagedMemory> {
        if layout.size() == 0 {
            return Err(Error::AllocZero);
        }
        let mem = unsafe { std::alloc::alloc_zeroed(layout) };
        if mem.is_null() {
            return Err(Error::OutOfMemory);
        }
        Ok(ManagedMemory {
            ptr: mem,
            layout,
            is_const: false,
        })
    }

    /// Allocate a piece of managed memory using global allocator
    pub fn from_slice(slice: &[u8]) -> Result<ManagedMemory> {
        if slice.len() == 0 {
            return Err(Error::AllocZero);
        }
        let layout = Layout::from_size_align(slice.len(), 8)?;
        let mem = unsafe { std::alloc::alloc_zeroed(layout) };
        if mem.is_null() {
            return Err(Error::OutOfMemory);
        }

        // copy slice content to memory
        unsafe {
            slice.as_ptr().copy_to_nonoverlapping(mem, slice.len());
        }

        Ok(ManagedMemory {
            ptr: mem,
            layout,
            is_const: false,
        })
    }

    /// Construct a piece of managed memory using raw pointer and length
    pub unsafe fn new(ptr: *mut u8, layout: Layout, is_const: bool) -> ManagedMemory {
        ManagedMemory {
            ptr,
            layout,
            is_const,
        }
    }

    /// Length of the memory
    pub fn len(&self) -> usize {
        self.layout.size()
    }

    /// Get the memory as slice
    pub unsafe fn get_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.ptr, self.layout.size())
    }

    /// Get the memory as mutable slice
    pub unsafe fn get_slice_mut(&mut self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.ptr, self.layout.size())
    }

    /// Get the memory as raw pointer
    pub fn get_ptr(&self) -> *mut u8 {
        self.ptr
    }
}

impl Drop for ManagedMemory {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.ptr, self.layout) }
    }
}

pub fn stack_idx_to_vm_addr(idx: usize) -> u64 {
    R0Vm::STACK_START + (idx as u64) * 8
}

pub fn vm_addr_to_stack_idx(addr: u64) -> (usize, usize) {
    let off = addr - R0Vm::STACK_START;
    let idx = off / 8;
    let off = off % 8;
    (idx as usize, off as usize)
}

#[inline]
pub fn round_up_to_multiple(x: u64, mult: u64) -> u64 {
    x + (mult - x % mult)
}

impl<'src> R0Vm<'src> {
    pub const HEAP_START: u64 = 0x00000001_00000000;
    pub const STACK_START: u64 = 0xffffffff_00000000;
    // pub const STACK_END: u64 = 0xffffffff_00000000;

    // * Heap stuff -->

    /// Find the piece of heap memory by address.
    /// Returns the managed memory instance and the index offset from start.
    pub fn get_heap_mem_managed_ref(&self, addr: u64) -> Result<(&ManagedMemory, usize)> {
        let range = self
            .heap
            .range((std::ops::Bound::Unbounded, std::ops::Bound::Included(addr)));
        // Get the last memory chunk that is less or equal than address
        let (start_addr, mem) = range.last().ok_or(Error::InvalidAddress(addr))?;
        let addr_offset = addr - start_addr;
        if addr_offset > mem.len() as u64 {
            Err(Error::InvalidAddress(addr))
        } else {
            Ok((mem, addr_offset as usize))
        }
    }

    fn get_heap_mem_ptr<T>(&self, addr: u64) -> Result<*mut u8> {
        assert!(addr < R0Vm::STACK_START);

        let alignment_of_t = std::mem::align_of::<T>();
        if addr % alignment_of_t as u64 != 0 {
            return Err(Error::UnalignedAccess(addr));
        }

        let (slice, offset) = self.get_heap_mem_managed_ref(addr)?;
        let sizeof_t = std::mem::size_of::<T>();

        // Check remaining space is enough
        if sizeof_t + offset > slice.len() {
            return Err(Error::InvalidAddress(addr));
        }

        let t_ptr = unsafe { slice.get_ptr().add(offset) };
        Ok(t_ptr)
    }

    /// Assuming `mem` is heap memory, get the reference of this memory as `&T`.
    ///
    /// # Safety
    ///
    /// addr must be a valid heap memory pointer
    pub unsafe fn heap_mem_ref<T>(&self, addr: u64) -> Result<&T> {
        let t_ptr = self.get_heap_mem_ptr::<T>(addr)?;
        let t_ptr = t_ptr as *mut T;
        Ok(&*t_ptr)
    }

    /// Assuming `mem` is heap memory, get the reference of this memory as `&mut T`.
    ///
    /// # Safety
    ///
    /// addr must be a valid heap memory pointer
    pub unsafe fn heap_mem_mut<T>(&self, addr: u64) -> Result<&mut T> {
        let t_ptr = self.get_heap_mem_ptr::<T>(addr)?;
        let t_ptr = t_ptr as *mut T;
        Ok(&mut *t_ptr)
    }

    /// Assuming `mem` is heap memory, get the reference of this memory as `&T`.
    ///
    /// # Safety
    ///
    /// addr must be a valid heap memory pointer
    pub unsafe fn heap_mem_get<T>(&self, addr: u64) -> Result<T>
    where
        T: Copy,
    {
        let t_ptr = self.get_heap_mem_ptr::<T>(addr)?;
        let t_ptr = t_ptr as *mut T;
        Ok(*t_ptr)
    }

    /// Assuming `mem` is heap memory, get the reference of this memory as `&mut T`.
    ///
    /// # Safety
    ///
    /// addr must be a valid heap memory pointer
    pub unsafe fn heap_mem_set<T>(&self, addr: u64, val: T) -> Result<()> {
        let t_ptr = self.get_heap_mem_ptr::<T>(addr)?;
        let t_ptr = t_ptr as *mut T;
        *t_ptr = val;
        Ok(())
    }

    /// Allocate a piece of memory of length `len` onto heap. Returns address.
    pub fn alloc_heap(&mut self, len: usize, alignment: usize) -> Result<u64> {
        let mem = unsafe { ManagedMemory::alloc(Layout::from_size_align(len, alignment)?)? };
        let mem_addr = self
            .heap
            .iter()
            .next_back()
            .map(|(k, v)| round_up_to_multiple(*k + v.len() as u64, alignment as u64))
            .unwrap_or(R0Vm::HEAP_START);
        self.heap.insert(mem_addr, mem);
        Ok(mem_addr)
    }

    /// Free a piece of memory specified by `addr`. Will return an error if
    /// memory is not the very same address as the allocator returns.
    pub fn free_heap(&mut self, addr: u64) -> Result<()> {
        let mem = self.heap.remove(&addr).ok_or(Error::InvalidDeallocation)?;
        drop(mem);
        Ok(())
    }

    // * Stack stuff -->

    /// Get the stack memory specified by `addr`.
    ///
    /// Note that this function allows reading past `bp`.
    pub fn get_stack_mem<T>(&self, addr: u64) -> Result<T>
    where
        T: Copy,
    {
        let alignof_t = std::mem::align_of::<T>();
        if addr % alignof_t as u64 != 0 {
            return Err(Error::UnalignedAccess(addr));
        }
        let raw_off = (addr - R0Vm::STACK_START) as usize;
        if raw_off > self.max_stack_size * std::mem::size_of::<Slot>() {
            return Err(Error::InvalidAddress(addr));
        }

        unsafe {
            let raw_ptr = (self.stack as *mut u8).add(raw_off) as *mut T;
            let val = raw_ptr.read();

            Ok(val)
        }
    }

    #[inline]
    pub fn stack_slot_get(&self, p: usize) -> Result<Slot> {
        if p > self.max_stack_size {
            return Err(Error::InvalidStackOffset(p as u64, 0));
        }
        unsafe {
            let ptr = self.stack.add(p);
            let val = *ptr;
            Ok(val)
        }
    }

    pub fn set_stack_mem<T>(&mut self, addr: u64, set_val: T) -> Result<()>
    where
        T: Copy + Into<u64>,
    {
        let alignof_t = std::mem::align_of::<T>();
        if addr % alignof_t as u64 != 0 {
            return Err(Error::UnalignedAccess(addr));
        }
        let raw_off = (addr - R0Vm::STACK_START) as usize;
        if raw_off > self.max_stack_size * std::mem::size_of::<Slot>() {
            return Err(Error::InvalidAddress(addr));
        }

        unsafe {
            let raw_ptr = (self.stack as *mut u8).add(raw_off) as *mut T;
            raw_ptr.write(set_val);

            Ok(())
        }
    }

    #[inline]
    pub fn stack_slot_set(&self, p: usize, val: Slot) -> Result<()> {
        if p > self.max_stack_size {
            return Err(Error::InvalidStackOffset(p as u64, 0));
        }
        unsafe {
            let ptr = self.stack.add(p);
            *ptr = val;
            Ok(())
        }
    }

    #[inline]
    pub fn stack_push(&mut self, val: Slot) -> Result<()> {
        self.stack_slot_set(self.sp, val)?;
        self.sp += 1;
        Ok(())
    }

    #[inline]
    pub fn stack_pop(&mut self) -> Result<Slot> {
        self.sp -= 1;
        self.stack_slot_get(self.sp)
    }

    #[inline]
    pub fn stack_top(&mut self) -> Result<Slot> {
        if self.sp == 0 {
            return Err(Error::StackUnderflow);
        }
        self.stack_slot_get(self.sp - 1)
    }

    #[inline]
    pub fn stack_truncate_by(&mut self, size: usize) -> Result<()> {
        if size > self.sp {
            return Err(Error::StackUnderflow);
        }
        self.sp -= size;
        Ok(())
    }

    // * Misc stuff -->

    /// Access an immutable reference of a piece of memory at `addr`
    pub fn access_mem_get<T>(&self, addr: u64) -> Result<T>
    where
        T: Copy,
    {
        if addr < R0Vm::STACK_START {
            // Heap vars
            unsafe { self.heap_mem_get::<T>(addr) }
        } else {
            // Stack vars
            self.get_stack_mem(addr)
        }
    }

    /// Access a mutable reference of a piece of memory at `addr`
    pub fn access_mem_set<T>(&mut self, addr: u64, val: T) -> Result<()>
    where
        T: Copy + Into<u64>,
    {
        if addr < R0Vm::STACK_START {
            // Heap vars
            unsafe { self.heap_mem_set::<T>(addr, val) }
        } else {
            // Stack vars
            self.set_stack_mem(addr, val)
        }
    }
}
