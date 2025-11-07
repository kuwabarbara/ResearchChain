use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

/// まずは雛形：論文メタデータ（必要最小限）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperMeta {
    pub title: String,
    pub author: String,
    pub hash_hex: String, // ファイルのSHA-256（16進）
}

/// フロントから受け取ってチェーンへ書く想定の関数群（今はスタブ）
pub struct ResearchContract;

impl ResearchContract {
    /// フロントから渡されたバイト列のハッシュを計算して PaperMeta を返す（将来ここでチェーン書き込み）
    pub fn register_paper(title: &str, author: &str, file_bytes: &[u8]) -> PaperMeta {
        let mut hasher = Sha256::new();
        hasher.update(file_bytes);
        let hash_hex = hex::encode(hasher.finalize());
        PaperMeta {
            title: title.to_string(),
            author: author.to_string(),
            hash_hex,
        }
    }
}
