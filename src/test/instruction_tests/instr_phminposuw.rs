use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phminposuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 246], OperandSize::Dword)
}

#[test]
fn phminposuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1289360431, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 188, 208, 47, 20, 218, 76], OperandSize::Dword)
}

#[test]
fn phminposuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 205], OperandSize::Qword)
}

#[test]
fn phminposuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 2], OperandSize::Qword)
}

