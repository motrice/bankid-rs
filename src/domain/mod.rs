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
        AuthRequestData{personal_number: None, end_user_ip: String::from(end_user_ip), requirement: Some(Requirement{card_reader: Some(CardReader::class2), certificatePolicies: vec![CertificatePolicy::bankid_on_file, CertificatePolicy::bankid_mobile], auto_start_token_required: Some(true), allow_fingerprint: None})}
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
    class1, // default value in BankID service
    class2
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
        write!(f, "CollectResponse(order_ref: {} {})", self.order_ref, self.status)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectRequestData {
    pub order_ref: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum HintCode {
    #[serde(rename = "outstandingTransaction")] pending_outstanding_transaction,
    #[serde(rename = "noClient")] pending_no_client,
    #[serde(rename = "started")] pending_started,
    #[serde(rename = "userSign")] pending_user_sign,
    #[serde(rename = "expiredTransaction")] failed_expired_transaction,
    #[serde(rename = "certificateErr")] failed_certificate_err,
    #[serde(rename = "userCancel")] failed_user_cancel,
    #[serde(rename = "cancelled")] failed_cancelled,
    #[serde(rename = "startFailed")] failed_start_failed,
    #[serde(other)] unknown
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
    #[serde(rename = "1.2.752.78.1.1")] bankid_on_file,
    #[serde(rename = "1.2.752.78.1.2")] bankid_on_smart_card,
    #[serde(rename = "1.2.752.78.1.5")] bankid_mobile,
    #[serde(rename = "1.2.752.71.1.3")] nordea_eid_on_file_smart_card,

    // test values 
    #[serde(rename = "1.2.3.4.5")] test_bankid_on_file,
    #[serde(rename = "1.2.3.4.10")] test_bankid_on_smart_card,
    #[serde(rename = "1.2.3.4.25")] test_bankid_mobile,
    #[serde(rename = "1.2.752.71.1.3")] test_nordea_eid_on_file_smart_card,
    #[serde(rename = "1.2.752.60.1.6")] test_bankid_for_some_bankid_banks
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirement {
    pub card_reader: Option<CardReader>,
    pub certificatePolicies: Vec<CertificatePolicy>,
    // issuerCn is not implemented 
    //   from bankid spec: 
    //      if issuerCn is not defined allallow_fingerprint relevant BankID 
    //      and Nordea issuers are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_start_token_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_fingerprint: Option<bool>
}