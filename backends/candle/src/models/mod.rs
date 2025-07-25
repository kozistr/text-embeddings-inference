#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

#[cfg(feature = "accelerate")]
extern crate accelerate_src;

mod bert;
mod dense;
mod distilbert;
mod jina;
mod jina_code;
mod mistral;
mod modernbert;
mod nomic;

#[cfg(feature = "cuda")]
mod flash_bert;

#[cfg(feature = "cuda")]
mod flash_jina;

#[cfg(feature = "cuda")]
mod flash_jina_code;

#[cfg(feature = "cuda")]
mod flash_nomic;

#[cfg(feature = "cuda")]
mod flash_distilbert;

#[cfg(feature = "cuda")]
mod flash_gte;

#[cfg(feature = "cuda")]
mod flash_mistral;

#[cfg(feature = "cuda")]
mod flash_qwen2;

#[cfg(feature = "cuda")]
mod flash_qwen3;

#[cfg(feature = "cuda")]
mod flash_modernbert;

mod gte;
mod mpnet;
mod qwen2;
mod qwen3;

pub use bert::{BertConfig, BertModel, PositionEmbeddingType};
use candle::{Result, Tensor};
pub use dense::{Dense, DenseConfig, DenseLayer};
pub use distilbert::{DistilBertConfig, DistilBertModel};
#[allow(unused_imports)]
pub use gte::{GTEClassificationHead, GTEConfig, GTEModel, GTEMLP};
pub use jina::JinaBertModel;
pub use jina_code::JinaCodeBertModel;
pub use mistral::MistralConfig;
pub use modernbert::{ModernBertConfig, ModernBertModel};
pub use mpnet::{MPNetConfig, MPNetModel};
pub use nomic::{NomicBertModel, NomicConfig};
pub use qwen2::Qwen2Config;
pub use qwen3::{Qwen3Config, Qwen3Model};
use text_embeddings_backend_core::Batch;

#[cfg(feature = "cuda")]
pub use flash_bert::FlashBertModel;

#[cfg(feature = "cuda")]
pub use flash_jina::FlashJinaBertModel;

#[cfg(feature = "cuda")]
pub use flash_jina_code::FlashJinaCodeBertModel;

#[cfg(feature = "cuda")]
pub use flash_nomic::FlashNomicBertModel;

#[cfg(feature = "cuda")]
pub use flash_distilbert::FlashDistilBertModel;

#[cfg(feature = "cuda")]
pub use flash_mistral::FlashMistralModel;

#[cfg(feature = "cuda")]
pub use flash_gte::FlashGTEModel;

#[cfg(feature = "cuda")]
pub use flash_qwen2::FlashQwen2Model;

#[cfg(feature = "cuda")]
pub use flash_qwen3::FlashQwen3Model;

#[cfg(feature = "cuda")]
pub use flash_modernbert::FlashModernBertModel;

pub(crate) trait Model {
    fn is_padded(&self) -> bool;

    fn embed(&self, _batch: Batch) -> Result<(Option<Tensor>, Option<Tensor>)> {
        candle::bail!("`embed` is not implemented for this model");
    }

    fn predict(&self, _batch: Batch) -> Result<Tensor> {
        candle::bail!("`predict` is not implemented for this model");
    }
}
