extern crate serde;

use serde::Serialize;
use serde::Deserialize;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequestData {
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_number: Option<String>,
    
    pub end_user_ip: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<Requirement>

}

impl AuthRequestData {
    pub fn new(end_user_ip:&str) -> AuthRequestData {
        AuthRequestData{personal_number: None, end_user_ip: String::from(end_user_ip), requirement: Some(Requirement{card_reader: Some(CardReader::Class2), certificate_policies: vec![CertificatePolicy::BankidOnFile, CertificatePolicy::BankidMobile], auto_start_token_required: Some(true), allow_fingerprint: None})}
    }

    pub fn new_with_personal_number(personal_number: String, end_user_ip: String) -> AuthRequestData {
        AuthRequestData{personal_number: Some(personal_number), end_user_ip: end_user_ip, requirement: None}
    }

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignRequestData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_number: Option<String>,
    pub end_user_ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<Requirement>,
    pub user_visible_data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_non_visible_data: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthSignResponse {
    pub auto_start_token: Option<String>,
    pub order_ref: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CardReader {
    #[serde(rename = "class1")] Class1, // default value in BankID service
    #[serde(rename = "class2")] Class2
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectResponse {
    pub order_ref: String,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint_code: Option<HintCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_data: Option<CompletionData>,
}

impl std::fmt::Display for CollectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CollectResponse(order_ref: {} {})", self.order_ref, self.status);
        match &self.hint_code {
            Some(hint_code) => match hint_code {
                HintCode::PendingOutstandingTransaction => write!(f, " hint_code: PendingOutstandingTransaction"),
                HintCode::PendingNoClient=> write!(f, " hint_code: PendingNoClient"),
                HintCode::PendingStarted=> write!(f, " hint_code: PendingStarted"),
                HintCode::PendingUserSign=> write!(f, " hint_code: PendingUserSign"),
                HintCode::FailedExpiredTransaction=> write!(f, " hint_code: FailedExpiredTransaction"),
                HintCode::FailedCertificateErr=> write!(f, " hint_code: FailedCertificateErr"),
                HintCode::FailedUserCancel=> write!(f, " hint_code: FailedUserCancel"),
                HintCode::FailedCancelled=> write!(f, " hint_code: FailedCancelled"),
                HintCode::FailedStartFailed=> write!(f, " hint_code: FailedStartFailed"),
                HintCode::Unknown=> write!(f, " hint_code: Unknown"),
            },
            None => write!(f, " hint_code: None"),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectRequestData {
    pub order_ref: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum HintCode {
    #[serde(rename = "outstandingTransaction")] PendingOutstandingTransaction,
    #[serde(rename = "noClient")] PendingNoClient,
    #[serde(rename = "started")] PendingStarted,
    #[serde(rename = "userSign")] PendingUserSign,
    #[serde(rename = "expiredTransaction")] FailedExpiredTransaction,
    #[serde(rename = "certificateErr")] FailedCertificateErr,
    #[serde(rename = "userCancel")] FailedUserCancel,
    #[serde(rename = "cancelled")] FailedCancelled,
    #[serde(rename = "startFailed")] FailedStartFailed,
    #[serde(other)] Unknown
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Pending, 
    Failed,
    Complete
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Status::Pending => f.write_str("Pending"),
            Status::Failed => f.write_str("Failed"),
            Status::Complete => f.write_str("Complete")
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompletionData {
    pub user: UserData,
    pub device: Option<DeviceData>,
    pub cert: Option<CertData>,
    pub signature: Option<String>, // TODO unkown if this is optional...check
    pub ocsp_response: Option<String>, // TODO unkown if this is optional...check

}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub personal_number: String,
    pub name: String,
    pub given_name: String,
    pub surname: String
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceData {
    pub ip_address: String
} 

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CertData {
    pub not_before: String,
    pub not_after: String,
}  

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CertificatePolicy {
    // production values
    #[serde(rename = "1.2.752.78.1.1")] BankidOnFile,
    #[serde(rename = "1.2.752.78.1.2")] BankidOnSmartCard,
    #[serde(rename = "1.2.752.78.1.5")] BankidMobile,
    #[serde(rename = "1.2.752.71.1.3")] NordeaEidOnFileSmartCard,

    // test values 
    #[serde(rename = "1.2.3.4.5")] TestBankidOnFile,
    #[serde(rename = "1.2.3.4.10")] TestBankidOnSmartCard,
    #[serde(rename = "1.2.3.4.25")] TestBankidMobile,
    #[serde(rename = "1.2.752.71.1.3")] TestNordeaEidOnFileSmartCard,
    #[serde(rename = "1.2.752.60.1.6")] TestBankidForSomeBankidBanks
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirement {
    pub card_reader: Option<CardReader>,
    pub certificate_policies: Vec<CertificatePolicy>,
    // issuerCn is not implemented 
    //   from bankid spec: 
    //      if issuerCn is not defined allallow_fingerprint relevant BankID 
    //      and Nordea issuers are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_start_token_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_fingerprint: Option<bool>
}