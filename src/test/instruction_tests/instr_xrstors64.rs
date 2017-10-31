use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xrstors64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS64, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 766776608, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 199, 156, 201, 32, 21, 180, 45], OperandSize::Qword)
}

