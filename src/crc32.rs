// SOURCE:
// https://github.com/gcc-mirror/gcc/blob/master/libiberty/crc32.c

static TABLE: [u32; 256] = generate_table();
const INIT: u32 = 0;

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

const fn encrypt(buf: &[u8]) -> u32 {
    let mut crc = INIT;
    let mut pos = 0usize;

    while pos < buf.len() {
        let index = ((crc >> 24) ^ buf[pos] as u32) & 255;
        crc = (crc << 8) ^ TABLE[index as usize];
        pos += 1;
    }

    crc
}

#[cfg(test)]
mod tests {
    use crate::crc32;

    #[test]
    fn table_gen_test() {
        for i in &crc32::TABLE {
            print!("{i:08x} ");
        }
    }

    #[test]
    fn encrypt_test() {
        let source = "Hello World";
        let correct_hash = 0x4A_17_B1_56u32;
        let my_hash = crc32::encrypt(source.as_bytes());
        assert_eq!(correct_hash, my_hash);
    }
}