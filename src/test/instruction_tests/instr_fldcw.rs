use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fldcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(IndirectDisplaced(DI, 157, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 173, 157, 0], OperandSize::Word)
}

#[test]
fn fldcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 47], OperandSize::Dword)
}

#[test]
fn fldcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(IndirectDisplaced(RSI, 1739117242, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 174, 186, 210, 168, 103], OperandSize::Qword)
}

