// Homework group 34

use std::io::{self, Write};

use sha3::{Shake256, digest::{ExtendableOutput, Update, XofReader}};

const LAMBDA: usize = 256;

fn main() -> io::Result<()> {
    let mut s = String::new();
    let mut m = String::new();

    let stdin = io::stdin();
    print!("s = ");
    io::stdout().flush()?;
    stdin.read_line(&mut s)?;
    print!("m = ");
    io::stdout().flush()?;
    stdin.read_line(&mut m)?;

    let tree = merkle_tree(
        arr_to_vec(m.as_bytes().try_into().unwrap()),
        arr_to_vec(s.as_bytes().try_into().unwrap()),
    );
    for byte in tree {
        print!("{}", byte as char);
    }

    Ok(())
}

fn merkle_tree(m: Vec<bool>, s: Vec<bool>) -> [u8; 32] {
    let mut hash = m.clone();

    if hash.len() <= LAMBDA {       // Line 1 of the
        while hash.len() < LAMBDA { // original algorithm
            hash.push(false);       //
        }                           // Return m if LEN(m) too short
        return vec_to_arr(hash);    //
    }

    let mut s = s.clone();
    while s.len() < LAMBDA {
        s.push(false);
    }

    if s.len() > LAMBDA {
        s = s.first_chunk::<LAMBDA>().unwrap().to_vec();
    }

    // This is being done instead of recursion;
    // Rust doesn't have TCO so a recursive apporach
    // could easily lead to a stack overflow.
    while hash.len() > LAMBDA {
        let l = ((hash.len() as f64 / LAMBDA as f64).log2()).ceil() as u32; // Line 2

        let mut m_prime = hash.clone();                               // Line 3
        m_prime.append(&mut vec![false; 2_usize.pow(l) * LAMBDA - hash.len()]);  // Define m'

        let mut block_hashes: Vec<Vec<bool>> = Vec::new();

        // Line 4 and the loop associated with it.
        for i in 0..(2_usize.pow(l-1)) {
            let mut block_hash = s.clone();
            let idx = 2 * i * LAMBDA;
            block_hash.append(&mut (m_prime[idx..(idx + 2*LAMBDA)]).to_vec());
            assert!(block_hash.len() > LAMBDA);
            
            let mut hasher = Shake256::default();
            hasher.update(&vec_to_arr(block_hash));

            let mut reader = hasher.finalize_xof();
            let mut result = [0; 32];
            reader.read(&mut result);

            block_hashes.push(arr_to_vec(&result));
        }

        // Clear the current hash and replace it with
        // the concatenation of subhashes in order to
        // replicate the input to what would have been
        // the next recursive call in the original.
        hash.clear();
        for block in &mut block_hashes {
            hash.append(block);
        }
    };
    
    vec_to_arr(hash)
}

fn arr_to_vec(value: &[u8]) -> Vec<bool> {
    let mut result = Vec::new();

    for x in value {
        for i in 0..=7 {
            result.push((x & (1 << i)) != 0);
        }
    }

    result
}

fn vec_to_arr(value: Vec<bool>) -> [u8; 32] {
    std::array::from_fn(|i| {
        let mut byte = 0;
        for j in 0..=7 {
            byte |= (value[(i<<3)+j] as u8) << j;
        }
        byte
    })
}