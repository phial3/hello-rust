// #![feature(map_first_last)]
#![allow(clippy::transmute_int_to_float)]

pub mod error;
pub mod opcodes;
pub mod s0;
#[cfg(test)]
mod tests;
mod util;
#[cfg(feature = "vm")]
pub mod vm;

#[macro_export]
/// Create an in-memory representation for s0 binary
macro_rules! s0_bin {
    (
        $(
            // TODO: global variable declaration
            const $const_val:expr;
        )*
        $(
            // TODO: global variable declaration
            let $val:expr;
        )*
        $(
            fn $name:ident $loc_slots:literal $param:literal -> $ret:literal {
                $($inst:expr $(,)?)*
            }
        )+
    ) => {{
        use $crate::opcodes::Op::*;
        use $crate::util::IntoBytes;
        let mut globals = vec![];

        $({
            let bytes = $const_val.into_bytes();
            let glob = GlobalValue {
                is_const: true,
                bytes
            };
            globals.push(glob);
        })*
        $({
            let bytes = $val.into_bytes();
            let glob = GlobalValue {
                is_const: false,
                bytes
            };
            globals.push(glob);
        })*

        let mut fns = vec![];
        $({
            let name = stringify!($name);
            let bytes = name.into_bytes();
            let glob = GlobalValue{ is_const:true, bytes };
            let name_idx = globals.len();
            globals.push(glob);

            let loc_slots = $loc_slots;
            let inst = vec![$($inst),*];
            let func = FnDef{
                name: name_idx as u32,
                loc_slots,
                param_slots: $param,
                ret_slots: $ret,
                ins: inst,
            };
            fns.push(func);
        })+
        let s0 = S0{
            globals,
            functions: fns,
        };
        s0
    }};
}
