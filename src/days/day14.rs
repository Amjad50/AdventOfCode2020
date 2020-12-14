use aoc_derive::impl_day;
use std::collections::HashMap;

const BIT_36: u64 = 0xFFFFFFFFF;

#[derive(Debug, Default)]
struct MaskInfo {
    mask_str: String,
    and_mask: u64,
    value_override: u64,
}

#[derive(Debug)]
enum Operation {
    Mask(MaskInfo),
    Mem { addr: u64, val: u64 },
}

#[derive(Default)]
struct DockingSystem {
    memory: HashMap<u64, u64>,
    mask_info: MaskInfo,
    floating_bits_buffer: Vec<usize>,
}

impl DockingSystem {
    fn reset(&mut self) {
        self.memory.clear();
        self.mask_info = MaskInfo::default();
    }

    fn put_mem_v1(&mut self, addr: u64, val: u64) {
        let val = val & self.mask_info.and_mask & BIT_36;
        let val = val | self.mask_info.value_override;

        self.memory.insert(addr, val);
    }

    fn put_mem_v2(&mut self, addr: u64, val: u64) {
        let addr = addr & (self.mask_info.and_mask ^ BIT_36);
        let addr_unfloated = addr | self.mask_info.value_override;

        for i in 0..2u64.pow(self.floating_bits_buffer.len() as u32) {
            let mut current_addr = addr_unfloated;
            for (j, floating_index) in self.floating_bits_buffer.iter().enumerate() {
                let bit = (i >> j) & 1;
                current_addr |= bit << floating_index;
            }
            self.memory.insert(current_addr, val);
        }
    }

    fn run_operations(&mut self, operations: &[Operation], v2: bool) {
        for op in operations {
            match op {
                Operation::Mask(mask_info) => {
                    self.mask_info = MaskInfo {
                        mask_str: mask_info.mask_str.to_string(),
                        ..*mask_info
                    };
                    // reload floating_bits buffer when mask is changed
                    self.floating_bits_buffer.clear();
                    let and_mask = self.mask_info.and_mask;

                    self.floating_bits_buffer
                        .extend((0..36).filter(|i| and_mask & (1 << i) != 0));
                }
                &Operation::Mem { addr, val } => {
                    if v2 {
                        self.put_mem_v2(addr, val);
                    } else {
                        self.put_mem_v1(addr, val);
                    }
                }
            }
        }
    }

    fn memory_sum(&self) -> u64 {
        self.memory.iter().map(|(_, v)| v).sum()
    }
}

impl_day!(14, |reader| {
    let operations: Vec<_> = reader
        .lines()
        .filter_map(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .filter_map(|l| {
            if let Some(l) = l.strip_prefix("mask = ") {
                let and_mask = l.replace('1', "0").replace('X', "1");
                let and_mask = u64::from_str_radix(&and_mask, 2).unwrap();

                let value_override = l.replace('X', "0");
                let value_override = u64::from_str_radix(&value_override, 2).unwrap();

                Some(Operation::Mask(MaskInfo {
                    mask_str: l.to_string(),
                    and_mask,
                    value_override,
                }))
            } else {
                if let Some(l) = l.strip_prefix("mem[") {
                    let parts = l
                        .split("] = ")
                        .filter_map(|p| p.parse::<u64>().ok())
                        .collect::<Vec<u64>>();
                    Some(Operation::Mem {
                        addr: parts[0],
                        val: parts[1],
                    })
                } else {
                    None
                }
            }
        })
        .collect();

    let mut docking_system = DockingSystem::default();
    docking_system.run_operations(&operations, false);
    let p1 = docking_system.memory_sum();

    docking_system.reset();
    docking_system.run_operations(&operations, true);
    let p2 = docking_system.memory_sum();

    println!("Part1: {}", p1);
    println!("Part2: {}", p2);
});
