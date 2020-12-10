use super::AocDay;
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    arg: i32,
}

impl Instruction {
    fn from_string(instr: &str) -> Self {
        let parts = instr.split(' ').collect::<Vec<&str>>();
        let first_char = parts[0].chars().next().unwrap();
        let arg = parts[1].parse::<i32>().unwrap();

        // match the fist character of the first part
        let opcode = match first_char {
            'n' => Opcode::Nop,
            'a' => Opcode::Acc,
            'j' => Opcode::Jmp,
            _ => unreachable!(),
        };
        Self { opcode, arg }
    }
}

#[derive(Default)]
struct Cpu {
    instructions: Vec<Instruction>,
    a: i32,
    pc: usize,
}

impl Cpu {
    fn reset(&mut self) {
        self.a = 0;
        self.pc = 0;
    }

    fn try_run(&mut self) -> bool {
        let mut executed_set = HashSet::<usize>::new();

        while !executed_set.contains(&self.pc) {
            executed_set.insert(self.pc);

            if self.pc >= self.instructions.len() {
                // normal execution
                return true;
            }

            let instr = &self.instructions[self.pc];

            self.pc += 1;

            match instr.opcode {
                Opcode::Nop => {}
                Opcode::Acc => self.a += instr.arg,
                Opcode::Jmp => {
                    self.pc = self.pc.wrapping_add(instr.arg as usize).wrapping_sub(1);
                }
            }
        }

        // infinite loop
        false
    }

    fn find_and_fix_bug(&mut self) {
        // used to keep track of which instruction we are modifying now
        let mut pc_counter = 0;

        loop {
            let mut instr = &mut self.instructions[pc_counter];

            while let Opcode::Acc = instr.opcode {
                pc_counter += 1;
                instr = &mut self.instructions[pc_counter];
            }

            let old_opcode = instr.opcode;

            instr.opcode = match instr.opcode {
                Opcode::Nop => Opcode::Jmp,
                Opcode::Jmp => Opcode::Nop,
                _ => unreachable!(),
            };

            drop(instr);

            self.reset();
            if self.try_run() {
                self.reset();
                break;
            }

            let instr = &mut self.instructions[pc_counter];
            instr.opcode = old_opcode;
            pc_counter += 1;
        }
    }
}

pub struct Day8;
impl AocDay for Day8 {
    fn run<R: BufRead>(reader: R) {
        let instructions = reader
            .lines()
            .filter_map(|l| l.ok())
            .take_while(|l| !l.is_empty())
            .map(|line| Instruction::from_string(&line))
            .collect::<Vec<Instruction>>();

        let mut cpu = Cpu {
            instructions,
            ..Cpu::default()
        };

        // assert that we are in an infinite loop
        assert!(!cpu.try_run());
        let p1 = cpu.a;

        cpu.find_and_fix_bug();
        // assert that we can run the code without infinite loop
        assert!(cpu.try_run());
        let p2 = cpu.a;

        println!("Part1: {}", p1);
        println!("Part2: {}", p2);
    }
}
