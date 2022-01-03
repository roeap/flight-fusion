use async_trait::async_trait;
use flight_fusion_ipc::SignalFrame;

#[async_trait]
pub trait SignalFrameService {
    async fn get_signal_frame<T: Into<String> + Send>(&self, signal_id: T) -> SignalFrame;
}

pub struct TestFrameService {}

#[async_trait]
impl SignalFrameService for TestFrameService {
    async fn get_signal_frame<T: Into<String> + Send>(&self, signal_id: T) -> SignalFrame {
        let provider = crate::test_utils::get_provider_1();
        SignalFrame {
            uid: "frame-id".to_string(),
            name: "frame".to_string(),
            description: "description".to_string(),
            providers: vec![provider],
        }
    }
}
