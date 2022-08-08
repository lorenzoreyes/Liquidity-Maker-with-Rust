use std::io;
use std::fs::File;
use serde_json::from_reader; 
use serde_json::Value;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

pub fn binance_streams(_: String) -> String {
    let file = File::open("src/config.json").expect("Not JSON format");
    let json: serde_json::Value = from_reader(file).expect("file should be a proper json");
    let ticket  = json.get("ticket").expect("File should be a currency key");
    let ticket2 = json.get("ticket2").expect("File should be a currency key");
    let ticket3 = json.get("ticket3").expect("File should be a currency key");
    let ticket4 = json.get("ticket4").expect("File should be a currency key");
    let ticket5 = json.get("ticket5").expect("File should be a currency key");
    /*println!("ticket1: {}, ticket2: {}, ticket3: {}, ticket4: {}, ticket5: {}",
             &ticket.as_str().unwrap(),&ticket2.as_str().unwrap(),
             &ticket3.as_str().unwrap(),ticket4.as_str().unwrap(),
             ticket5.as_str().unwrap()); */
    
    let binance_url = format!("{}/stream?streams={}@depth10@100ms/{}@depth10@100ms/{}@depth10@100ms/{}@depth10@100ms/{}@depth10@100ms", 
                              BINANCE_WS_API,&ticket.as_str().unwrap(),&ticket2.as_str().unwrap(),
                              &ticket3.as_str().unwrap(),ticket4.as_str().unwrap(),ticket5.as_str().unwrap());
    
    return binance_url
}
/*    //println!("\n\n\nYou requested the following stream:\n{}\n\n\n", binance_url);
pub fn binance_streams<R>(request: R) -> Result<WebSocket<Stream<Plain(S),Tls(T)>, error::Error> {

    let (mut socket, response) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

    return socket
}*/
