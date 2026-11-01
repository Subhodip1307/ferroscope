use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
// base Format
#[derive(Debug, Serialize)]
pub(super) struct BaseFormat {
    pub service_name: String,
    pub status: String, //runing or dead
    pub error_msg: String,
    pub category: String,
    pub ssl_exp: Option<OffsetDateTime>,
}

/*..............host checking ..................
format--------------
name: <name-of-the-job> //optional
host: <IPv4>
*/

#[derive(Debug, Deserialize)]
pub(super) struct Host {
    pub name: String,
    host: String,
    port: u16,
   pub recovery_file_path:Option<String>,
    pub recover_file_256_hash:Option<String>,
}
impl Host {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
/*..............web checking ..................
format--------------
name: <name-of-the-job> //optional
url: <IPv4>
status: //expected status code
*/
#[derive(Debug, Deserialize)]
pub(super) struct Web {
    pub name: String,
    url: String,
    status_code: u16,
    pub recovery_file_path:Option<String>,
    pub recover_file_256_hash:Option<String>,
}
impl Web {
    pub fn get_url(&self) -> &str {
        &self.url
    }
    pub fn match_status(&self, code: u16) -> bool {
        self.status_code == code
    }
}

pub (super) struct Recovery{
   pub file:String,
    pub hash:String
}