use std::path::PathBuf;

use crate::rom::Rom;

fn main() {
    let path = std::env::args().nth(1).expect("should contain path to binary");
    let path = PathBuf::from(path);

    let binary = std::fs::read(path).expect("should read binary");
    let elf_file = elf::ElfBytes::<elf::endian::AnyEndian>::minimal_parse(&binary)
        .expect("should parse ELF file");

    let _start = find_start_address(&elf_file);

    let _rom = Rom::new([0x00; 1024 * 1024]);
}

fn find_start_address(elf_file: &elf::ElfBytes<elf::endian::AnyEndian>) -> u64 {
    let common = elf_file.symbol_table().unwrap();
    if let Some((symbols, string_table)) = common {
        let start_sym = symbols
            .iter()
            .find(|sym| string_table.get(sym.st_name as usize).ok() == Some("_start"));

        match start_sym {
            Some(sym) => {
                return sym.st_value;
            }
            None => {
                println!("_start not found in .symtab");
            }
        }
    }

    elf_file.ehdr.e_entry
}

mod rom {
    pub struct Rom<const SIZE: usize>([u8; SIZE]);

    impl<const SIZE: usize> Rom<SIZE> {
        pub fn new(data: [u8; SIZE]) -> Self {
            Self(data)
        }
    }

    impl<const SIZE: usize> Default for Rom<SIZE> {
        fn default() -> Self {
            Self([0x00; SIZE])
        }
    }
}
