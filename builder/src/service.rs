use crate::input::Devices;
use crate::mascot::get_mascot;

use crate::service::grpc::mascot_server::Mascot;
use crate::service::grpc::{MascotRequest, MascotResponse};
use tonic::{Request, Response, Status};

use log::{debug, error, info, warn};

pub mod grpc {
    tonic::include_proto!("mascot");
}

pub struct MascotService {
    pub devices: Devices,
}

#[tonic::async_trait]
impl Mascot for MascotService {
    async fn get_mascot(&self, _: Request<MascotRequest>) -> Result<Response<MascotResponse>, Status> {
        debug!("Got mascot request");
        let mascot = get_mascot(&self.devices);
        info!("Sending response: {:?}", mascot);

        debug!("Sent mascot response");
        Ok(Response::new(MascotResponse {
            emotion: mascot.emotion,
            blink: mascot.blink,
            lips: mascot.lips,
            voice: mascot.voice as u32,
        }))
    }
}
