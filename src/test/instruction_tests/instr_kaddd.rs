use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDD, operand1: Some(Direct(K7)), operand2: Some(Direct(K5)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 213, 74, 252], OperandSize::Dword)
}

#[test]
fn kaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDD, operand1: Some(Direct(K5)), operand2: Some(Direct(K6)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 205, 74, 236], OperandSize::Qword)
}

