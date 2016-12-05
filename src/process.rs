use ptrace_util::Process;
use inferior::*;
use elftools::*;
use std::collections::HashMap;

impl Process {
    pub fn is_elf(&mut self) -> bool {
        let base_addr = InferiorPointer(0x400000);

        let mut elf_header: [u8; 4] = [0, 0, 0, 0];
        self.peek_text_bytes(base_addr, &mut elf_header[0..]);

        return (elf_header == [b'\x7f', b'E', b'L', b'F'])
    }

    pub fn ehdr(&mut self) -> Result<Elf64_Ehdr, ()> {
        let base_addr = InferiorPointer(0x400000);

        let ehdr: Elf64_Ehdr = self.read_struct::<Elf64_Ehdr>(base_addr).expect("Ehdr");
        Ok(ehdr)
    }

    pub fn program_headers(&mut self) -> Result< HashMap<Elf64_Word, Elf64_Phdr>, ()> {
        let mut ehdr = self.ehdr().unwrap();
        let base_addr = InferiorPointer(0x400000);

        let mut program_headers: HashMap<Elf64_Word, Elf64_Phdr> = HashMap::new();

        for n in 0..ehdr.e_phnum {
            let phdr = self.read_struct::<Elf64_Phdr>(base_addr + ehdr.e_phoff as i64 +
                                                         (ehdr.e_phentsize as i64 * n as i64))
                .expect("Could not read elf phdr");

            program_headers.insert(phdr.p_type, phdr.clone());
        }

        Ok(program_headers)
    }
}
