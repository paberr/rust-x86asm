use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsubrp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBRP, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 226], OperandSize::Word)
}

#[test]
fn fsubrp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBRP, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 230], OperandSize::Dword)
}

#[test]
fn fsubrp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBRP, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 228], OperandSize::Qword)
}

