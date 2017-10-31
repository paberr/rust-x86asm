use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLQ, operand1: Some(Direct(K7)), operand2: Some(Direct(K7)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 51, 255, 67], OperandSize::Dword)
}

#[test]
fn kshiftlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLQ, operand1: Some(Direct(K6)), operand2: Some(Direct(K7)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 51, 247, 112], OperandSize::Qword)
}

