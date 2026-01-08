mod clock;
mod cpu;
mod flags;
mod gpu;
mod opcodes;

use flags::FlagsRegister;
use cpu::CPU;
use opcodes::load_opcodes;


/* 
Register codes
0, 000 -> B
1, 001 -> C
2, 010 -> D
3, 011 -> E
4, 100 -> H
5, 101 -> L
6, 110 -> HL
7, 111 -> A
*/

static mut RAM: [u8; 8192] = [0; 8192];
static mut VRAM: [u8; 8192] = [0; 8192];

fn main() {
    println!("main!");
    let (un, cb) = load_opcodes().unwrap();
    println!("{:?}", un.first());
    println!("{:?}", cb.first());

}
