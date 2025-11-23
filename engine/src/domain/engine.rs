use crate::domain::errors::*;
use crate::ports::driven::cloud_driven_port::CloudDrivenPort;

pub struct Engine<TCloudDrivenPort: CloudDrivenPort>  {
    cloud_driven_port: TCloudDrivenPort,
}

impl <TCloudDrivenPort: CloudDrivenPort> Engine<TCloudDrivenPort> {
    pub fn new(cloud_driven_port: TCloudDrivenPort) -> Self {
        Self { cloud_driven_port }
    }

    pub async fn list_files(&self) -> Result<Vec<String>, AppError> {


        let auth_url = self.cloud_driven_port.get_authentication_url_to_be_opened_by_user();
        let a = self.cloud_driven_port.get_authorization_code().await;
        let b = a.unwrap();
        

        //
        // let auth_flow =
        //     self.cloud_driven_port.start_auth_flow(&env_vars)?;
        //
        // let callback_endpoint = auth_flow
        //     .client
        //     .redirect_uri()
        //     .ok_or_else(|| AuthFlowError::MissingRedirectUrl)?
        //     .parse()?;
        //
        // let (sender, receiver) = oneshot::channel::<String>();
        // let router = self.cloud_driven_port.construct_callback_router(sender, &callback_endpoint.path);





        Ok(TCloudDrivenPort::list_files(&self.cloud_driven_port))
    }

}

#[cfg(test)]
mod tests {
    // #[test]
    // fn connection_test() {
    //     let port : KDrive {};
    // }
}
//     use crate::domain::auth::AuthFlow;
//     use crate::domain::errors::ConfigurationError;
//     use super::*;
//
//     struct FakeCloud;
//ssssssss
//     impl CloudDrivenPort for FakeCloud {
//         fn list_files(&self) -> Vec<String> {
//             vec![String::from("test")]
//         }
//
//         fn start_auth_flow(environment_file: Option<&str>) -> Result<AuthFlow, ConfigurationError> {
//             todo!()
//         }
//     }
//
//     #[test]
//     fn the_engine_can_list_the_files_from_the_cloud() {
//         let storage = FakeCloud;
//         let files = Engine::new(storage).list_files();
//         assert_eq!(files, vec!["test"]);
//     }
//
//     #[test]
//     fn the_engine_does_not_list_files_from_an_empty_storage() {
//         struct EmptyCloud;
//         impl CloudDrivenPort for EmptyCloud {
//             fn list_files(&self) -> Vec<String> {
//                 vec![]
//             }
//         }
//
//         let engine = Engine::new(EmptyCloud);
//         let files = engine.list_files();
//         assert!(files.is_empty());
//     }
// }