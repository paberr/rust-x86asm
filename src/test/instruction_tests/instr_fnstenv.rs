use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectDisplaced(SI, 24463, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 180, 143, 95], OperandSize::Word)
}

#[test]
fn fnstenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 398732828, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 180, 250, 28, 46, 196, 23], OperandSize::Dword)
}

#[test]
fn fnstenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 863129526, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 180, 216, 182, 79, 114, 51], OperandSize::Qword)
}

