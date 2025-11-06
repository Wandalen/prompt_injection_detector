// Generated from ONNX "../../artifacts/model.onnx" by burn-import
extern crate alloc;
use alloc::vec::Vec;
use burn::nn::LayerNorm;
use burn::nn::LayerNormConfig;
use burn::nn::Linear;
use burn::nn::LinearConfig;
use burn::prelude::*;
use burn::record::HalfPrecisionSettings;
use burn::record::Recorder;
use core::ops::Neg;

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    constant25: burn::module::Param<Tensor<B, 2>>,
    layernormalization1: LayerNorm<B>,
    matmul1: Linear<B>,
    constant33: burn::module::Param<Tensor<B, 3>>,
    constant71: burn::module::Param<Tensor<B, 1>>,
    matmul5: Linear<B>,
    layernormalization2: LayerNorm<B>,
    matmul6: Linear<B>,
    matmul7: Linear<B>,
    layernormalization3: LayerNorm<B>,
    matmul8: Linear<B>,
    constant92: burn::module::Param<Tensor<B, 3>>,
    constant128: burn::module::Param<Tensor<B, 1>>,
    matmul12: Linear<B>,
    layernormalization4: LayerNorm<B>,
    matmul13: Linear<B>,
    matmul14: Linear<B>,
    layernormalization5: LayerNorm<B>,
    matmul15: Linear<B>,
    constant175: burn::module::Param<Tensor<B, 1>>,
    matmul18: Linear<B>,
    layernormalization6: LayerNorm<B>,
    matmul19: Linear<B>,
    matmul20: Linear<B>,
    layernormalization7: LayerNorm<B>,
    matmul21: Linear<B>,
    constant222: burn::module::Param<Tensor<B, 1>>,
    matmul24: Linear<B>,
    layernormalization8: LayerNorm<B>,
    matmul25: Linear<B>,
    matmul26: Linear<B>,
    layernormalization9: LayerNorm<B>,
    matmul27: Linear<B>,
    constant269: burn::module::Param<Tensor<B, 1>>,
    matmul30: Linear<B>,
    layernormalization10: LayerNorm<B>,
    matmul31: Linear<B>,
    matmul32: Linear<B>,
    layernormalization11: LayerNorm<B>,
    matmul33: Linear<B>,
    constant316: burn::module::Param<Tensor<B, 1>>,
    matmul36: Linear<B>,
    layernormalization12: LayerNorm<B>,
    matmul37: Linear<B>,
    matmul38: Linear<B>,
    layernormalization13: LayerNorm<B>,
    matmul39: Linear<B>,
    constant363: burn::module::Param<Tensor<B, 1>>,
    matmul42: Linear<B>,
    layernormalization14: LayerNorm<B>,
    matmul43: Linear<B>,
    matmul44: Linear<B>,
    layernormalization15: LayerNorm<B>,
    matmul45: Linear<B>,
    constant410: burn::module::Param<Tensor<B, 1>>,
    matmul48: Linear<B>,
    layernormalization16: LayerNorm<B>,
    matmul49: Linear<B>,
    matmul50: Linear<B>,
    layernormalization17: LayerNorm<B>,
    matmul51: Linear<B>,
    constant457: burn::module::Param<Tensor<B, 1>>,
    matmul54: Linear<B>,
    layernormalization18: LayerNorm<B>,
    matmul55: Linear<B>,
    matmul56: Linear<B>,
    layernormalization19: LayerNorm<B>,
    matmul57: Linear<B>,
    constant504: burn::module::Param<Tensor<B, 1>>,
    matmul60: Linear<B>,
    layernormalization20: LayerNorm<B>,
    matmul61: Linear<B>,
    matmul62: Linear<B>,
    layernormalization21: LayerNorm<B>,
    matmul63: Linear<B>,
    constant551: burn::module::Param<Tensor<B, 1>>,
    matmul66: Linear<B>,
    layernormalization22: LayerNorm<B>,
    matmul67: Linear<B>,
    matmul68: Linear<B>,
    layernormalization23: LayerNorm<B>,
    matmul69: Linear<B>,
    constant598: burn::module::Param<Tensor<B, 1>>,
    matmul72: Linear<B>,
    layernormalization24: LayerNorm<B>,
    matmul73: Linear<B>,
    matmul74: Linear<B>,
    layernormalization25: LayerNorm<B>,
    matmul75: Linear<B>,
    constant645: burn::module::Param<Tensor<B, 1>>,
    matmul78: Linear<B>,
    layernormalization26: LayerNorm<B>,
    matmul79: Linear<B>,
    matmul80: Linear<B>,
    layernormalization27: LayerNorm<B>,
    matmul81: Linear<B>,
    constant692: burn::module::Param<Tensor<B, 1>>,
    matmul84: Linear<B>,
    layernormalization28: LayerNorm<B>,
    matmul85: Linear<B>,
    matmul86: Linear<B>,
    layernormalization29: LayerNorm<B>,
    matmul87: Linear<B>,
    constant739: burn::module::Param<Tensor<B, 1>>,
    matmul90: Linear<B>,
    layernormalization30: LayerNorm<B>,
    matmul91: Linear<B>,
    matmul92: Linear<B>,
    layernormalization31: LayerNorm<B>,
    matmul93: Linear<B>,
    constant786: burn::module::Param<Tensor<B, 1>>,
    matmul96: Linear<B>,
    layernormalization32: LayerNorm<B>,
    matmul97: Linear<B>,
    matmul98: Linear<B>,
    layernormalization33: LayerNorm<B>,
    matmul99: Linear<B>,
    constant833: burn::module::Param<Tensor<B, 1>>,
    matmul102: Linear<B>,
    layernormalization34: LayerNorm<B>,
    matmul103: Linear<B>,
    matmul104: Linear<B>,
    layernormalization35: LayerNorm<B>,
    matmul105: Linear<B>,
    constant880: burn::module::Param<Tensor<B, 1>>,
    matmul108: Linear<B>,
    layernormalization36: LayerNorm<B>,
    matmul109: Linear<B>,
    matmul110: Linear<B>,
    layernormalization37: LayerNorm<B>,
    matmul111: Linear<B>,
    constant927: burn::module::Param<Tensor<B, 1>>,
    matmul114: Linear<B>,
    layernormalization38: LayerNorm<B>,
    matmul115: Linear<B>,
    matmul116: Linear<B>,
    layernormalization39: LayerNorm<B>,
    matmul117: Linear<B>,
    constant974: burn::module::Param<Tensor<B, 1>>,
    matmul120: Linear<B>,
    layernormalization40: LayerNorm<B>,
    matmul121: Linear<B>,
    matmul122: Linear<B>,
    layernormalization41: LayerNorm<B>,
    matmul123: Linear<B>,
    constant1021: burn::module::Param<Tensor<B, 1>>,
    matmul126: Linear<B>,
    layernormalization42: LayerNorm<B>,
    matmul127: Linear<B>,
    matmul128: Linear<B>,
    layernormalization43: LayerNorm<B>,
    matmul129: Linear<B>,
    constant1068: burn::module::Param<Tensor<B, 1>>,
    matmul132: Linear<B>,
    layernormalization44: LayerNorm<B>,
    matmul133: Linear<B>,
    matmul134: Linear<B>,
    layernormalization45: LayerNorm<B>,
    constant1084: burn::module::Param<Tensor<B, 1, Int>>,
    matmul135: Linear<B>,
    layernormalization46: LayerNorm<B>,
    gemm1: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    device: burn::module::Ignored<B::Device>,
}

impl<B: Backend> Default for Model<B> {
    fn default() -> Self {
        Self::from_file("src/burn_impl/model/model", &Default::default())
    }
}

impl<B: Backend> Model<B> {
    pub fn from_file(file: &str, device: &B::Device) -> Self {
        let record = burn::record::NamedMpkFileRecorder::<HalfPrecisionSettings>::new()
            .load(file.into(), device)
            .expect("Record file to exist.");
        Self::new(device).load_record(record)
    }
}

impl<B: Backend> Model<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let constant25: burn::module::Param<Tensor<B, 2>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 2>::zeros([50368, 768], device),
            device.clone(),
            false,
            [50368, 768].into(),
        );
        let layernormalization1 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul1 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant33: burn::module::Param<Tensor<B, 3>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 3>::zeros([1, 32, 1], device),
            device.clone(),
            false,
            [1, 32, 1].into(),
        );
        let constant71: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul5 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization2 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul6 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul7 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization3 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul8 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant92: burn::module::Param<Tensor<B, 3>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 3>::zeros([1, 32, 1], device),
            device.clone(),
            false,
            [1, 32, 1].into(),
        );
        let constant128: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul12 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization4 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul13 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul14 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization5 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul15 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant175: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul18 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization6 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul19 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul20 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization7 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul21 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant222: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul24 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization8 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul25 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul26 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization9 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul27 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant269: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul30 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization10 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul31 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul32 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization11 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul33 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant316: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul36 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization12 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul37 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul38 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization13 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul39 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant363: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul42 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization14 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul43 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul44 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization15 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul45 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant410: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul48 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization16 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul49 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul50 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization17 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul51 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant457: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul54 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization18 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul55 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul56 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization19 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul57 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant504: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul60 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization20 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul61 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul62 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization21 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul63 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant551: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul66 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization22 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul67 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul68 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization23 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul69 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant598: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul72 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization24 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul73 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul74 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization25 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul75 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant645: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul78 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization26 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul79 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul80 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization27 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul81 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant692: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul84 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization28 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul85 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul86 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization29 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul87 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant739: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul90 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization30 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul91 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul92 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization31 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul93 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant786: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul96 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization32 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul97 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul98 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization33 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul99 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant833: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul102 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization34 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul103 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul104 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization35 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul105 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant880: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul108 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization36 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul109 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul110 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization37 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul111 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant927: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul114 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization38 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul115 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul116 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization39 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul117 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant974: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul120 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization40 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul121 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul122 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization41 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul123 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant1021: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul126 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization42 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul127 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul128 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization43 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul129 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let constant1068: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 1>::zeros([1], device),
            device.clone(),
            false,
            [1].into(),
        );
        let matmul132 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization44 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let matmul133 = LinearConfig::new(768, 2304).with_bias(false).init(device);
        let matmul134 = LinearConfig::new(1152, 768).with_bias(false).init(device);
        let layernormalization45 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let constant1084: burn::module::Param<Tensor<B, 1, Int>> =
            burn::module::Param::uninitialized(
                burn::module::ParamId::new(),
                move |device, _require_grad| Tensor::<B, 1, Int>::zeros([1], device),
                device.clone(),
                false,
                [1].into(),
            );
        let matmul135 = LinearConfig::new(768, 768).with_bias(false).init(device);
        let layernormalization46 = LayerNormConfig::new(768)
            .with_epsilon(0.000009999999747378752f64)
            .init(device);
        let gemm1 = LinearConfig::new(768, 2).with_bias(true).init(device);
        Self {
            constant25,
            layernormalization1,
            matmul1,
            constant33,
            constant71,
            matmul5,
            layernormalization2,
            matmul6,
            matmul7,
            layernormalization3,
            matmul8,
            constant92,
            constant128,
            matmul12,
            layernormalization4,
            matmul13,
            matmul14,
            layernormalization5,
            matmul15,
            constant175,
            matmul18,
            layernormalization6,
            matmul19,
            matmul20,
            layernormalization7,
            matmul21,
            constant222,
            matmul24,
            layernormalization8,
            matmul25,
            matmul26,
            layernormalization9,
            matmul27,
            constant269,
            matmul30,
            layernormalization10,
            matmul31,
            matmul32,
            layernormalization11,
            matmul33,
            constant316,
            matmul36,
            layernormalization12,
            matmul37,
            matmul38,
            layernormalization13,
            matmul39,
            constant363,
            matmul42,
            layernormalization14,
            matmul43,
            matmul44,
            layernormalization15,
            matmul45,
            constant410,
            matmul48,
            layernormalization16,
            matmul49,
            matmul50,
            layernormalization17,
            matmul51,
            constant457,
            matmul54,
            layernormalization18,
            matmul55,
            matmul56,
            layernormalization19,
            matmul57,
            constant504,
            matmul60,
            layernormalization20,
            matmul61,
            matmul62,
            layernormalization21,
            matmul63,
            constant551,
            matmul66,
            layernormalization22,
            matmul67,
            matmul68,
            layernormalization23,
            matmul69,
            constant598,
            matmul72,
            layernormalization24,
            matmul73,
            matmul74,
            layernormalization25,
            matmul75,
            constant645,
            matmul78,
            layernormalization26,
            matmul79,
            matmul80,
            layernormalization27,
            matmul81,
            constant692,
            matmul84,
            layernormalization28,
            matmul85,
            matmul86,
            layernormalization29,
            matmul87,
            constant739,
            matmul90,
            layernormalization30,
            matmul91,
            matmul92,
            layernormalization31,
            matmul93,
            constant786,
            matmul96,
            layernormalization32,
            matmul97,
            matmul98,
            layernormalization33,
            matmul99,
            constant833,
            matmul102,
            layernormalization34,
            matmul103,
            matmul104,
            layernormalization35,
            matmul105,
            constant880,
            matmul108,
            layernormalization36,
            matmul109,
            matmul110,
            layernormalization37,
            matmul111,
            constant927,
            matmul114,
            layernormalization38,
            matmul115,
            matmul116,
            layernormalization39,
            matmul117,
            constant974,
            matmul120,
            layernormalization40,
            matmul121,
            matmul122,
            layernormalization41,
            matmul123,
            constant1021,
            matmul126,
            layernormalization42,
            matmul127,
            matmul128,
            layernormalization43,
            matmul129,
            constant1068,
            matmul132,
            layernormalization44,
            matmul133,
            matmul134,
            layernormalization45,
            constant1084,
            matmul135,
            layernormalization46,
            gemm1,
            phantom: core::marker::PhantomData,
            device: burn::module::Ignored(device.clone()),
        }
    }

    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(&self, input1: Tensor<B, 2, Int>, input2: Tensor<B, 2, Int>) -> Tensor<B, 2> {
        let shape1_out1: [i64; 2] = input1.clone().dims()[0..2]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant1_out1: i64 = 1i64;
        let actual_idx = if constant1_out1 < 0 {
            (shape1_out1.len() as i64 + constant1_out1) as usize
        } else {
            constant1_out1 as usize
        };
        let gather1_out1 = shape1_out1[actual_idx] as i64;
        let cast1_out1 = gather1_out1;
        let constant2_out1: i64 = 0i64;
        let range1_out1 =
            Tensor::arange_step(constant2_out1..cast1_out1, 1i64 as usize, &*self.device);
        let unsqueeze1_out1: Tensor<B, 2, Int> = range1_out1.unsqueeze_dims(&[0]);
        let shape2_out1: [i64; 2] = input2.clone().dims()[0..2]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant5_out1: i64 = 0i64;
        let actual_idx = if constant5_out1 < 0 {
            (shape2_out1.len() as i64 + constant5_out1) as usize
        } else {
            constant5_out1 as usize
        };
        let gather2_out1 = shape2_out1[actual_idx] as i64;
        let shape3_out1: [i64; 2] = input2.clone().dims()[0..2]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant6_out1: i64 = 1i64;
        let actual_idx = if constant6_out1 < 0 {
            (shape3_out1.len() as i64 + constant6_out1) as usize
        } else {
            constant6_out1 as usize
        };
        let gather3_out1 = shape3_out1[actual_idx] as i64;
        let unsqueeze2_out1: Tensor<B, 3, Int> = input2.clone().unsqueeze_dims(&[1]);
        let unsqueeze3_out1: Tensor<B, 4, Int> = unsqueeze2_out1.unsqueeze_dims(&[2]);
        let unsqueeze4_out1 = [gather2_out1];
        let constant10_out1: [i64; 1] = [1i64];
        let unsqueeze5_out1 = [gather3_out1];
        let unsqueeze6_out1 = [gather3_out1];
        let concat1_out1: [i64; 4usize] = [
            &unsqueeze4_out1[..],
            &constant10_out1[..],
            &unsqueeze5_out1[..],
            &unsqueeze6_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape1_out1 = concat1_out1;
        let shape4_out1: [i64; 1] = [4i64];
        let constantofshape1_out1: [i64; 1] = [1i64];
        let constant14_out1: i64 = -1i64;
        let mul1_out1 = {
            let mut result = constantofshape1_out1;
            for result_item in result.iter_mut() {
                *result_item = result_item.saturating_mul(constant14_out1 as i64);
            }
            result
        };
        let equal1_out1 = {
            let mut result = reshape1_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(mul1_out1.iter()) {
                *result_item = if result_item == rhs_item { 1i64 } else { 0i64 };
            }
            result
        };
        let where1_out1 = {
            let mut result = reshape1_out1;
            for (i, (cond_item, x_item)) in equal1_out1
                .iter()
                .zip(constantofshape1_out1.iter())
                .enumerate()
            {
                if *cond_item != 0 {
                    result[i] = *x_item;
                }
            }
            result
        };
        let expand1_out1 = unsqueeze3_out1.expand(where1_out1);
        let cast2_out1 = expand1_out1.float();
        let constant15_out1: f32 = 1f32;
        let sub1_out1 = -cast2_out1.sub_scalar(constant15_out1);
        let cast3_out1 = sub1_out1.clone().bool();
        let cast4_out1 = cast3_out1;
        let constant16_out1: f32 = -340282350000000000000000000000000000000f32;
        let where2_out1 = sub1_out1.mask_fill(cast4_out1, constant16_out1);
        let shape5_out1: [i64; 4] = where2_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant17_out1: i64 = 2i64;
        let actual_idx = if constant17_out1 < 0 {
            (shape5_out1.len() as i64 + constant17_out1) as usize
        } else {
            constant17_out1 as usize
        };
        let gather4_out1 = shape5_out1[actual_idx] as i64;
        let cast5_out1 = gather4_out1;
        let constant18_out1: i64 = 0i64;
        let range2_out1 =
            Tensor::arange_step(constant18_out1..cast5_out1, 1i64 as usize, &*self.device);
        let unsqueeze7_out1: Tensor<B, 2, Int> = range2_out1.unsqueeze_dims(&[0]);
        let transpose1_out1 = unsqueeze7_out1.clone().permute([1, 0]);
        let sub2_out1 = unsqueeze7_out1.sub(transpose1_out1);
        let abs1_out1 = sub2_out1.abs();
        let constant21_out1: i64 = 64i64;
        let lessorequal1_out1 = abs1_out1.lower_equal_elem(constant21_out1);
        let unsqueeze8_out1: Tensor<B, 3, Bool> = lessorequal1_out1.unsqueeze_dims(&[0]);
        let unsqueeze9_out1: Tensor<B, 4, Bool> = unsqueeze8_out1.unsqueeze_dims(&[0]);
        let cast6_out1 = unsqueeze9_out1;
        let cast7_out1 = cast6_out1;
        let not1_out1 = cast7_out1.bool_not();
        let cast8_out1 = not1_out1;
        let constant24_out1: f32 = -340282350000000000000000000000000000000f32;
        let where3_out1 = where2_out1.clone().mask_fill(cast8_out1, constant24_out1);
        let constant25_out1 = self.constant25.val();
        let gather5_out1 = constant25_out1.take::<2, 3>(0, input1);
        let layernormalization1_out1 = {
            let dtype = gather5_out1.dtype();
            self.layernormalization1
                .forward(gather5_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul1_out1 = self.matmul1.forward(layernormalization1_out1.clone());
        let shape6_out1: [i64; 3] = layernormalization1_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant27_out1: i64 = 0i64;
        let actual_idx = if constant27_out1 < 0 {
            (shape6_out1.len() as i64 + constant27_out1) as usize
        } else {
            constant27_out1 as usize
        };
        let gather6_out1 = shape6_out1[actual_idx] as i64;
        let unsqueeze10_out1 = [gather6_out1];
        let constant29_out1: [i64; 1] = [-1i64];
        let constant30_out1: [i64; 1] = [3i64];
        let constant31_out1: [i64; 1] = [12i64];
        let constant32_out1: [i64; 1] = [64i64];
        let concat2_out1: [i64; 5usize] = [
            &unsqueeze10_out1[..],
            &constant29_out1[..],
            &constant30_out1[..],
            &constant31_out1[..],
            &constant32_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape2_out1 = matmul1_out1.reshape(concat2_out1);
        let constant33_out1 = self.constant33.val();
        let shape7_out1: [i64; 2] = unsqueeze1_out1.clone().dims()[0..2]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant34_out1: i64 = 0i64;
        let actual_idx = if constant34_out1 < 0 {
            (shape7_out1.len() as i64 + constant34_out1) as usize
        } else {
            constant34_out1 as usize
        };
        let gather7_out1 = shape7_out1[actual_idx] as i64;
        let unsqueeze11_out1 = [gather7_out1];
        let constant36_out1: [i64; 1] = [-1i64];
        let constant37_out1: [i64; 1] = [1i64];
        let concat3_out1: [i64; 3usize] = [
            &unsqueeze11_out1[..],
            &constant36_out1[..],
            &constant37_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape3_out1 = concat3_out1;
        let shape8_out1: [i64; 1] = [3i64];
        let constantofshape2_out1: [i64; 1] = [1i64];
        let constant39_out1: i64 = -1i64;
        let mul2_out1 = {
            let mut result = constantofshape2_out1;
            for result_item in result.iter_mut() {
                *result_item = result_item.saturating_mul(constant39_out1 as i64);
            }
            result
        };
        let equal2_out1 = {
            let mut result = reshape3_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(mul2_out1.iter()) {
                *result_item = if result_item == rhs_item { 1i64 } else { 0i64 };
            }
            result
        };
        let where4_out1 = {
            let mut result = reshape3_out1;
            for (i, (cond_item, x_item)) in equal2_out1
                .iter()
                .zip(constantofshape2_out1.iter())
                .enumerate()
            {
                if *cond_item != 0 {
                    result[i] = *x_item;
                }
            }
            result
        };
        let expand2_out1 = constant33_out1.expand(where4_out1);
        let cast9_out1 = expand2_out1;
        let unsqueeze12_out1: Tensor<B, 3, Int> = unsqueeze1_out1.unsqueeze_dims(&[1]);
        let cast10_out1 = unsqueeze12_out1.float();
        let cast11_out1 = cast9_out1;
        let cast12_out1 = cast10_out1;
        let matmul2_out1 = cast11_out1.matmul(cast12_out1.clone());
        let transpose2_out1 = matmul2_out1.permute([0, 2, 1]);
        let concat4_out1 =
            burn::tensor::Tensor::cat([transpose2_out1.clone(), transpose2_out1].into(), 2);
        let cos1_out1 = concat4_out1.clone().cos();
        let constant41_out1: f32 = 1f32;
        let mul3_out1 = cos1_out1.mul_scalar(constant41_out1);
        let sin1_out1 = concat4_out1.sin();
        let constant42_out1: f32 = 1f32;
        let mul4_out1 = sin1_out1.mul_scalar(constant42_out1);
        let cast13_out1 = mul3_out1;
        let cast14_out1 = mul4_out1;
        let transpose3_out1 = reshape2_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose3_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split1_out1, split1_out2, split1_out3] = split_tensors.try_into().unwrap();
        let squeeze1_out1 = split1_out1.squeeze_dims(&[2]);
        let squeeze2_out1 = split1_out2.squeeze_dims(&[2]);
        let squeeze3_out1 = split1_out3.squeeze_dims(&[2]);
        let unsqueeze13_out1: Tensor<B, 4> = cast13_out1.unsqueeze_dims(&[1]);
        let unsqueeze14_out1: Tensor<B, 4> = cast14_out1.unsqueeze_dims(&[1]);
        let mul5_out1 = squeeze1_out1.clone().mul(unsqueeze13_out1.clone());
        let shape9_out1: [i64; 4] = squeeze1_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant49_out1: i64 = 3i64;
        let actual_idx = if constant49_out1 < 0 {
            (shape9_out1.len() as i64 + constant49_out1) as usize
        } else {
            constant49_out1 as usize
        };
        let gather8_out1 = shape9_out1[actual_idx] as i64;
        let constant50_out1: i64 = 2i64;
        let div1_out1 = gather8_out1 / constant50_out1;
        let cast15_out1 = div1_out1;
        let cast16_out1 = cast15_out1;
        let unsqueeze15_out1 = [cast16_out1];
        let slice1_out1 = squeeze1_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze15_out1[0]]);
        let unsqueeze16_out1 = [cast16_out1];
        let slice2_out1 =
            squeeze1_out1.slice(s![.., .., .., unsqueeze16_out1[0]..9223372036854775807]);
        let neg1_out1 = slice2_out1.neg();
        let concat5_out1 = burn::tensor::Tensor::cat([neg1_out1, slice1_out1].into(), 3);
        let mul6_out1 = concat5_out1.mul(unsqueeze14_out1.clone());
        let add1_out1 = mul5_out1.add(mul6_out1);
        let mul7_out1 = squeeze2_out1.clone().mul(unsqueeze13_out1.clone());
        let shape10_out1: [i64; 4] = squeeze2_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant59_out1: i64 = 3i64;
        let actual_idx = if constant59_out1 < 0 {
            (shape10_out1.len() as i64 + constant59_out1) as usize
        } else {
            constant59_out1 as usize
        };
        let gather9_out1 = shape10_out1[actual_idx] as i64;
        let constant60_out1: i64 = 2i64;
        let div2_out1 = gather9_out1 / constant60_out1;
        let cast17_out1 = div2_out1;
        let cast18_out1 = cast17_out1;
        let unsqueeze17_out1 = [cast18_out1];
        let slice3_out1 = squeeze2_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze17_out1[0]]);
        let unsqueeze18_out1 = [cast18_out1];
        let slice4_out1 =
            squeeze2_out1.slice(s![.., .., .., unsqueeze18_out1[0]..9223372036854775807]);
        let neg2_out1 = slice4_out1.neg();
        let concat6_out1 = burn::tensor::Tensor::cat([neg2_out1, slice3_out1].into(), 3);
        let mul8_out1 = concat6_out1.mul(unsqueeze14_out1.clone());
        let add2_out1 = mul7_out1.add(mul8_out1);
        let shape11_out1: [i64; 4] = add1_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice5_out1: [i64; 1] = shape11_out1[3..4].try_into().unwrap();
        let cast19_out1 = {
            let shape_array = slice5_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt1_out1 = cast19_out1.sqrt();
        let constant71_out1 = self.constant71.val();
        let div3_out1 = constant71_out1.div(sqrt1_out1);
        let cast20_out1 = div3_out1;
        let transpose4_out1 = add2_out1.permute([0, 1, 3, 2]);
        let sqrt2_out1 = cast20_out1.clone().sqrt();
        let mul9_out1 = add1_out1.mul(sqrt2_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt3_out1 = cast20_out1.sqrt();
        let mul10_out1 = transpose4_out1.mul(sqrt3_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul3_out1 = mul9_out1.matmul(mul10_out1);
        let add3_out1 = matmul3_out1.add(where2_out1.clone());
        let softmax1_out1 = burn::tensor::activation::softmax(add3_out1, 3);
        let matmul4_out1 = softmax1_out1.matmul(squeeze3_out1);
        let transpose5_out1 = matmul4_out1.permute([0, 2, 1, 3]);
        let unsqueeze19_out1 = [gather6_out1];
        let constant73_out1: [i64; 1] = [-1i64];
        let constant74_out1: [i64; 1] = [768i64];
        let concat7_out1: [i64; 3usize] = [
            &unsqueeze19_out1[..],
            &constant73_out1[..],
            &constant74_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape4_out1 = transpose5_out1.reshape(concat7_out1);
        let matmul5_out1 = self.matmul5.forward(reshape4_out1);
        let add4_out1 = layernormalization1_out1.add(matmul5_out1);
        let layernormalization2_out1 = {
            let dtype = add4_out1.clone().dtype();
            self.layernormalization2
                .forward(add4_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul6_out1 = self.matmul6.forward(layernormalization2_out1);
        let shape12_out1: [i64; 3] = matmul6_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant76_out1: [i64; 1] = [-1i64];
        let gather10_out1: [i64; 1usize] = constant76_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape12_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape12_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant78_out1: [i64; 1] = [1i64];
        let add5_out1 = {
            let mut result = gather10_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant78_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant79_out1: [i64; 1] = [2i64];
        let div4_out1 = {
            let mut result = add5_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant79_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant80_out1: [i64; 1] = [1i64];
        let mul11_out1 = {
            let mut result = div4_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant80_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice6_out1 = matmul6_out1.clone().slice(s![.., .., 0..mul11_out1[0]]);
        let constant81_out1: [i64; 1] = [2i64];
        let mul12_out1 = {
            let mut result = div4_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant81_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice7_out1 = matmul6_out1.slice(s![.., .., mul11_out1[0]..mul12_out1[0]]);
        let constant82_out1: f32 = 1.4142135f32;
        let div5_out1 = slice6_out1.clone().div_scalar(constant82_out1);
        let erf1_out1 = div5_out1.erf();
        let constant83_out1: f32 = 1f32;
        let add6_out1 = erf1_out1.add_scalar(constant83_out1);
        let mul13_out1 = slice6_out1.mul(add6_out1);
        let constant84_out1: f32 = 0.5f32;
        let mul14_out1 = mul13_out1.mul_scalar(constant84_out1);
        let mul15_out1 = mul14_out1.mul(slice7_out1);
        let matmul7_out1 = self.matmul7.forward(mul15_out1);
        let add7_out1 = add4_out1.add(matmul7_out1);
        let layernormalization3_out1 = {
            let dtype = add7_out1.clone().dtype();
            self.layernormalization3
                .forward(add7_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul8_out1 = self.matmul8.forward(layernormalization3_out1.clone());
        let shape13_out1: [i64; 3] = layernormalization3_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant86_out1: i64 = 0i64;
        let actual_idx = if constant86_out1 < 0 {
            (shape13_out1.len() as i64 + constant86_out1) as usize
        } else {
            constant86_out1 as usize
        };
        let gather11_out1 = shape13_out1[actual_idx] as i64;
        let unsqueeze20_out1 = [gather11_out1];
        let constant88_out1: [i64; 1] = [-1i64];
        let constant89_out1: [i64; 1] = [3i64];
        let constant90_out1: [i64; 1] = [12i64];
        let constant91_out1: [i64; 1] = [64i64];
        let concat8_out1: [i64; 5usize] = [
            &unsqueeze20_out1[..],
            &constant88_out1[..],
            &constant89_out1[..],
            &constant90_out1[..],
            &constant91_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape5_out1 = matmul8_out1.reshape(concat8_out1);
        let constant92_out1 = self.constant92.val();
        let unsqueeze21_out1 = [gather7_out1];
        let constant94_out1: [i64; 1] = [-1i64];
        let constant95_out1: [i64; 1] = [1i64];
        let concat9_out1: [i64; 3usize] = [
            &unsqueeze21_out1[..],
            &constant94_out1[..],
            &constant95_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape6_out1 = concat9_out1;
        let shape14_out1: [i64; 1] = [3i64];
        let constantofshape3_out1: [i64; 1] = [1i64];
        let constant97_out1: i64 = -1i64;
        let mul16_out1 = {
            let mut result = constantofshape3_out1;
            for result_item in result.iter_mut() {
                *result_item = result_item.saturating_mul(constant97_out1 as i64);
            }
            result
        };
        let equal3_out1 = {
            let mut result = reshape6_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(mul16_out1.iter()) {
                *result_item = if result_item == rhs_item { 1i64 } else { 0i64 };
            }
            result
        };
        let where5_out1 = {
            let mut result = reshape6_out1;
            for (i, (cond_item, x_item)) in equal3_out1
                .iter()
                .zip(constantofshape3_out1.iter())
                .enumerate()
            {
                if *cond_item != 0 {
                    result[i] = *x_item;
                }
            }
            result
        };
        let expand3_out1 = constant92_out1.expand(where5_out1);
        let cast21_out1 = expand3_out1;
        let cast22_out1 = cast21_out1;
        let matmul9_out1 = cast22_out1.matmul(cast12_out1);
        let transpose6_out1 = matmul9_out1.permute([0, 2, 1]);
        let concat10_out1 =
            burn::tensor::Tensor::cat([transpose6_out1.clone(), transpose6_out1].into(), 2);
        let cos2_out1 = concat10_out1.clone().cos();
        let constant98_out1: f32 = 1f32;
        let mul17_out1 = cos2_out1.mul_scalar(constant98_out1);
        let sin2_out1 = concat10_out1.sin();
        let constant99_out1: f32 = 1f32;
        let mul18_out1 = sin2_out1.mul_scalar(constant99_out1);
        let cast23_out1 = mul17_out1;
        let cast24_out1 = mul18_out1;
        let transpose7_out1 = reshape5_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose7_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split2_out1, split2_out2, split2_out3] = split_tensors.try_into().unwrap();
        let squeeze4_out1 = split2_out1.squeeze_dims(&[2]);
        let squeeze5_out1 = split2_out2.squeeze_dims(&[2]);
        let squeeze6_out1 = split2_out3.squeeze_dims(&[2]);
        let unsqueeze22_out1: Tensor<B, 4> = cast23_out1.unsqueeze_dims(&[1]);
        let unsqueeze23_out1: Tensor<B, 4> = cast24_out1.unsqueeze_dims(&[1]);
        let mul19_out1 = squeeze4_out1.clone().mul(unsqueeze22_out1.clone());
        let shape15_out1: [i64; 4] = squeeze4_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant106_out1: i64 = 3i64;
        let actual_idx = if constant106_out1 < 0 {
            (shape15_out1.len() as i64 + constant106_out1) as usize
        } else {
            constant106_out1 as usize
        };
        let gather12_out1 = shape15_out1[actual_idx] as i64;
        let constant107_out1: i64 = 2i64;
        let div6_out1 = gather12_out1 / constant107_out1;
        let cast25_out1 = div6_out1;
        let cast26_out1 = cast25_out1;
        let unsqueeze24_out1 = [cast26_out1];
        let slice8_out1 = squeeze4_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze24_out1[0]]);
        let unsqueeze25_out1 = [cast26_out1];
        let slice9_out1 =
            squeeze4_out1.slice(s![.., .., .., unsqueeze25_out1[0]..9223372036854775807]);
        let neg3_out1 = slice9_out1.neg();
        let concat11_out1 = burn::tensor::Tensor::cat([neg3_out1, slice8_out1].into(), 3);
        let mul20_out1 = concat11_out1.mul(unsqueeze23_out1.clone());
        let add8_out1 = mul19_out1.add(mul20_out1);
        let mul21_out1 = squeeze5_out1.clone().mul(unsqueeze22_out1.clone());
        let shape16_out1: [i64; 4] = squeeze5_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant116_out1: i64 = 3i64;
        let actual_idx = if constant116_out1 < 0 {
            (shape16_out1.len() as i64 + constant116_out1) as usize
        } else {
            constant116_out1 as usize
        };
        let gather13_out1 = shape16_out1[actual_idx] as i64;
        let constant117_out1: i64 = 2i64;
        let div7_out1 = gather13_out1 / constant117_out1;
        let cast27_out1 = div7_out1;
        let cast28_out1 = cast27_out1;
        let unsqueeze26_out1 = [cast28_out1];
        let slice10_out1 = squeeze5_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze26_out1[0]]);
        let unsqueeze27_out1 = [cast28_out1];
        let slice11_out1 =
            squeeze5_out1.slice(s![.., .., .., unsqueeze27_out1[0]..9223372036854775807]);
        let neg4_out1 = slice11_out1.neg();
        let concat12_out1 = burn::tensor::Tensor::cat([neg4_out1, slice10_out1].into(), 3);
        let mul22_out1 = concat12_out1.mul(unsqueeze23_out1.clone());
        let add9_out1 = mul21_out1.add(mul22_out1);
        let shape17_out1: [i64; 4] = add8_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice12_out1: [i64; 1] = shape17_out1[3..4].try_into().unwrap();
        let cast29_out1 = {
            let shape_array = slice12_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt4_out1 = cast29_out1.sqrt();
        let constant128_out1 = self.constant128.val();
        let div8_out1 = constant128_out1.div(sqrt4_out1);
        let cast30_out1 = div8_out1;
        let transpose8_out1 = add9_out1.permute([0, 1, 3, 2]);
        let sqrt5_out1 = cast30_out1.clone().sqrt();
        let mul23_out1 = add8_out1.mul(sqrt5_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt6_out1 = cast30_out1.sqrt();
        let mul24_out1 = transpose8_out1.mul(sqrt6_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul10_out1 = mul23_out1.matmul(mul24_out1);
        let add10_out1 = matmul10_out1.add(where3_out1.clone());
        let softmax2_out1 = burn::tensor::activation::softmax(add10_out1, 3);
        let matmul11_out1 = softmax2_out1.matmul(squeeze6_out1);
        let transpose9_out1 = matmul11_out1.permute([0, 2, 1, 3]);
        let unsqueeze28_out1 = [gather11_out1];
        let constant130_out1: [i64; 1] = [-1i64];
        let constant131_out1: [i64; 1] = [768i64];
        let concat13_out1: [i64; 3usize] = [
            &unsqueeze28_out1[..],
            &constant130_out1[..],
            &constant131_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape7_out1 = transpose9_out1.reshape(concat13_out1);
        let matmul12_out1 = self.matmul12.forward(reshape7_out1);
        let add11_out1 = add7_out1.add(matmul12_out1);
        let layernormalization4_out1 = {
            let dtype = add11_out1.clone().dtype();
            self.layernormalization4
                .forward(add11_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul13_out1 = self.matmul13.forward(layernormalization4_out1);
        let shape18_out1: [i64; 3] = matmul13_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant133_out1: [i64; 1] = [-1i64];
        let gather14_out1: [i64; 1usize] = constant133_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape18_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape18_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant135_out1: [i64; 1] = [1i64];
        let add12_out1 = {
            let mut result = gather14_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant135_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant136_out1: [i64; 1] = [2i64];
        let div9_out1 = {
            let mut result = add12_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant136_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant137_out1: [i64; 1] = [1i64];
        let mul25_out1 = {
            let mut result = div9_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant137_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice13_out1 = matmul13_out1.clone().slice(s![.., .., 0..mul25_out1[0]]);
        let constant138_out1: [i64; 1] = [2i64];
        let mul26_out1 = {
            let mut result = div9_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant138_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice14_out1 = matmul13_out1.slice(s![.., .., mul25_out1[0]..mul26_out1[0]]);
        let constant139_out1: f32 = 1.4142135f32;
        let div10_out1 = slice13_out1.clone().div_scalar(constant139_out1);
        let erf2_out1 = div10_out1.erf();
        let constant140_out1: f32 = 1f32;
        let add13_out1 = erf2_out1.add_scalar(constant140_out1);
        let mul27_out1 = slice13_out1.mul(add13_out1);
        let constant141_out1: f32 = 0.5f32;
        let mul28_out1 = mul27_out1.mul_scalar(constant141_out1);
        let mul29_out1 = mul28_out1.mul(slice14_out1);
        let matmul14_out1 = self.matmul14.forward(mul29_out1);
        let add14_out1 = add11_out1.add(matmul14_out1);
        let layernormalization5_out1 = {
            let dtype = add14_out1.clone().dtype();
            self.layernormalization5
                .forward(add14_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul15_out1 = self.matmul15.forward(layernormalization5_out1.clone());
        let shape19_out1: [i64; 3] = layernormalization5_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant143_out1: i64 = 0i64;
        let actual_idx = if constant143_out1 < 0 {
            (shape19_out1.len() as i64 + constant143_out1) as usize
        } else {
            constant143_out1 as usize
        };
        let gather15_out1 = shape19_out1[actual_idx] as i64;
        let unsqueeze29_out1 = [gather15_out1];
        let constant145_out1: [i64; 1] = [-1i64];
        let constant146_out1: [i64; 1] = [3i64];
        let constant147_out1: [i64; 1] = [12i64];
        let constant148_out1: [i64; 1] = [64i64];
        let concat14_out1: [i64; 5usize] = [
            &unsqueeze29_out1[..],
            &constant145_out1[..],
            &constant146_out1[..],
            &constant147_out1[..],
            &constant148_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape8_out1 = matmul15_out1.reshape(concat14_out1);
        let transpose10_out1 = reshape8_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose10_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split3_out1, split3_out2, split3_out3] = split_tensors.try_into().unwrap();
        let squeeze7_out1 = split3_out1.squeeze_dims(&[2]);
        let squeeze8_out1 = split3_out2.squeeze_dims(&[2]);
        let squeeze9_out1 = split3_out3.squeeze_dims(&[2]);
        let mul30_out1 = squeeze7_out1.clone().mul(unsqueeze22_out1.clone());
        let shape20_out1: [i64; 4] = squeeze7_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant153_out1: i64 = 3i64;
        let actual_idx = if constant153_out1 < 0 {
            (shape20_out1.len() as i64 + constant153_out1) as usize
        } else {
            constant153_out1 as usize
        };
        let gather16_out1 = shape20_out1[actual_idx] as i64;
        let constant154_out1: i64 = 2i64;
        let div11_out1 = gather16_out1 / constant154_out1;
        let cast31_out1 = div11_out1;
        let cast32_out1 = cast31_out1;
        let unsqueeze30_out1 = [cast32_out1];
        let slice15_out1 = squeeze7_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze30_out1[0]]);
        let unsqueeze31_out1 = [cast32_out1];
        let slice16_out1 =
            squeeze7_out1.slice(s![.., .., .., unsqueeze31_out1[0]..9223372036854775807]);
        let neg5_out1 = slice16_out1.neg();
        let concat15_out1 = burn::tensor::Tensor::cat([neg5_out1, slice15_out1].into(), 3);
        let mul31_out1 = concat15_out1.mul(unsqueeze23_out1.clone());
        let add15_out1 = mul30_out1.add(mul31_out1);
        let mul32_out1 = squeeze8_out1.clone().mul(unsqueeze22_out1.clone());
        let shape21_out1: [i64; 4] = squeeze8_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant163_out1: i64 = 3i64;
        let actual_idx = if constant163_out1 < 0 {
            (shape21_out1.len() as i64 + constant163_out1) as usize
        } else {
            constant163_out1 as usize
        };
        let gather17_out1 = shape21_out1[actual_idx] as i64;
        let constant164_out1: i64 = 2i64;
        let div12_out1 = gather17_out1 / constant164_out1;
        let cast33_out1 = div12_out1;
        let cast34_out1 = cast33_out1;
        let unsqueeze32_out1 = [cast34_out1];
        let slice17_out1 = squeeze8_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze32_out1[0]]);
        let unsqueeze33_out1 = [cast34_out1];
        let slice18_out1 =
            squeeze8_out1.slice(s![.., .., .., unsqueeze33_out1[0]..9223372036854775807]);
        let neg6_out1 = slice18_out1.neg();
        let concat16_out1 = burn::tensor::Tensor::cat([neg6_out1, slice17_out1].into(), 3);
        let mul33_out1 = concat16_out1.mul(unsqueeze23_out1.clone());
        let add16_out1 = mul32_out1.add(mul33_out1);
        let shape22_out1: [i64; 4] = add15_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice19_out1: [i64; 1] = shape22_out1[3..4].try_into().unwrap();
        let cast35_out1 = {
            let shape_array = slice19_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt7_out1 = cast35_out1.sqrt();
        let constant175_out1 = self.constant175.val();
        let div13_out1 = constant175_out1.div(sqrt7_out1);
        let cast36_out1 = div13_out1;
        let transpose11_out1 = add16_out1.permute([0, 1, 3, 2]);
        let sqrt8_out1 = cast36_out1.clone().sqrt();
        let mul34_out1 = add15_out1.mul(sqrt8_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt9_out1 = cast36_out1.sqrt();
        let mul35_out1 = transpose11_out1.mul(sqrt9_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul16_out1 = mul34_out1.matmul(mul35_out1);
        let add17_out1 = matmul16_out1.add(where3_out1.clone());
        let softmax3_out1 = burn::tensor::activation::softmax(add17_out1, 3);
        let matmul17_out1 = softmax3_out1.matmul(squeeze9_out1);
        let transpose12_out1 = matmul17_out1.permute([0, 2, 1, 3]);
        let unsqueeze34_out1 = [gather15_out1];
        let constant177_out1: [i64; 1] = [-1i64];
        let constant178_out1: [i64; 1] = [768i64];
        let concat17_out1: [i64; 3usize] = [
            &unsqueeze34_out1[..],
            &constant177_out1[..],
            &constant178_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape9_out1 = transpose12_out1.reshape(concat17_out1);
        let matmul18_out1 = self.matmul18.forward(reshape9_out1);
        let add18_out1 = add14_out1.add(matmul18_out1);
        let layernormalization6_out1 = {
            let dtype = add18_out1.clone().dtype();
            self.layernormalization6
                .forward(add18_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul19_out1 = self.matmul19.forward(layernormalization6_out1);
        let shape23_out1: [i64; 3] = matmul19_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant180_out1: [i64; 1] = [-1i64];
        let gather18_out1: [i64; 1usize] = constant180_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape23_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape23_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant182_out1: [i64; 1] = [1i64];
        let add19_out1 = {
            let mut result = gather18_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant182_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant183_out1: [i64; 1] = [2i64];
        let div14_out1 = {
            let mut result = add19_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant183_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant184_out1: [i64; 1] = [1i64];
        let mul36_out1 = {
            let mut result = div14_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant184_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice20_out1 = matmul19_out1.clone().slice(s![.., .., 0..mul36_out1[0]]);
        let constant185_out1: [i64; 1] = [2i64];
        let mul37_out1 = {
            let mut result = div14_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant185_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice21_out1 = matmul19_out1.slice(s![.., .., mul36_out1[0]..mul37_out1[0]]);
        let constant186_out1: f32 = 1.4142135f32;
        let div15_out1 = slice20_out1.clone().div_scalar(constant186_out1);
        let erf3_out1 = div15_out1.erf();
        let constant187_out1: f32 = 1f32;
        let add20_out1 = erf3_out1.add_scalar(constant187_out1);
        let mul38_out1 = slice20_out1.mul(add20_out1);
        let constant188_out1: f32 = 0.5f32;
        let mul39_out1 = mul38_out1.mul_scalar(constant188_out1);
        let mul40_out1 = mul39_out1.mul(slice21_out1);
        let matmul20_out1 = self.matmul20.forward(mul40_out1);
        let add21_out1 = add18_out1.add(matmul20_out1);
        let layernormalization7_out1 = {
            let dtype = add21_out1.clone().dtype();
            self.layernormalization7
                .forward(add21_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul21_out1 = self.matmul21.forward(layernormalization7_out1.clone());
        let shape24_out1: [i64; 3] = layernormalization7_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant190_out1: i64 = 0i64;
        let actual_idx = if constant190_out1 < 0 {
            (shape24_out1.len() as i64 + constant190_out1) as usize
        } else {
            constant190_out1 as usize
        };
        let gather19_out1 = shape24_out1[actual_idx] as i64;
        let unsqueeze35_out1 = [gather19_out1];
        let constant192_out1: [i64; 1] = [-1i64];
        let constant193_out1: [i64; 1] = [3i64];
        let constant194_out1: [i64; 1] = [12i64];
        let constant195_out1: [i64; 1] = [64i64];
        let concat18_out1: [i64; 5usize] = [
            &unsqueeze35_out1[..],
            &constant192_out1[..],
            &constant193_out1[..],
            &constant194_out1[..],
            &constant195_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape10_out1 = matmul21_out1.reshape(concat18_out1);
        let transpose13_out1 = reshape10_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose13_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split4_out1, split4_out2, split4_out3] = split_tensors.try_into().unwrap();
        let squeeze10_out1 = split4_out1.squeeze_dims(&[2]);
        let squeeze11_out1 = split4_out2.squeeze_dims(&[2]);
        let squeeze12_out1 = split4_out3.squeeze_dims(&[2]);
        let mul41_out1 = squeeze10_out1.clone().mul(unsqueeze13_out1.clone());
        let shape25_out1: [i64; 4] = squeeze10_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant200_out1: i64 = 3i64;
        let actual_idx = if constant200_out1 < 0 {
            (shape25_out1.len() as i64 + constant200_out1) as usize
        } else {
            constant200_out1 as usize
        };
        let gather20_out1 = shape25_out1[actual_idx] as i64;
        let constant201_out1: i64 = 2i64;
        let div16_out1 = gather20_out1 / constant201_out1;
        let cast37_out1 = div16_out1;
        let cast38_out1 = cast37_out1;
        let unsqueeze36_out1 = [cast38_out1];
        let slice22_out1 = squeeze10_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze36_out1[0]]);
        let unsqueeze37_out1 = [cast38_out1];
        let slice23_out1 =
            squeeze10_out1.slice(s![.., .., .., unsqueeze37_out1[0]..9223372036854775807]);
        let neg7_out1 = slice23_out1.neg();
        let concat19_out1 = burn::tensor::Tensor::cat([neg7_out1, slice22_out1].into(), 3);
        let mul42_out1 = concat19_out1.mul(unsqueeze14_out1.clone());
        let add22_out1 = mul41_out1.add(mul42_out1);
        let mul43_out1 = squeeze11_out1.clone().mul(unsqueeze13_out1.clone());
        let shape26_out1: [i64; 4] = squeeze11_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant210_out1: i64 = 3i64;
        let actual_idx = if constant210_out1 < 0 {
            (shape26_out1.len() as i64 + constant210_out1) as usize
        } else {
            constant210_out1 as usize
        };
        let gather21_out1 = shape26_out1[actual_idx] as i64;
        let constant211_out1: i64 = 2i64;
        let div17_out1 = gather21_out1 / constant211_out1;
        let cast39_out1 = div17_out1;
        let cast40_out1 = cast39_out1;
        let unsqueeze38_out1 = [cast40_out1];
        let slice24_out1 = squeeze11_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze38_out1[0]]);
        let unsqueeze39_out1 = [cast40_out1];
        let slice25_out1 =
            squeeze11_out1.slice(s![.., .., .., unsqueeze39_out1[0]..9223372036854775807]);
        let neg8_out1 = slice25_out1.neg();
        let concat20_out1 = burn::tensor::Tensor::cat([neg8_out1, slice24_out1].into(), 3);
        let mul44_out1 = concat20_out1.mul(unsqueeze14_out1.clone());
        let add23_out1 = mul43_out1.add(mul44_out1);
        let shape27_out1: [i64; 4] = add22_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice26_out1: [i64; 1] = shape27_out1[3..4].try_into().unwrap();
        let cast41_out1 = {
            let shape_array = slice26_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt10_out1 = cast41_out1.sqrt();
        let constant222_out1 = self.constant222.val();
        let div18_out1 = constant222_out1.div(sqrt10_out1);
        let cast42_out1 = div18_out1;
        let transpose14_out1 = add23_out1.permute([0, 1, 3, 2]);
        let sqrt11_out1 = cast42_out1.clone().sqrt();
        let mul45_out1 = add22_out1.mul(sqrt11_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt12_out1 = cast42_out1.sqrt();
        let mul46_out1 =
            transpose14_out1.mul(sqrt12_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul22_out1 = mul45_out1.matmul(mul46_out1);
        let add24_out1 = matmul22_out1.add(where2_out1.clone());
        let softmax4_out1 = burn::tensor::activation::softmax(add24_out1, 3);
        let matmul23_out1 = softmax4_out1.matmul(squeeze12_out1);
        let transpose15_out1 = matmul23_out1.permute([0, 2, 1, 3]);
        let unsqueeze40_out1 = [gather19_out1];
        let constant224_out1: [i64; 1] = [-1i64];
        let constant225_out1: [i64; 1] = [768i64];
        let concat21_out1: [i64; 3usize] = [
            &unsqueeze40_out1[..],
            &constant224_out1[..],
            &constant225_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape11_out1 = transpose15_out1.reshape(concat21_out1);
        let matmul24_out1 = self.matmul24.forward(reshape11_out1);
        let add25_out1 = add21_out1.add(matmul24_out1);
        let layernormalization8_out1 = {
            let dtype = add25_out1.clone().dtype();
            self.layernormalization8
                .forward(add25_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul25_out1 = self.matmul25.forward(layernormalization8_out1);
        let shape28_out1: [i64; 3] = matmul25_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant227_out1: [i64; 1] = [-1i64];
        let gather22_out1: [i64; 1usize] = constant227_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape28_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape28_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant229_out1: [i64; 1] = [1i64];
        let add26_out1 = {
            let mut result = gather22_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant229_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant230_out1: [i64; 1] = [2i64];
        let div19_out1 = {
            let mut result = add26_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant230_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant231_out1: [i64; 1] = [1i64];
        let mul47_out1 = {
            let mut result = div19_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant231_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice27_out1 = matmul25_out1.clone().slice(s![.., .., 0..mul47_out1[0]]);
        let constant232_out1: [i64; 1] = [2i64];
        let mul48_out1 = {
            let mut result = div19_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant232_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice28_out1 = matmul25_out1.slice(s![.., .., mul47_out1[0]..mul48_out1[0]]);
        let constant233_out1: f32 = 1.4142135f32;
        let div20_out1 = slice27_out1.clone().div_scalar(constant233_out1);
        let erf4_out1 = div20_out1.erf();
        let constant234_out1: f32 = 1f32;
        let add27_out1 = erf4_out1.add_scalar(constant234_out1);
        let mul49_out1 = slice27_out1.mul(add27_out1);
        let constant235_out1: f32 = 0.5f32;
        let mul50_out1 = mul49_out1.mul_scalar(constant235_out1);
        let mul51_out1 = mul50_out1.mul(slice28_out1);
        let matmul26_out1 = self.matmul26.forward(mul51_out1);
        let add28_out1 = add25_out1.add(matmul26_out1);
        let layernormalization9_out1 = {
            let dtype = add28_out1.clone().dtype();
            self.layernormalization9
                .forward(add28_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul27_out1 = self.matmul27.forward(layernormalization9_out1.clone());
        let shape29_out1: [i64; 3] = layernormalization9_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant237_out1: i64 = 0i64;
        let actual_idx = if constant237_out1 < 0 {
            (shape29_out1.len() as i64 + constant237_out1) as usize
        } else {
            constant237_out1 as usize
        };
        let gather23_out1 = shape29_out1[actual_idx] as i64;
        let unsqueeze41_out1 = [gather23_out1];
        let constant239_out1: [i64; 1] = [-1i64];
        let constant240_out1: [i64; 1] = [3i64];
        let constant241_out1: [i64; 1] = [12i64];
        let constant242_out1: [i64; 1] = [64i64];
        let concat22_out1: [i64; 5usize] = [
            &unsqueeze41_out1[..],
            &constant239_out1[..],
            &constant240_out1[..],
            &constant241_out1[..],
            &constant242_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape12_out1 = matmul27_out1.reshape(concat22_out1);
        let transpose16_out1 = reshape12_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose16_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split5_out1, split5_out2, split5_out3] = split_tensors.try_into().unwrap();
        let squeeze13_out1 = split5_out1.squeeze_dims(&[2]);
        let squeeze14_out1 = split5_out2.squeeze_dims(&[2]);
        let squeeze15_out1 = split5_out3.squeeze_dims(&[2]);
        let mul52_out1 = squeeze13_out1.clone().mul(unsqueeze22_out1.clone());
        let shape30_out1: [i64; 4] = squeeze13_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant247_out1: i64 = 3i64;
        let actual_idx = if constant247_out1 < 0 {
            (shape30_out1.len() as i64 + constant247_out1) as usize
        } else {
            constant247_out1 as usize
        };
        let gather24_out1 = shape30_out1[actual_idx] as i64;
        let constant248_out1: i64 = 2i64;
        let div21_out1 = gather24_out1 / constant248_out1;
        let cast43_out1 = div21_out1;
        let cast44_out1 = cast43_out1;
        let unsqueeze42_out1 = [cast44_out1];
        let slice29_out1 = squeeze13_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze42_out1[0]]);
        let unsqueeze43_out1 = [cast44_out1];
        let slice30_out1 =
            squeeze13_out1.slice(s![.., .., .., unsqueeze43_out1[0]..9223372036854775807]);
        let neg9_out1 = slice30_out1.neg();
        let concat23_out1 = burn::tensor::Tensor::cat([neg9_out1, slice29_out1].into(), 3);
        let mul53_out1 = concat23_out1.mul(unsqueeze23_out1.clone());
        let add29_out1 = mul52_out1.add(mul53_out1);
        let mul54_out1 = squeeze14_out1.clone().mul(unsqueeze22_out1.clone());
        let shape31_out1: [i64; 4] = squeeze14_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant257_out1: i64 = 3i64;
        let actual_idx = if constant257_out1 < 0 {
            (shape31_out1.len() as i64 + constant257_out1) as usize
        } else {
            constant257_out1 as usize
        };
        let gather25_out1 = shape31_out1[actual_idx] as i64;
        let constant258_out1: i64 = 2i64;
        let div22_out1 = gather25_out1 / constant258_out1;
        let cast45_out1 = div22_out1;
        let cast46_out1 = cast45_out1;
        let unsqueeze44_out1 = [cast46_out1];
        let slice31_out1 = squeeze14_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze44_out1[0]]);
        let unsqueeze45_out1 = [cast46_out1];
        let slice32_out1 =
            squeeze14_out1.slice(s![.., .., .., unsqueeze45_out1[0]..9223372036854775807]);
        let neg10_out1 = slice32_out1.neg();
        let concat24_out1 = burn::tensor::Tensor::cat([neg10_out1, slice31_out1].into(), 3);
        let mul55_out1 = concat24_out1.mul(unsqueeze23_out1.clone());
        let add30_out1 = mul54_out1.add(mul55_out1);
        let shape32_out1: [i64; 4] = add29_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice33_out1: [i64; 1] = shape32_out1[3..4].try_into().unwrap();
        let cast47_out1 = {
            let shape_array = slice33_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt13_out1 = cast47_out1.sqrt();
        let constant269_out1 = self.constant269.val();
        let div23_out1 = constant269_out1.div(sqrt13_out1);
        let cast48_out1 = div23_out1;
        let transpose17_out1 = add30_out1.permute([0, 1, 3, 2]);
        let sqrt14_out1 = cast48_out1.clone().sqrt();
        let mul56_out1 = add29_out1.mul(sqrt14_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt15_out1 = cast48_out1.sqrt();
        let mul57_out1 =
            transpose17_out1.mul(sqrt15_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul28_out1 = mul56_out1.matmul(mul57_out1);
        let add31_out1 = matmul28_out1.add(where3_out1.clone());
        let softmax5_out1 = burn::tensor::activation::softmax(add31_out1, 3);
        let matmul29_out1 = softmax5_out1.matmul(squeeze15_out1);
        let transpose18_out1 = matmul29_out1.permute([0, 2, 1, 3]);
        let unsqueeze46_out1 = [gather23_out1];
        let constant271_out1: [i64; 1] = [-1i64];
        let constant272_out1: [i64; 1] = [768i64];
        let concat25_out1: [i64; 3usize] = [
            &unsqueeze46_out1[..],
            &constant271_out1[..],
            &constant272_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape13_out1 = transpose18_out1.reshape(concat25_out1);
        let matmul30_out1 = self.matmul30.forward(reshape13_out1);
        let add32_out1 = add28_out1.add(matmul30_out1);
        let layernormalization10_out1 = {
            let dtype = add32_out1.clone().dtype();
            self.layernormalization10
                .forward(add32_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul31_out1 = self.matmul31.forward(layernormalization10_out1);
        let shape33_out1: [i64; 3] = matmul31_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant274_out1: [i64; 1] = [-1i64];
        let gather26_out1: [i64; 1usize] = constant274_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape33_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape33_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant276_out1: [i64; 1] = [1i64];
        let add33_out1 = {
            let mut result = gather26_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant276_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant277_out1: [i64; 1] = [2i64];
        let div24_out1 = {
            let mut result = add33_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant277_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant278_out1: [i64; 1] = [1i64];
        let mul58_out1 = {
            let mut result = div24_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant278_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice34_out1 = matmul31_out1.clone().slice(s![.., .., 0..mul58_out1[0]]);
        let constant279_out1: [i64; 1] = [2i64];
        let mul59_out1 = {
            let mut result = div24_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant279_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice35_out1 = matmul31_out1.slice(s![.., .., mul58_out1[0]..mul59_out1[0]]);
        let constant280_out1: f32 = 1.4142135f32;
        let div25_out1 = slice34_out1.clone().div_scalar(constant280_out1);
        let erf5_out1 = div25_out1.erf();
        let constant281_out1: f32 = 1f32;
        let add34_out1 = erf5_out1.add_scalar(constant281_out1);
        let mul60_out1 = slice34_out1.mul(add34_out1);
        let constant282_out1: f32 = 0.5f32;
        let mul61_out1 = mul60_out1.mul_scalar(constant282_out1);
        let mul62_out1 = mul61_out1.mul(slice35_out1);
        let matmul32_out1 = self.matmul32.forward(mul62_out1);
        let add35_out1 = add32_out1.add(matmul32_out1);
        let layernormalization11_out1 = {
            let dtype = add35_out1.clone().dtype();
            self.layernormalization11
                .forward(add35_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul33_out1 = self.matmul33.forward(layernormalization11_out1.clone());
        let shape34_out1: [i64; 3] = layernormalization11_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant284_out1: i64 = 0i64;
        let actual_idx = if constant284_out1 < 0 {
            (shape34_out1.len() as i64 + constant284_out1) as usize
        } else {
            constant284_out1 as usize
        };
        let gather27_out1 = shape34_out1[actual_idx] as i64;
        let unsqueeze47_out1 = [gather27_out1];
        let constant286_out1: [i64; 1] = [-1i64];
        let constant287_out1: [i64; 1] = [3i64];
        let constant288_out1: [i64; 1] = [12i64];
        let constant289_out1: [i64; 1] = [64i64];
        let concat26_out1: [i64; 5usize] = [
            &unsqueeze47_out1[..],
            &constant286_out1[..],
            &constant287_out1[..],
            &constant288_out1[..],
            &constant289_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape14_out1 = matmul33_out1.reshape(concat26_out1);
        let transpose19_out1 = reshape14_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose19_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split6_out1, split6_out2, split6_out3] = split_tensors.try_into().unwrap();
        let squeeze16_out1 = split6_out1.squeeze_dims(&[2]);
        let squeeze17_out1 = split6_out2.squeeze_dims(&[2]);
        let squeeze18_out1 = split6_out3.squeeze_dims(&[2]);
        let mul63_out1 = squeeze16_out1.clone().mul(unsqueeze22_out1.clone());
        let shape35_out1: [i64; 4] = squeeze16_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant294_out1: i64 = 3i64;
        let actual_idx = if constant294_out1 < 0 {
            (shape35_out1.len() as i64 + constant294_out1) as usize
        } else {
            constant294_out1 as usize
        };
        let gather28_out1 = shape35_out1[actual_idx] as i64;
        let constant295_out1: i64 = 2i64;
        let div26_out1 = gather28_out1 / constant295_out1;
        let cast49_out1 = div26_out1;
        let cast50_out1 = cast49_out1;
        let unsqueeze48_out1 = [cast50_out1];
        let slice36_out1 = squeeze16_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze48_out1[0]]);
        let unsqueeze49_out1 = [cast50_out1];
        let slice37_out1 =
            squeeze16_out1.slice(s![.., .., .., unsqueeze49_out1[0]..9223372036854775807]);
        let neg11_out1 = slice37_out1.neg();
        let concat27_out1 = burn::tensor::Tensor::cat([neg11_out1, slice36_out1].into(), 3);
        let mul64_out1 = concat27_out1.mul(unsqueeze23_out1.clone());
        let add36_out1 = mul63_out1.add(mul64_out1);
        let mul65_out1 = squeeze17_out1.clone().mul(unsqueeze22_out1.clone());
        let shape36_out1: [i64; 4] = squeeze17_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant304_out1: i64 = 3i64;
        let actual_idx = if constant304_out1 < 0 {
            (shape36_out1.len() as i64 + constant304_out1) as usize
        } else {
            constant304_out1 as usize
        };
        let gather29_out1 = shape36_out1[actual_idx] as i64;
        let constant305_out1: i64 = 2i64;
        let div27_out1 = gather29_out1 / constant305_out1;
        let cast51_out1 = div27_out1;
        let cast52_out1 = cast51_out1;
        let unsqueeze50_out1 = [cast52_out1];
        let slice38_out1 = squeeze17_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze50_out1[0]]);
        let unsqueeze51_out1 = [cast52_out1];
        let slice39_out1 =
            squeeze17_out1.slice(s![.., .., .., unsqueeze51_out1[0]..9223372036854775807]);
        let neg12_out1 = slice39_out1.neg();
        let concat28_out1 = burn::tensor::Tensor::cat([neg12_out1, slice38_out1].into(), 3);
        let mul66_out1 = concat28_out1.mul(unsqueeze23_out1.clone());
        let add37_out1 = mul65_out1.add(mul66_out1);
        let shape37_out1: [i64; 4] = add36_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice40_out1: [i64; 1] = shape37_out1[3..4].try_into().unwrap();
        let cast53_out1 = {
            let shape_array = slice40_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt16_out1 = cast53_out1.sqrt();
        let constant316_out1 = self.constant316.val();
        let div28_out1 = constant316_out1.div(sqrt16_out1);
        let cast54_out1 = div28_out1;
        let transpose20_out1 = add37_out1.permute([0, 1, 3, 2]);
        let sqrt17_out1 = cast54_out1.clone().sqrt();
        let mul67_out1 = add36_out1.mul(sqrt17_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt18_out1 = cast54_out1.sqrt();
        let mul68_out1 =
            transpose20_out1.mul(sqrt18_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul34_out1 = mul67_out1.matmul(mul68_out1);
        let add38_out1 = matmul34_out1.add(where3_out1.clone());
        let softmax6_out1 = burn::tensor::activation::softmax(add38_out1, 3);
        let matmul35_out1 = softmax6_out1.matmul(squeeze18_out1);
        let transpose21_out1 = matmul35_out1.permute([0, 2, 1, 3]);
        let unsqueeze52_out1 = [gather27_out1];
        let constant318_out1: [i64; 1] = [-1i64];
        let constant319_out1: [i64; 1] = [768i64];
        let concat29_out1: [i64; 3usize] = [
            &unsqueeze52_out1[..],
            &constant318_out1[..],
            &constant319_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape15_out1 = transpose21_out1.reshape(concat29_out1);
        let matmul36_out1 = self.matmul36.forward(reshape15_out1);
        let add39_out1 = add35_out1.add(matmul36_out1);
        let layernormalization12_out1 = {
            let dtype = add39_out1.clone().dtype();
            self.layernormalization12
                .forward(add39_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul37_out1 = self.matmul37.forward(layernormalization12_out1);
        let shape38_out1: [i64; 3] = matmul37_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant321_out1: [i64; 1] = [-1i64];
        let gather30_out1: [i64; 1usize] = constant321_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape38_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape38_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant323_out1: [i64; 1] = [1i64];
        let add40_out1 = {
            let mut result = gather30_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant323_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant324_out1: [i64; 1] = [2i64];
        let div29_out1 = {
            let mut result = add40_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant324_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant325_out1: [i64; 1] = [1i64];
        let mul69_out1 = {
            let mut result = div29_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant325_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice41_out1 = matmul37_out1.clone().slice(s![.., .., 0..mul69_out1[0]]);
        let constant326_out1: [i64; 1] = [2i64];
        let mul70_out1 = {
            let mut result = div29_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant326_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice42_out1 = matmul37_out1.slice(s![.., .., mul69_out1[0]..mul70_out1[0]]);
        let constant327_out1: f32 = 1.4142135f32;
        let div30_out1 = slice41_out1.clone().div_scalar(constant327_out1);
        let erf6_out1 = div30_out1.erf();
        let constant328_out1: f32 = 1f32;
        let add41_out1 = erf6_out1.add_scalar(constant328_out1);
        let mul71_out1 = slice41_out1.mul(add41_out1);
        let constant329_out1: f32 = 0.5f32;
        let mul72_out1 = mul71_out1.mul_scalar(constant329_out1);
        let mul73_out1 = mul72_out1.mul(slice42_out1);
        let matmul38_out1 = self.matmul38.forward(mul73_out1);
        let add42_out1 = add39_out1.add(matmul38_out1);
        let layernormalization13_out1 = {
            let dtype = add42_out1.clone().dtype();
            self.layernormalization13
                .forward(add42_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul39_out1 = self.matmul39.forward(layernormalization13_out1.clone());
        let shape39_out1: [i64; 3] = layernormalization13_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant331_out1: i64 = 0i64;
        let actual_idx = if constant331_out1 < 0 {
            (shape39_out1.len() as i64 + constant331_out1) as usize
        } else {
            constant331_out1 as usize
        };
        let gather31_out1 = shape39_out1[actual_idx] as i64;
        let unsqueeze53_out1 = [gather31_out1];
        let constant333_out1: [i64; 1] = [-1i64];
        let constant334_out1: [i64; 1] = [3i64];
        let constant335_out1: [i64; 1] = [12i64];
        let constant336_out1: [i64; 1] = [64i64];
        let concat30_out1: [i64; 5usize] = [
            &unsqueeze53_out1[..],
            &constant333_out1[..],
            &constant334_out1[..],
            &constant335_out1[..],
            &constant336_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape16_out1 = matmul39_out1.reshape(concat30_out1);
        let transpose22_out1 = reshape16_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose22_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split7_out1, split7_out2, split7_out3] = split_tensors.try_into().unwrap();
        let squeeze19_out1 = split7_out1.squeeze_dims(&[2]);
        let squeeze20_out1 = split7_out2.squeeze_dims(&[2]);
        let squeeze21_out1 = split7_out3.squeeze_dims(&[2]);
        let mul74_out1 = squeeze19_out1.clone().mul(unsqueeze13_out1.clone());
        let shape40_out1: [i64; 4] = squeeze19_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant341_out1: i64 = 3i64;
        let actual_idx = if constant341_out1 < 0 {
            (shape40_out1.len() as i64 + constant341_out1) as usize
        } else {
            constant341_out1 as usize
        };
        let gather32_out1 = shape40_out1[actual_idx] as i64;
        let constant342_out1: i64 = 2i64;
        let div31_out1 = gather32_out1 / constant342_out1;
        let cast55_out1 = div31_out1;
        let cast56_out1 = cast55_out1;
        let unsqueeze54_out1 = [cast56_out1];
        let slice43_out1 = squeeze19_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze54_out1[0]]);
        let unsqueeze55_out1 = [cast56_out1];
        let slice44_out1 =
            squeeze19_out1.slice(s![.., .., .., unsqueeze55_out1[0]..9223372036854775807]);
        let neg13_out1 = slice44_out1.neg();
        let concat31_out1 = burn::tensor::Tensor::cat([neg13_out1, slice43_out1].into(), 3);
        let mul75_out1 = concat31_out1.mul(unsqueeze14_out1.clone());
        let add43_out1 = mul74_out1.add(mul75_out1);
        let mul76_out1 = squeeze20_out1.clone().mul(unsqueeze13_out1.clone());
        let shape41_out1: [i64; 4] = squeeze20_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant351_out1: i64 = 3i64;
        let actual_idx = if constant351_out1 < 0 {
            (shape41_out1.len() as i64 + constant351_out1) as usize
        } else {
            constant351_out1 as usize
        };
        let gather33_out1 = shape41_out1[actual_idx] as i64;
        let constant352_out1: i64 = 2i64;
        let div32_out1 = gather33_out1 / constant352_out1;
        let cast57_out1 = div32_out1;
        let cast58_out1 = cast57_out1;
        let unsqueeze56_out1 = [cast58_out1];
        let slice45_out1 = squeeze20_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze56_out1[0]]);
        let unsqueeze57_out1 = [cast58_out1];
        let slice46_out1 =
            squeeze20_out1.slice(s![.., .., .., unsqueeze57_out1[0]..9223372036854775807]);
        let neg14_out1 = slice46_out1.neg();
        let concat32_out1 = burn::tensor::Tensor::cat([neg14_out1, slice45_out1].into(), 3);
        let mul77_out1 = concat32_out1.mul(unsqueeze14_out1.clone());
        let add44_out1 = mul76_out1.add(mul77_out1);
        let shape42_out1: [i64; 4] = add43_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice47_out1: [i64; 1] = shape42_out1[3..4].try_into().unwrap();
        let cast59_out1 = {
            let shape_array = slice47_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt19_out1 = cast59_out1.sqrt();
        let constant363_out1 = self.constant363.val();
        let div33_out1 = constant363_out1.div(sqrt19_out1);
        let cast60_out1 = div33_out1;
        let transpose23_out1 = add44_out1.permute([0, 1, 3, 2]);
        let sqrt20_out1 = cast60_out1.clone().sqrt();
        let mul78_out1 = add43_out1.mul(sqrt20_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt21_out1 = cast60_out1.sqrt();
        let mul79_out1 =
            transpose23_out1.mul(sqrt21_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul40_out1 = mul78_out1.matmul(mul79_out1);
        let add45_out1 = matmul40_out1.add(where2_out1.clone());
        let softmax7_out1 = burn::tensor::activation::softmax(add45_out1, 3);
        let matmul41_out1 = softmax7_out1.matmul(squeeze21_out1);
        let transpose24_out1 = matmul41_out1.permute([0, 2, 1, 3]);
        let unsqueeze58_out1 = [gather31_out1];
        let constant365_out1: [i64; 1] = [-1i64];
        let constant366_out1: [i64; 1] = [768i64];
        let concat33_out1: [i64; 3usize] = [
            &unsqueeze58_out1[..],
            &constant365_out1[..],
            &constant366_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape17_out1 = transpose24_out1.reshape(concat33_out1);
        let matmul42_out1 = self.matmul42.forward(reshape17_out1);
        let add46_out1 = add42_out1.add(matmul42_out1);
        let layernormalization14_out1 = {
            let dtype = add46_out1.clone().dtype();
            self.layernormalization14
                .forward(add46_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul43_out1 = self.matmul43.forward(layernormalization14_out1);
        let shape43_out1: [i64; 3] = matmul43_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant368_out1: [i64; 1] = [-1i64];
        let gather34_out1: [i64; 1usize] = constant368_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape43_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape43_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant370_out1: [i64; 1] = [1i64];
        let add47_out1 = {
            let mut result = gather34_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant370_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant371_out1: [i64; 1] = [2i64];
        let div34_out1 = {
            let mut result = add47_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant371_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant372_out1: [i64; 1] = [1i64];
        let mul80_out1 = {
            let mut result = div34_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant372_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice48_out1 = matmul43_out1.clone().slice(s![.., .., 0..mul80_out1[0]]);
        let constant373_out1: [i64; 1] = [2i64];
        let mul81_out1 = {
            let mut result = div34_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant373_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice49_out1 = matmul43_out1.slice(s![.., .., mul80_out1[0]..mul81_out1[0]]);
        let constant374_out1: f32 = 1.4142135f32;
        let div35_out1 = slice48_out1.clone().div_scalar(constant374_out1);
        let erf7_out1 = div35_out1.erf();
        let constant375_out1: f32 = 1f32;
        let add48_out1 = erf7_out1.add_scalar(constant375_out1);
        let mul82_out1 = slice48_out1.mul(add48_out1);
        let constant376_out1: f32 = 0.5f32;
        let mul83_out1 = mul82_out1.mul_scalar(constant376_out1);
        let mul84_out1 = mul83_out1.mul(slice49_out1);
        let matmul44_out1 = self.matmul44.forward(mul84_out1);
        let add49_out1 = add46_out1.add(matmul44_out1);
        let layernormalization15_out1 = {
            let dtype = add49_out1.clone().dtype();
            self.layernormalization15
                .forward(add49_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul45_out1 = self.matmul45.forward(layernormalization15_out1.clone());
        let shape44_out1: [i64; 3] = layernormalization15_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant378_out1: i64 = 0i64;
        let actual_idx = if constant378_out1 < 0 {
            (shape44_out1.len() as i64 + constant378_out1) as usize
        } else {
            constant378_out1 as usize
        };
        let gather35_out1 = shape44_out1[actual_idx] as i64;
        let unsqueeze59_out1 = [gather35_out1];
        let constant380_out1: [i64; 1] = [-1i64];
        let constant381_out1: [i64; 1] = [3i64];
        let constant382_out1: [i64; 1] = [12i64];
        let constant383_out1: [i64; 1] = [64i64];
        let concat34_out1: [i64; 5usize] = [
            &unsqueeze59_out1[..],
            &constant380_out1[..],
            &constant381_out1[..],
            &constant382_out1[..],
            &constant383_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape18_out1 = matmul45_out1.reshape(concat34_out1);
        let transpose25_out1 = reshape18_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose25_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split8_out1, split8_out2, split8_out3] = split_tensors.try_into().unwrap();
        let squeeze22_out1 = split8_out1.squeeze_dims(&[2]);
        let squeeze23_out1 = split8_out2.squeeze_dims(&[2]);
        let squeeze24_out1 = split8_out3.squeeze_dims(&[2]);
        let mul85_out1 = squeeze22_out1.clone().mul(unsqueeze22_out1.clone());
        let shape45_out1: [i64; 4] = squeeze22_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant388_out1: i64 = 3i64;
        let actual_idx = if constant388_out1 < 0 {
            (shape45_out1.len() as i64 + constant388_out1) as usize
        } else {
            constant388_out1 as usize
        };
        let gather36_out1 = shape45_out1[actual_idx] as i64;
        let constant389_out1: i64 = 2i64;
        let div36_out1 = gather36_out1 / constant389_out1;
        let cast61_out1 = div36_out1;
        let cast62_out1 = cast61_out1;
        let unsqueeze60_out1 = [cast62_out1];
        let slice50_out1 = squeeze22_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze60_out1[0]]);
        let unsqueeze61_out1 = [cast62_out1];
        let slice51_out1 =
            squeeze22_out1.slice(s![.., .., .., unsqueeze61_out1[0]..9223372036854775807]);
        let neg15_out1 = slice51_out1.neg();
        let concat35_out1 = burn::tensor::Tensor::cat([neg15_out1, slice50_out1].into(), 3);
        let mul86_out1 = concat35_out1.mul(unsqueeze23_out1.clone());
        let add50_out1 = mul85_out1.add(mul86_out1);
        let mul87_out1 = squeeze23_out1.clone().mul(unsqueeze22_out1.clone());
        let shape46_out1: [i64; 4] = squeeze23_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant398_out1: i64 = 3i64;
        let actual_idx = if constant398_out1 < 0 {
            (shape46_out1.len() as i64 + constant398_out1) as usize
        } else {
            constant398_out1 as usize
        };
        let gather37_out1 = shape46_out1[actual_idx] as i64;
        let constant399_out1: i64 = 2i64;
        let div37_out1 = gather37_out1 / constant399_out1;
        let cast63_out1 = div37_out1;
        let cast64_out1 = cast63_out1;
        let unsqueeze62_out1 = [cast64_out1];
        let slice52_out1 = squeeze23_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze62_out1[0]]);
        let unsqueeze63_out1 = [cast64_out1];
        let slice53_out1 =
            squeeze23_out1.slice(s![.., .., .., unsqueeze63_out1[0]..9223372036854775807]);
        let neg16_out1 = slice53_out1.neg();
        let concat36_out1 = burn::tensor::Tensor::cat([neg16_out1, slice52_out1].into(), 3);
        let mul88_out1 = concat36_out1.mul(unsqueeze23_out1.clone());
        let add51_out1 = mul87_out1.add(mul88_out1);
        let shape47_out1: [i64; 4] = add50_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice54_out1: [i64; 1] = shape47_out1[3..4].try_into().unwrap();
        let cast65_out1 = {
            let shape_array = slice54_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt22_out1 = cast65_out1.sqrt();
        let constant410_out1 = self.constant410.val();
        let div38_out1 = constant410_out1.div(sqrt22_out1);
        let cast66_out1 = div38_out1;
        let transpose26_out1 = add51_out1.permute([0, 1, 3, 2]);
        let sqrt23_out1 = cast66_out1.clone().sqrt();
        let mul89_out1 = add50_out1.mul(sqrt23_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt24_out1 = cast66_out1.sqrt();
        let mul90_out1 =
            transpose26_out1.mul(sqrt24_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul46_out1 = mul89_out1.matmul(mul90_out1);
        let add52_out1 = matmul46_out1.add(where3_out1.clone());
        let softmax8_out1 = burn::tensor::activation::softmax(add52_out1, 3);
        let matmul47_out1 = softmax8_out1.matmul(squeeze24_out1);
        let transpose27_out1 = matmul47_out1.permute([0, 2, 1, 3]);
        let unsqueeze64_out1 = [gather35_out1];
        let constant412_out1: [i64; 1] = [-1i64];
        let constant413_out1: [i64; 1] = [768i64];
        let concat37_out1: [i64; 3usize] = [
            &unsqueeze64_out1[..],
            &constant412_out1[..],
            &constant413_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape19_out1 = transpose27_out1.reshape(concat37_out1);
        let matmul48_out1 = self.matmul48.forward(reshape19_out1);
        let add53_out1 = add49_out1.add(matmul48_out1);
        let layernormalization16_out1 = {
            let dtype = add53_out1.clone().dtype();
            self.layernormalization16
                .forward(add53_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul49_out1 = self.matmul49.forward(layernormalization16_out1);
        let shape48_out1: [i64; 3] = matmul49_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant415_out1: [i64; 1] = [-1i64];
        let gather38_out1: [i64; 1usize] = constant415_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape48_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape48_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant417_out1: [i64; 1] = [1i64];
        let add54_out1 = {
            let mut result = gather38_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant417_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant418_out1: [i64; 1] = [2i64];
        let div39_out1 = {
            let mut result = add54_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant418_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant419_out1: [i64; 1] = [1i64];
        let mul91_out1 = {
            let mut result = div39_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant419_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice55_out1 = matmul49_out1.clone().slice(s![.., .., 0..mul91_out1[0]]);
        let constant420_out1: [i64; 1] = [2i64];
        let mul92_out1 = {
            let mut result = div39_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant420_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice56_out1 = matmul49_out1.slice(s![.., .., mul91_out1[0]..mul92_out1[0]]);
        let constant421_out1: f32 = 1.4142135f32;
        let div40_out1 = slice55_out1.clone().div_scalar(constant421_out1);
        let erf8_out1 = div40_out1.erf();
        let constant422_out1: f32 = 1f32;
        let add55_out1 = erf8_out1.add_scalar(constant422_out1);
        let mul93_out1 = slice55_out1.mul(add55_out1);
        let constant423_out1: f32 = 0.5f32;
        let mul94_out1 = mul93_out1.mul_scalar(constant423_out1);
        let mul95_out1 = mul94_out1.mul(slice56_out1);
        let matmul50_out1 = self.matmul50.forward(mul95_out1);
        let add56_out1 = add53_out1.add(matmul50_out1);
        let layernormalization17_out1 = {
            let dtype = add56_out1.clone().dtype();
            self.layernormalization17
                .forward(add56_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul51_out1 = self.matmul51.forward(layernormalization17_out1.clone());
        let shape49_out1: [i64; 3] = layernormalization17_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant425_out1: i64 = 0i64;
        let actual_idx = if constant425_out1 < 0 {
            (shape49_out1.len() as i64 + constant425_out1) as usize
        } else {
            constant425_out1 as usize
        };
        let gather39_out1 = shape49_out1[actual_idx] as i64;
        let unsqueeze65_out1 = [gather39_out1];
        let constant427_out1: [i64; 1] = [-1i64];
        let constant428_out1: [i64; 1] = [3i64];
        let constant429_out1: [i64; 1] = [12i64];
        let constant430_out1: [i64; 1] = [64i64];
        let concat38_out1: [i64; 5usize] = [
            &unsqueeze65_out1[..],
            &constant427_out1[..],
            &constant428_out1[..],
            &constant429_out1[..],
            &constant430_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape20_out1 = matmul51_out1.reshape(concat38_out1);
        let transpose28_out1 = reshape20_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose28_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split9_out1, split9_out2, split9_out3] = split_tensors.try_into().unwrap();
        let squeeze25_out1 = split9_out1.squeeze_dims(&[2]);
        let squeeze26_out1 = split9_out2.squeeze_dims(&[2]);
        let squeeze27_out1 = split9_out3.squeeze_dims(&[2]);
        let mul96_out1 = squeeze25_out1.clone().mul(unsqueeze22_out1.clone());
        let shape50_out1: [i64; 4] = squeeze25_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant435_out1: i64 = 3i64;
        let actual_idx = if constant435_out1 < 0 {
            (shape50_out1.len() as i64 + constant435_out1) as usize
        } else {
            constant435_out1 as usize
        };
        let gather40_out1 = shape50_out1[actual_idx] as i64;
        let constant436_out1: i64 = 2i64;
        let div41_out1 = gather40_out1 / constant436_out1;
        let cast67_out1 = div41_out1;
        let cast68_out1 = cast67_out1;
        let unsqueeze66_out1 = [cast68_out1];
        let slice57_out1 = squeeze25_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze66_out1[0]]);
        let unsqueeze67_out1 = [cast68_out1];
        let slice58_out1 =
            squeeze25_out1.slice(s![.., .., .., unsqueeze67_out1[0]..9223372036854775807]);
        let neg17_out1 = slice58_out1.neg();
        let concat39_out1 = burn::tensor::Tensor::cat([neg17_out1, slice57_out1].into(), 3);
        let mul97_out1 = concat39_out1.mul(unsqueeze23_out1.clone());
        let add57_out1 = mul96_out1.add(mul97_out1);
        let mul98_out1 = squeeze26_out1.clone().mul(unsqueeze22_out1.clone());
        let shape51_out1: [i64; 4] = squeeze26_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant445_out1: i64 = 3i64;
        let actual_idx = if constant445_out1 < 0 {
            (shape51_out1.len() as i64 + constant445_out1) as usize
        } else {
            constant445_out1 as usize
        };
        let gather41_out1 = shape51_out1[actual_idx] as i64;
        let constant446_out1: i64 = 2i64;
        let div42_out1 = gather41_out1 / constant446_out1;
        let cast69_out1 = div42_out1;
        let cast70_out1 = cast69_out1;
        let unsqueeze68_out1 = [cast70_out1];
        let slice59_out1 = squeeze26_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze68_out1[0]]);
        let unsqueeze69_out1 = [cast70_out1];
        let slice60_out1 =
            squeeze26_out1.slice(s![.., .., .., unsqueeze69_out1[0]..9223372036854775807]);
        let neg18_out1 = slice60_out1.neg();
        let concat40_out1 = burn::tensor::Tensor::cat([neg18_out1, slice59_out1].into(), 3);
        let mul99_out1 = concat40_out1.mul(unsqueeze23_out1.clone());
        let add58_out1 = mul98_out1.add(mul99_out1);
        let shape52_out1: [i64; 4] = add57_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice61_out1: [i64; 1] = shape52_out1[3..4].try_into().unwrap();
        let cast71_out1 = {
            let shape_array = slice61_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt25_out1 = cast71_out1.sqrt();
        let constant457_out1 = self.constant457.val();
        let div43_out1 = constant457_out1.div(sqrt25_out1);
        let cast72_out1 = div43_out1;
        let transpose29_out1 = add58_out1.permute([0, 1, 3, 2]);
        let sqrt26_out1 = cast72_out1.clone().sqrt();
        let mul100_out1 = add57_out1.mul(sqrt26_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt27_out1 = cast72_out1.sqrt();
        let mul101_out1 =
            transpose29_out1.mul(sqrt27_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul52_out1 = mul100_out1.matmul(mul101_out1);
        let add59_out1 = matmul52_out1.add(where3_out1.clone());
        let softmax9_out1 = burn::tensor::activation::softmax(add59_out1, 3);
        let matmul53_out1 = softmax9_out1.matmul(squeeze27_out1);
        let transpose30_out1 = matmul53_out1.permute([0, 2, 1, 3]);
        let unsqueeze70_out1 = [gather39_out1];
        let constant459_out1: [i64; 1] = [-1i64];
        let constant460_out1: [i64; 1] = [768i64];
        let concat41_out1: [i64; 3usize] = [
            &unsqueeze70_out1[..],
            &constant459_out1[..],
            &constant460_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape21_out1 = transpose30_out1.reshape(concat41_out1);
        let matmul54_out1 = self.matmul54.forward(reshape21_out1);
        let add60_out1 = add56_out1.add(matmul54_out1);
        let layernormalization18_out1 = {
            let dtype = add60_out1.clone().dtype();
            self.layernormalization18
                .forward(add60_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul55_out1 = self.matmul55.forward(layernormalization18_out1);
        let shape53_out1: [i64; 3] = matmul55_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant462_out1: [i64; 1] = [-1i64];
        let gather42_out1: [i64; 1usize] = constant462_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape53_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape53_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant464_out1: [i64; 1] = [1i64];
        let add61_out1 = {
            let mut result = gather42_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant464_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant465_out1: [i64; 1] = [2i64];
        let div44_out1 = {
            let mut result = add61_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant465_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant466_out1: [i64; 1] = [1i64];
        let mul102_out1 = {
            let mut result = div44_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant466_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice62_out1 = matmul55_out1.clone().slice(s![.., .., 0..mul102_out1[0]]);
        let constant467_out1: [i64; 1] = [2i64];
        let mul103_out1 = {
            let mut result = div44_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant467_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice63_out1 = matmul55_out1.slice(s![.., .., mul102_out1[0]..mul103_out1[0]]);
        let constant468_out1: f32 = 1.4142135f32;
        let div45_out1 = slice62_out1.clone().div_scalar(constant468_out1);
        let erf9_out1 = div45_out1.erf();
        let constant469_out1: f32 = 1f32;
        let add62_out1 = erf9_out1.add_scalar(constant469_out1);
        let mul104_out1 = slice62_out1.mul(add62_out1);
        let constant470_out1: f32 = 0.5f32;
        let mul105_out1 = mul104_out1.mul_scalar(constant470_out1);
        let mul106_out1 = mul105_out1.mul(slice63_out1);
        let matmul56_out1 = self.matmul56.forward(mul106_out1);
        let add63_out1 = add60_out1.add(matmul56_out1);
        let layernormalization19_out1 = {
            let dtype = add63_out1.clone().dtype();
            self.layernormalization19
                .forward(add63_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul57_out1 = self.matmul57.forward(layernormalization19_out1.clone());
        let shape54_out1: [i64; 3] = layernormalization19_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant472_out1: i64 = 0i64;
        let actual_idx = if constant472_out1 < 0 {
            (shape54_out1.len() as i64 + constant472_out1) as usize
        } else {
            constant472_out1 as usize
        };
        let gather43_out1 = shape54_out1[actual_idx] as i64;
        let unsqueeze71_out1 = [gather43_out1];
        let constant474_out1: [i64; 1] = [-1i64];
        let constant475_out1: [i64; 1] = [3i64];
        let constant476_out1: [i64; 1] = [12i64];
        let constant477_out1: [i64; 1] = [64i64];
        let concat42_out1: [i64; 5usize] = [
            &unsqueeze71_out1[..],
            &constant474_out1[..],
            &constant475_out1[..],
            &constant476_out1[..],
            &constant477_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape22_out1 = matmul57_out1.reshape(concat42_out1);
        let transpose31_out1 = reshape22_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose31_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split10_out1, split10_out2, split10_out3] = split_tensors.try_into().unwrap();
        let squeeze28_out1 = split10_out1.squeeze_dims(&[2]);
        let squeeze29_out1 = split10_out2.squeeze_dims(&[2]);
        let squeeze30_out1 = split10_out3.squeeze_dims(&[2]);
        let mul107_out1 = squeeze28_out1.clone().mul(unsqueeze13_out1.clone());
        let shape55_out1: [i64; 4] = squeeze28_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant482_out1: i64 = 3i64;
        let actual_idx = if constant482_out1 < 0 {
            (shape55_out1.len() as i64 + constant482_out1) as usize
        } else {
            constant482_out1 as usize
        };
        let gather44_out1 = shape55_out1[actual_idx] as i64;
        let constant483_out1: i64 = 2i64;
        let div46_out1 = gather44_out1 / constant483_out1;
        let cast73_out1 = div46_out1;
        let cast74_out1 = cast73_out1;
        let unsqueeze72_out1 = [cast74_out1];
        let slice64_out1 = squeeze28_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze72_out1[0]]);
        let unsqueeze73_out1 = [cast74_out1];
        let slice65_out1 =
            squeeze28_out1.slice(s![.., .., .., unsqueeze73_out1[0]..9223372036854775807]);
        let neg19_out1 = slice65_out1.neg();
        let concat43_out1 = burn::tensor::Tensor::cat([neg19_out1, slice64_out1].into(), 3);
        let mul108_out1 = concat43_out1.mul(unsqueeze14_out1.clone());
        let add64_out1 = mul107_out1.add(mul108_out1);
        let mul109_out1 = squeeze29_out1.clone().mul(unsqueeze13_out1.clone());
        let shape56_out1: [i64; 4] = squeeze29_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant492_out1: i64 = 3i64;
        let actual_idx = if constant492_out1 < 0 {
            (shape56_out1.len() as i64 + constant492_out1) as usize
        } else {
            constant492_out1 as usize
        };
        let gather45_out1 = shape56_out1[actual_idx] as i64;
        let constant493_out1: i64 = 2i64;
        let div47_out1 = gather45_out1 / constant493_out1;
        let cast75_out1 = div47_out1;
        let cast76_out1 = cast75_out1;
        let unsqueeze74_out1 = [cast76_out1];
        let slice66_out1 = squeeze29_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze74_out1[0]]);
        let unsqueeze75_out1 = [cast76_out1];
        let slice67_out1 =
            squeeze29_out1.slice(s![.., .., .., unsqueeze75_out1[0]..9223372036854775807]);
        let neg20_out1 = slice67_out1.neg();
        let concat44_out1 = burn::tensor::Tensor::cat([neg20_out1, slice66_out1].into(), 3);
        let mul110_out1 = concat44_out1.mul(unsqueeze14_out1.clone());
        let add65_out1 = mul109_out1.add(mul110_out1);
        let shape57_out1: [i64; 4] = add64_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice68_out1: [i64; 1] = shape57_out1[3..4].try_into().unwrap();
        let cast77_out1 = {
            let shape_array = slice68_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt28_out1 = cast77_out1.sqrt();
        let constant504_out1 = self.constant504.val();
        let div48_out1 = constant504_out1.div(sqrt28_out1);
        let cast78_out1 = div48_out1;
        let transpose32_out1 = add65_out1.permute([0, 1, 3, 2]);
        let sqrt29_out1 = cast78_out1.clone().sqrt();
        let mul111_out1 = add64_out1.mul(sqrt29_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt30_out1 = cast78_out1.sqrt();
        let mul112_out1 =
            transpose32_out1.mul(sqrt30_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul58_out1 = mul111_out1.matmul(mul112_out1);
        let add66_out1 = matmul58_out1.add(where2_out1.clone());
        let softmax10_out1 = burn::tensor::activation::softmax(add66_out1, 3);
        let matmul59_out1 = softmax10_out1.matmul(squeeze30_out1);
        let transpose33_out1 = matmul59_out1.permute([0, 2, 1, 3]);
        let unsqueeze76_out1 = [gather43_out1];
        let constant506_out1: [i64; 1] = [-1i64];
        let constant507_out1: [i64; 1] = [768i64];
        let concat45_out1: [i64; 3usize] = [
            &unsqueeze76_out1[..],
            &constant506_out1[..],
            &constant507_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape23_out1 = transpose33_out1.reshape(concat45_out1);
        let matmul60_out1 = self.matmul60.forward(reshape23_out1);
        let add67_out1 = add63_out1.add(matmul60_out1);
        let layernormalization20_out1 = {
            let dtype = add67_out1.clone().dtype();
            self.layernormalization20
                .forward(add67_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul61_out1 = self.matmul61.forward(layernormalization20_out1);
        let shape58_out1: [i64; 3] = matmul61_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant509_out1: [i64; 1] = [-1i64];
        let gather46_out1: [i64; 1usize] = constant509_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape58_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape58_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant511_out1: [i64; 1] = [1i64];
        let add68_out1 = {
            let mut result = gather46_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant511_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant512_out1: [i64; 1] = [2i64];
        let div49_out1 = {
            let mut result = add68_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant512_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant513_out1: [i64; 1] = [1i64];
        let mul113_out1 = {
            let mut result = div49_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant513_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice69_out1 = matmul61_out1.clone().slice(s![.., .., 0..mul113_out1[0]]);
        let constant514_out1: [i64; 1] = [2i64];
        let mul114_out1 = {
            let mut result = div49_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant514_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice70_out1 = matmul61_out1.slice(s![.., .., mul113_out1[0]..mul114_out1[0]]);
        let constant515_out1: f32 = 1.4142135f32;
        let div50_out1 = slice69_out1.clone().div_scalar(constant515_out1);
        let erf10_out1 = div50_out1.erf();
        let constant516_out1: f32 = 1f32;
        let add69_out1 = erf10_out1.add_scalar(constant516_out1);
        let mul115_out1 = slice69_out1.mul(add69_out1);
        let constant517_out1: f32 = 0.5f32;
        let mul116_out1 = mul115_out1.mul_scalar(constant517_out1);
        let mul117_out1 = mul116_out1.mul(slice70_out1);
        let matmul62_out1 = self.matmul62.forward(mul117_out1);
        let add70_out1 = add67_out1.add(matmul62_out1);
        let layernormalization21_out1 = {
            let dtype = add70_out1.clone().dtype();
            self.layernormalization21
                .forward(add70_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul63_out1 = self.matmul63.forward(layernormalization21_out1.clone());
        let shape59_out1: [i64; 3] = layernormalization21_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant519_out1: i64 = 0i64;
        let actual_idx = if constant519_out1 < 0 {
            (shape59_out1.len() as i64 + constant519_out1) as usize
        } else {
            constant519_out1 as usize
        };
        let gather47_out1 = shape59_out1[actual_idx] as i64;
        let unsqueeze77_out1 = [gather47_out1];
        let constant521_out1: [i64; 1] = [-1i64];
        let constant522_out1: [i64; 1] = [3i64];
        let constant523_out1: [i64; 1] = [12i64];
        let constant524_out1: [i64; 1] = [64i64];
        let concat46_out1: [i64; 5usize] = [
            &unsqueeze77_out1[..],
            &constant521_out1[..],
            &constant522_out1[..],
            &constant523_out1[..],
            &constant524_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape24_out1 = matmul63_out1.reshape(concat46_out1);
        let transpose34_out1 = reshape24_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose34_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split11_out1, split11_out2, split11_out3] = split_tensors.try_into().unwrap();
        let squeeze31_out1 = split11_out1.squeeze_dims(&[2]);
        let squeeze32_out1 = split11_out2.squeeze_dims(&[2]);
        let squeeze33_out1 = split11_out3.squeeze_dims(&[2]);
        let mul118_out1 = squeeze31_out1.clone().mul(unsqueeze22_out1.clone());
        let shape60_out1: [i64; 4] = squeeze31_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant529_out1: i64 = 3i64;
        let actual_idx = if constant529_out1 < 0 {
            (shape60_out1.len() as i64 + constant529_out1) as usize
        } else {
            constant529_out1 as usize
        };
        let gather48_out1 = shape60_out1[actual_idx] as i64;
        let constant530_out1: i64 = 2i64;
        let div51_out1 = gather48_out1 / constant530_out1;
        let cast79_out1 = div51_out1;
        let cast80_out1 = cast79_out1;
        let unsqueeze78_out1 = [cast80_out1];
        let slice71_out1 = squeeze31_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze78_out1[0]]);
        let unsqueeze79_out1 = [cast80_out1];
        let slice72_out1 =
            squeeze31_out1.slice(s![.., .., .., unsqueeze79_out1[0]..9223372036854775807]);
        let neg21_out1 = slice72_out1.neg();
        let concat47_out1 = burn::tensor::Tensor::cat([neg21_out1, slice71_out1].into(), 3);
        let mul119_out1 = concat47_out1.mul(unsqueeze23_out1.clone());
        let add71_out1 = mul118_out1.add(mul119_out1);
        let mul120_out1 = squeeze32_out1.clone().mul(unsqueeze22_out1.clone());
        let shape61_out1: [i64; 4] = squeeze32_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant539_out1: i64 = 3i64;
        let actual_idx = if constant539_out1 < 0 {
            (shape61_out1.len() as i64 + constant539_out1) as usize
        } else {
            constant539_out1 as usize
        };
        let gather49_out1 = shape61_out1[actual_idx] as i64;
        let constant540_out1: i64 = 2i64;
        let div52_out1 = gather49_out1 / constant540_out1;
        let cast81_out1 = div52_out1;
        let cast82_out1 = cast81_out1;
        let unsqueeze80_out1 = [cast82_out1];
        let slice73_out1 = squeeze32_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze80_out1[0]]);
        let unsqueeze81_out1 = [cast82_out1];
        let slice74_out1 =
            squeeze32_out1.slice(s![.., .., .., unsqueeze81_out1[0]..9223372036854775807]);
        let neg22_out1 = slice74_out1.neg();
        let concat48_out1 = burn::tensor::Tensor::cat([neg22_out1, slice73_out1].into(), 3);
        let mul121_out1 = concat48_out1.mul(unsqueeze23_out1.clone());
        let add72_out1 = mul120_out1.add(mul121_out1);
        let shape62_out1: [i64; 4] = add71_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice75_out1: [i64; 1] = shape62_out1[3..4].try_into().unwrap();
        let cast83_out1 = {
            let shape_array = slice75_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt31_out1 = cast83_out1.sqrt();
        let constant551_out1 = self.constant551.val();
        let div53_out1 = constant551_out1.div(sqrt31_out1);
        let cast84_out1 = div53_out1;
        let transpose35_out1 = add72_out1.permute([0, 1, 3, 2]);
        let sqrt32_out1 = cast84_out1.clone().sqrt();
        let mul122_out1 = add71_out1.mul(sqrt32_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt33_out1 = cast84_out1.sqrt();
        let mul123_out1 =
            transpose35_out1.mul(sqrt33_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul64_out1 = mul122_out1.matmul(mul123_out1);
        let add73_out1 = matmul64_out1.add(where3_out1.clone());
        let softmax11_out1 = burn::tensor::activation::softmax(add73_out1, 3);
        let matmul65_out1 = softmax11_out1.matmul(squeeze33_out1);
        let transpose36_out1 = matmul65_out1.permute([0, 2, 1, 3]);
        let unsqueeze82_out1 = [gather47_out1];
        let constant553_out1: [i64; 1] = [-1i64];
        let constant554_out1: [i64; 1] = [768i64];
        let concat49_out1: [i64; 3usize] = [
            &unsqueeze82_out1[..],
            &constant553_out1[..],
            &constant554_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape25_out1 = transpose36_out1.reshape(concat49_out1);
        let matmul66_out1 = self.matmul66.forward(reshape25_out1);
        let add74_out1 = add70_out1.add(matmul66_out1);
        let layernormalization22_out1 = {
            let dtype = add74_out1.clone().dtype();
            self.layernormalization22
                .forward(add74_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul67_out1 = self.matmul67.forward(layernormalization22_out1);
        let shape63_out1: [i64; 3] = matmul67_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant556_out1: [i64; 1] = [-1i64];
        let gather50_out1: [i64; 1usize] = constant556_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape63_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape63_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant558_out1: [i64; 1] = [1i64];
        let add75_out1 = {
            let mut result = gather50_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant558_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant559_out1: [i64; 1] = [2i64];
        let div54_out1 = {
            let mut result = add75_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant559_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant560_out1: [i64; 1] = [1i64];
        let mul124_out1 = {
            let mut result = div54_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant560_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice76_out1 = matmul67_out1.clone().slice(s![.., .., 0..mul124_out1[0]]);
        let constant561_out1: [i64; 1] = [2i64];
        let mul125_out1 = {
            let mut result = div54_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant561_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice77_out1 = matmul67_out1.slice(s![.., .., mul124_out1[0]..mul125_out1[0]]);
        let constant562_out1: f32 = 1.4142135f32;
        let div55_out1 = slice76_out1.clone().div_scalar(constant562_out1);
        let erf11_out1 = div55_out1.erf();
        let constant563_out1: f32 = 1f32;
        let add76_out1 = erf11_out1.add_scalar(constant563_out1);
        let mul126_out1 = slice76_out1.mul(add76_out1);
        let constant564_out1: f32 = 0.5f32;
        let mul127_out1 = mul126_out1.mul_scalar(constant564_out1);
        let mul128_out1 = mul127_out1.mul(slice77_out1);
        let matmul68_out1 = self.matmul68.forward(mul128_out1);
        let add77_out1 = add74_out1.add(matmul68_out1);
        let layernormalization23_out1 = {
            let dtype = add77_out1.clone().dtype();
            self.layernormalization23
                .forward(add77_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul69_out1 = self.matmul69.forward(layernormalization23_out1.clone());
        let shape64_out1: [i64; 3] = layernormalization23_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant566_out1: i64 = 0i64;
        let actual_idx = if constant566_out1 < 0 {
            (shape64_out1.len() as i64 + constant566_out1) as usize
        } else {
            constant566_out1 as usize
        };
        let gather51_out1 = shape64_out1[actual_idx] as i64;
        let unsqueeze83_out1 = [gather51_out1];
        let constant568_out1: [i64; 1] = [-1i64];
        let constant569_out1: [i64; 1] = [3i64];
        let constant570_out1: [i64; 1] = [12i64];
        let constant571_out1: [i64; 1] = [64i64];
        let concat50_out1: [i64; 5usize] = [
            &unsqueeze83_out1[..],
            &constant568_out1[..],
            &constant569_out1[..],
            &constant570_out1[..],
            &constant571_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape26_out1 = matmul69_out1.reshape(concat50_out1);
        let transpose37_out1 = reshape26_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose37_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split12_out1, split12_out2, split12_out3] = split_tensors.try_into().unwrap();
        let squeeze34_out1 = split12_out1.squeeze_dims(&[2]);
        let squeeze35_out1 = split12_out2.squeeze_dims(&[2]);
        let squeeze36_out1 = split12_out3.squeeze_dims(&[2]);
        let mul129_out1 = squeeze34_out1.clone().mul(unsqueeze22_out1.clone());
        let shape65_out1: [i64; 4] = squeeze34_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant576_out1: i64 = 3i64;
        let actual_idx = if constant576_out1 < 0 {
            (shape65_out1.len() as i64 + constant576_out1) as usize
        } else {
            constant576_out1 as usize
        };
        let gather52_out1 = shape65_out1[actual_idx] as i64;
        let constant577_out1: i64 = 2i64;
        let div56_out1 = gather52_out1 / constant577_out1;
        let cast85_out1 = div56_out1;
        let cast86_out1 = cast85_out1;
        let unsqueeze84_out1 = [cast86_out1];
        let slice78_out1 = squeeze34_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze84_out1[0]]);
        let unsqueeze85_out1 = [cast86_out1];
        let slice79_out1 =
            squeeze34_out1.slice(s![.., .., .., unsqueeze85_out1[0]..9223372036854775807]);
        let neg23_out1 = slice79_out1.neg();
        let concat51_out1 = burn::tensor::Tensor::cat([neg23_out1, slice78_out1].into(), 3);
        let mul130_out1 = concat51_out1.mul(unsqueeze23_out1.clone());
        let add78_out1 = mul129_out1.add(mul130_out1);
        let mul131_out1 = squeeze35_out1.clone().mul(unsqueeze22_out1.clone());
        let shape66_out1: [i64; 4] = squeeze35_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant586_out1: i64 = 3i64;
        let actual_idx = if constant586_out1 < 0 {
            (shape66_out1.len() as i64 + constant586_out1) as usize
        } else {
            constant586_out1 as usize
        };
        let gather53_out1 = shape66_out1[actual_idx] as i64;
        let constant587_out1: i64 = 2i64;
        let div57_out1 = gather53_out1 / constant587_out1;
        let cast87_out1 = div57_out1;
        let cast88_out1 = cast87_out1;
        let unsqueeze86_out1 = [cast88_out1];
        let slice80_out1 = squeeze35_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze86_out1[0]]);
        let unsqueeze87_out1 = [cast88_out1];
        let slice81_out1 =
            squeeze35_out1.slice(s![.., .., .., unsqueeze87_out1[0]..9223372036854775807]);
        let neg24_out1 = slice81_out1.neg();
        let concat52_out1 = burn::tensor::Tensor::cat([neg24_out1, slice80_out1].into(), 3);
        let mul132_out1 = concat52_out1.mul(unsqueeze23_out1.clone());
        let add79_out1 = mul131_out1.add(mul132_out1);
        let shape67_out1: [i64; 4] = add78_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice82_out1: [i64; 1] = shape67_out1[3..4].try_into().unwrap();
        let cast89_out1 = {
            let shape_array = slice82_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt34_out1 = cast89_out1.sqrt();
        let constant598_out1 = self.constant598.val();
        let div58_out1 = constant598_out1.div(sqrt34_out1);
        let cast90_out1 = div58_out1;
        let transpose38_out1 = add79_out1.permute([0, 1, 3, 2]);
        let sqrt35_out1 = cast90_out1.clone().sqrt();
        let mul133_out1 = add78_out1.mul(sqrt35_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt36_out1 = cast90_out1.sqrt();
        let mul134_out1 =
            transpose38_out1.mul(sqrt36_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul70_out1 = mul133_out1.matmul(mul134_out1);
        let add80_out1 = matmul70_out1.add(where3_out1.clone());
        let softmax12_out1 = burn::tensor::activation::softmax(add80_out1, 3);
        let matmul71_out1 = softmax12_out1.matmul(squeeze36_out1);
        let transpose39_out1 = matmul71_out1.permute([0, 2, 1, 3]);
        let unsqueeze88_out1 = [gather51_out1];
        let constant600_out1: [i64; 1] = [-1i64];
        let constant601_out1: [i64; 1] = [768i64];
        let concat53_out1: [i64; 3usize] = [
            &unsqueeze88_out1[..],
            &constant600_out1[..],
            &constant601_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape27_out1 = transpose39_out1.reshape(concat53_out1);
        let matmul72_out1 = self.matmul72.forward(reshape27_out1);
        let add81_out1 = add77_out1.add(matmul72_out1);
        let layernormalization24_out1 = {
            let dtype = add81_out1.clone().dtype();
            self.layernormalization24
                .forward(add81_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul73_out1 = self.matmul73.forward(layernormalization24_out1);
        let shape68_out1: [i64; 3] = matmul73_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant603_out1: [i64; 1] = [-1i64];
        let gather54_out1: [i64; 1usize] = constant603_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape68_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape68_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant605_out1: [i64; 1] = [1i64];
        let add82_out1 = {
            let mut result = gather54_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant605_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant606_out1: [i64; 1] = [2i64];
        let div59_out1 = {
            let mut result = add82_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant606_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant607_out1: [i64; 1] = [1i64];
        let mul135_out1 = {
            let mut result = div59_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant607_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice83_out1 = matmul73_out1.clone().slice(s![.., .., 0..mul135_out1[0]]);
        let constant608_out1: [i64; 1] = [2i64];
        let mul136_out1 = {
            let mut result = div59_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant608_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice84_out1 = matmul73_out1.slice(s![.., .., mul135_out1[0]..mul136_out1[0]]);
        let constant609_out1: f32 = 1.4142135f32;
        let div60_out1 = slice83_out1.clone().div_scalar(constant609_out1);
        let erf12_out1 = div60_out1.erf();
        let constant610_out1: f32 = 1f32;
        let add83_out1 = erf12_out1.add_scalar(constant610_out1);
        let mul137_out1 = slice83_out1.mul(add83_out1);
        let constant611_out1: f32 = 0.5f32;
        let mul138_out1 = mul137_out1.mul_scalar(constant611_out1);
        let mul139_out1 = mul138_out1.mul(slice84_out1);
        let matmul74_out1 = self.matmul74.forward(mul139_out1);
        let add84_out1 = add81_out1.add(matmul74_out1);
        let layernormalization25_out1 = {
            let dtype = add84_out1.clone().dtype();
            self.layernormalization25
                .forward(add84_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul75_out1 = self.matmul75.forward(layernormalization25_out1.clone());
        let shape69_out1: [i64; 3] = layernormalization25_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant613_out1: i64 = 0i64;
        let actual_idx = if constant613_out1 < 0 {
            (shape69_out1.len() as i64 + constant613_out1) as usize
        } else {
            constant613_out1 as usize
        };
        let gather55_out1 = shape69_out1[actual_idx] as i64;
        let unsqueeze89_out1 = [gather55_out1];
        let constant615_out1: [i64; 1] = [-1i64];
        let constant616_out1: [i64; 1] = [3i64];
        let constant617_out1: [i64; 1] = [12i64];
        let constant618_out1: [i64; 1] = [64i64];
        let concat54_out1: [i64; 5usize] = [
            &unsqueeze89_out1[..],
            &constant615_out1[..],
            &constant616_out1[..],
            &constant617_out1[..],
            &constant618_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape28_out1 = matmul75_out1.reshape(concat54_out1);
        let transpose40_out1 = reshape28_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose40_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split13_out1, split13_out2, split13_out3] = split_tensors.try_into().unwrap();
        let squeeze37_out1 = split13_out1.squeeze_dims(&[2]);
        let squeeze38_out1 = split13_out2.squeeze_dims(&[2]);
        let squeeze39_out1 = split13_out3.squeeze_dims(&[2]);
        let mul140_out1 = squeeze37_out1.clone().mul(unsqueeze13_out1.clone());
        let shape70_out1: [i64; 4] = squeeze37_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant623_out1: i64 = 3i64;
        let actual_idx = if constant623_out1 < 0 {
            (shape70_out1.len() as i64 + constant623_out1) as usize
        } else {
            constant623_out1 as usize
        };
        let gather56_out1 = shape70_out1[actual_idx] as i64;
        let constant624_out1: i64 = 2i64;
        let div61_out1 = gather56_out1 / constant624_out1;
        let cast91_out1 = div61_out1;
        let cast92_out1 = cast91_out1;
        let unsqueeze90_out1 = [cast92_out1];
        let slice85_out1 = squeeze37_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze90_out1[0]]);
        let unsqueeze91_out1 = [cast92_out1];
        let slice86_out1 =
            squeeze37_out1.slice(s![.., .., .., unsqueeze91_out1[0]..9223372036854775807]);
        let neg25_out1 = slice86_out1.neg();
        let concat55_out1 = burn::tensor::Tensor::cat([neg25_out1, slice85_out1].into(), 3);
        let mul141_out1 = concat55_out1.mul(unsqueeze14_out1.clone());
        let add85_out1 = mul140_out1.add(mul141_out1);
        let mul142_out1 = squeeze38_out1.clone().mul(unsqueeze13_out1.clone());
        let shape71_out1: [i64; 4] = squeeze38_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant633_out1: i64 = 3i64;
        let actual_idx = if constant633_out1 < 0 {
            (shape71_out1.len() as i64 + constant633_out1) as usize
        } else {
            constant633_out1 as usize
        };
        let gather57_out1 = shape71_out1[actual_idx] as i64;
        let constant634_out1: i64 = 2i64;
        let div62_out1 = gather57_out1 / constant634_out1;
        let cast93_out1 = div62_out1;
        let cast94_out1 = cast93_out1;
        let unsqueeze92_out1 = [cast94_out1];
        let slice87_out1 = squeeze38_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze92_out1[0]]);
        let unsqueeze93_out1 = [cast94_out1];
        let slice88_out1 =
            squeeze38_out1.slice(s![.., .., .., unsqueeze93_out1[0]..9223372036854775807]);
        let neg26_out1 = slice88_out1.neg();
        let concat56_out1 = burn::tensor::Tensor::cat([neg26_out1, slice87_out1].into(), 3);
        let mul143_out1 = concat56_out1.mul(unsqueeze14_out1.clone());
        let add86_out1 = mul142_out1.add(mul143_out1);
        let shape72_out1: [i64; 4] = add85_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice89_out1: [i64; 1] = shape72_out1[3..4].try_into().unwrap();
        let cast95_out1 = {
            let shape_array = slice89_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt37_out1 = cast95_out1.sqrt();
        let constant645_out1 = self.constant645.val();
        let div63_out1 = constant645_out1.div(sqrt37_out1);
        let cast96_out1 = div63_out1;
        let transpose41_out1 = add86_out1.permute([0, 1, 3, 2]);
        let sqrt38_out1 = cast96_out1.clone().sqrt();
        let mul144_out1 = add85_out1.mul(sqrt38_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt39_out1 = cast96_out1.sqrt();
        let mul145_out1 =
            transpose41_out1.mul(sqrt39_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul76_out1 = mul144_out1.matmul(mul145_out1);
        let add87_out1 = matmul76_out1.add(where2_out1.clone());
        let softmax13_out1 = burn::tensor::activation::softmax(add87_out1, 3);
        let matmul77_out1 = softmax13_out1.matmul(squeeze39_out1);
        let transpose42_out1 = matmul77_out1.permute([0, 2, 1, 3]);
        let unsqueeze94_out1 = [gather55_out1];
        let constant647_out1: [i64; 1] = [-1i64];
        let constant648_out1: [i64; 1] = [768i64];
        let concat57_out1: [i64; 3usize] = [
            &unsqueeze94_out1[..],
            &constant647_out1[..],
            &constant648_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape29_out1 = transpose42_out1.reshape(concat57_out1);
        let matmul78_out1 = self.matmul78.forward(reshape29_out1);
        let add88_out1 = add84_out1.add(matmul78_out1);
        let layernormalization26_out1 = {
            let dtype = add88_out1.clone().dtype();
            self.layernormalization26
                .forward(add88_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul79_out1 = self.matmul79.forward(layernormalization26_out1);
        let shape73_out1: [i64; 3] = matmul79_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant650_out1: [i64; 1] = [-1i64];
        let gather58_out1: [i64; 1usize] = constant650_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape73_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape73_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant652_out1: [i64; 1] = [1i64];
        let add89_out1 = {
            let mut result = gather58_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant652_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant653_out1: [i64; 1] = [2i64];
        let div64_out1 = {
            let mut result = add89_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant653_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant654_out1: [i64; 1] = [1i64];
        let mul146_out1 = {
            let mut result = div64_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant654_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice90_out1 = matmul79_out1.clone().slice(s![.., .., 0..mul146_out1[0]]);
        let constant655_out1: [i64; 1] = [2i64];
        let mul147_out1 = {
            let mut result = div64_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant655_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice91_out1 = matmul79_out1.slice(s![.., .., mul146_out1[0]..mul147_out1[0]]);
        let constant656_out1: f32 = 1.4142135f32;
        let div65_out1 = slice90_out1.clone().div_scalar(constant656_out1);
        let erf13_out1 = div65_out1.erf();
        let constant657_out1: f32 = 1f32;
        let add90_out1 = erf13_out1.add_scalar(constant657_out1);
        let mul148_out1 = slice90_out1.mul(add90_out1);
        let constant658_out1: f32 = 0.5f32;
        let mul149_out1 = mul148_out1.mul_scalar(constant658_out1);
        let mul150_out1 = mul149_out1.mul(slice91_out1);
        let matmul80_out1 = self.matmul80.forward(mul150_out1);
        let add91_out1 = add88_out1.add(matmul80_out1);
        let layernormalization27_out1 = {
            let dtype = add91_out1.clone().dtype();
            self.layernormalization27
                .forward(add91_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul81_out1 = self.matmul81.forward(layernormalization27_out1.clone());
        let shape74_out1: [i64; 3] = layernormalization27_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant660_out1: i64 = 0i64;
        let actual_idx = if constant660_out1 < 0 {
            (shape74_out1.len() as i64 + constant660_out1) as usize
        } else {
            constant660_out1 as usize
        };
        let gather59_out1 = shape74_out1[actual_idx] as i64;
        let unsqueeze95_out1 = [gather59_out1];
        let constant662_out1: [i64; 1] = [-1i64];
        let constant663_out1: [i64; 1] = [3i64];
        let constant664_out1: [i64; 1] = [12i64];
        let constant665_out1: [i64; 1] = [64i64];
        let concat58_out1: [i64; 5usize] = [
            &unsqueeze95_out1[..],
            &constant662_out1[..],
            &constant663_out1[..],
            &constant664_out1[..],
            &constant665_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape30_out1 = matmul81_out1.reshape(concat58_out1);
        let transpose43_out1 = reshape30_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose43_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split14_out1, split14_out2, split14_out3] = split_tensors.try_into().unwrap();
        let squeeze40_out1 = split14_out1.squeeze_dims(&[2]);
        let squeeze41_out1 = split14_out2.squeeze_dims(&[2]);
        let squeeze42_out1 = split14_out3.squeeze_dims(&[2]);
        let mul151_out1 = squeeze40_out1.clone().mul(unsqueeze22_out1.clone());
        let shape75_out1: [i64; 4] = squeeze40_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant670_out1: i64 = 3i64;
        let actual_idx = if constant670_out1 < 0 {
            (shape75_out1.len() as i64 + constant670_out1) as usize
        } else {
            constant670_out1 as usize
        };
        let gather60_out1 = shape75_out1[actual_idx] as i64;
        let constant671_out1: i64 = 2i64;
        let div66_out1 = gather60_out1 / constant671_out1;
        let cast97_out1 = div66_out1;
        let cast98_out1 = cast97_out1;
        let unsqueeze96_out1 = [cast98_out1];
        let slice92_out1 = squeeze40_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze96_out1[0]]);
        let unsqueeze97_out1 = [cast98_out1];
        let slice93_out1 =
            squeeze40_out1.slice(s![.., .., .., unsqueeze97_out1[0]..9223372036854775807]);
        let neg27_out1 = slice93_out1.neg();
        let concat59_out1 = burn::tensor::Tensor::cat([neg27_out1, slice92_out1].into(), 3);
        let mul152_out1 = concat59_out1.mul(unsqueeze23_out1.clone());
        let add92_out1 = mul151_out1.add(mul152_out1);
        let mul153_out1 = squeeze41_out1.clone().mul(unsqueeze22_out1.clone());
        let shape76_out1: [i64; 4] = squeeze41_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant680_out1: i64 = 3i64;
        let actual_idx = if constant680_out1 < 0 {
            (shape76_out1.len() as i64 + constant680_out1) as usize
        } else {
            constant680_out1 as usize
        };
        let gather61_out1 = shape76_out1[actual_idx] as i64;
        let constant681_out1: i64 = 2i64;
        let div67_out1 = gather61_out1 / constant681_out1;
        let cast99_out1 = div67_out1;
        let cast100_out1 = cast99_out1;
        let unsqueeze98_out1 = [cast100_out1];
        let slice94_out1 = squeeze41_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze98_out1[0]]);
        let unsqueeze99_out1 = [cast100_out1];
        let slice95_out1 =
            squeeze41_out1.slice(s![.., .., .., unsqueeze99_out1[0]..9223372036854775807]);
        let neg28_out1 = slice95_out1.neg();
        let concat60_out1 = burn::tensor::Tensor::cat([neg28_out1, slice94_out1].into(), 3);
        let mul154_out1 = concat60_out1.mul(unsqueeze23_out1.clone());
        let add93_out1 = mul153_out1.add(mul154_out1);
        let shape77_out1: [i64; 4] = add92_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice96_out1: [i64; 1] = shape77_out1[3..4].try_into().unwrap();
        let cast101_out1 = {
            let shape_array = slice96_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt40_out1 = cast101_out1.sqrt();
        let constant692_out1 = self.constant692.val();
        let div68_out1 = constant692_out1.div(sqrt40_out1);
        let cast102_out1 = div68_out1;
        let transpose44_out1 = add93_out1.permute([0, 1, 3, 2]);
        let sqrt41_out1 = cast102_out1.clone().sqrt();
        let mul155_out1 = add92_out1.mul(sqrt41_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt42_out1 = cast102_out1.sqrt();
        let mul156_out1 =
            transpose44_out1.mul(sqrt42_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul82_out1 = mul155_out1.matmul(mul156_out1);
        let add94_out1 = matmul82_out1.add(where3_out1.clone());
        let softmax14_out1 = burn::tensor::activation::softmax(add94_out1, 3);
        let matmul83_out1 = softmax14_out1.matmul(squeeze42_out1);
        let transpose45_out1 = matmul83_out1.permute([0, 2, 1, 3]);
        let unsqueeze100_out1 = [gather59_out1];
        let constant694_out1: [i64; 1] = [-1i64];
        let constant695_out1: [i64; 1] = [768i64];
        let concat61_out1: [i64; 3usize] = [
            &unsqueeze100_out1[..],
            &constant694_out1[..],
            &constant695_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape31_out1 = transpose45_out1.reshape(concat61_out1);
        let matmul84_out1 = self.matmul84.forward(reshape31_out1);
        let add95_out1 = add91_out1.add(matmul84_out1);
        let layernormalization28_out1 = {
            let dtype = add95_out1.clone().dtype();
            self.layernormalization28
                .forward(add95_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul85_out1 = self.matmul85.forward(layernormalization28_out1);
        let shape78_out1: [i64; 3] = matmul85_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant697_out1: [i64; 1] = [-1i64];
        let gather62_out1: [i64; 1usize] = constant697_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape78_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape78_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant699_out1: [i64; 1] = [1i64];
        let add96_out1 = {
            let mut result = gather62_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant699_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant700_out1: [i64; 1] = [2i64];
        let div69_out1 = {
            let mut result = add96_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant700_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant701_out1: [i64; 1] = [1i64];
        let mul157_out1 = {
            let mut result = div69_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant701_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice97_out1 = matmul85_out1.clone().slice(s![.., .., 0..mul157_out1[0]]);
        let constant702_out1: [i64; 1] = [2i64];
        let mul158_out1 = {
            let mut result = div69_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant702_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice98_out1 = matmul85_out1.slice(s![.., .., mul157_out1[0]..mul158_out1[0]]);
        let constant703_out1: f32 = 1.4142135f32;
        let div70_out1 = slice97_out1.clone().div_scalar(constant703_out1);
        let erf14_out1 = div70_out1.erf();
        let constant704_out1: f32 = 1f32;
        let add97_out1 = erf14_out1.add_scalar(constant704_out1);
        let mul159_out1 = slice97_out1.mul(add97_out1);
        let constant705_out1: f32 = 0.5f32;
        let mul160_out1 = mul159_out1.mul_scalar(constant705_out1);
        let mul161_out1 = mul160_out1.mul(slice98_out1);
        let matmul86_out1 = self.matmul86.forward(mul161_out1);
        let add98_out1 = add95_out1.add(matmul86_out1);
        let layernormalization29_out1 = {
            let dtype = add98_out1.clone().dtype();
            self.layernormalization29
                .forward(add98_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul87_out1 = self.matmul87.forward(layernormalization29_out1.clone());
        let shape79_out1: [i64; 3] = layernormalization29_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant707_out1: i64 = 0i64;
        let actual_idx = if constant707_out1 < 0 {
            (shape79_out1.len() as i64 + constant707_out1) as usize
        } else {
            constant707_out1 as usize
        };
        let gather63_out1 = shape79_out1[actual_idx] as i64;
        let unsqueeze101_out1 = [gather63_out1];
        let constant709_out1: [i64; 1] = [-1i64];
        let constant710_out1: [i64; 1] = [3i64];
        let constant711_out1: [i64; 1] = [12i64];
        let constant712_out1: [i64; 1] = [64i64];
        let concat62_out1: [i64; 5usize] = [
            &unsqueeze101_out1[..],
            &constant709_out1[..],
            &constant710_out1[..],
            &constant711_out1[..],
            &constant712_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape32_out1 = matmul87_out1.reshape(concat62_out1);
        let transpose46_out1 = reshape32_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose46_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split15_out1, split15_out2, split15_out3] = split_tensors.try_into().unwrap();
        let squeeze43_out1 = split15_out1.squeeze_dims(&[2]);
        let squeeze44_out1 = split15_out2.squeeze_dims(&[2]);
        let squeeze45_out1 = split15_out3.squeeze_dims(&[2]);
        let mul162_out1 = squeeze43_out1.clone().mul(unsqueeze22_out1.clone());
        let shape80_out1: [i64; 4] = squeeze43_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant717_out1: i64 = 3i64;
        let actual_idx = if constant717_out1 < 0 {
            (shape80_out1.len() as i64 + constant717_out1) as usize
        } else {
            constant717_out1 as usize
        };
        let gather64_out1 = shape80_out1[actual_idx] as i64;
        let constant718_out1: i64 = 2i64;
        let div71_out1 = gather64_out1 / constant718_out1;
        let cast103_out1 = div71_out1;
        let cast104_out1 = cast103_out1;
        let unsqueeze102_out1 = [cast104_out1];
        let slice99_out1 = squeeze43_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze102_out1[0]]);
        let unsqueeze103_out1 = [cast104_out1];
        let slice100_out1 =
            squeeze43_out1.slice(s![.., .., .., unsqueeze103_out1[0]..9223372036854775807]);
        let neg29_out1 = slice100_out1.neg();
        let concat63_out1 = burn::tensor::Tensor::cat([neg29_out1, slice99_out1].into(), 3);
        let mul163_out1 = concat63_out1.mul(unsqueeze23_out1.clone());
        let add99_out1 = mul162_out1.add(mul163_out1);
        let mul164_out1 = squeeze44_out1.clone().mul(unsqueeze22_out1.clone());
        let shape81_out1: [i64; 4] = squeeze44_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant727_out1: i64 = 3i64;
        let actual_idx = if constant727_out1 < 0 {
            (shape81_out1.len() as i64 + constant727_out1) as usize
        } else {
            constant727_out1 as usize
        };
        let gather65_out1 = shape81_out1[actual_idx] as i64;
        let constant728_out1: i64 = 2i64;
        let div72_out1 = gather65_out1 / constant728_out1;
        let cast105_out1 = div72_out1;
        let cast106_out1 = cast105_out1;
        let unsqueeze104_out1 = [cast106_out1];
        let slice101_out1 = squeeze44_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze104_out1[0]]);
        let unsqueeze105_out1 = [cast106_out1];
        let slice102_out1 =
            squeeze44_out1.slice(s![.., .., .., unsqueeze105_out1[0]..9223372036854775807]);
        let neg30_out1 = slice102_out1.neg();
        let concat64_out1 = burn::tensor::Tensor::cat([neg30_out1, slice101_out1].into(), 3);
        let mul165_out1 = concat64_out1.mul(unsqueeze23_out1.clone());
        let add100_out1 = mul164_out1.add(mul165_out1);
        let shape82_out1: [i64; 4] = add99_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice103_out1: [i64; 1] = shape82_out1[3..4].try_into().unwrap();
        let cast107_out1 = {
            let shape_array = slice103_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt43_out1 = cast107_out1.sqrt();
        let constant739_out1 = self.constant739.val();
        let div73_out1 = constant739_out1.div(sqrt43_out1);
        let cast108_out1 = div73_out1;
        let transpose47_out1 = add100_out1.permute([0, 1, 3, 2]);
        let sqrt44_out1 = cast108_out1.clone().sqrt();
        let mul166_out1 = add99_out1.mul(sqrt44_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt45_out1 = cast108_out1.sqrt();
        let mul167_out1 =
            transpose47_out1.mul(sqrt45_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul88_out1 = mul166_out1.matmul(mul167_out1);
        let add101_out1 = matmul88_out1.add(where3_out1.clone());
        let softmax15_out1 = burn::tensor::activation::softmax(add101_out1, 3);
        let matmul89_out1 = softmax15_out1.matmul(squeeze45_out1);
        let transpose48_out1 = matmul89_out1.permute([0, 2, 1, 3]);
        let unsqueeze106_out1 = [gather63_out1];
        let constant741_out1: [i64; 1] = [-1i64];
        let constant742_out1: [i64; 1] = [768i64];
        let concat65_out1: [i64; 3usize] = [
            &unsqueeze106_out1[..],
            &constant741_out1[..],
            &constant742_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape33_out1 = transpose48_out1.reshape(concat65_out1);
        let matmul90_out1 = self.matmul90.forward(reshape33_out1);
        let add102_out1 = add98_out1.add(matmul90_out1);
        let layernormalization30_out1 = {
            let dtype = add102_out1.clone().dtype();
            self.layernormalization30
                .forward(add102_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul91_out1 = self.matmul91.forward(layernormalization30_out1);
        let shape83_out1: [i64; 3] = matmul91_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant744_out1: [i64; 1] = [-1i64];
        let gather66_out1: [i64; 1usize] = constant744_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape83_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape83_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant746_out1: [i64; 1] = [1i64];
        let add103_out1 = {
            let mut result = gather66_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant746_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant747_out1: [i64; 1] = [2i64];
        let div74_out1 = {
            let mut result = add103_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant747_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant748_out1: [i64; 1] = [1i64];
        let mul168_out1 = {
            let mut result = div74_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant748_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice104_out1 = matmul91_out1.clone().slice(s![.., .., 0..mul168_out1[0]]);
        let constant749_out1: [i64; 1] = [2i64];
        let mul169_out1 = {
            let mut result = div74_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant749_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice105_out1 = matmul91_out1.slice(s![.., .., mul168_out1[0]..mul169_out1[0]]);
        let constant750_out1: f32 = 1.4142135f32;
        let div75_out1 = slice104_out1.clone().div_scalar(constant750_out1);
        let erf15_out1 = div75_out1.erf();
        let constant751_out1: f32 = 1f32;
        let add104_out1 = erf15_out1.add_scalar(constant751_out1);
        let mul170_out1 = slice104_out1.mul(add104_out1);
        let constant752_out1: f32 = 0.5f32;
        let mul171_out1 = mul170_out1.mul_scalar(constant752_out1);
        let mul172_out1 = mul171_out1.mul(slice105_out1);
        let matmul92_out1 = self.matmul92.forward(mul172_out1);
        let add105_out1 = add102_out1.add(matmul92_out1);
        let layernormalization31_out1 = {
            let dtype = add105_out1.clone().dtype();
            self.layernormalization31
                .forward(add105_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul93_out1 = self.matmul93.forward(layernormalization31_out1.clone());
        let shape84_out1: [i64; 3] = layernormalization31_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant754_out1: i64 = 0i64;
        let actual_idx = if constant754_out1 < 0 {
            (shape84_out1.len() as i64 + constant754_out1) as usize
        } else {
            constant754_out1 as usize
        };
        let gather67_out1 = shape84_out1[actual_idx] as i64;
        let unsqueeze107_out1 = [gather67_out1];
        let constant756_out1: [i64; 1] = [-1i64];
        let constant757_out1: [i64; 1] = [3i64];
        let constant758_out1: [i64; 1] = [12i64];
        let constant759_out1: [i64; 1] = [64i64];
        let concat66_out1: [i64; 5usize] = [
            &unsqueeze107_out1[..],
            &constant756_out1[..],
            &constant757_out1[..],
            &constant758_out1[..],
            &constant759_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape34_out1 = matmul93_out1.reshape(concat66_out1);
        let transpose49_out1 = reshape34_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose49_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split16_out1, split16_out2, split16_out3] = split_tensors.try_into().unwrap();
        let squeeze46_out1 = split16_out1.squeeze_dims(&[2]);
        let squeeze47_out1 = split16_out2.squeeze_dims(&[2]);
        let squeeze48_out1 = split16_out3.squeeze_dims(&[2]);
        let mul173_out1 = squeeze46_out1.clone().mul(unsqueeze13_out1.clone());
        let shape85_out1: [i64; 4] = squeeze46_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant764_out1: i64 = 3i64;
        let actual_idx = if constant764_out1 < 0 {
            (shape85_out1.len() as i64 + constant764_out1) as usize
        } else {
            constant764_out1 as usize
        };
        let gather68_out1 = shape85_out1[actual_idx] as i64;
        let constant765_out1: i64 = 2i64;
        let div76_out1 = gather68_out1 / constant765_out1;
        let cast109_out1 = div76_out1;
        let cast110_out1 = cast109_out1;
        let unsqueeze108_out1 = [cast110_out1];
        let slice106_out1 = squeeze46_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze108_out1[0]]);
        let unsqueeze109_out1 = [cast110_out1];
        let slice107_out1 =
            squeeze46_out1.slice(s![.., .., .., unsqueeze109_out1[0]..9223372036854775807]);
        let neg31_out1 = slice107_out1.neg();
        let concat67_out1 = burn::tensor::Tensor::cat([neg31_out1, slice106_out1].into(), 3);
        let mul174_out1 = concat67_out1.mul(unsqueeze14_out1.clone());
        let add106_out1 = mul173_out1.add(mul174_out1);
        let mul175_out1 = squeeze47_out1.clone().mul(unsqueeze13_out1.clone());
        let shape86_out1: [i64; 4] = squeeze47_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant774_out1: i64 = 3i64;
        let actual_idx = if constant774_out1 < 0 {
            (shape86_out1.len() as i64 + constant774_out1) as usize
        } else {
            constant774_out1 as usize
        };
        let gather69_out1 = shape86_out1[actual_idx] as i64;
        let constant775_out1: i64 = 2i64;
        let div77_out1 = gather69_out1 / constant775_out1;
        let cast111_out1 = div77_out1;
        let cast112_out1 = cast111_out1;
        let unsqueeze110_out1 = [cast112_out1];
        let slice108_out1 = squeeze47_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze110_out1[0]]);
        let unsqueeze111_out1 = [cast112_out1];
        let slice109_out1 =
            squeeze47_out1.slice(s![.., .., .., unsqueeze111_out1[0]..9223372036854775807]);
        let neg32_out1 = slice109_out1.neg();
        let concat68_out1 = burn::tensor::Tensor::cat([neg32_out1, slice108_out1].into(), 3);
        let mul176_out1 = concat68_out1.mul(unsqueeze14_out1.clone());
        let add107_out1 = mul175_out1.add(mul176_out1);
        let shape87_out1: [i64; 4] = add106_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice110_out1: [i64; 1] = shape87_out1[3..4].try_into().unwrap();
        let cast113_out1 = {
            let shape_array = slice110_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt46_out1 = cast113_out1.sqrt();
        let constant786_out1 = self.constant786.val();
        let div78_out1 = constant786_out1.div(sqrt46_out1);
        let cast114_out1 = div78_out1;
        let transpose50_out1 = add107_out1.permute([0, 1, 3, 2]);
        let sqrt47_out1 = cast114_out1.clone().sqrt();
        let mul177_out1 = add106_out1.mul(sqrt47_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt48_out1 = cast114_out1.sqrt();
        let mul178_out1 =
            transpose50_out1.mul(sqrt48_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul94_out1 = mul177_out1.matmul(mul178_out1);
        let add108_out1 = matmul94_out1.add(where2_out1.clone());
        let softmax16_out1 = burn::tensor::activation::softmax(add108_out1, 3);
        let matmul95_out1 = softmax16_out1.matmul(squeeze48_out1);
        let transpose51_out1 = matmul95_out1.permute([0, 2, 1, 3]);
        let unsqueeze112_out1 = [gather67_out1];
        let constant788_out1: [i64; 1] = [-1i64];
        let constant789_out1: [i64; 1] = [768i64];
        let concat69_out1: [i64; 3usize] = [
            &unsqueeze112_out1[..],
            &constant788_out1[..],
            &constant789_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape35_out1 = transpose51_out1.reshape(concat69_out1);
        let matmul96_out1 = self.matmul96.forward(reshape35_out1);
        let add109_out1 = add105_out1.add(matmul96_out1);
        let layernormalization32_out1 = {
            let dtype = add109_out1.clone().dtype();
            self.layernormalization32
                .forward(add109_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul97_out1 = self.matmul97.forward(layernormalization32_out1);
        let shape88_out1: [i64; 3] = matmul97_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant791_out1: [i64; 1] = [-1i64];
        let gather70_out1: [i64; 1usize] = constant791_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape88_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape88_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant793_out1: [i64; 1] = [1i64];
        let add110_out1 = {
            let mut result = gather70_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant793_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant794_out1: [i64; 1] = [2i64];
        let div79_out1 = {
            let mut result = add110_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant794_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant795_out1: [i64; 1] = [1i64];
        let mul179_out1 = {
            let mut result = div79_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant795_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice111_out1 = matmul97_out1.clone().slice(s![.., .., 0..mul179_out1[0]]);
        let constant796_out1: [i64; 1] = [2i64];
        let mul180_out1 = {
            let mut result = div79_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant796_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice112_out1 = matmul97_out1.slice(s![.., .., mul179_out1[0]..mul180_out1[0]]);
        let constant797_out1: f32 = 1.4142135f32;
        let div80_out1 = slice111_out1.clone().div_scalar(constant797_out1);
        let erf16_out1 = div80_out1.erf();
        let constant798_out1: f32 = 1f32;
        let add111_out1 = erf16_out1.add_scalar(constant798_out1);
        let mul181_out1 = slice111_out1.mul(add111_out1);
        let constant799_out1: f32 = 0.5f32;
        let mul182_out1 = mul181_out1.mul_scalar(constant799_out1);
        let mul183_out1 = mul182_out1.mul(slice112_out1);
        let matmul98_out1 = self.matmul98.forward(mul183_out1);
        let add112_out1 = add109_out1.add(matmul98_out1);
        let layernormalization33_out1 = {
            let dtype = add112_out1.clone().dtype();
            self.layernormalization33
                .forward(add112_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul99_out1 = self.matmul99.forward(layernormalization33_out1.clone());
        let shape89_out1: [i64; 3] = layernormalization33_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant801_out1: i64 = 0i64;
        let actual_idx = if constant801_out1 < 0 {
            (shape89_out1.len() as i64 + constant801_out1) as usize
        } else {
            constant801_out1 as usize
        };
        let gather71_out1 = shape89_out1[actual_idx] as i64;
        let unsqueeze113_out1 = [gather71_out1];
        let constant803_out1: [i64; 1] = [-1i64];
        let constant804_out1: [i64; 1] = [3i64];
        let constant805_out1: [i64; 1] = [12i64];
        let constant806_out1: [i64; 1] = [64i64];
        let concat70_out1: [i64; 5usize] = [
            &unsqueeze113_out1[..],
            &constant803_out1[..],
            &constant804_out1[..],
            &constant805_out1[..],
            &constant806_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape36_out1 = matmul99_out1.reshape(concat70_out1);
        let transpose52_out1 = reshape36_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose52_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split17_out1, split17_out2, split17_out3] = split_tensors.try_into().unwrap();
        let squeeze49_out1 = split17_out1.squeeze_dims(&[2]);
        let squeeze50_out1 = split17_out2.squeeze_dims(&[2]);
        let squeeze51_out1 = split17_out3.squeeze_dims(&[2]);
        let mul184_out1 = squeeze49_out1.clone().mul(unsqueeze22_out1.clone());
        let shape90_out1: [i64; 4] = squeeze49_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant811_out1: i64 = 3i64;
        let actual_idx = if constant811_out1 < 0 {
            (shape90_out1.len() as i64 + constant811_out1) as usize
        } else {
            constant811_out1 as usize
        };
        let gather72_out1 = shape90_out1[actual_idx] as i64;
        let constant812_out1: i64 = 2i64;
        let div81_out1 = gather72_out1 / constant812_out1;
        let cast115_out1 = div81_out1;
        let cast116_out1 = cast115_out1;
        let unsqueeze114_out1 = [cast116_out1];
        let slice113_out1 = squeeze49_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze114_out1[0]]);
        let unsqueeze115_out1 = [cast116_out1];
        let slice114_out1 =
            squeeze49_out1.slice(s![.., .., .., unsqueeze115_out1[0]..9223372036854775807]);
        let neg33_out1 = slice114_out1.neg();
        let concat71_out1 = burn::tensor::Tensor::cat([neg33_out1, slice113_out1].into(), 3);
        let mul185_out1 = concat71_out1.mul(unsqueeze23_out1.clone());
        let add113_out1 = mul184_out1.add(mul185_out1);
        let mul186_out1 = squeeze50_out1.clone().mul(unsqueeze22_out1.clone());
        let shape91_out1: [i64; 4] = squeeze50_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant821_out1: i64 = 3i64;
        let actual_idx = if constant821_out1 < 0 {
            (shape91_out1.len() as i64 + constant821_out1) as usize
        } else {
            constant821_out1 as usize
        };
        let gather73_out1 = shape91_out1[actual_idx] as i64;
        let constant822_out1: i64 = 2i64;
        let div82_out1 = gather73_out1 / constant822_out1;
        let cast117_out1 = div82_out1;
        let cast118_out1 = cast117_out1;
        let unsqueeze116_out1 = [cast118_out1];
        let slice115_out1 = squeeze50_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze116_out1[0]]);
        let unsqueeze117_out1 = [cast118_out1];
        let slice116_out1 =
            squeeze50_out1.slice(s![.., .., .., unsqueeze117_out1[0]..9223372036854775807]);
        let neg34_out1 = slice116_out1.neg();
        let concat72_out1 = burn::tensor::Tensor::cat([neg34_out1, slice115_out1].into(), 3);
        let mul187_out1 = concat72_out1.mul(unsqueeze23_out1.clone());
        let add114_out1 = mul186_out1.add(mul187_out1);
        let shape92_out1: [i64; 4] = add113_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice117_out1: [i64; 1] = shape92_out1[3..4].try_into().unwrap();
        let cast119_out1 = {
            let shape_array = slice117_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt49_out1 = cast119_out1.sqrt();
        let constant833_out1 = self.constant833.val();
        let div83_out1 = constant833_out1.div(sqrt49_out1);
        let cast120_out1 = div83_out1;
        let transpose53_out1 = add114_out1.permute([0, 1, 3, 2]);
        let sqrt50_out1 = cast120_out1.clone().sqrt();
        let mul188_out1 = add113_out1.mul(sqrt50_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt51_out1 = cast120_out1.sqrt();
        let mul189_out1 =
            transpose53_out1.mul(sqrt51_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul100_out1 = mul188_out1.matmul(mul189_out1);
        let add115_out1 = matmul100_out1.add(where3_out1.clone());
        let softmax17_out1 = burn::tensor::activation::softmax(add115_out1, 3);
        let matmul101_out1 = softmax17_out1.matmul(squeeze51_out1);
        let transpose54_out1 = matmul101_out1.permute([0, 2, 1, 3]);
        let unsqueeze118_out1 = [gather71_out1];
        let constant835_out1: [i64; 1] = [-1i64];
        let constant836_out1: [i64; 1] = [768i64];
        let concat73_out1: [i64; 3usize] = [
            &unsqueeze118_out1[..],
            &constant835_out1[..],
            &constant836_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape37_out1 = transpose54_out1.reshape(concat73_out1);
        let matmul102_out1 = self.matmul102.forward(reshape37_out1);
        let add116_out1 = add112_out1.add(matmul102_out1);
        let layernormalization34_out1 = {
            let dtype = add116_out1.clone().dtype();
            self.layernormalization34
                .forward(add116_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul103_out1 = self.matmul103.forward(layernormalization34_out1);
        let shape93_out1: [i64; 3] = matmul103_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant838_out1: [i64; 1] = [-1i64];
        let gather74_out1: [i64; 1usize] = constant838_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape93_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape93_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant840_out1: [i64; 1] = [1i64];
        let add117_out1 = {
            let mut result = gather74_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant840_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant841_out1: [i64; 1] = [2i64];
        let div84_out1 = {
            let mut result = add117_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant841_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant842_out1: [i64; 1] = [1i64];
        let mul190_out1 = {
            let mut result = div84_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant842_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice118_out1 = matmul103_out1.clone().slice(s![.., .., 0..mul190_out1[0]]);
        let constant843_out1: [i64; 1] = [2i64];
        let mul191_out1 = {
            let mut result = div84_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant843_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice119_out1 = matmul103_out1.slice(s![.., .., mul190_out1[0]..mul191_out1[0]]);
        let constant844_out1: f32 = 1.4142135f32;
        let div85_out1 = slice118_out1.clone().div_scalar(constant844_out1);
        let erf17_out1 = div85_out1.erf();
        let constant845_out1: f32 = 1f32;
        let add118_out1 = erf17_out1.add_scalar(constant845_out1);
        let mul192_out1 = slice118_out1.mul(add118_out1);
        let constant846_out1: f32 = 0.5f32;
        let mul193_out1 = mul192_out1.mul_scalar(constant846_out1);
        let mul194_out1 = mul193_out1.mul(slice119_out1);
        let matmul104_out1 = self.matmul104.forward(mul194_out1);
        let add119_out1 = add116_out1.add(matmul104_out1);
        let layernormalization35_out1 = {
            let dtype = add119_out1.clone().dtype();
            self.layernormalization35
                .forward(add119_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul105_out1 = self.matmul105.forward(layernormalization35_out1.clone());
        let shape94_out1: [i64; 3] = layernormalization35_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant848_out1: i64 = 0i64;
        let actual_idx = if constant848_out1 < 0 {
            (shape94_out1.len() as i64 + constant848_out1) as usize
        } else {
            constant848_out1 as usize
        };
        let gather75_out1 = shape94_out1[actual_idx] as i64;
        let unsqueeze119_out1 = [gather75_out1];
        let constant850_out1: [i64; 1] = [-1i64];
        let constant851_out1: [i64; 1] = [3i64];
        let constant852_out1: [i64; 1] = [12i64];
        let constant853_out1: [i64; 1] = [64i64];
        let concat74_out1: [i64; 5usize] = [
            &unsqueeze119_out1[..],
            &constant850_out1[..],
            &constant851_out1[..],
            &constant852_out1[..],
            &constant853_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape38_out1 = matmul105_out1.reshape(concat74_out1);
        let transpose55_out1 = reshape38_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose55_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split18_out1, split18_out2, split18_out3] = split_tensors.try_into().unwrap();
        let squeeze52_out1 = split18_out1.squeeze_dims(&[2]);
        let squeeze53_out1 = split18_out2.squeeze_dims(&[2]);
        let squeeze54_out1 = split18_out3.squeeze_dims(&[2]);
        let mul195_out1 = squeeze52_out1.clone().mul(unsqueeze22_out1.clone());
        let shape95_out1: [i64; 4] = squeeze52_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant858_out1: i64 = 3i64;
        let actual_idx = if constant858_out1 < 0 {
            (shape95_out1.len() as i64 + constant858_out1) as usize
        } else {
            constant858_out1 as usize
        };
        let gather76_out1 = shape95_out1[actual_idx] as i64;
        let constant859_out1: i64 = 2i64;
        let div86_out1 = gather76_out1 / constant859_out1;
        let cast121_out1 = div86_out1;
        let cast122_out1 = cast121_out1;
        let unsqueeze120_out1 = [cast122_out1];
        let slice120_out1 = squeeze52_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze120_out1[0]]);
        let unsqueeze121_out1 = [cast122_out1];
        let slice121_out1 =
            squeeze52_out1.slice(s![.., .., .., unsqueeze121_out1[0]..9223372036854775807]);
        let neg35_out1 = slice121_out1.neg();
        let concat75_out1 = burn::tensor::Tensor::cat([neg35_out1, slice120_out1].into(), 3);
        let mul196_out1 = concat75_out1.mul(unsqueeze23_out1.clone());
        let add120_out1 = mul195_out1.add(mul196_out1);
        let mul197_out1 = squeeze53_out1.clone().mul(unsqueeze22_out1.clone());
        let shape96_out1: [i64; 4] = squeeze53_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant868_out1: i64 = 3i64;
        let actual_idx = if constant868_out1 < 0 {
            (shape96_out1.len() as i64 + constant868_out1) as usize
        } else {
            constant868_out1 as usize
        };
        let gather77_out1 = shape96_out1[actual_idx] as i64;
        let constant869_out1: i64 = 2i64;
        let div87_out1 = gather77_out1 / constant869_out1;
        let cast123_out1 = div87_out1;
        let cast124_out1 = cast123_out1;
        let unsqueeze122_out1 = [cast124_out1];
        let slice122_out1 = squeeze53_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze122_out1[0]]);
        let unsqueeze123_out1 = [cast124_out1];
        let slice123_out1 =
            squeeze53_out1.slice(s![.., .., .., unsqueeze123_out1[0]..9223372036854775807]);
        let neg36_out1 = slice123_out1.neg();
        let concat76_out1 = burn::tensor::Tensor::cat([neg36_out1, slice122_out1].into(), 3);
        let mul198_out1 = concat76_out1.mul(unsqueeze23_out1.clone());
        let add121_out1 = mul197_out1.add(mul198_out1);
        let shape97_out1: [i64; 4] = add120_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice124_out1: [i64; 1] = shape97_out1[3..4].try_into().unwrap();
        let cast125_out1 = {
            let shape_array = slice124_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt52_out1 = cast125_out1.sqrt();
        let constant880_out1 = self.constant880.val();
        let div88_out1 = constant880_out1.div(sqrt52_out1);
        let cast126_out1 = div88_out1;
        let transpose56_out1 = add121_out1.permute([0, 1, 3, 2]);
        let sqrt53_out1 = cast126_out1.clone().sqrt();
        let mul199_out1 = add120_out1.mul(sqrt53_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt54_out1 = cast126_out1.sqrt();
        let mul200_out1 =
            transpose56_out1.mul(sqrt54_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul106_out1 = mul199_out1.matmul(mul200_out1);
        let add122_out1 = matmul106_out1.add(where3_out1.clone());
        let softmax18_out1 = burn::tensor::activation::softmax(add122_out1, 3);
        let matmul107_out1 = softmax18_out1.matmul(squeeze54_out1);
        let transpose57_out1 = matmul107_out1.permute([0, 2, 1, 3]);
        let unsqueeze124_out1 = [gather75_out1];
        let constant882_out1: [i64; 1] = [-1i64];
        let constant883_out1: [i64; 1] = [768i64];
        let concat77_out1: [i64; 3usize] = [
            &unsqueeze124_out1[..],
            &constant882_out1[..],
            &constant883_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape39_out1 = transpose57_out1.reshape(concat77_out1);
        let matmul108_out1 = self.matmul108.forward(reshape39_out1);
        let add123_out1 = add119_out1.add(matmul108_out1);
        let layernormalization36_out1 = {
            let dtype = add123_out1.clone().dtype();
            self.layernormalization36
                .forward(add123_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul109_out1 = self.matmul109.forward(layernormalization36_out1);
        let shape98_out1: [i64; 3] = matmul109_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant885_out1: [i64; 1] = [-1i64];
        let gather78_out1: [i64; 1usize] = constant885_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape98_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape98_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant887_out1: [i64; 1] = [1i64];
        let add124_out1 = {
            let mut result = gather78_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant887_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant888_out1: [i64; 1] = [2i64];
        let div89_out1 = {
            let mut result = add124_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant888_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant889_out1: [i64; 1] = [1i64];
        let mul201_out1 = {
            let mut result = div89_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant889_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice125_out1 = matmul109_out1.clone().slice(s![.., .., 0..mul201_out1[0]]);
        let constant890_out1: [i64; 1] = [2i64];
        let mul202_out1 = {
            let mut result = div89_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant890_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice126_out1 = matmul109_out1.slice(s![.., .., mul201_out1[0]..mul202_out1[0]]);
        let constant891_out1: f32 = 1.4142135f32;
        let div90_out1 = slice125_out1.clone().div_scalar(constant891_out1);
        let erf18_out1 = div90_out1.erf();
        let constant892_out1: f32 = 1f32;
        let add125_out1 = erf18_out1.add_scalar(constant892_out1);
        let mul203_out1 = slice125_out1.mul(add125_out1);
        let constant893_out1: f32 = 0.5f32;
        let mul204_out1 = mul203_out1.mul_scalar(constant893_out1);
        let mul205_out1 = mul204_out1.mul(slice126_out1);
        let matmul110_out1 = self.matmul110.forward(mul205_out1);
        let add126_out1 = add123_out1.add(matmul110_out1);
        let layernormalization37_out1 = {
            let dtype = add126_out1.clone().dtype();
            self.layernormalization37
                .forward(add126_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul111_out1 = self.matmul111.forward(layernormalization37_out1.clone());
        let shape99_out1: [i64; 3] = layernormalization37_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant895_out1: i64 = 0i64;
        let actual_idx = if constant895_out1 < 0 {
            (shape99_out1.len() as i64 + constant895_out1) as usize
        } else {
            constant895_out1 as usize
        };
        let gather79_out1 = shape99_out1[actual_idx] as i64;
        let unsqueeze125_out1 = [gather79_out1];
        let constant897_out1: [i64; 1] = [-1i64];
        let constant898_out1: [i64; 1] = [3i64];
        let constant899_out1: [i64; 1] = [12i64];
        let constant900_out1: [i64; 1] = [64i64];
        let concat78_out1: [i64; 5usize] = [
            &unsqueeze125_out1[..],
            &constant897_out1[..],
            &constant898_out1[..],
            &constant899_out1[..],
            &constant900_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape40_out1 = matmul111_out1.reshape(concat78_out1);
        let transpose58_out1 = reshape40_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose58_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split19_out1, split19_out2, split19_out3] = split_tensors.try_into().unwrap();
        let squeeze55_out1 = split19_out1.squeeze_dims(&[2]);
        let squeeze56_out1 = split19_out2.squeeze_dims(&[2]);
        let squeeze57_out1 = split19_out3.squeeze_dims(&[2]);
        let mul206_out1 = squeeze55_out1.clone().mul(unsqueeze13_out1.clone());
        let shape100_out1: [i64; 4] = squeeze55_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant905_out1: i64 = 3i64;
        let actual_idx = if constant905_out1 < 0 {
            (shape100_out1.len() as i64 + constant905_out1) as usize
        } else {
            constant905_out1 as usize
        };
        let gather80_out1 = shape100_out1[actual_idx] as i64;
        let constant906_out1: i64 = 2i64;
        let div91_out1 = gather80_out1 / constant906_out1;
        let cast127_out1 = div91_out1;
        let cast128_out1 = cast127_out1;
        let unsqueeze126_out1 = [cast128_out1];
        let slice127_out1 = squeeze55_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze126_out1[0]]);
        let unsqueeze127_out1 = [cast128_out1];
        let slice128_out1 =
            squeeze55_out1.slice(s![.., .., .., unsqueeze127_out1[0]..9223372036854775807]);
        let neg37_out1 = slice128_out1.neg();
        let concat79_out1 = burn::tensor::Tensor::cat([neg37_out1, slice127_out1].into(), 3);
        let mul207_out1 = concat79_out1.mul(unsqueeze14_out1.clone());
        let add127_out1 = mul206_out1.add(mul207_out1);
        let mul208_out1 = squeeze56_out1.clone().mul(unsqueeze13_out1.clone());
        let shape101_out1: [i64; 4] = squeeze56_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant915_out1: i64 = 3i64;
        let actual_idx = if constant915_out1 < 0 {
            (shape101_out1.len() as i64 + constant915_out1) as usize
        } else {
            constant915_out1 as usize
        };
        let gather81_out1 = shape101_out1[actual_idx] as i64;
        let constant916_out1: i64 = 2i64;
        let div92_out1 = gather81_out1 / constant916_out1;
        let cast129_out1 = div92_out1;
        let cast130_out1 = cast129_out1;
        let unsqueeze128_out1 = [cast130_out1];
        let slice129_out1 = squeeze56_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze128_out1[0]]);
        let unsqueeze129_out1 = [cast130_out1];
        let slice130_out1 =
            squeeze56_out1.slice(s![.., .., .., unsqueeze129_out1[0]..9223372036854775807]);
        let neg38_out1 = slice130_out1.neg();
        let concat80_out1 = burn::tensor::Tensor::cat([neg38_out1, slice129_out1].into(), 3);
        let mul209_out1 = concat80_out1.mul(unsqueeze14_out1.clone());
        let add128_out1 = mul208_out1.add(mul209_out1);
        let shape102_out1: [i64; 4] = add127_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice131_out1: [i64; 1] = shape102_out1[3..4].try_into().unwrap();
        let cast131_out1 = {
            let shape_array = slice131_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt55_out1 = cast131_out1.sqrt();
        let constant927_out1 = self.constant927.val();
        let div93_out1 = constant927_out1.div(sqrt55_out1);
        let cast132_out1 = div93_out1;
        let transpose59_out1 = add128_out1.permute([0, 1, 3, 2]);
        let sqrt56_out1 = cast132_out1.clone().sqrt();
        let mul210_out1 = add127_out1.mul(sqrt56_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt57_out1 = cast132_out1.sqrt();
        let mul211_out1 =
            transpose59_out1.mul(sqrt57_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul112_out1 = mul210_out1.matmul(mul211_out1);
        let add129_out1 = matmul112_out1.add(where2_out1.clone());
        let softmax19_out1 = burn::tensor::activation::softmax(add129_out1, 3);
        let matmul113_out1 = softmax19_out1.matmul(squeeze57_out1);
        let transpose60_out1 = matmul113_out1.permute([0, 2, 1, 3]);
        let unsqueeze130_out1 = [gather79_out1];
        let constant929_out1: [i64; 1] = [-1i64];
        let constant930_out1: [i64; 1] = [768i64];
        let concat81_out1: [i64; 3usize] = [
            &unsqueeze130_out1[..],
            &constant929_out1[..],
            &constant930_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape41_out1 = transpose60_out1.reshape(concat81_out1);
        let matmul114_out1 = self.matmul114.forward(reshape41_out1);
        let add130_out1 = add126_out1.add(matmul114_out1);
        let layernormalization38_out1 = {
            let dtype = add130_out1.clone().dtype();
            self.layernormalization38
                .forward(add130_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul115_out1 = self.matmul115.forward(layernormalization38_out1);
        let shape103_out1: [i64; 3] = matmul115_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant932_out1: [i64; 1] = [-1i64];
        let gather82_out1: [i64; 1usize] = constant932_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape103_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape103_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant934_out1: [i64; 1] = [1i64];
        let add131_out1 = {
            let mut result = gather82_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant934_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant935_out1: [i64; 1] = [2i64];
        let div94_out1 = {
            let mut result = add131_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant935_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant936_out1: [i64; 1] = [1i64];
        let mul212_out1 = {
            let mut result = div94_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant936_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice132_out1 = matmul115_out1.clone().slice(s![.., .., 0..mul212_out1[0]]);
        let constant937_out1: [i64; 1] = [2i64];
        let mul213_out1 = {
            let mut result = div94_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant937_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice133_out1 = matmul115_out1.slice(s![.., .., mul212_out1[0]..mul213_out1[0]]);
        let constant938_out1: f32 = 1.4142135f32;
        let div95_out1 = slice132_out1.clone().div_scalar(constant938_out1);
        let erf19_out1 = div95_out1.erf();
        let constant939_out1: f32 = 1f32;
        let add132_out1 = erf19_out1.add_scalar(constant939_out1);
        let mul214_out1 = slice132_out1.mul(add132_out1);
        let constant940_out1: f32 = 0.5f32;
        let mul215_out1 = mul214_out1.mul_scalar(constant940_out1);
        let mul216_out1 = mul215_out1.mul(slice133_out1);
        let matmul116_out1 = self.matmul116.forward(mul216_out1);
        let add133_out1 = add130_out1.add(matmul116_out1);
        let layernormalization39_out1 = {
            let dtype = add133_out1.clone().dtype();
            self.layernormalization39
                .forward(add133_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul117_out1 = self.matmul117.forward(layernormalization39_out1.clone());
        let shape104_out1: [i64; 3] = layernormalization39_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant942_out1: i64 = 0i64;
        let actual_idx = if constant942_out1 < 0 {
            (shape104_out1.len() as i64 + constant942_out1) as usize
        } else {
            constant942_out1 as usize
        };
        let gather83_out1 = shape104_out1[actual_idx] as i64;
        let unsqueeze131_out1 = [gather83_out1];
        let constant944_out1: [i64; 1] = [-1i64];
        let constant945_out1: [i64; 1] = [3i64];
        let constant946_out1: [i64; 1] = [12i64];
        let constant947_out1: [i64; 1] = [64i64];
        let concat82_out1: [i64; 5usize] = [
            &unsqueeze131_out1[..],
            &constant944_out1[..],
            &constant945_out1[..],
            &constant946_out1[..],
            &constant947_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape42_out1 = matmul117_out1.reshape(concat82_out1);
        let transpose61_out1 = reshape42_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose61_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split20_out1, split20_out2, split20_out3] = split_tensors.try_into().unwrap();
        let squeeze58_out1 = split20_out1.squeeze_dims(&[2]);
        let squeeze59_out1 = split20_out2.squeeze_dims(&[2]);
        let squeeze60_out1 = split20_out3.squeeze_dims(&[2]);
        let mul217_out1 = squeeze58_out1.clone().mul(unsqueeze22_out1.clone());
        let shape105_out1: [i64; 4] = squeeze58_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant952_out1: i64 = 3i64;
        let actual_idx = if constant952_out1 < 0 {
            (shape105_out1.len() as i64 + constant952_out1) as usize
        } else {
            constant952_out1 as usize
        };
        let gather84_out1 = shape105_out1[actual_idx] as i64;
        let constant953_out1: i64 = 2i64;
        let div96_out1 = gather84_out1 / constant953_out1;
        let cast133_out1 = div96_out1;
        let cast134_out1 = cast133_out1;
        let unsqueeze132_out1 = [cast134_out1];
        let slice134_out1 = squeeze58_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze132_out1[0]]);
        let unsqueeze133_out1 = [cast134_out1];
        let slice135_out1 =
            squeeze58_out1.slice(s![.., .., .., unsqueeze133_out1[0]..9223372036854775807]);
        let neg39_out1 = slice135_out1.neg();
        let concat83_out1 = burn::tensor::Tensor::cat([neg39_out1, slice134_out1].into(), 3);
        let mul218_out1 = concat83_out1.mul(unsqueeze23_out1.clone());
        let add134_out1 = mul217_out1.add(mul218_out1);
        let mul219_out1 = squeeze59_out1.clone().mul(unsqueeze22_out1.clone());
        let shape106_out1: [i64; 4] = squeeze59_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant962_out1: i64 = 3i64;
        let actual_idx = if constant962_out1 < 0 {
            (shape106_out1.len() as i64 + constant962_out1) as usize
        } else {
            constant962_out1 as usize
        };
        let gather85_out1 = shape106_out1[actual_idx] as i64;
        let constant963_out1: i64 = 2i64;
        let div97_out1 = gather85_out1 / constant963_out1;
        let cast135_out1 = div97_out1;
        let cast136_out1 = cast135_out1;
        let unsqueeze134_out1 = [cast136_out1];
        let slice136_out1 = squeeze59_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze134_out1[0]]);
        let unsqueeze135_out1 = [cast136_out1];
        let slice137_out1 =
            squeeze59_out1.slice(s![.., .., .., unsqueeze135_out1[0]..9223372036854775807]);
        let neg40_out1 = slice137_out1.neg();
        let concat84_out1 = burn::tensor::Tensor::cat([neg40_out1, slice136_out1].into(), 3);
        let mul220_out1 = concat84_out1.mul(unsqueeze23_out1.clone());
        let add135_out1 = mul219_out1.add(mul220_out1);
        let shape107_out1: [i64; 4] = add134_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice138_out1: [i64; 1] = shape107_out1[3..4].try_into().unwrap();
        let cast137_out1 = {
            let shape_array = slice138_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt58_out1 = cast137_out1.sqrt();
        let constant974_out1 = self.constant974.val();
        let div98_out1 = constant974_out1.div(sqrt58_out1);
        let cast138_out1 = div98_out1;
        let transpose62_out1 = add135_out1.permute([0, 1, 3, 2]);
        let sqrt59_out1 = cast138_out1.clone().sqrt();
        let mul221_out1 = add134_out1.mul(sqrt59_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt60_out1 = cast138_out1.sqrt();
        let mul222_out1 =
            transpose62_out1.mul(sqrt60_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul118_out1 = mul221_out1.matmul(mul222_out1);
        let add136_out1 = matmul118_out1.add(where3_out1.clone());
        let softmax20_out1 = burn::tensor::activation::softmax(add136_out1, 3);
        let matmul119_out1 = softmax20_out1.matmul(squeeze60_out1);
        let transpose63_out1 = matmul119_out1.permute([0, 2, 1, 3]);
        let unsqueeze136_out1 = [gather83_out1];
        let constant976_out1: [i64; 1] = [-1i64];
        let constant977_out1: [i64; 1] = [768i64];
        let concat85_out1: [i64; 3usize] = [
            &unsqueeze136_out1[..],
            &constant976_out1[..],
            &constant977_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape43_out1 = transpose63_out1.reshape(concat85_out1);
        let matmul120_out1 = self.matmul120.forward(reshape43_out1);
        let add137_out1 = add133_out1.add(matmul120_out1);
        let layernormalization40_out1 = {
            let dtype = add137_out1.clone().dtype();
            self.layernormalization40
                .forward(add137_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul121_out1 = self.matmul121.forward(layernormalization40_out1);
        let shape108_out1: [i64; 3] = matmul121_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant979_out1: [i64; 1] = [-1i64];
        let gather86_out1: [i64; 1usize] = constant979_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape108_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape108_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant981_out1: [i64; 1] = [1i64];
        let add138_out1 = {
            let mut result = gather86_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant981_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant982_out1: [i64; 1] = [2i64];
        let div99_out1 = {
            let mut result = add138_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant982_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant983_out1: [i64; 1] = [1i64];
        let mul223_out1 = {
            let mut result = div99_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant983_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice139_out1 = matmul121_out1.clone().slice(s![.., .., 0..mul223_out1[0]]);
        let constant984_out1: [i64; 1] = [2i64];
        let mul224_out1 = {
            let mut result = div99_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant984_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice140_out1 = matmul121_out1.slice(s![.., .., mul223_out1[0]..mul224_out1[0]]);
        let constant985_out1: f32 = 1.4142135f32;
        let div100_out1 = slice139_out1.clone().div_scalar(constant985_out1);
        let erf20_out1 = div100_out1.erf();
        let constant986_out1: f32 = 1f32;
        let add139_out1 = erf20_out1.add_scalar(constant986_out1);
        let mul225_out1 = slice139_out1.mul(add139_out1);
        let constant987_out1: f32 = 0.5f32;
        let mul226_out1 = mul225_out1.mul_scalar(constant987_out1);
        let mul227_out1 = mul226_out1.mul(slice140_out1);
        let matmul122_out1 = self.matmul122.forward(mul227_out1);
        let add140_out1 = add137_out1.add(matmul122_out1);
        let layernormalization41_out1 = {
            let dtype = add140_out1.clone().dtype();
            self.layernormalization41
                .forward(add140_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul123_out1 = self.matmul123.forward(layernormalization41_out1.clone());
        let shape109_out1: [i64; 3] = layernormalization41_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant989_out1: i64 = 0i64;
        let actual_idx = if constant989_out1 < 0 {
            (shape109_out1.len() as i64 + constant989_out1) as usize
        } else {
            constant989_out1 as usize
        };
        let gather87_out1 = shape109_out1[actual_idx] as i64;
        let unsqueeze137_out1 = [gather87_out1];
        let constant991_out1: [i64; 1] = [-1i64];
        let constant992_out1: [i64; 1] = [3i64];
        let constant993_out1: [i64; 1] = [12i64];
        let constant994_out1: [i64; 1] = [64i64];
        let concat86_out1: [i64; 5usize] = [
            &unsqueeze137_out1[..],
            &constant991_out1[..],
            &constant992_out1[..],
            &constant993_out1[..],
            &constant994_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape44_out1 = matmul123_out1.reshape(concat86_out1);
        let transpose64_out1 = reshape44_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose64_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split21_out1, split21_out2, split21_out3] = split_tensors.try_into().unwrap();
        let squeeze61_out1 = split21_out1.squeeze_dims(&[2]);
        let squeeze62_out1 = split21_out2.squeeze_dims(&[2]);
        let squeeze63_out1 = split21_out3.squeeze_dims(&[2]);
        let mul228_out1 = squeeze61_out1.clone().mul(unsqueeze22_out1.clone());
        let shape110_out1: [i64; 4] = squeeze61_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant999_out1: i64 = 3i64;
        let actual_idx = if constant999_out1 < 0 {
            (shape110_out1.len() as i64 + constant999_out1) as usize
        } else {
            constant999_out1 as usize
        };
        let gather88_out1 = shape110_out1[actual_idx] as i64;
        let constant1000_out1: i64 = 2i64;
        let div101_out1 = gather88_out1 / constant1000_out1;
        let cast139_out1 = div101_out1;
        let cast140_out1 = cast139_out1;
        let unsqueeze138_out1 = [cast140_out1];
        let slice141_out1 = squeeze61_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze138_out1[0]]);
        let unsqueeze139_out1 = [cast140_out1];
        let slice142_out1 =
            squeeze61_out1.slice(s![.., .., .., unsqueeze139_out1[0]..9223372036854775807]);
        let neg41_out1 = slice142_out1.neg();
        let concat87_out1 = burn::tensor::Tensor::cat([neg41_out1, slice141_out1].into(), 3);
        let mul229_out1 = concat87_out1.mul(unsqueeze23_out1.clone());
        let add141_out1 = mul228_out1.add(mul229_out1);
        let mul230_out1 = squeeze62_out1.clone().mul(unsqueeze22_out1);
        let shape111_out1: [i64; 4] = squeeze62_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant1009_out1: i64 = 3i64;
        let actual_idx = if constant1009_out1 < 0 {
            (shape111_out1.len() as i64 + constant1009_out1) as usize
        } else {
            constant1009_out1 as usize
        };
        let gather89_out1 = shape111_out1[actual_idx] as i64;
        let constant1010_out1: i64 = 2i64;
        let div102_out1 = gather89_out1 / constant1010_out1;
        let cast141_out1 = div102_out1;
        let cast142_out1 = cast141_out1;
        let unsqueeze140_out1 = [cast142_out1];
        let slice143_out1 = squeeze62_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze140_out1[0]]);
        let unsqueeze141_out1 = [cast142_out1];
        let slice144_out1 =
            squeeze62_out1.slice(s![.., .., .., unsqueeze141_out1[0]..9223372036854775807]);
        let neg42_out1 = slice144_out1.neg();
        let concat88_out1 = burn::tensor::Tensor::cat([neg42_out1, slice143_out1].into(), 3);
        let mul231_out1 = concat88_out1.mul(unsqueeze23_out1);
        let add142_out1 = mul230_out1.add(mul231_out1);
        let shape112_out1: [i64; 4] = add141_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice145_out1: [i64; 1] = shape112_out1[3..4].try_into().unwrap();
        let cast143_out1 = {
            let shape_array = slice145_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt61_out1 = cast143_out1.sqrt();
        let constant1021_out1 = self.constant1021.val();
        let div103_out1 = constant1021_out1.div(sqrt61_out1);
        let cast144_out1 = div103_out1;
        let transpose65_out1 = add142_out1.permute([0, 1, 3, 2]);
        let sqrt62_out1 = cast144_out1.clone().sqrt();
        let mul232_out1 = add141_out1.mul(sqrt62_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt63_out1 = cast144_out1.sqrt();
        let mul233_out1 =
            transpose65_out1.mul(sqrt63_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul124_out1 = mul232_out1.matmul(mul233_out1);
        let add143_out1 = matmul124_out1.add(where3_out1);
        let softmax21_out1 = burn::tensor::activation::softmax(add143_out1, 3);
        let matmul125_out1 = softmax21_out1.matmul(squeeze63_out1);
        let transpose66_out1 = matmul125_out1.permute([0, 2, 1, 3]);
        let unsqueeze142_out1 = [gather87_out1];
        let constant1023_out1: [i64; 1] = [-1i64];
        let constant1024_out1: [i64; 1] = [768i64];
        let concat89_out1: [i64; 3usize] = [
            &unsqueeze142_out1[..],
            &constant1023_out1[..],
            &constant1024_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape45_out1 = transpose66_out1.reshape(concat89_out1);
        let matmul126_out1 = self.matmul126.forward(reshape45_out1);
        let add144_out1 = add140_out1.add(matmul126_out1);
        let layernormalization42_out1 = {
            let dtype = add144_out1.clone().dtype();
            self.layernormalization42
                .forward(add144_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul127_out1 = self.matmul127.forward(layernormalization42_out1);
        let shape113_out1: [i64; 3] = matmul127_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant1026_out1: [i64; 1] = [-1i64];
        let gather90_out1: [i64; 1usize] = constant1026_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape113_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape113_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant1028_out1: [i64; 1] = [1i64];
        let add145_out1 = {
            let mut result = gather90_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant1028_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant1029_out1: [i64; 1] = [2i64];
        let div104_out1 = {
            let mut result = add145_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant1029_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant1030_out1: [i64; 1] = [1i64];
        let mul234_out1 = {
            let mut result = div104_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant1030_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice146_out1 = matmul127_out1.clone().slice(s![.., .., 0..mul234_out1[0]]);
        let constant1031_out1: [i64; 1] = [2i64];
        let mul235_out1 = {
            let mut result = div104_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant1031_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice147_out1 = matmul127_out1.slice(s![.., .., mul234_out1[0]..mul235_out1[0]]);
        let constant1032_out1: f32 = 1.4142135f32;
        let div105_out1 = slice146_out1.clone().div_scalar(constant1032_out1);
        let erf21_out1 = div105_out1.erf();
        let constant1033_out1: f32 = 1f32;
        let add146_out1 = erf21_out1.add_scalar(constant1033_out1);
        let mul236_out1 = slice146_out1.mul(add146_out1);
        let constant1034_out1: f32 = 0.5f32;
        let mul237_out1 = mul236_out1.mul_scalar(constant1034_out1);
        let mul238_out1 = mul237_out1.mul(slice147_out1);
        let matmul128_out1 = self.matmul128.forward(mul238_out1);
        let add147_out1 = add144_out1.add(matmul128_out1);
        let layernormalization43_out1 = {
            let dtype = add147_out1.clone().dtype();
            self.layernormalization43
                .forward(add147_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul129_out1 = self.matmul129.forward(layernormalization43_out1.clone());
        let shape114_out1: [i64; 3] = layernormalization43_out1.dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant1036_out1: i64 = 0i64;
        let actual_idx = if constant1036_out1 < 0 {
            (shape114_out1.len() as i64 + constant1036_out1) as usize
        } else {
            constant1036_out1 as usize
        };
        let gather91_out1 = shape114_out1[actual_idx] as i64;
        let unsqueeze143_out1 = [gather91_out1];
        let constant1038_out1: [i64; 1] = [-1i64];
        let constant1039_out1: [i64; 1] = [3i64];
        let constant1040_out1: [i64; 1] = [12i64];
        let constant1041_out1: [i64; 1] = [64i64];
        let concat90_out1: [i64; 5usize] = [
            &unsqueeze143_out1[..],
            &constant1038_out1[..],
            &constant1039_out1[..],
            &constant1040_out1[..],
            &constant1041_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape46_out1 = matmul129_out1.reshape(concat90_out1);
        let transpose67_out1 = reshape46_out1.permute([0, 3, 2, 1, 4]);
        let split_tensors = transpose67_out1.split_with_sizes([1, 1, 1].to_vec(), 2);
        let [split22_out1, split22_out2, split22_out3] = split_tensors.try_into().unwrap();
        let squeeze64_out1 = split22_out1.squeeze_dims(&[2]);
        let squeeze65_out1 = split22_out2.squeeze_dims(&[2]);
        let squeeze66_out1 = split22_out3.squeeze_dims(&[2]);
        let mul239_out1 = squeeze64_out1.clone().mul(unsqueeze13_out1.clone());
        let shape115_out1: [i64; 4] = squeeze64_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant1046_out1: i64 = 3i64;
        let actual_idx = if constant1046_out1 < 0 {
            (shape115_out1.len() as i64 + constant1046_out1) as usize
        } else {
            constant1046_out1 as usize
        };
        let gather92_out1 = shape115_out1[actual_idx] as i64;
        let constant1047_out1: i64 = 2i64;
        let div106_out1 = gather92_out1 / constant1047_out1;
        let cast145_out1 = div106_out1;
        let cast146_out1 = cast145_out1;
        let unsqueeze144_out1 = [cast146_out1];
        let slice148_out1 = squeeze64_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze144_out1[0]]);
        let unsqueeze145_out1 = [cast146_out1];
        let slice149_out1 =
            squeeze64_out1.slice(s![.., .., .., unsqueeze145_out1[0]..9223372036854775807]);
        let neg43_out1 = slice149_out1.neg();
        let concat91_out1 = burn::tensor::Tensor::cat([neg43_out1, slice148_out1].into(), 3);
        let mul240_out1 = concat91_out1.mul(unsqueeze14_out1.clone());
        let add148_out1 = mul239_out1.add(mul240_out1);
        let mul241_out1 = squeeze65_out1.clone().mul(unsqueeze13_out1);
        let shape116_out1: [i64; 4] = squeeze65_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant1056_out1: i64 = 3i64;
        let actual_idx = if constant1056_out1 < 0 {
            (shape116_out1.len() as i64 + constant1056_out1) as usize
        } else {
            constant1056_out1 as usize
        };
        let gather93_out1 = shape116_out1[actual_idx] as i64;
        let constant1057_out1: i64 = 2i64;
        let div107_out1 = gather93_out1 / constant1057_out1;
        let cast147_out1 = div107_out1;
        let cast148_out1 = cast147_out1;
        let unsqueeze146_out1 = [cast148_out1];
        let slice150_out1 = squeeze65_out1
            .clone()
            .slice(s![.., .., .., 0..unsqueeze146_out1[0]]);
        let unsqueeze147_out1 = [cast148_out1];
        let slice151_out1 =
            squeeze65_out1.slice(s![.., .., .., unsqueeze147_out1[0]..9223372036854775807]);
        let neg44_out1 = slice151_out1.neg();
        let concat92_out1 = burn::tensor::Tensor::cat([neg44_out1, slice150_out1].into(), 3);
        let mul242_out1 = concat92_out1.mul(unsqueeze14_out1);
        let add149_out1 = mul241_out1.add(mul242_out1);
        let shape117_out1: [i64; 4] = add148_out1.clone().dims()[0..4]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let slice152_out1: [i64; 1] = shape117_out1[3..4].try_into().unwrap();
        let cast149_out1 = {
            let shape_array = slice152_out1 as [i64; 1usize];
            let float_array: [f32; 1usize] = shape_array.map(|x| x as f32);
            Tensor::<B, 1>::from_data(TensorData::from(float_array), &self.device)
        };
        let sqrt64_out1 = cast149_out1.sqrt();
        let constant1068_out1 = self.constant1068.val();
        let div108_out1 = constant1068_out1.div(sqrt64_out1);
        let cast150_out1 = div108_out1;
        let transpose68_out1 = add149_out1.permute([0, 1, 3, 2]);
        let sqrt65_out1 = cast150_out1.clone().sqrt();
        let mul243_out1 = add148_out1.mul(sqrt65_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let sqrt66_out1 = cast150_out1.sqrt();
        let mul244_out1 =
            transpose68_out1.mul(sqrt66_out1.unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul130_out1 = mul243_out1.matmul(mul244_out1);
        let add150_out1 = matmul130_out1.add(where2_out1);
        let softmax22_out1 = burn::tensor::activation::softmax(add150_out1, 3);
        let matmul131_out1 = softmax22_out1.matmul(squeeze66_out1);
        let transpose69_out1 = matmul131_out1.permute([0, 2, 1, 3]);
        let unsqueeze148_out1 = [gather91_out1];
        let constant1070_out1: [i64; 1] = [-1i64];
        let constant1071_out1: [i64; 1] = [768i64];
        let concat93_out1: [i64; 3usize] = [
            &unsqueeze148_out1[..],
            &constant1070_out1[..],
            &constant1071_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape47_out1 = transpose69_out1.reshape(concat93_out1);
        let matmul132_out1 = self.matmul132.forward(reshape47_out1);
        let add151_out1 = add147_out1.add(matmul132_out1);
        let layernormalization44_out1 = {
            let dtype = add151_out1.clone().dtype();
            self.layernormalization44
                .forward(add151_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let matmul133_out1 = self.matmul133.forward(layernormalization44_out1);
        let shape118_out1: [i64; 3] = matmul133_out1.clone().dims()[0..3]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let constant1073_out1: [i64; 1] = [-1i64];
        let gather94_out1: [i64; 1usize] = constant1073_out1
            .iter()
            .map(|&idx| {
                let actual_idx = if idx < 0 {
                    (shape118_out1.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                shape118_out1[actual_idx]
            })
            .collect::<alloc::vec::Vec<_>>()
            .try_into()
            .unwrap();
        let constant1075_out1: [i64; 1] = [1i64];
        let add152_out1 = {
            let mut result = gather94_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant1075_out1.iter()) {
                *result_item = result_item.saturating_add(*rhs_item);
            }
            result
        };
        let constant1076_out1: [i64; 1] = [2i64];
        let div109_out1 = {
            let mut result = add152_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant1076_out1.iter()) {
                *result_item = if *rhs_item != 0 {
                    *result_item / *rhs_item
                } else {
                    *result_item
                };
            }
            result
        };
        let constant1077_out1: [i64; 1] = [1i64];
        let mul245_out1 = {
            let mut result = div109_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant1077_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice153_out1 = matmul133_out1.clone().slice(s![.., .., 0..mul245_out1[0]]);
        let constant1078_out1: [i64; 1] = [2i64];
        let mul246_out1 = {
            let mut result = div109_out1;
            for (result_item, rhs_item) in result.iter_mut().zip(constant1078_out1.iter()) {
                *result_item = result_item.saturating_mul(*rhs_item);
            }
            result
        };
        let slice154_out1 = matmul133_out1.slice(s![.., .., mul245_out1[0]..mul246_out1[0]]);
        let constant1079_out1: f32 = 1.4142135f32;
        let div110_out1 = slice153_out1.clone().div_scalar(constant1079_out1);
        let erf22_out1 = div110_out1.erf();
        let constant1080_out1: f32 = 1f32;
        let add153_out1 = erf22_out1.add_scalar(constant1080_out1);
        let mul247_out1 = slice153_out1.mul(add153_out1);
        let constant1081_out1: f32 = 0.5f32;
        let mul248_out1 = mul247_out1.mul_scalar(constant1081_out1);
        let mul249_out1 = mul248_out1.mul(slice154_out1);
        let matmul134_out1 = self.matmul134.forward(mul249_out1);
        let add154_out1 = add151_out1.add(matmul134_out1);
        let layernormalization45_out1 = {
            let dtype = add154_out1.dtype();
            self.layernormalization45
                .forward(add154_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let unsqueeze149_out1: Tensor<B, 3, Int> = input2.clone().unsqueeze_dims(&[-1]);
        let cast151_out1 = unsqueeze149_out1.float();
        let mul250_out1 = layernormalization45_out1.mul(cast151_out1);
        let constant1084_out1 = self.constant1084.val();
        let reducesum1_out1 = { mul250_out1.sum_dim(1usize).squeeze_dims(&[1]) };
        let reducesum2_out1 = { input2.sum_dim(1usize) };
        let cast152_out1 = reducesum2_out1.float();
        let div111_out1 = reducesum1_out1.div(cast152_out1);
        let matmul135_out1 = self.matmul135.forward(div111_out1);
        let constant1085_out1: f32 = 1.4142135f32;
        let div112_out1 = matmul135_out1.clone().div_scalar(constant1085_out1);
        let erf23_out1 = div112_out1.erf();
        let constant1086_out1: f32 = 1f32;
        let add155_out1 = erf23_out1.add_scalar(constant1086_out1);
        let mul251_out1 = matmul135_out1.mul(add155_out1);
        let constant1087_out1: f32 = 0.5f32;
        let mul252_out1 = mul251_out1.mul_scalar(constant1087_out1);
        let layernormalization46_out1 = {
            let dtype = mul252_out1.dtype();
            self.layernormalization46
                .forward(mul252_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let gemm1_out1 = self.gemm1.forward(layernormalization46_out1);
        gemm1_out1
    }
}
