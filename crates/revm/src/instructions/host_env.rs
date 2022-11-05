use crate::{interpreter::Interpreter, Host, Return, Spec, SpecId::*};
use ruint::aliases::{B256, U256};

pub fn chainid<SPEC: Spec>(interpreter: &mut Interpreter, host: &mut dyn Host) {
    // gas!(interp, gas::BASE);
    // EIP-1344: ChainID opcode
    check!(interpreter, SPEC::enabled(ISTANBUL));
    push!(interpreter, host.env().cfg.chain_id);
}

pub fn coinbase(interpreter: &mut Interpreter, host: &mut dyn Host) {
    // gas!(interp, gas::BASE);
    push_b256!(
        interpreter,
        // TODO(shekhirin): replace with `B256::from(bits: Bits)`
        B256::from(U256::from(host.env().block.coinbase.into_inner()))
    );
}

pub fn timestamp(interpreter: &mut Interpreter, host: &mut dyn Host) {
    // gas!(interp, gas::BASE);
    push!(interpreter, host.env().block.timestamp);
}

pub fn number(interpreter: &mut Interpreter, host: &mut dyn Host) {
    // gas!(interp, gas::BASE);
    push!(interpreter, host.env().block.number);
}

pub fn difficulty(interpreter: &mut Interpreter, host: &mut dyn Host) {
    // gas!(interp, gas::BASE);
    push!(interpreter, host.env().block.difficulty);
}

pub fn gaslimit(interpreter: &mut Interpreter, host: &mut dyn Host) {
    // gas!(interp, gas::BASE);
    push!(interpreter, host.env().block.gas_limit);
}

pub fn gasprice(interpreter: &mut Interpreter, host: &mut dyn Host) {
    // gas!(interp, gas::BASE);
    push!(interpreter, host.env().effective_gas_price());
}

pub fn basefee<SPEC: Spec>(interpreter: &mut Interpreter, host: &mut dyn Host) {
    // gas!(interp, gas::BASE);
    // EIP-3198: BASEFEE opcode
    check!(interpreter, SPEC::enabled(LONDON));
    push!(interpreter, host.env().block.basefee);
}

pub fn origin(interpreter: &mut Interpreter, host: &mut dyn Host) {
    // gas!(interp, gas::BASE);
    push_b256!(
        interpreter,
        // TODO(shekhirin): replace with `B256::from(bits: Bits)`
        B256::from(U256::from(host.env().tx.caller.into_inner()))
    );
}
