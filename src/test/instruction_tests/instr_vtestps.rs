use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vtestps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 225], OperandSize::Dword)
}

#[test]
fn vtestps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EAX, 1957771399, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 168, 135, 56, 177, 116], OperandSize::Dword)
}

#[test]
fn vtestps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 199], OperandSize::Qword)
}

#[test]
fn vtestps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 14], OperandSize::Qword)
}

#[test]
fn vtestps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 203], OperandSize::Dword)
}

#[test]
fn vtestps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 209986072, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 140, 80, 24, 34, 132, 12], OperandSize::Dword)
}

#[test]
fn vtestps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 198], OperandSize::Qword)
}

#[test]
fn vtestps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 702757660, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 12, 221, 28, 59, 227, 41], OperandSize::Qword)
}

