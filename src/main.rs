use sha3::{Shake256, digest::{ExtendableOutput, Update, XofReader}};

const LAMBDA: usize = 256;

fn main() {
    let tree = merkle_tree(arr_to_vec(*b"Hello, world!"), vec![false; LAMBDA]);
    for byte in tree {
        print!("{}", byte as char);
    }
}

fn merkle_tree(m: Vec<bool>, s: Vec<bool>) -> [u8; 32] {
    dbg!(m.len());
    assert!(s.len() == LAMBDA);
    let mut hash = m.clone();

    if m.len() <= LAMBDA {
        let mut m = m.clone();
        while m.len() < LAMBDA {
            m.push(false);
        }
        return vec_to_arr(m);
    }

    while hash.len() > LAMBDA {
        let l = ((hash.len() as f64 / LAMBDA as f64).log2()).ceil() as u32;

        let mut m_prime = hash.clone();
        m_prime.append(&mut vec![false; 2_usize.pow(l) * LAMBDA - hash.len()]);

        let mut block_hashes: Vec<Vec<bool>> = Vec::new();

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

            block_hashes.push(arr_to_vec(result));
        }

        hash.clear();
        for block in &mut block_hashes {
            hash.append(block);
        }
    };
    
    vec_to_arr(hash)
}

fn arr_to_vec<const N: usize>(value: [u8; N]) -> Vec<bool> {
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