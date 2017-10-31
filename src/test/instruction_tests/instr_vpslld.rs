use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpslld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 114, 243, 122], OperandSize::Dword)
}

#[test]
fn vpslld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 114, 245, 14], OperandSize::Qword)
}

#[test]
fn vpslld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 114, 246, 47], OperandSize::Dword)
}

#[test]
fn vpslld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 114, 243, 22], OperandSize::Qword)
}

#[test]
fn vpslld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 114, 240, 38], OperandSize::Dword)
}

#[test]
fn vpslld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1677366043, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 114, 52, 157, 27, 147, 250, 99, 85], OperandSize::Dword)
}

#[test]
fn vpslld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 77, 154, 114, 52, 123, 53], OperandSize::Dword)
}

#[test]
fn vpslld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM8)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 21, 142, 114, 240, 83], OperandSize::Qword)
}

#[test]
fn vpslld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 127894402, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 114, 52, 133, 130, 131, 159, 7, 0], OperandSize::Qword)
}

#[test]
fn vpslld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDX, 348966789, Some(OperandSize::Dword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 157, 114, 178, 133, 207, 204, 20, 76], OperandSize::Qword)
}

#[test]
fn vpslld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 171, 114, 244, 46], OperandSize::Dword)
}

#[test]
fn vpslld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 173, 114, 52, 198, 107], OperandSize::Dword)
}

#[test]
fn vpslld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 109, 188, 114, 48, 118], OperandSize::Dword)
}

#[test]
fn vpslld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM17)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 109, 172, 114, 241, 62], OperandSize::Qword)
}

#[test]
fn vpslld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 827426441, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 169, 114, 180, 89, 137, 134, 81, 49, 39], OperandSize::Qword)
}

#[test]
fn vpslld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 2046876199, Some(OperandSize::Dword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 37, 177, 114, 52, 133, 39, 218, 0, 122, 124], OperandSize::Qword)
}

#[test]
fn vpslld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 202, 114, 242, 85], OperandSize::Dword)
}

#[test]
fn vpslld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 1945039879, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 205, 114, 180, 247, 7, 244, 238, 115, 17], OperandSize::Dword)
}

#[test]
fn vpslld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 117, 220, 114, 52, 131, 119], OperandSize::Dword)
}

#[test]
fn vpslld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM18)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 13, 198, 114, 242, 94], OperandSize::Qword)
}

#[test]
fn vpslld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM11)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 37, 203, 114, 50, 9], OperandSize::Qword)
}

#[test]
fn vpslld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 37, 217, 114, 52, 88, 121], OperandSize::Qword)
}

#[test]
fn vpslld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 242, 234], OperandSize::Dword)
}

#[test]
fn vpslld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1259896349, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 242, 4, 213, 29, 126, 24, 75], OperandSize::Dword)
}

#[test]
fn vpslld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 242, 201], OperandSize::Qword)
}

#[test]
fn vpslld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 242, 62], OperandSize::Qword)
}

#[test]
fn vpslld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 242, 226], OperandSize::Dword)
}

#[test]
fn vpslld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 1636732999, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 242, 170, 71, 144, 142, 97], OperandSize::Dword)
}

#[test]
fn vpslld_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 242, 238], OperandSize::Qword)
}

#[test]
fn vpslld_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 242, 58], OperandSize::Qword)
}

#[test]
fn vpslld_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 140, 242, 248], OperandSize::Dword)
}

#[test]
fn vpslld_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 275405654, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 142, 242, 60, 117, 86, 91, 106, 16], OperandSize::Dword)
}

#[test]
fn vpslld_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 61, 138, 242, 219], OperandSize::Qword)
}

#[test]
fn vpslld_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1043987045, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 242, 20, 213, 101, 250, 57, 62], OperandSize::Qword)
}

#[test]
fn vpslld_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 172, 242, 216], OperandSize::Dword)
}

#[test]
fn vpslld_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 928002917, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 170, 242, 140, 202, 101, 51, 80, 55], OperandSize::Dword)
}

#[test]
fn vpslld_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 37, 166, 242, 219], OperandSize::Qword)
}

#[test]
fn vpslld_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 37, 175, 242, 12, 211], OperandSize::Qword)
}

#[test]
fn vpslld_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 202, 242, 244], OperandSize::Dword)
}

#[test]
fn vpslld_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 242, 36, 82], OperandSize::Dword)
}

#[test]
fn vpslld_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 45, 196, 242, 220], OperandSize::Qword)
}

#[test]
fn vpslld_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 29, 196, 242, 40], OperandSize::Qword)
}

