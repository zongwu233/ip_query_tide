use tide::Request;
extern crate ip2region;

use ip2region::*;
use std::net::Ipv4Addr;
use ipnet::Ipv4Net;
use serde::{ Serialize};

#[allow(non_snake_case)]
#[derive(Serialize )]
pub struct RegionInfo{
    pub country: &'static str,
    pub province: &'static str,
    pub city: &'static str,
    pub ISP: &'static str,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let mut app = tide::new();
    app.at("/").get(|_| async {
        Ok("Hello, world!")
    });
    app.at("/ip").get(client_ip);
    app.at("/ip/2region").get(ip2region);
    
    app.listen("0.0.0.0:3000").await?;
    Ok(())
}

async fn client_ip(req: Request<()>) -> tide::Result{
    let rawip = req.remote().unwrap();
    let ip = rawip.split(":").collect::<Vec<&str>>()[0];
    log::info!("remote ip: {:?}",ip);
    Ok(ip.into())
}

async fn ip2region(req: Request<()>) -> tide::Result{
    let rawip = req.remote().unwrap();
    let ip = rawip.split(":").collect::<Vec<&str>>()[0];
    
    if check_is_private_net(ip)||check_is_localhost(ip) {
        return Ok(format!("{} is private network",&ip).into());
    }

    let res = memory_search(ip);
    match res {
        Ok(ip_info) =>  {
            let region_info = RegionInfo{
                country : ip_info.country,
                province : ip_info.province,
                city: ip_info.city,
                ISP : ip_info.ISP, 
            };
            let json = serde_json::to_string(&region_info)?;
            log::info!("ip {} region:{}",ip,json);
            Ok(json.into())
        },
        _ => {
            log::warn!("ip2region failed {}",ip);
            Ok(format!("ip2region failed {}",ip).into())
        }   
    }
}


fn check_is_private_net(ip: &str ) -> bool {
    let net1:Ipv4Net =  "10.0.0.0/8".parse().unwrap();
    let net2:Ipv4Net = "172.16.0.0/12".parse().unwrap();
    let net3:Ipv4Net = "192.168.0.0/16".parse().unwrap();

    let addr: Ipv4Addr = ip.parse().unwrap();
    net1.contains(&addr) || net2.contains(&addr) || net3.contains(&addr)
}

fn check_is_localhost(ip:&str)->bool{
    ip == "127.0.0.1" || ip == "::1"|| ip == "localhost"
}


#[async_std::test]
async fn test()->Result<()>{
    assert!(check_is_private_net("192.168.31.12"));
    assert!(check_is_localhost("127.0.0.1"));
    Ok(())
}

