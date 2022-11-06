pub use crate::Return;

macro_rules! check {
    ($interp:expr, $expresion:expr) => {
        if !$expresion {
            $interp.instruction_result = Return::NotActivated;
            return;
        }
    };
}

macro_rules! gas {
    ($interp:expr, $gas:expr) => {
        if crate::USE_GAS {
            if !$interp.gas.record_cost(($gas)) {
                $interp.instruction_result = Return::OutOfGas;
                return;
            }
        }
    };
}

macro_rules! refund {
    ($interp:expr, $gas:expr) => {{
        if crate::USE_GAS {
            $interp.gas.gas_refund($gas);
        }
    }};
}

macro_rules! gas_or_fail {
    ($interp:expr, $gas:expr) => {
        if crate::USE_GAS {
            match $gas {
                Some(gas_used) => gas!($interp, gas_used),
                None => {
                    $interp.instruction_result = Return::OutOfGas;
                    return;
                }
            }
        }
    };
}

macro_rules! memory_resize {
    ($interp:expr, $offset:expr, $len:expr) => {{
        let len: usize = $len;
        let offset: usize = $offset;
        if let Some(new_size) =
            crate::interpreter::memory::next_multiple_of_32(offset.saturating_add(len))
        {
            #[cfg(feature = "memory_limit")]
            if new_size > ($interp.memory_limit as usize) {
                $interp.instruction_result = Return::OutOfGas;
                return;
            }

            if new_size > $interp.memory.len() {
                if crate::USE_GAS {
                    let num_bytes = new_size / 32;
                    if !$interp.gas.record_memory(crate::gas::memory_gas(num_bytes)) {
                        $interp.instruction_result = Return::OutOfGas;
                        return;
                    }
                }
                $interp.memory.resize(new_size);
            }
        } else {
            $interp.instruction_result = Return::OutOfGas;
            return;
        }
    }};
}

macro_rules! pop_address {
    ( $interp:expr, $x1:ident) => {
        if $interp.stack.len() < 1 {
            $interp.instruction_result = Return::StackUnderflow;
            return;
        }
        // Safety: Length is checked above.
        let $x1: B160 = ruint::aliases::B160::try_from_be_slice(
            &unsafe { $interp.stack.pop_unsafe() }.to_be_bytes_vec()[12..],
        )
        .unwrap();
    };
    ( $interp:expr, $x1:ident, $x2:ident) => {
        if $interp.stack.len() < 2 {
            $interp.instruction_result = Return::StackUnderflow;
            return;
        }
        let mut temp = H256::zero();
        $x1: H160 = {
            // Safety: Length is checked above.
            unsafe {
                $interp
                    .stack
                    .pop_unsafe()
                    .to_big_endian(temp.as_bytes_mut())
            };
            temp.into()
        };
        $x2: H160 = {
            temp = H256::zero();
            // Safety: Length is checked above.
            unsafe {
                $interp
                    .stack
                    .pop_unsafe()
                    .to_big_endian(temp.as_bytes_mut())
            };
            temp.into();
        };
    };
}

macro_rules! pop {
    ( $interp:expr, $x1:ident) => {
        if $interp.stack.len() < 1 {
            $interp.instruction_result = Return::StackUnderflow;
            return;
        }
        // Safety: Length is checked above.
        let $x1 = unsafe { $interp.stack.pop_unsafe() };
    };
    ( $interp:expr, $x1:ident, $x2:ident) => {
        if $interp.stack.len() < 2 {
            $interp.instruction_result = Return::StackUnderflow;
            return;
        }
        // Safety: Length is checked above.
        let ($x1, $x2) = unsafe { $interp.stack.pop2_unsafe() };
    };
    ( $interp:expr, $x1:ident, $x2:ident, $x3:ident) => {
        if $interp.stack.len() < 3 {
            $interp.instruction_result = Return::StackUnderflow;
            return;
        }
        // Safety: Length is checked above.
        let ($x1, $x2, $x3) = unsafe { $interp.stack.pop3_unsafe() };
    };

    ( $interp:expr, $x1:ident, $x2:ident, $x3:ident, $x4:ident) => {
        if $interp.stack.len() < 4 {
            $interp.instruction_result = Return::StackUnderflow;
            return;
        }
        // Safety: Length is checked above.
        let ($x1, $x2, $x3, $x4) = unsafe { $interp.stack.pop4_unsafe() };
    };
}

macro_rules! pop_top {
    ( $interp:expr, $x1:ident) => {
        if $interp.stack.len() < 1 {
            $interp.instruction_result = Return::StackUnderflow;
            return;
        }
        // Safety: Length is checked above.
        let $x1 = unsafe { $interp.stack.top_unsafe() };
    };
    ( $interp:expr, $x1:ident, $x2:ident) => {
        if $interp.stack.len() < 2 {
            $interp.instruction_result = Return::StackUnderflow;
            return;
        }
        // Safety: Length is checked above.
        let ($x1, $x2) = unsafe { $interp.stack.pop_top_unsafe() };
    };
    ( $interp:expr, $x1:ident, $x2:ident, $x3:ident) => {
        if $interp.stack.len() < 3 {
            $interp.instruction_result = Return::StackUnderflow;
            return;
        }
        // Safety: Length is checked above.
        let ($x1, $x2, $x3) = unsafe { $interp.stack.pop2_top_unsafe() };
    };
}

macro_rules! push_b256 {
	( $interp:expr, $( $x:expr ),* ) => (
		$(
			match $interp.stack.push_b256($x) {
				Ok(()) => (),
				Err(e) => {
                    $interp.instruction_result = e;
                    return
                },
			}
		)*
	)
}

macro_rules! push {
    ( $interp:expr, $( $x:expr ),* ) => (
		$(
			match $interp.stack.push($x) {
				Ok(()) => (),
				Err(e) => { $interp.instruction_result = e;
                    return
                } ,
			}
		)*
	)
}

// macro_rules! as_usize_saturated {
//     ( $v:expr ) => {
//         $v.saturating_to::<usize>()
//     };
// }

// macro_rules! as_usize_or_fail {
//     ( $interp:expr, $v:expr ) => {{
//         as_usize_or_fail!($interp, $v, Return::OutOfGas)
//     }};

//     ( $interp:expr, $v:expr, $reason:expr ) => {
//         match $v[0] == 0 {
//             Ok(value) => value,
//             Err(_) => {
//                 $interp.instruction_result = $reason;
//                 return;
//             }
//         }
//     };
// }

macro_rules! as_usize_saturated {
    ( $v:expr ) => {{
        if $v.as_limbs()[1] != 0 || $v.as_limbs()[2] != 0 || $v.as_limbs()[3] != 0 {
            usize::MAX
        } else {
            $v.as_limbs()[0] as usize
        }
    }};
}

macro_rules! as_usize_or_fail {
    (  $interp:expr, $v:expr ) => {{
        as_usize_or_fail!($interp, $v, Return::OutOfGas)
    }};

    (  $interp:expr, $v:expr, $reason:expr ) => {{
        if $v.as_limbs()[1] != 0 || $v.as_limbs()[2] != 0 || $v.as_limbs()[3] != 0 {
            $interp.instruction_result = $reason;
            return;
        }

        $v.as_limbs()[0] as usize
    }};
}
