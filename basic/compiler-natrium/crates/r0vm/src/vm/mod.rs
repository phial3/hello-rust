pub mod mem;
pub mod ops;

use crate::error::*;
use crate::{opcodes::Op, s0::*};
use mem::*;
use ops::*;
use smol_str::SmolStr;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    io::Write,
    io::{Bytes, Read},
};

pub const MAX_STACK_SIZE: usize = 131072;

pub type Slot = u64;
pub type Addr = u64;

/// An interpreter running S0 code.
pub struct R0Vm<'src> {
    /// Source file
    src: &'src S0,
    max_stack_size: usize,

    /// Global variable index
    global_idx: HashMap<u32, Addr>,
    /// Global variable index
    function_idx: HashMap<SmolStr, u32>,

    /// Memory heap
    heap: BTreeMap<Addr, ManagedMemory>,
    /// Memory stack
    stack: *mut u64,

    /// Function Pointer
    fn_info: &'src FnDef,
    /// Function ID
    fn_id: usize,
    /// Instruction Pointer
    ip: usize,
    /// Stack Pointer
    sp: usize,
    /// Base Pointer
    bp: usize,

    /// Standard Input Stream
    stdin: Bytes<Box<dyn Read>>,
    /// Standard Output Stream
    stdout: Box<dyn Write>,
}

impl<'src> R0Vm<'src> {
    pub fn new(src: &'src S0, stdin: Box<dyn Read>, stdout: Box<dyn Write>) -> Result<R0Vm<'src>> {
        let start = src.functions.get(0).ok_or(Error::NoEntryPoint)?;
        let stack = unsafe {
            std::alloc::alloc_zeroed(std::alloc::Layout::array::<u64>(MAX_STACK_SIZE).unwrap())
                as *mut u64
        };

        unsafe {
            // push sentinel values
            let usize_max = usize::max_value() as u64;
            stack.add(0).write(usize_max);
            stack.add(1).write(usize_max);
            stack.add(2).write(usize_max);
        }

        let bp = 0usize;
        let sp = (start.loc_slots + 3) as usize;
        let (globals, global_idx) = Self::index_globals(&src.globals[..])?;
        let function_idx = Self::index_functions(src)?;
        Ok(R0Vm {
            src,
            max_stack_size: MAX_STACK_SIZE,
            global_idx,
            function_idx,
            heap: globals,
            stack,
            fn_info: start,
            fn_id: 0,
            ip: 0,
            bp,
            sp,
            stdin: stdin.bytes(),
            stdout,
        })
    }

    fn index_globals(
        globals: &[GlobalValue],
    ) -> Result<(BTreeMap<Addr, ManagedMemory>, HashMap<u32, Addr>)> {
        let mut curr_max_addr = 0u64;

        let mut globals_map = BTreeMap::new();
        let mut idx = HashMap::new();

        for val in globals.into_iter().enumerate() {
            let (i, x) = val;
            let x: &GlobalValue = x;
            let len = x.bytes.len();
            let managed = ManagedMemory::from_slice(&x.bytes[..])?;

            let mem_addr = round_up_to_multiple(curr_max_addr + len as u64, 8);
            curr_max_addr = mem_addr;
            if mem_addr >= R0Vm::HEAP_START {
                return Err(Error::OutOfMemory);
            }

            globals_map.insert(mem_addr, managed);
            idx.insert(i as u32, mem_addr);
        }
        Ok((globals_map, idx))
    }

    fn index_functions(asm: &S0) -> Result<HashMap<SmolStr, u32>> {
        let mut res = HashMap::new();
        for (idx, f) in asm.functions.iter().enumerate() {
            let name = asm
                .globals
                .get(f.name as usize)
                .ok_or(Error::InvalidFunctionNameIndex(idx, f.name))?;
            let name = String::from_utf8_lossy(&name.bytes);
            let name = SmolStr::new(&name);
            res.insert(name, idx as u32);
        }
        Ok(res)
    }

    #[inline]
    pub fn step(&mut self) -> Result<Op> {
        let op = self.get_next_instruction()?;
        self.exec_instruction(op)?;
        Ok(op)
    }

    /// Drive virtual machine to end, and abort when any error occurs.
    pub fn run_to_end(&mut self) -> Result<()> {
        loop {
            match self.step() {
                Ok(_) => (),
                Err(Error::ControlReachesEnd(0)) => break Ok(()),
                Err(e) => break Err(e),
            }
        }
    }

    /// Drive virtual machine to end with an inspecting function to break when returning
    /// false, and abort when any error occurs.
    pub fn run_to_end_inspect<F>(&mut self, mut inspect: F) -> Result<()>
    where
        F: FnMut(&Self) -> bool,
    {
        loop {
            let res = self.step();
            if !inspect(self) {
                break Ok(());
            }
            match res {
                Ok(_) => (),
                Err(Error::ControlReachesEnd(0)) => break Ok(()),
                Err(e) => return Err(e),
            }
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.fn_id == 0 && self.ip == self.fn_info.ins.len()
    }

    #[inline]
    fn get_next_instruction(&mut self) -> Result<Op> {
        let op = *self
            .fn_info
            .ins
            .get(self.ip)
            .ok_or(Error::ControlReachesEnd(self.fn_id))?;
        self.ip += 1;
        Ok(op)
    }

    pub fn fn_info(&self) -> &FnDef {
        self.fn_info
    }

    pub fn fn_id(&self) -> usize {
        self.fn_id
    }

    pub fn ip(&self) -> usize {
        self.ip
    }

    pub fn sp(&self) -> usize {
        self.sp
    }

    pub fn bp(&self) -> usize {
        self.bp
    }

    pub(crate) fn check_stack_overflow(&self, push_cnt: usize) -> Result<()> {
        if self.bp + push_cnt < self.max_stack_size {
            Ok(())
        } else {
            Err(Error::StackOverflow)
        }
    }

    pub fn get_fn_by_id(&self, id: u32) -> Result<&'src FnDef> {
        self.src
            .functions
            .get(id as usize)
            .ok_or(Error::InvalidFnId(id))
    }

    pub fn get_fn_by_name(&self, name: &str) -> Result<u32> {
        self.function_idx
            .get(name)
            .copied()
            .ok_or_else(|| Error::UnknownFunctionName(name.to_owned()))
    }

    pub fn get_global_by_id(&self, id: u32) -> Result<&'src GlobalValue> {
        self.src
            .globals
            .get(id as usize)
            .ok_or(Error::InvalidFnId(id))
    }

    pub fn get_fn_name_by_id(&self, id: u32) -> Result<String> {
        let func = self.get_fn_by_id(id)?;
        Ok(String::from_utf8_lossy(&self.get_global_by_id(func.name)?.bytes).into_owned())
    }

    pub fn exec_instruction(&mut self, op: Op) -> Result<()> {
        use Op::*;
        match op {
            Nop => Ok(()),
            Push(x) => self.push(x),
            Pop => self.pop().map(|_| ()),
            PopN(n) => self.pop_n(n),
            Dup => self.dup(),
            LocA(n) => self.loc_a(n),
            ArgA(n) => self.arg_a(n),
            GlobA(n) => self.glob_a(n),
            Load8 => self.load8(),
            Load16 => self.load16(),
            Load32 => self.load32(),
            Load64 => self.load64(),
            Store8 => self.store8(),
            Store16 => self.store16(),
            Store32 => self.store32(),
            Store64 => self.store64(),
            Alloc => self.alloc(),
            Free => self.free(),
            StackAlloc(n) => self.stack_alloc(n),
            AddI => self.add_i(),
            SubI => self.sub_i(),
            MulI => self.mul_i(),
            DivI => self.div_i(),
            AddF => self.add_f(),
            SubF => self.sub_f(),
            MulF => self.mul_f(),
            DivF => self.div_f(),
            DivU => self.div_u(),
            Shl => self.shl(),
            Shr => self.shr(),
            And => self.and(),
            Or => self.or(),
            Xor => self.xor(),
            Not => self.not(),
            CmpI => self.cmp_i(),
            CmpU => self.cmp_u(),
            CmpF => self.cmp_f(),
            NegI => self.neg_i(),
            NegF => self.neg_f(),
            IToF => self.itof(),
            FToI => self.ftoi(),
            ShrL => self.shr_l(),
            SetLt => self.set_lt(),
            SetGt => self.set_gt(),
            BrA(addr) => self.br_a(addr),
            Br(off) => self.br(off),
            BrFalse(off) => self.bz(off),
            BrTrue(off) => self.bnz(off),
            Call(id) => self.call(id),
            Ret => self.ret(),
            CallName(id) => self.call_by_name(id),
            ScanI => self.scan_i(),
            ScanC => self.scan_c(),
            ScanF => self.scan_f(),
            PrintI => self.print_i(),
            PrintC => self.print_c(),
            PrintF => self.print_f(),
            PrintS => self.print_s(),
            PrintLn => self.print_ln(),
            Panic => self.halt(),
        }
    }

    /// All information from current runtime stack. Usually being called
    /// during panic, halt, stack overflow or debug. Returns stacks and whether
    /// the stack is corrupted
    pub fn stack_trace(&self) -> (Vec<StackInfo>, bool) {
        let mut infos = Vec::new();
        let cur_stack = match self.cur_stack_info() {
            Ok(i) => i,
            Err(_) => {
                return (Vec::new(), true);
            }
        };
        infos.push(cur_stack);

        let mut bp = self.bp;
        let mut corrupted = false;
        while bp != usize::max_value() {
            let (info, bp_) = match self.stack_info(bp) {
                Ok(info) => info,
                Err(_) => {
                    corrupted = true;
                    break;
                }
            };
            if info.fn_id == usize::max_value() as u64 {
                // Stack bottom sentinel item
                break;
            }
            bp = bp_;
            infos.push(info);
        }
        (infos, corrupted)
    }

    /// Return the information of current running function
    pub fn cur_stack_info(&self) -> Result<StackInfo> {
        Ok(StackInfo {
            fn_id: self.fn_id as u64,
            inst: self.ip as u64,
            fn_name: self
                .src
                .globals
                .get(self.fn_info.name as usize)
                .map(|val| String::from_utf8_lossy(&val.bytes[..]).into()),
        })
    }

    /// Returns information of the stack function at `bp` and the base pointer of the
    /// caller of this function.
    pub fn stack_info(&self, bp: usize) -> Result<(StackInfo, usize)> {
        let prev_bp = self.stack_slot_get(bp)?;
        let ip = self.stack_slot_get(bp + 1)?;
        let fn_id = self.stack_slot_get(bp + 2)?;
        let fn_name = self.src.functions.get(fn_id as usize).and_then(|f| {
            self.src
                .globals
                .get(f.name as usize)
                .map(|val| String::from_utf8_lossy(&val.bytes[..]).into())
        });
        Ok((
            StackInfo {
                fn_name,
                fn_id,
                inst: ip,
            },
            prev_bp as usize,
        ))
    }

    pub fn debug_stack(&self) -> StackDebugger {
        StackDebugger::new(self.sp, self.bp, self.fn_info, self.stack().into())
    }

    pub fn debug_frame(&self, frame: usize) -> Result<StackDebugger> {
        let (sp, bp, fn_id) =
            (0..frame).try_fold((self.sp, self.bp, self.fn_id as u64), |(_sp, bp, _), _| {
                let (info, nbp) = self.stack_info(bp)?;
                Ok::<_, Error>((bp, nbp, info.fn_id))
            })?;
        let fn_info = self.get_fn_by_id(fn_id as u32)?;
        Ok(StackDebugger::new(sp, bp, fn_info, self.stack().into()))
    }

    pub fn stack(&self) -> &[Slot] {
        unsafe { std::slice::from_raw_parts(self.stack, self.sp) }
    }

    #[inline]
    fn total_loc(&self) -> usize {
        let total_loc = self.fn_info.loc_slots + self.fn_info.param_slots + self.fn_info.ret_slots;
        total_loc as usize
    }
}

impl<'s> Drop for R0Vm<'s> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.stack as *mut u8,
                std::alloc::Layout::array::<u64>(MAX_STACK_SIZE).unwrap(),
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StackInfo {
    pub fn_name: Option<String>,
    pub fn_id: u64,
    pub inst: u64,
}

impl std::fmt::Display for StackInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.fn_name.as_deref().unwrap_or("Unnamed function");
        write!(f, "{} (id={}) +{}", name, self.fn_id, self.inst)
    }
}

pub struct StackDebugger<'s, 'stack> {
    sp: usize,
    bp: usize,
    fn_info: &'s FnDef,
    stack: Cow<'stack, [Slot]>,
    stacktrace: bool,
    bounds: bool,
}

impl<'s, 'stack> StackDebugger<'s, 'stack> {
    pub fn new(
        sp: usize,
        bp: usize,
        fn_info: &'s FnDef,
        stack: Cow<'stack, [Slot]>,
    ) -> StackDebugger<'s, 'stack> {
        StackDebugger {
            sp,
            bp,
            fn_info,
            stack,
            stacktrace: true,
            bounds: true,
        }
    }

    pub fn snapshot_stack(&mut self) {
        self.stack = self.stack.to_owned()
    }

    pub fn bounds(mut self, op: bool) -> Self {
        self.bounds = op;
        self
    }
}

impl<'s, 'stack> std::fmt::Display for StackDebugger<'s, 'stack> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sp = self.sp;
        let bp = self.bp;

        let ret_slots = self.fn_info.ret_slots as usize;
        let param_slots = self.fn_info.param_slots as usize;
        let loc_slots = self.fn_info.loc_slots as usize;

        let upper_bound = std::cmp::min(sp + 5, self.stack.len());
        let lower_bound = bp.saturating_sub((param_slots + ret_slots) as usize);

        let loc_start = bp + 3;
        let loc_end = loc_start + loc_slots;
        let ret_end = bp - param_slots;

        writeln!(f, "Stack:")?;
        for i in (lower_bound..upper_bound).rev() {
            write!(f, "{:5} | {:#018x} |", i, self.stack.get(i).unwrap())?;
            if i == sp {
                write!(f, " <- sp")?;
            }
            if i == bp {
                write!(f, " <- bp")?;
            }
            writeln!(f)?;

            if self.bounds {
                if i == sp {
                    writeln!(f, "------v {:18} -", "expression")?;
                }
                if i == loc_end {
                    writeln!(f, "------v {:18} -", "local variable")?;
                }
                if i == loc_start {
                    writeln!(f, "------v {:18} -", "compiler info")?;
                }
                if i == bp {
                    writeln!(f, "------v {:18} -", "params")?;
                }
                if i == ret_end {
                    writeln!(f, "------v {:18} -", "return value")?;
                }
            }
        }

        Ok(())
    }
}

impl<'s> std::fmt::Debug for StackDebugger<'s, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn std::fmt::Display).fmt(f)
    }
}
