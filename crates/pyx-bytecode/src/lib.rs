pub const MAGIC: &[u8; 3] = b"JUZ";
pub const VERSION: [u8; 3] = [0, 1, 0];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    String(String),
    Integer(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    PushConst(u16),
    PushInt8(u8),
    Halt,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Program {
    constants: Vec<Constant>,
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_constant(&mut self, constant: Constant) -> Result<u16, EncodeError> {
        let index = self.constants.len();
        let index = u16::try_from(index).map_err(|_| EncodeError::TooManyConstants(index))?;
        self.constants.push(constant);
        Ok(index)
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION);
        write_section(
            &mut bytes,
            SectionTag::ConstantPool,
            &self.encode_constants()?,
        )?;
        write_section(
            &mut bytes,
            SectionTag::Bytecode,
            &self.encode_instructions(),
        )?;
        bytes.push(SectionTag::Eof as u8);
        Ok(bytes)
    }

    fn encode_constants(&self) -> Result<Vec<u8>, EncodeError> {
        let count = u16::try_from(self.constants.len())
            .map_err(|_| EncodeError::TooManyConstants(self.constants.len()))?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&count.to_le_bytes());

        for constant in &self.constants {
            match constant {
                Constant::String(value) => {
                    bytes.push(0x01);
                    let len = u32::try_from(value.len())
                        .map_err(|_| EncodeError::StringTooLong(value.len()))?;
                    bytes.extend_from_slice(&len.to_le_bytes());
                    bytes.extend_from_slice(value.as_bytes());
                }
                Constant::Integer(value) => {
                    bytes.push(0x02);
                    bytes.extend_from_slice(&value.to_le_bytes());
                }
            }
        }

        Ok(bytes)
    }

    fn encode_instructions(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for instruction in &self.instructions {
            match *instruction {
                Instruction::PushConst(index) => {
                    bytes.push(0x00);
                    bytes.extend_from_slice(&index.to_le_bytes());
                }
                Instruction::PushInt8(value) => {
                    bytes.push(0x01);
                    bytes.push(value);
                }
                Instruction::Halt => bytes.push(0x63),
            }
        }

        bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodeError {
    TooManyConstants(usize),
    StringTooLong(usize),
    SectionTooLong(usize),
}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodeError::TooManyConstants(count) => {
                write!(f, "constant pool has {count} entries, exceeding u16::MAX")
            }
            EncodeError::StringTooLong(len) => {
                write!(f, "string constant has {len} bytes, exceeding u32::MAX")
            }
            EncodeError::SectionTooLong(len) => {
                write!(f, "section payload has {len} bytes, exceeding u32::MAX")
            }
        }
    }
}

impl std::error::Error for EncodeError {}

#[repr(u8)]
enum SectionTag {
    ConstantPool = 0x01,
    Bytecode = 0x02,
    Eof = 0xff,
}

fn write_section(bytes: &mut Vec<u8>, tag: SectionTag, payload: &[u8]) -> Result<(), EncodeError> {
    let len =
        u32::try_from(payload.len()).map_err(|_| EncodeError::SectionTooLong(payload.len()))?;
    bytes.push(tag as u8);
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(payload);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Instruction, MAGIC, Program, VERSION};

    #[test]
    fn encodes_empty_program_with_halt() {
        let mut program = Program::new();
        program.push(Instruction::Halt);

        let bytes = program.encode().expect("program should encode");

        let mut expected = Vec::new();
        expected.extend_from_slice(MAGIC);
        expected.extend_from_slice(&VERSION);
        expected.extend_from_slice(&[0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00]);
        expected.extend_from_slice(&[0x02, 0x01, 0x00, 0x00, 0x00, 0x63]);
        expected.push(0xff);

        assert_eq!(bytes, expected);
    }

    #[test]
    fn encodes_integer_constant_and_push_const() {
        let mut program = Program::new();
        let index = program
            .add_constant(super::Constant::Integer(300))
            .expect("constant index should fit");
        program.push(Instruction::PushConst(index));
        program.push(Instruction::Halt);

        let bytes = program.encode().expect("program should encode");

        assert_eq!(&bytes[0..3], MAGIC);
        assert_eq!(bytes[6], 0x01);
        assert!(
            bytes
                .windows(8)
                .any(|window| window == 300_i64.to_le_bytes())
        );
        assert!(bytes.ends_with(&[0x02, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0xff]));
    }
}
