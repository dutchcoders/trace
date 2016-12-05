use ptrace_util::Process;
use elftools::*;
use std::collections::HashMap;
use inferior::InferiorPointer;
use std::mem;

impl Process {
    pub fn is_elf(&mut self) -> bool {
        let base_addr = InferiorPointer(0x400000);

        let mut elf_header: [u8; 4] = [0, 0, 0, 0];
        self.peek_text_bytes(base_addr, &mut elf_header[0..]);

        elf_header == [b'\x7f', b'E', b'L', b'F']
    }

    pub fn ehdr(&mut self) -> Result<Elf64_Ehdr, ()> {
        let base_addr = InferiorPointer(0x400000);

        let ehdr: Elf64_Ehdr = self.read_struct::<Elf64_Ehdr>(base_addr).expect("Ehdr");
        Ok(ehdr)
    }

    pub fn program_headers(&mut self) -> Result< HashMap<Elf64_Word, Elf64_Phdr>, ()> {
        let ehdr = self.ehdr().unwrap();
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

    pub fn sections(&mut self, tag: i32 ) -> Result< Vec<InferiorPointer>, () > {
        let mut program_headers: HashMap<Elf64_Word, Elf64_Phdr> = self.program_headers().unwrap();

        let dynamic_section = program_headers.get(&(PT_DYNAMIC as Elf64_Word))
            .expect("pt_dynamic not found");

        let mut s = vec![];

        let mut dyn_addr = InferiorPointer(dynamic_section.p_vaddr as u64);

        loop {
            let dyn = self.read_struct::<Elf64_Dyn>(dyn_addr).expect("dyn");

            if dyn.d_tag == 0 {
                break;
            } else if dyn.d_tag == tag as i64 {
                s.push(InferiorPointer(dyn.d_ptr as u64))
            }

            dyn_addr = dyn_addr + mem::size_of::<Elf64_Dyn>() as i64;
        }

        Ok(s)
    }
}
