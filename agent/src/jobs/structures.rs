use serde::{Deserialize,Serialize};
// base Format
#[derive(Debug, Serialize)]
pub(super)  struct BaseFormat {
   pub service_name:String,
   pub status:String,//runing or dead
   pub error_msg:String
}



/*..............host checking ..................
format--------------
name: <name-of-the-job> //optional
host: <IPv4>
*/


#[derive(Debug, Deserialize)]
pub(super)  struct  Host {
    pub name:String,
    host:String,
    port:u16
}
impl Host {
    pub fn addr(&self)->String{
        format!("{}:{}",self.host,self.port)
    }
}
/*..............web checking ..................
format--------------
name: <name-of-the-job> //optional
url: <IPv4>
status: //expected status code
*/
#[derive(Debug, Deserialize)]
pub(super)  struct Web {
    pub name:String,
    url:String,
    status_code:u16
}
impl Web {
    pub fn get_url(&self)->&str{
        &self.url
    }
    pub fn match_status(&self,code:u16)->bool{
        self.status_code==code
    }
}