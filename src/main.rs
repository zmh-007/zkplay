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

    let middle = recursive_proof(&inner)?;
    let (_, c) = &middle;
    eprintln!(
        "Single recursion degree {} = 2^{}",
        c.common.degree(),
        c.common.degree_bits()
    );

    let outer = recursive_proof(&middle)?;
    let (proof_with_pis, c) = &outer;
    eprintln!(
        "Double recursion degree {} = 2^{}",
        c.common.degree(),
        c.common.degree_bits()
    );

    c.verify(proof_with_pis.clone())?;
    eprintln!("verify success");

    Ok(())
}