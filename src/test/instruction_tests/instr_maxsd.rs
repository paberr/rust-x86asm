use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 207], OperandSize::Dword)
}

#[test]
fn maxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EAX, 1544982037, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 152, 21, 142, 22, 92], OperandSize::Dword)
}

#[test]
fn maxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 211], OperandSize::Qword)
}

#[test]
fn maxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 688350697, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 164, 202, 233, 101, 7, 41], OperandSize::Qword)
}

