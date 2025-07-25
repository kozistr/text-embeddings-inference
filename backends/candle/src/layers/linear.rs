use crate::layers::cublaslt::get_cublas_lt_wrapper;
use candle::{Device, Result, Tensor};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HiddenAct {
    Gelu,
    Relu,
    Silu,
    Swiglu,
}

impl HiddenAct {
    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        match self {
            Self::Gelu => x.gelu(),
            Self::Relu => x.relu(),
            Self::Silu => x.silu(),
            Self::Swiglu => candle_nn::ops::swiglu(x),
        }
    }
}

#[derive(Debug)]
pub struct Linear {
    weight: Tensor,
    bias: Option<Tensor>,
    act: Option<HiddenAct>,
    span: tracing::Span,
}

impl Linear {
    pub fn new(weight: Tensor, bias: Option<Tensor>, act: Option<HiddenAct>) -> Self {
        let span = tracing::span!(tracing::Level::TRACE, "linear");

        Self {
            weight,
            bias,
            act,
            span,
        }
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let _enter = self.span.enter();

        #[allow(unused)]
        if let (Device::Cuda(_), Some(cublaslt)) = (x.device(), get_cublas_lt_wrapper()) {
            match x.dims() {
                &[bsize, _, _] => cublaslt.batch_matmul(
                    &self.weight.broadcast_left(bsize)?,
                    x,
                    None,
                    None,
                    None,
                    self.bias.as_ref(),
                    self.act.clone(),
                ),
                _ => cublaslt.matmul(
                    &self.weight,
                    x,
                    None,
                    None,
                    None,
                    self.bias.as_ref(),
                    self.act.clone(),
                ),
            }
        } else {
            let (x, w) = match x.dims() {
                &[bsize, _, _] => (x, self.weight.broadcast_left(bsize)?.t()?),
                // Metal devices require contiguous tensors for 2D matrix multiplication apparently
                _ if matches!(x.device(), Device::Metal(_)) => (&x.contiguous()?, self.weight.t()?),
                _ => (x, self.weight.t()?),
            };
            let x = x.matmul(&w)?;

            let x = match &self.bias {
                None => Ok(x),
                Some(bias) => x.broadcast_add(bias),
            }?;

            if let Some(act) = &self.act {
                match act {
                    HiddenAct::Gelu => x.gelu(),
                    HiddenAct::Relu => x.relu(),
                    HiddenAct::Silu => x.silu(),
                    HiddenAct::Swiglu => candle_nn::ops::swiglu(&x),
                }
            } else {
                Ok(x)
            }
        }
    }
}
