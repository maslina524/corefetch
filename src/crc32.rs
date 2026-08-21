static TABLE: [u32; 256] = generate_table();

const fn generate_table() -> [u32; 256] {
    let mut table = [0u32; 256];

    let mut i = 0u32; 
    while i < 256 {
        let mut c = i << 24;
        let mut j = 8u32;
        while j > 0 {
            c = if c & 0x8000_0000 != 0 { 
                c << 1 ^ 0x04c1_1db7  
            } else { 
                c << 1 
            };

            j -= 1;
        }

        table[i as usize] = c;
        i += 1;
    }

    table
}

#[cfg(test)]
mod tests {
    use crate::crc32::TABLE;

    #[test]
    fn table_gen_test() {
        for i in &TABLE {
            print!("{i:08x} ");
        }
    }
}