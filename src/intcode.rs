#[derive(Copy, Clone)]
pub enum Instructions {
    Add,
    Mul,
    Halt,
    Unknown(usize),
}

impl From<usize> for Instructions {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Mul,
            99 => Self::Halt,
            v => Self::Unknown(v),
        }
    }
}

pub struct Cpu {
    prog: Vec<usize>,
    memory: Vec<usize>,
    eip: usize,
    is_halted: bool,
}

impl Cpu {
    #[must_use]
    pub fn new(prog: &[usize]) -> Self {
        Self {
            prog: prog.to_vec(),
            memory: prog.to_vec(),
            eip: 0_usize,
            is_halted: false,
        }
    }

    pub fn reset_to_value(&mut self, noun: usize, verb: usize) {
        self.memory = self.prog.clone();
        self.memory[1] = noun;
        self.memory[2] = verb;
        self.is_halted = false;
        self.eip = 0;
    }

    /// Run an instruction
    ///
    /// # Panics
    ///
    /// If match an unknow opcode
    pub fn run_next(&mut self) {
        match self.prog.get(self.eip) {
            Some(1) => {
                let a = self.memory[self.memory[self.eip + 1]];
                let b = self.memory[self.memory[self.eip + 2]];
                let dest = self.memory[self.eip + 3];
                self.memory[dest] = a + b;
                self.eip += 4;
            }
            Some(2) => {
                let a = self.memory[self.memory[self.eip + 1]];
                let b = self.memory[self.memory[self.eip + 2]];
                let dest = self.memory[self.eip + 3];
                self.memory[dest] = a * b;
                self.eip += 4;
            }
            Some(99) => self.is_halted = true,
            Some(opcode) => panic!("Unexpected opcode: {}", opcode),
            None => panic!("Bad EIP?: {}", self.eip),
        }
    }

    pub fn run(&mut self) {
        while !self.is_halted {
            self.run_next();
        }
    }

    #[must_use]
    pub fn get_register(&self, position: usize) -> usize {
        self.memory[position]
    }

    pub fn force_value(&mut self, position: usize, value: usize) {
        self.memory[position] = value;
    }

    #[must_use]
    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    #[must_use]
    pub fn run_all(&mut self) -> usize {
        while !self.is_halted() {
            self.run_next();
        }
        self.memory[0]
    }
}
