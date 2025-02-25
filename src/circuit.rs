use anyhow::Result;
use zkplay::ProofTuple;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::field::types::Field;

pub type C = PoseidonGoldilocksConfig;
pub type F = GoldilocksField;
const D: usize = 2;

pub fn build_base_circuit() -> Result<ProofTuple<F, C>> {
    let config = CircuitConfig::standard_recursion_zk_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);
    
    let a = builder.add_virtual_target();
    let b = builder.add_virtual_target();
    let c = builder.add_virtual_target();
    
    builder.register_public_input(c);
    
    let sum = builder.add(a, b);
    builder.connect(sum, c);
    
    let mut pw = PartialWitness::new();
    pw.set_target(a, F::ONE)?;
    pw.set_target(b, F::ONE)?;
    pw.set_target(c, F::TWO)?;
    let data = builder.build::<C>();
    let proof = data.prove(pw)?;

    Ok((proof, data))
}