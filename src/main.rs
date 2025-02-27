use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use zkplay::SparseMerkleTree;

type F = GoldilocksField;

fn main() {
    // 初始化稀疏Merkle树
    let mut tree = SparseMerkleTree::new();

    // 定义测试索引和值
    let index1 = [0u8; 32]; // 示例索引1（全0）
    let value1 = F::ONE;

    let mut index2 = [0u8; 32];
    index2[31] = 0x01; // 示例索引2（最后一个字节不同）
    let value2 = F::from_canonical_u64(42);

    // 插入值并获取根哈希
    tree.insert(index1, value1);
    tree.insert(index2, value2);
    let root = tree.root();

    let proof1 = tree.prove(&index1);
    let proof2 = tree.prove(&index2);

    assert!(SparseMerkleTree::verify_proof(&root, &index1, value1, &proof1));
    assert!(SparseMerkleTree::verify_proof(&root, &index2, value2, &proof2));

    assert!(!SparseMerkleTree::verify_proof(&root, &index1, F::TWO, &proof1));

    let mut non_existent_index = [0u8; 32];
    non_existent_index[0] = 0xFF;
    let non_existent_proof = tree.prove(&non_existent_index);
    assert!(!SparseMerkleTree::verify_proof(&root, &non_existent_index, F::ONE, &non_existent_proof));

    let mut non_existent_index = [0u8; 32];
    non_existent_index[0] = 0x59;
    let non_existent_proof = tree.prove(&non_existent_index);
    assert!(SparseMerkleTree::verify_proof(&root, &non_existent_index, F::ZERO, &non_existent_proof));

    println!("All tests passed!");
}