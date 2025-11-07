use contracts::ResearchContract;

#[test]
fn hash_should_be_sha256_hex() {
    let title = "paper v1";
    let author = "nobuaki";
    let bytes = b"hello world";
    let meta = ResearchContract::register_paper(title, author, bytes);
    assert_eq!(meta.title, title);
    assert_eq!(meta.author, author);
    // 期待される SHA-256("hello world")
    assert_eq!(
        meta.hash_hex,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}
