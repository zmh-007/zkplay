mod circuit;

use anyhow::Result;
use zkplay::recursive_proof;

fn main() -> Result<()> {
    let inner = circuit::build_base_circuit()?;
    let (_, c) = &inner;
    eprintln!(
        "Initial degree {} = 2^{}",
        c.common.degree(),
        c.common.degree_bits()
    );

    let second = recursive_proof(&inner)?;
    let (_, c) = &second;
    eprintln!(
        "Single recursion degree {} = 2^{}",
        c.common.degree(),
        c.common.degree_bits()
    );

    let third = recursive_proof(&second)?;
    let (proof_with_pis, c) = &third;
    eprintln!(
        "Double recursion degree {} = 2^{}",
        c.common.degree(),
        c.common.degree_bits()
    );

    c.verify(proof_with_pis.clone())?;
    eprintln!("verify success");

    Ok(())
}