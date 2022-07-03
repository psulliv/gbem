// Todo: take these warning blockers out
#![allow(unreachable_code)]
#![allow(unused)]

use bitflags::bitflags;

bitflags! {
    struct ConditionFlags: u8 {
    const Z =  0b0000_0001;
    const CY = 0b0000_0010;
    const S =  0b0000_0100;
    const AC = 0b0000_1000;
    const P =  0b0001_0000;
    }
}

#[derive(PartialEq, Debug)]
pub struct ProcessorState {
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,
    stack_pointer: u16,
    prog_counter: usize,
    memory: Vec<u8>,
    flags: ConditionFlags,
}

impl ProcessorState {
    pub fn new() -> Self {
        ProcessorState {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            stack_pointer: 0,
            prog_counter: 0,
            memory: vec![0; 4096],
            flags: ConditionFlags {
                bits: (0b0000_0000),
            },
        }
    }
}

pub fn iterate_processor_state(state: &mut ProcessorState, mem_map: &[u8; 8192]) {
    // get the next opcode at the current program counter
    let cur_instruction = mem_map[state.prog_counter];
    match cur_instruction {
        0x00 => opcode_nop(state),
        0x01 => {
            panic!(" 	LXI B,D16	3		B <- byte 3, C <- byte 2");
            // 3
        }
        0x02 => {
            panic!(" 	STAX B	1		(BC) <- A");
            // 1
        }
        0x03 => {
            panic!(" 	INX B	1		BC <- BC+1");
            // 1
        }
        0x04 => {
            panic!(" 	INR B	1	Z, S, P, AC	B <- B+1");
            // 1
        }
        0x05 => {
            panic!(" 	DCR B	1	Z, S, P, AC	B <- B-1");
            // 1
        }
        0x06 => {
            panic!(" 	MVI B, D8	2		B <- byte 2");
            // 2
        }
        0x07 => {
            panic!(" 	RLC	1	CY	A = A << 1; bit 0 = prev bit 7; CY = prev bit 7");
            // 1
        }
        0x08 => {
            panic!(" 	-			");
            // 1
        }
        0x09 => {
            panic!(" 	DAD B	1	CY	HL = HL + BC");
            // 1
        }
        0x0a => {
            panic!(" 	LDAX B	1		A <- (BC)");
            // 1
        }
        0x0b => {
            panic!(" 	DCX B	1		BC = BC-1");
            // 1
        }
        0x0c => {
            panic!(" 	INR C	1	Z, S, P, AC	C <- C+1");
            // 1
        }
        0x0d => {
            panic!(" 	DCR C	1	Z, S, P, AC	C <-C-1");
            // 1
        }
        0x0e => {
            panic!(" 	MVI C,D8	2		C <- byte 2");
            // 2
        }
        0x0f => {
            panic!(" 	RRC	1	CY	A = A >> 1; bit 7 = prev bit 0; CY = prev bit 0");
            // 1
        }
        0x10 => {
            panic!(" 	-			");
            // 1
        }
        0x11 => {
            panic!(" 	LXI D,D16	3		D <- byte 3, E <- byte 2");
            // 3
        }
        0x12 => {
            panic!(" 	STAX D	1		(DE) <- A");
            // 1
        }
        0x13 => {
            panic!(" 	INX D	1		DE <- DE + 1");
            // 1
        }
        0x14 => {
            panic!(" 	INR D	1	Z, S, P, AC	D <- D+1");
            // 1
        }
        0x15 => {
            panic!(" 	DCR D	1	Z, S, P, AC	D <- D-1");
            // 1
        }
        0x16 => {
            panic!(" 	MVI D, D8	2		D <- byte 2");
            // 2
        }
        0x17 => {
            panic!(" 	RAL	1	CY	A = A << 1; bit 0 = prev CY; CY = prev bit 7");
            // 1
        }
        0x18 => {
            panic!(" 	-			");
            // 1
        }
        0x19 => {
            panic!(" 	DAD D	1	CY	HL = HL + DE");
            // 1
        }
        0x1a => {
            panic!(" 	LDAX D	1		A <- (DE)");
            // 1
        }
        0x1b => {
            panic!(" 	DCX D	1		DE = DE-1");
            // 1
        }
        0x1c => {
            panic!(" 	INR E	1	Z, S, P, AC	E <-E+1");
            // 1
        }
        0x1d => {
            panic!(" 	DCR E	1	Z, S, P, AC	E <- E-1");
            // 1
        }
        0x1e => {
            panic!(" 	MVI E,D8	2		E <- byte 2");
            // 2
        }
        0x1f => {
            panic!(" 	RAR	1	CY	A = A >> 1; bit 7 = prev bit 7; CY = prev bit 0");
            // 1
        }
        0x20 => {
            panic!(" 	-			");
            // 1
        }
        0x21 => {
            panic!(" 	LXI H,D16	3		H <- byte 3, L <- byte 2");
            // 3
        }
        0x22 => {
            panic!(" 	SHLD adr	3		(adr) <-L; (adr+1)<-H");
            // 3
        }
        0x23 => {
            panic!(" 	INX H	1		HL <- HL + 1");
            // 1
        }
        0x24 => {
            panic!(" 	INR H	1	Z, S, P, AC	H <- H+1");
            // 1
        }
        0x25 => {
            panic!(" 	DCR H	1	Z, S, P, AC	H <- H-1");
            // 1
        }
        0x26 => {
            panic!(" 	MVI H,D8	2		H <- byte 2");
            // 1
        }
        0x27 => {
            panic!(" 	DAA	1		special");
            // 1
        }
        0x28 => {
            panic!(" 	-			");
            // 1
        }
        0x29 => {
            panic!(" 	DAD H	1	CY	HL = HL + HI");
            // 1
        }
        0x2a => {
            panic!(" 	LHLD adr	3		L <- (adr); H<-(adr+1)");
            // 3
        }
        0x2b => {
            panic!(" 	DCX H	1		HL = HL-1");
            // 1
        }
        0x2c => {
            panic!(" 	INR L	1	Z, S, P, AC	L <- L+1");
            // 1
        }
        0x2d => {
            panic!(" 	DCR L	1	Z, S, P, AC	L <- L-1");
            // 1
        }
        0x2e => {
            panic!(" 	MVI L, D8	2		L <- byte 2");
            // 2
        }
        0x2f => {
            panic!(" 	CMA	1		A <- !A");
            // 1
        }
        0x30 => {
            panic!(" 	-			");
            // 1
        }
        0x31 => {
            panic!(" 	LXI SP, D16	3		SP.hi <- byte 3, SP.lo <- byte 2");
            // 3
        }
        0x32 => {
            panic!(" 	STA adr	3		(adr) <- A");
            // 3
        }
        0x33 => {
            panic!(" 	INX SP	1		SP = SP + 1");
            // 1
        }
        0x34 => {
            panic!(" 	INR M	1	Z, S, P, AC	(HL) <- (HL)+1");
            // 1
        }
        0x35 => {
            panic!(" 	DCR M	1	Z, S, P, AC	(HL) <- (HL)-1");
            // 1
        }
        0x36 => {
            panic!(" 	MVI M,D8	2		(HL) <- byte 2");
            // 2
        }
        0x37 => {
            panic!(" 	STC	1	CY	CY = 1");
            // 1
        }
        0x38 => {
            panic!(" 	-			");
            // 1
        }
        0x39 => {
            panic!(" 	DAD SP	1	CY	HL = HL + SP");
            // 1
        }
        0x3a => {
            panic!(" 	LDA adr	3		A <- (adr)");
            // 3
        }
        0x3b => {
            panic!(" 	DCX SP	1		SP = SP-1");
            // 1
        }
        0x3c => {
            panic!(" 	INR A	1	Z, S, P, AC	A <- A+1");
            // 1
        }
        0x3d => {
            panic!(" 	DCR A	1	Z, S, P, AC	A <- A-1");
            // 1
        }
        0x3e => {
            panic!(" 	MVI A,D8	2		A <- byte 2");
            // 2
        }
        0x3f => {
            panic!(" 	CMC	1	CY	CY=!CY");
            // 1
        }
        0x40 => {
            panic!(" 	MOV B,B	1		B <- B");
            // 1
        }
        0x41 => {
            panic!(" 	MOV B,C	1		B <- C");
            // 1
        }
        0x42 => {
            panic!(" 	MOV B,D	1		B <- D");
            // 1
        }
        0x43 => {
            panic!(" 	MOV B,E	1		B <- E");
            // 1
        }
        0x44 => {
            panic!(" 	MOV B,H	1		B <- H");
            // 1
        }
        0x45 => {
            panic!(" 	MOV B,L	1		B <- L");
            // 1
        }
        0x46 => {
            panic!(" 	MOV B,M	1		B <- (HL)");
            // 1
        }
        0x47 => {
            panic!(" 	MOV B,A	1		B <- A");
            // 1
        }
        0x48 => {
            panic!(" 	MOV C,B	1		C <- B");
            // 1
        }
        0x49 => {
            panic!(" 	MOV C,C	1		C <- C");
            // 1
        }
        0x4a => {
            panic!(" 	MOV C,D	1		C <- D");
            // 1
        }
        0x4b => {
            panic!(" 	MOV C,E	1		C <- E");
            // 1
        }
        0x4c => {
            panic!(" 	MOV C,H	1		C <- H");
            // 1
        }
        0x4d => {
            panic!(" 	MOV C,L	1		C <- L");
            // 1
        }
        0x4e => {
            panic!(" 	MOV C,M	1		C <- (HL)");
            // 1
        }
        0x4f => {
            panic!(" 	MOV C,A	1		C <- A");
            // 1
        }
        0x50 => {
            panic!(" 	MOV D,B	1		D <- B");
            // 1
        }
        0x51 => {
            panic!(" 	MOV D,C	1		D <- C");
            // 1
        }
        0x52 => {
            panic!(" 	MOV D,D	1		D <- D");
            // 1
        }
        0x53 => {
            panic!(" 	MOV D,E	1		D <- E");
            // 1
        }
        0x54 => {
            panic!(" 	MOV D,H	1		D <- H");
            // 1
        }
        0x55 => {
            panic!(" 	MOV D,L	1		D <- L");
            // 1
        }
        0x56 => {
            panic!(" 	MOV D,M	1		D <- (HL)");
            // 1
        }
        0x57 => {
            panic!(" 	MOV D,A	1		D <- A");
            // 1
        }
        0x58 => {
            panic!(" 	MOV E,B	1		E <- B");
            // 1
        }
        0x59 => {
            panic!(" 	MOV E,C	1		E <- C");
            // 1
        }
        0x5a => {
            panic!(" 	MOV E,D	1		E <- D");
            // 1
        }
        0x5b => {
            panic!(" 	MOV E,E	1		E <- E");
            // 1
        }
        0x5c => {
            panic!(" 	MOV E,H	1		E <- H");
            // 1
        }
        0x5d => {
            panic!(" 	MOV E,L	1		E <- L");
            // 1
        }
        0x5e => {
            panic!(" 	MOV E,M	1		E <- (HL)");
            // 1
        }
        0x5f => {
            panic!(" 	MOV E,A	1		E <- A");
            // 1
        }
        0x60 => {
            panic!(" 	MOV H,B	1		H <- B");
            // 1
        }
        0x61 => {
            panic!(" 	MOV H,C	1		H <- C");
            // 1
        }
        0x62 => {
            panic!(" 	MOV H,D	1		H <- D");
            // 1
        }
        0x63 => {
            panic!(" 	MOV H,E	1		H <- E");
            // 1
        }
        0x64 => {
            panic!(" 	MOV H,H	1		H <- H");
            // 1
        }
        0x65 => {
            panic!(" 	MOV H,L	1		H <- L");
            // 1
        }
        0x66 => {
            panic!(" 	MOV H,M	1		H <- (HL)");
            // 1
        }
        0x67 => {
            panic!(" 	MOV H,A	1		H <- A");
            // 1
        }
        0x68 => {
            panic!(" 	MOV L,B	1		L <- B");
            // 1
        }
        0x69 => {
            panic!(" 	MOV L,C	1		L <- C");
            // 1
        }
        0x6a => {
            panic!(" 	MOV L,D	1		L <- D");
            // 1
        }
        0x6b => {
            panic!(" 	MOV L,E	1		L <- E");
            // 1
        }
        0x6c => {
            panic!(" 	MOV L,H	1		L <- H");
            // 1
        }
        0x6d => {
            panic!(" 	MOV L,L	1		L <- L");
            // 1
        }
        0x6e => {
            panic!(" 	MOV L,M	1		L <- (HL)");
            // 1
        }
        0x6f => {
            panic!(" 	MOV L,A	1		L <- A");
            // 1
        }
        0x70 => {
            panic!(" 	MOV M,B	1		(HL) <- B");
            // 1
        }
        0x71 => {
            panic!(" 	MOV M,C	1		(HL) <- C");
            // 1
        }
        0x72 => {
            panic!(" 	MOV M,D	1		(HL) <- D");
            // 1
        }
        0x73 => {
            panic!(" 	MOV M,E	1		(HL) <- E");
            // 1
        }
        0x74 => {
            panic!(" 	MOV M,H	1		(HL) <- H");
            // 1
        }
        0x75 => {
            panic!(" 	MOV M,L	1		(HL) <- L");
            // 1
        }
        0x76 => {
            panic!(" 	HLT	1		special");
            // 1
        }
        0x77 => {
            panic!(" 	MOV M,A	1		(HL) <- A");
            // 1
        }
        0x78 => {
            panic!(" 	MOV A,B	1		A <- B");
            // 1
        }
        0x79 => {
            panic!(" 	MOV A,C	1		A <- C");
            // 1
        }
        0x7a => {
            panic!(" 	MOV A,D	1		A <- D");
            // 1
        }
        0x7b => {
            panic!(" 	MOV A,E	1		A <- E");
            // 1
        }
        0x7c => {
            panic!(" 	MOV A,H	1		A <- H");
            // 1
        }
        0x7d => {
            panic!(" 	MOV A,L	1		A <- L");
            // 1
        }
        0x7e => {
            panic!(" 	MOV A,M	1		A <- (HL)");
            // 1
        }
        0x7f => {
            panic!(" 	MOV A,A	1		A <- A");
            // 1
        }
        0x80 => {
            panic!(" 	ADD B	1	Z, S, P, CY, AC	A <- A + B");
            // 1
        }
        0x81 => {
            panic!(" 	ADD C	1	Z, S, P, CY, AC	A <- A + C");
            // 1
        }
        0x82 => {
            panic!(" 	ADD D	1	Z, S, P, CY, AC	A <- A + D");
            // 1
        }
        0x83 => {
            panic!(" 	ADD E	1	Z, S, P, CY, AC	A <- A + E");
            // 1
        }
        0x84 => {
            panic!(" 	ADD H	1	Z, S, P, CY, AC	A <- A + H");
            // 1
        }
        0x85 => {
            panic!(" 	ADD L	1	Z, S, P, CY, AC	A <- A + L");
            // 1
        }
        0x86 => {
            panic!(" 	ADD M	1	Z, S, P, CY, AC	A <- A + (HL)");
            // 1
        }
        0x87 => {
            panic!(" 	ADD A	1	Z, S, P, CY, AC	A <- A + A");
            // 1
        }
        0x88 => {
            panic!(" 	ADC B	1	Z, S, P, CY, AC	A <- A + B + CY");
            // 1
        }
        0x89 => {
            panic!(" 	ADC C	1	Z, S, P, CY, AC	A <- A + C + CY");
            // 1
        }
        0x8a => {
            panic!(" 	ADC D	1	Z, S, P, CY, AC	A <- A + D + CY");
            // 1
        }
        0x8b => {
            panic!(" 	ADC E	1	Z, S, P, CY, AC	A <- A + E + CY");
            // 1
        }
        0x8c => {
            panic!(" 	ADC H	1	Z, S, P, CY, AC	A <- A + H + CY");
            // 1
        }
        0x8d => {
            panic!(" 	ADC L	1	Z, S, P, CY, AC	A <- A + L + CY");
            // 1
        }
        0x8e => {
            panic!(" 	ADC M	1	Z, S, P, CY, AC	A <- A + (HL) + CY");
            // 1
        }
        0x8f => {
            panic!(" 	ADC A	1	Z, S, P, CY, AC	A <- A + A + CY");
            // 1
        }
        0x90 => {
            panic!(" 	SUB B	1	Z, S, P, CY, AC	A <- A - B");
            // 1
        }
        0x91 => {
            panic!(" 	SUB C	1	Z, S, P, CY, AC	A <- A - C");
            // 1
        }
        0x92 => {
            panic!(" 	SUB D	1	Z, S, P, CY, AC	A <- A + D");
            // 1
        }
        0x93 => {
            panic!(" 	SUB E	1	Z, S, P, CY, AC	A <- A - E");
            // 1
        }
        0x94 => {
            panic!(" 	SUB H	1	Z, S, P, CY, AC	A <- A + H");
            // 1
        }
        0x95 => {
            panic!(" 	SUB L	1	Z, S, P, CY, AC	A <- A - L");
            // 1
        }
        0x96 => {
            panic!(" 	SUB M	1	Z, S, P, CY, AC	A <- A + (HL)");
            // 1
        }
        0x97 => {
            panic!(" 	SUB A	1	Z, S, P, CY, AC	A <- A - A");
            // 1
        }
        0x98 => {
            panic!(" 	SBB B	1	Z, S, P, CY, AC	A <- A - B - CY");
            // 1
        }
        0x99 => {
            panic!(" 	SBB C	1	Z, S, P, CY, AC	A <- A - C - CY");
            // 1
        }
        0x9a => {
            panic!(" 	SBB D	1	Z, S, P, CY, AC	A <- A - D - CY");
            // 1
        }
        0x9b => {
            panic!(" 	SBB E	1	Z, S, P, CY, AC	A <- A - E - CY");
            // 1
        }
        0x9c => {
            panic!(" 	SBB H	1	Z, S, P, CY, AC	A <- A - H - CY");
            // 1
        }
        0x9d => {
            panic!(" 	SBB L	1	Z, S, P, CY, AC	A <- A - L - CY");
            // 1
        }
        0x9e => {
            panic!(" 	SBB M	1	Z, S, P, CY, AC	A <- A - (HL) - CY");
            // 1
        }
        0x9f => {
            panic!(" 	SBB A	1	Z, S, P, CY, AC	A <- A - A - CY");
            // 1
        }
        0xa0 => {
            panic!(" 	ANA B	1	Z, S, P, CY, AC	A <- A & B");
            // 1
        }
        0xa1 => {
            panic!(" 	ANA C	1	Z, S, P, CY, AC	A <- A & C");
            // 1
        }
        0xa2 => {
            panic!(" 	ANA D	1	Z, S, P, CY, AC	A <- A & D");
            // 1
        }
        0xa3 => {
            panic!(" 	ANA E	1	Z, S, P, CY, AC	A <- A & E");
            // 1
        }
        0xa4 => {
            panic!(" 	ANA H	1	Z, S, P, CY, AC	A <- A & H");
            // 1
        }
        0xa5 => {
            panic!(" 	ANA L	1	Z, S, P, CY, AC	A <- A & L");
            // 1
        }
        0xa6 => {
            panic!(" 	ANA M	1	Z, S, P, CY, AC	A <- A & (HL)");
            // 1
        }
        0xa7 => {
            panic!(" 	ANA A	1	Z, S, P, CY, AC	A <- A & A");
            // 1
        }
        0xa8 => {
            panic!(" 	XRA B	1	Z, S, P, CY, AC	A <- A ^ B");
            // 1
        }
        0xa9 => {
            panic!(" 	XRA C	1	Z, S, P, CY, AC	A <- A ^ C");
            // 1
        }
        0xaa => {
            panic!(" 	XRA D	1	Z, S, P, CY, AC	A <- A ^ D");
            // 1
        }
        0xab => {
            panic!(" 	XRA E	1	Z, S, P, CY, AC	A <- A ^ E");
            // 1
        }
        0xac => {
            panic!(" 	XRA H	1	Z, S, P, CY, AC	A <- A ^ H");
            // 1
        }
        0xad => {
            panic!(" 	XRA L	1	Z, S, P, CY, AC	A <- A ^ L");
            // 1
        }
        0xae => {
            panic!(" 	XRA M	1	Z, S, P, CY, AC	A <- A ^ (HL)");
            // 1
        }
        0xaf => {
            panic!(" 	XRA A	1	Z, S, P, CY, AC	A <- A ^ A");
            // 1
        }
        0xb0 => {
            panic!(" 	ORA B	1	Z, S, P, CY, AC	A <- A | B");
            // 1
        }
        0xb1 => {
            panic!(" 	ORA C	1	Z, S, P, CY, AC	A <- A | C");
            // 1
        }
        0xb2 => {
            panic!(" 	ORA D	1	Z, S, P, CY, AC	A <- A | D");
            // 1
        }
        0xb3 => {
            panic!(" 	ORA E	1	Z, S, P, CY, AC	A <- A | E");
            // 1
        }
        0xb4 => {
            panic!(" 	ORA H	1	Z, S, P, CY, AC	A <- A | H");
            // 1
        }
        0xb5 => {
            panic!(" 	ORA L	1	Z, S, P, CY, AC	A <- A | L");
            // 1
        }
        0xb6 => {
            panic!(" 	ORA M	1	Z, S, P, CY, AC	A <- A | (HL)");
            // 1
        }
        0xb7 => {
            panic!(" 	ORA A	1	Z, S, P, CY, AC	A <- A | A");
            // 1
        }
        0xb8 => {
            panic!(" 	CMP B	1	Z, S, P, CY, AC	A - B");
            // 1
        }
        0xb9 => {
            panic!(" 	CMP C	1	Z, S, P, CY, AC	A - C");
            // 1
        }
        0xba => {
            panic!(" 	CMP D	1	Z, S, P, CY, AC	A - D");
            // 1
        }
        0xbb => {
            panic!(" 	CMP E	1	Z, S, P, CY, AC	A - E");
            // 1
        }
        0xbc => {
            panic!(" 	CMP H	1	Z, S, P, CY, AC	A - H");
            // 1
        }
        0xbd => {
            panic!(" 	CMP L	1	Z, S, P, CY, AC	A - L");
            // 1
        }
        0xbe => {
            panic!(" 	CMP M	1	Z, S, P, CY, AC	A - (HL)");
            // 1
        }
        0xbf => {
            panic!(" 	CMP A	1	Z, S, P, CY, AC	A - A");
            // 1
        }
        0xc0 => {
            panic!(" 	RNZ	1		if NZ, RET");
            // 1
        }
        0xc1 => {
            panic!(" 	POP B	1		C <- (sp); B <- (sp+1); sp <- sp+2");
            // 1
        }
        0xc2 => {
            panic!(" 	JNZ adr	3		if NZ, PC <- adr");
            // 3
        }
        0xc3 => {
            panic!(" 	JMP adr	3		PC <= adr");
            // 3
        }
        0xc4 => {
            panic!(" 	CNZ adr	3		if NZ, CALL adr");
            // 3
        }
        0xc5 => {
            panic!(" 	PUSH B	1		(sp-2)<-C; (sp-1)<-B; sp <- sp - 2");
            // 1
        }
        0xc6 => {
            panic!(" 	ADI D8	2	Z, S, P, CY, AC	A <- A + byte");
            // 2
        }
        0xc7 => {
            panic!(" 	RST 0	1		CALL $0");
            // 1
        }
        0xc8 => {
            panic!(" 	RZ	1		if Z, RET");
            // 1
        }
        0xc9 => {
            panic!(" 	RET	1		PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2");
            // 1
        }
        0xca => {
            panic!(" 	JZ adr	3		if Z, PC <- adr");
            // 3
        }
        0xcb => {
            panic!(" 	-			");
            // 1
        }
        0xcc => {
            panic!(" 	CZ adr	3		if Z, CALL adr");
            // 3
        }
        0xcd => {
            panic!(" 	CALL adr	3		(SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr");
            // 3
        }
        0xce => {
            panic!(" 	ACI D8	2	Z, S, P, CY, AC	A <- A + data + CY");
            // 2
        }
        0xcf => {
            panic!(" 	RST 1	1		CALL $8");
            // 1
        }
        0xd0 => {
            panic!(" 	RNC	1		if NCY, RET");
            // 1
        }
        0xd1 => {
            panic!(" 	POP D	1		E <- (sp); D <- (sp+1); sp <- sp+2");
            // 1
        }
        0xd2 => {
            panic!(" 	JNC adr	3		if NCY, PC<-adr");
            // 3
        }
        0xd3 => {
            panic!(" 	OUT D8	2		special");
            // 2
        }
        0xd4 => {
            panic!(" 	CNC adr	3		if NCY, CALL adr");
            // 3
        }
        0xd5 => {
            panic!(" 	PUSH D	1		(sp-2)<-E; (sp-1)<-D; sp <- sp - 2");
            // 1
        }
        0xd6 => {
            panic!(" 	SUI D8	2	Z, S, P, CY, AC	A <- A - data");
            // 2
        }
        0xd7 => {
            panic!(" 	RST 2	1		CALL $10");
            // 1
        }
        0xd8 => {
            panic!(" 	RC	1		if CY, RET");
            // 1
        }
        0xd9 => {
            panic!(" 	-			");
            // 1
        }
        0xda => {
            panic!(" 	JC adr	3		if CY, PC<-adr");
            // 3
        }
        0xdb => {
            panic!(" 	IN D8	2		special");
            // 2
        }
        0xdc => {
            panic!(" 	CC adr	3		if CY, CALL adr");
            // 3
        }
        0xdd => {
            panic!(" 	-			");
            // 1
        }
        0xde => {
            panic!(" 	SBI D8	2	Z, S, P, CY, AC	A <- A - data - CY");
            // 2
        }
        0xdf => {
            panic!(" 	RST 3	1		CALL $18");
            // 1
        }
        0xe0 => {
            panic!(" 	RPO	1		if PO, RET");
            // 1
        }
        0xe1 => {
            panic!(" 	POP H	1		L <- (sp); H <- (sp+1); sp <- sp+2");
            // 1
        }
        0xe2 => {
            panic!(" 	JPO adr	3		if PO, PC <- adr");
            // 3
        }
        0xe3 => {
            panic!(" 	XTHL	1		L <-> (SP); H <-> (SP+1)");
            // 1
        }
        0xe4 => {
            panic!(" 	CPO adr	3		if PO, CALL adr");
            // 3
        }
        0xe5 => {
            panic!(" 	PUSH H	1		(sp-2)<-L; (sp-1)<-H; sp <- sp - 2");
            // 1
        }
        0xe6 => {
            panic!(" 	ANI D8	2	Z, S, P, CY, AC	A <- A & data");
            // 2
        }
        0xe7 => {
            panic!(" 	RST 4	1		CALL $20");
            // 1
        }
        0xe8 => {
            panic!(" 	RPE	1		if PE, RET");
            // 1
        }
        0xe9 => {
            panic!(" 	PCHL	1		PC.hi <- H; PC.lo <- L");
            // 1
        }
        0xea => {
            panic!(" 	JPE adr	3		if PE, PC <- adr");
            // 3
        }
        0xeb => {
            panic!(" 	XCHG	1		H <-> D; L <-> E");
            // 1
        }
        0xec => {
            panic!(" 	CPE adr	3		if PE, CALL adr");
            // 3
        }
        0xed => {
            panic!(" 	-			");
            // 1
        }
        0xee => {
            panic!(" 	XRI D8	2	Z, S, P, CY, AC	A <- A ^ data");
            // 2
        }
        0xef => {
            panic!(" 	RST 5	1		CALL $28");
            // 1
        }
        0xf0 => {
            panic!(" 	RP	1		if P, RET");
            // 1
        }
        0xf1 => {
            panic!(" 	POP PSW	1		flags <- (sp); A <- (sp+1); sp <- sp+2");
            // 1
        }
        0xf2 => {
            panic!(" 	JP adr	3		if P=1 PC <- adr");
            // 3
        }
        0xf3 => {
            panic!(" 	DI	1		special");
            // 1
        }
        0xf4 => {
            panic!(" 	CP adr	3		if P, PC <- adr");
            // 3
        }
        0xf5 => {
            panic!(" 	PUSH PSW	1		(sp-2)<-flags; (sp-1)<-A; sp <- sp - 2");
            // 1
        }
        0xf6 => {
            panic!(" 	ORI D8	2	Z, S, P, CY, AC	A <- A | data");
            // 2
        }
        0xf7 => {
            panic!(" 	RST 6	1		CALL $30");
            // 1
        }
        0xf8 => {
            panic!(" 	RM	1		if M, RET");
            // 1
        }
        0xf9 => {
            panic!(" 	SPHL	1		SP=HL");
            // 1
        }
        0xfa => {
            panic!(" 	JM adr	3		if M, PC <- adr");
            // 3
        }
        0xfb => {
            panic!(" 	EI	1		special");
            // 1
        }
        0xfc => {
            panic!(" 	CM adr	3		if M, CALL adr");
            // 3
        }
        0xfd => {
            panic!(" 	-			");
            // 1
        }
        0xfe => {
            panic!(" 	CPI D8	2	Z, S, P, CY, AC	A - data");
            // 2
        }
        0xff => {
            panic!(" 	RST 7	1		CALL $38");
            // 1
        }
    };
}

fn opcode_nop(state: &mut ProcessorState) {
    state.prog_counter += 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space_invaders_rom;
    #[test]
    fn basic_nop_step() {
        // This tests the very first instruction in the space invaders ROM file
        // which is a NOP
        let mut test_state = ProcessorState::new();
        iterate_processor_state(&mut test_state, &space_invaders_rom::SPACE_INVADERS_ROM);

        // Not a real test, just a placeholder
        assert!(test_state.reg_b == test_state.reg_a);
        // Todo: actually implement this test properly.
    }
}
