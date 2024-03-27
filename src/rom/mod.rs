
pub fn initialise_rom() -> Box<dyn Fn(usize) -> u8> {
    let bytes_array = include_bytes!("../../gameboy/gb_bios.bin");
    let bytes_length = bytes_array.len();

    Box::new(move |index| {
        if index < bytes_length {
            return bytes_array[index];
        }
        panic!("Index out of bounds {}", index);
    })
}







