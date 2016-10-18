use std::env;

use gb::cpu::Cpu;
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]

fn ins00() { // NOP

}

// LD BC,d16
fn ins01() {

}

// LD (BC),A - 8
pub fn ins02(cpu: &mut Cpu) {
    cpu.reg_c = cpu.reg_a;
    cpu.reg_b = 0b00000000u8;
}

fn ins03() { // INC BC

}

fn ins04() { // INC B

}

fn ins05() { // DEC B

}

fn ins06() { // LD B,d8

}

fn ins07() { // RLCA

}

fn ins08() { // LD (a16),SP

}

fn ins09() { // ADD HL,BC

}
