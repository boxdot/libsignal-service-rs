//! At install time, clients need to register with the Signal server.
//!
//! ```java
//! private final String     URL         = "https://my.signal.server.com";
//! private final TrustStore TRUST_STORE = new MyTrustStoreImpl();
//! private final String     USERNAME    = "+14151231234";
//! private final String     PASSWORD    = generateRandomPassword();
//! private final String     USER_AGENT  = "[FILL_IN]";
//!
//! SignalServiceAccountManager accountManager = new SignalServiceAccountManager(URL, TRUST_STORE,
//!                                                                              USERNAME, PASSWORD, USER_AGENT);
//!
//! accountManager.requestSmsVerificationCode();
//! accountManager.verifyAccountWithCode(receivedSmsVerificationCode, generateRandomSignalingKey(),
//!                                      generateRandomInstallId(), false);
//! accountManager.setGcmId(Optional.of(GoogleCloudMessaging.getInstance(this).register(REGISTRATION_ID)));
//! accountManager.setPreKeys(identityKey.getPublicKey(), lastResortKey, signedPreKeyRecord, oneTimePreKeys);
//! ```

use anyhow::Error;
use libsignal_service::{
    configuration::*, provisioning::ProvisioningManager, USER_AGENT,
};
use libsignal_service_actix::prelude::AwcPushService;
use structopt::StructOpt;

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let args = Args::from_args();

    // Only used with MessageSender and MessageReceiver
    // let password = args.get_password()?;

    let mut push_service =
        AwcPushService::new(args.servers, None, USER_AGENT.into());
    let mut provision_manager: ProvisioningManager<AwcPushService> =
        ProvisioningManager::new(
            &mut push_service,
            args.username,
            args.password.unwrap(),
        );

    provision_manager
        // You probably want to generate a reCAPTCHA though!
        .request_sms_verification_code(None, None)
        .await?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, StructOpt)]
pub struct Args {
    #[structopt(
        short = "s",
        long = "servers",
        help = "The servers to connect to",
        default_value = "staging"
    )]
    pub servers: SignalServers,
    #[structopt(
        short = "u",
        long = "username",
        help = "Your username or other identifier",
        default_value = "+14151231234"
    )]
    pub username: phonenumber::PhoneNumber,
    #[structopt(
        short = "p",
        long = "password",
        help = "The password to use. Read from stdin if not provided"
    )]
    pub password: Option<String>,
}
