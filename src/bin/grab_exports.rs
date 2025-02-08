use std::{
    env,
    fs::File,
    io::{self, BufReader, Read, Write},
    path::Path,
};

const DOS_HEADER_SIZE: usize = 64;
const FILE_HEADER_SIZE: usize = 20;
const SECTION_HEADER_SIZE: usize = 40;
const IMPORT_DESCRIPTOR_SIZE: usize = 20;

#[repr(C)]
struct ImageDosHeader {
    e_magic: [u8; 2],
    _unused: [u8; 58],
    e_lfanew: u32,
}

#[repr(C)]
struct ImageFileHeader {
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ImageSectionHeader {
    name: [u8; 8],
    virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    _rest: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ImageImportDescriptor {
    original_first_thunk: u32,
    time_date_stamp: u32,
    forwarder_chain: u32,
    name: u32,
    first_thunk: u32,
}

fn read_dos_header(file_bytes: &[u8]) -> Result<&ImageDosHeader, io::Error> {
    if file_bytes.len() < DOS_HEADER_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File too small to be a valid PE",
        ));
    }
    let dos_header: &ImageDosHeader = unsafe { &*(file_bytes.as_ptr() as *const ImageDosHeader) };
    if &dos_header.e_magic != b"MZ" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a valid PE file (missing MZ header)",
        ));
    }
    Ok(dos_header)
}

fn read_file_header<'a>(
    file_bytes: &'a [u8],
    dos_header: &'a ImageDosHeader,
) -> Result<&'a ImageFileHeader, io::Error> {
    let nt_header_offset = dos_header.e_lfanew as usize;
    if nt_header_offset + 4 > file_bytes.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid e_lfanew value",
        ));
    }
    if &file_bytes[nt_header_offset..nt_header_offset + 4] != b"PE\0\0" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Missing PE signature",
        ));
    }
    let file_header_offset = nt_header_offset + 4;
    if file_header_offset + FILE_HEADER_SIZE > file_bytes.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Incomplete File Header",
        ));
    }
    let file_header: &ImageFileHeader =
        unsafe { &*(file_bytes[file_header_offset..].as_ptr() as *const ImageFileHeader) };
    Ok(file_header)
}

fn read_section_headers(
    file_bytes: &[u8],
    file_header: &ImageFileHeader,
) -> Result<Vec<ImageSectionHeader>, io::Error> {
    let section_headers_offset = file_header.size_of_optional_header as usize;
    let num_sections = file_header.number_of_sections as usize;
    let mut sections = Vec::new();
    let mut offset = FILE_HEADER_SIZE + section_headers_offset;

    for _ in 0..num_sections {
        if offset + SECTION_HEADER_SIZE > file_bytes.len() {
            break;
        }

        let section = unsafe { *(file_bytes.as_ptr().add(offset) as *const ImageSectionHeader) };
        sections.push(section);
        offset += SECTION_HEADER_SIZE;
    }
    Ok(sections)
}

fn read_import_descriptors(
    file_bytes: &[u8],
    sections: &[ImageSectionHeader],
) -> Result<Vec<ImageImportDescriptor>, io::Error> {
    let mut import_descriptors = Vec::new();
    for section in sections {
        let start = section.pointer_to_raw_data as usize;
        if start + IMPORT_DESCRIPTOR_SIZE > file_bytes.len() {
            break;
        }
        let descriptor =
            unsafe { *(file_bytes.as_ptr().add(start) as *const ImageImportDescriptor) };
        import_descriptors.push(descriptor);
    }
    Ok(import_descriptors)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_dll_or_exe>", args[0]);
        return Ok(());
    }
    let pe_path = &args[1];
    let file = File::open(pe_path)?;
    let mut reader = BufReader::new(file);
    let mut file_bytes = Vec::new();
    reader.read_to_end(&mut file_bytes)?;

    let dos_header = read_dos_header(&file_bytes)?;
    let file_header = read_file_header(&file_bytes, dos_header)?;
    let sections = read_section_headers(&file_bytes, file_header)?;
    let import_descriptors = read_import_descriptors(&file_bytes, &sections)?;

    let mut output = String::new();
    output.push_str(&format!("PE Analysis for '{}'\n", pe_path));
    output.push_str("============================\n\n");

    output.push_str("DOS Header:\n");
    output.push_str(&format!("  e_lfanew: 0x{:X}\n\n", dos_header.e_lfanew));

    output.push_str("NT Header Signature: PE\\0\\0\n\n");

    output.push_str("File Header:\n");
    output.push_str(&format!("  machine: 0x{:X}\n", file_header.machine));
    output.push_str(&format!(
        "  number_of_sections: {}\n",
        file_header.number_of_sections
    ));
    output.push_str(&format!(
        "  time_date_stamp: 0x{:X}\n",
        file_header.time_date_stamp
    ));
    output.push_str(&format!(
        "  size_of_optional_header: {}\n\n",
        file_header.size_of_optional_header
    ));

    output.push_str("Section Headers:\n");
    for (i, s) in sections.iter().enumerate() {
        let name = std::str::from_utf8(&s.name)
            .unwrap_or("<invalid>")
            .trim_end_matches(char::from(0));
        output.push_str(&format!(
            "  Section {}: {:8}  VA: 0x{:X}  Size: 0x{:X}\n",
            i, name, s.virtual_address, s.size_of_raw_data
        ));
    }
    output.push_str("\n");

    output.push_str("Imports:\n");
    for descriptor in import_descriptors {
        output.push_str(&format!("  DLL name RVA: 0x{:X}\n", descriptor.name));
        output.push_str(&format!(
            "  time_date_stamp: 0x{:X}\n",
            descriptor.time_date_stamp
        ));
        output.push_str(&format!(
            "  forwarder_chain: 0x{:X}\n",
            descriptor.forwarder_chain
        ));
        output.push_str(&format!("  first_thunk: 0x{:X}\n", descriptor.first_thunk));
    }

    println!("{}", output);
    let output_file_name = {
        let p = Path::new(pe_path);
        if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
            format!("{}.txt", stem)
        } else {
            "output.txt".to_string()
        }
    };
    let mut file = File::create(&output_file_name)?;
    file.write_all(output.as_bytes())?;
    println!("Output saved to {}", output_file_name);

    Ok(())
}
