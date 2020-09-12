// MIT License
//
// Copyright (c) 2020 Mario Sieg (KerboGames)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::bytecode::*;
use super::record::*;

/// Executes the bytecode.
/// Returns the interrupt id (exitcode) and the number of cycles.
pub fn execute(mut code: BytecodeChunk) -> (i32, u64) {
    let mut cycles: u64 = 0; // Cycles counter.
    let mut opcode: u32 = 0; // Opcode.
    let mut interrupt: i32 = 0; // Interrupt id.

    loop {
        cycles += 1;
        opcode = code.fetch().u32();

        match opcode {
            asm::INTERRUPT => {
                interrupt = code.fetch().i32();
                if interrupt <= 0 {
                    break;
                } else {
                    // Trigger exception
                }
            },
            _ => (),
        }
    }

    (interrupt, cycles)
}