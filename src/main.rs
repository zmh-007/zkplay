use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use zkplay::SparseMerkleTree;

type F = GoldilocksField;

fn main() {
    let mut tree = SparseMerkleTree::new();

    let index1 = [0u8; 32];
    let value1 = F::ONE;

    let mut index2 = [0u8; 32];
    index2[31] = 0x01;
    let value2 = F::from_canonical_u64(42);

    tree.insert(index1, value1);
    tree.insert(index2, value2);
    let root = tree.root();

    let (value1, proof1) = tree.prove(&index1);
    let (value2, proof2) = tree.prove(&index2);

    assert!(value1 == F::ONE);
    assert!(value2 == F::from_canonical_u64(42));
    assert!(tree.verify_proof(&root, &index1, &proof1));
    assert!(tree.verify_proof(&root, &index2, &proof2));

    assert!(SparseMerkleTree::verify_proof_with_data(&root, &index1, value1, &proof1));
    assert!(SparseMerkleTree::verify_proof_with_data(&root, &index2, value2, &proof2));
    assert!(!SparseMerkleTree::verify_proof_with_data(&root, &index1, F::TWO, &proof1));

    let mut non_existent_index = [0u8; 32];
    non_existent_index[0] = 0xFF;
    let (value, non_existent_proof) = tree.prove(&non_existent_index);
    assert!(!tree.verify_proof(&root, &non_existent_index, &non_existent_proof));
    assert!(value == F::ZERO);

    let (_, non_existent_proof) = tree.prove(&non_existent_index);
    assert!(!SparseMerkleTree::verify_proof_with_data(&root, &non_existent_index, F::ONE, &non_existent_proof));

    let mut non_existent_index = [0u8; 32];
    non_existent_index[0] = 0x59;
    let (_, non_existent_proof) = tree.prove(&non_existent_index);
    assert!(SparseMerkleTree::verify_proof_with_data(&root, &non_existent_index, F::ZERO, &non_existent_proof));


    println!("All tests passed!");
}