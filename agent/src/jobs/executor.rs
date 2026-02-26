// execute the logics
use super::config_reader::{load_config,file_name_list};
use super::structures::{Host,Web,BaseFormat};
use super::config_parse::{host_check,web_check};
use reqwest::{Client};
use tokio::time::{interval,Duration};
use std::sync::Arc;


pub async fn run(api_client:Arc<Client>){
    let mut ticker=interval(Duration::from_secs(30));
    loop {
        
    tokio::join!(
        host_runner(Arc::clone(&api_client)),
        web_runner(Arc::clone(&api_client))
    );
    println!("Execution done retarting again");
    ticker.tick().await;
    }
}


async fn host_runner(api_client:Arc<Client>){
    let all_files=match file_name_list("./CONF/Host").await{
        Ok(value)=>value,
        Err(e)=>{
            println!("no config founnd err: {}",e);
            Vec::new()
        }
    };
    for file in all_files{
        let a: config::Config=match load_config(file).await{
            Ok(value)=>value,
            Err(e)=>{println!("error is {}",e); continue;}
        };

        let value:Host=match a.try_deserialize(){
             Ok(value)=>value,
            Err(e)=>{println!("error is {}",e); continue;}
        };
        let host_status=host_check(&value).await;
        let res=api_client
        .post("http://localhost:8000/send_service")
        .json(&BaseFormat{service_name:value.name,status:if host_status {"up".to_string()} else {"down".to_string()},error_msg:"".to_string()})
        .send().await.unwrap()
        ;
        println!("the res is {:?}",res);
        // println!("the host {} is  {}",value.name,if host_status {"up"} else {"down"});
    }//endfor
}


async  fn web_runner(api_client:Arc<Client>){
     let all_files=match file_name_list("./CONF/Web").await{
        Ok(value)=>value,
        Err(e)=>{
            println!("no config founnd err: {}",e);
            Vec::new()
        }
    };
    let _client=Client::builder()
        .timeout(Duration::from_secs(5))
        .build().unwrap();
    for file in all_files{
        let a: config::Config=match load_config(file).await{
            Ok(value)=>value,
            Err(e)=>{println!("error is {}",e); continue;}
        };

        let value:Web=match a.try_deserialize(){
             Ok(value)=>value,
            Err(e)=>{println!("error is {}",e); continue;}
        };
        let (status,code)=web_check(&value,&_client).await;
        if status =="Success".to_string()&& value.match_status(code){
            let res=api_client
            .post("http://localhost:8000/send_service")
            .json(&BaseFormat{service_name:value.name,status:"up".to_string(),error_msg:"".to_string()})
            .send().await.unwrap()
            ;
            println!("the res is {:?}",res);
        }else{
            let res=api_client
            .post("http://localhost:8000/send_service")
            .json(&BaseFormat{service_name:value.name,status:"down".to_string(),error_msg:format!("The status code is {}",code)})
            .send().await.unwrap();
            println!("the res is {:?}",res);
        }
    }//endfor
}