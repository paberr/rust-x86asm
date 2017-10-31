use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movq2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVQ2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 214, 254], OperandSize::Word)
}

#[test]
fn movq2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVQ2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 214, 253], OperandSize::Dword)
}

#[test]
fn movq2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVQ2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 214, 248], OperandSize::Qword)
}

