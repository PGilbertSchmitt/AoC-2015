use md5::Context;

fn generate_hash(base: &[u8], padding: usize) -> [u8; 16] {
    let mut ctx = Context::new();
    ctx.consume(base);
    ctx.consume(padding.to_string().as_bytes());
    ctx.compute().0
}

fn count_hash_zeros(hash: [u8; 16]) -> usize {
    let mut num_zeros = 0;
    for byte in hash.into_iter() {
        if byte == 0 {
            num_zeros += 2;
        } else if byte < 0x10 {
            return num_zeros + 1;
        } else {
            return num_zeros;
        }
    }
    return 32;
}

fn find_hash_input(input: &[u8], num_zeros: usize) -> usize {
    let mut pad = 1;
    loop {
        let hash = generate_hash(input, pad);
        if count_hash_zeros(hash) >= num_zeros {
            return pad;
        }
        pad += 1;
    }
}

static S1: &[u8] = b"abcdef";
static S2: &[u8] = b"pqrstuv";
static INPUT: &[u8] = b"yzbqklnj";

#[test]
fn part_1() {
    assert_eq!(609043, find_hash_input(S1, 5));
    assert_eq!(1048970, find_hash_input(S2, 5));
    assert_eq!(282749, find_hash_input(INPUT, 5));
}

#[test]
fn part_2() {
    // The samples add a whole 1.6 seconds in release mode
    // assert_eq!(6742839, find_hash_input(S1, 6));
    // assert_eq!(5714438, find_hash_input(S2, 6));
    assert_eq!(9962624, find_hash_input(INPUT, 6));
}
