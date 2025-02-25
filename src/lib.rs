use anyhow::Result;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::field::goldilocks_field::GoldilocksField;

pub type C = PoseidonGoldilocksConfig;
pub type F = GoldilocksField;
const D: usize = 2;

pub type ProofTuple<F, C> = (
    ProofWithPublicInputs<F, C, D>,
    CircuitData<F, C, D>,
);

pub fn recursive_proof(
    inner: &ProofTuple<F, C>,
) -> Result<ProofTuple<F, C>>
{
    let (inner_proof, inner_cd) = inner;
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_zk_config());
    let pt = builder.add_virtual_proof_with_pis(&inner_cd.common);

    let inner_data = builder.add_virtual_verifier_data(inner_cd.common.config.fri_config.cap_height);

    builder.verify_proof::<C>(&pt, &inner_data, &inner_cd.common);

    let data = builder.build::<C>();

    let mut pw = PartialWitness::new();
    pw.set_proof_with_pis_target(&pt, inner_proof)?;
    pw.set_verifier_data_target(&inner_data, &inner_cd.verifier_only)?;

    let proof = data.prove(pw)?;

    data.verify(proof.clone())?;

    Ok((proof, data))
}