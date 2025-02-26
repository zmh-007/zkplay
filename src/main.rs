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

    // 生成证明并验证
    let proof1 = tree.prove(&index1);
    let proof2 = tree.prove(&index2);

    // 验证正确的证明
    assert!(SparseMerkleTree::verify_proof(&root, &index1, value1, &proof1));
    assert!(SparseMerkleTree::verify_proof(&root, &index2, value2, &proof2));

    // 验证错误的证明（错误的值）
    assert!(!SparseMerkleTree::verify_proof(&root, &index1, F::TWO, &proof1));

    // 验证不存在的索引（应失败）
    let mut non_existent_index = [0u8; 32];
    non_existent_index[0] = 0xFF;
    let non_existent_proof = tree.prove(&non_existent_index);
    assert!(!SparseMerkleTree::verify_proof(&root, &non_existent_index, F::ONE, &non_existent_proof));

    println!("All tests passed!");
}