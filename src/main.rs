
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::io::{BufReader, Write, Read};
use std::fs;

#[derive(Debug)]
struct WebServer{
    listener: Option<TcpListener>,
    single_connection: Option<TcpStream>,
    ip_address: Option<String>,
    port_address: Option<String>,
    max_connections: Option<u128>,
    web_server_pages: WebServerPages,
}

impl WebServer {
    fn new(
        listener: Option<TcpListener>, 
        single_connection: Option<TcpStream>, 
        ip_address: String, 
        port_address: String, 
        max_connections: u128) -> WebServer { 
            WebServer {
                listener: None, 
                single_connection:None,
                ip_address: Some(ip_address), 
                port_address: Some(port_address), 
                max_connections: Some(max_connections),
                web_server_pages: WebServerPages::new(),
            } 
        }
        fn start_listening(&mut self,ip_address: String,port_address: String) {
            let addr = format!(
                "{}:{}", ip_address,port_address
            );
            println!("{}",addr);
            self.listener = match TcpListener::bind(addr.to_string()){
                Ok(listener) => Some(listener),
                Err(err) => panic!("Error: start_listening = {}", err),
            }
        }
}
#[derive(Debug)]
struct WebServerPages{
    index_page: Option<String>,
    error_page: Option<String>,
}
impl  WebServerPages {
    fn new() -> WebServerPages{
        WebServerPages{
            index_page: {
                match fs::read_to_string("hello.html"){
                    Ok(page) => Some(page),
                    Err(err) => Some(err.to_string()),
            }
        },
            error_page:{
                match fs::read_to_string("error.html"){
                    Ok(page) => Some(page),
                    Err(err) => Some(err.to_string()),
                    }
                }
        }
    }
}
// impl WebServer{
//     fn new(ip_address,port_add) -> WebServer{
//         WebServer { 
//             listener: TcpListener::bind(ip_address),
//             single_connection: (), 
//             ip_address: (), 
//             port_address: (), 
//             max_connections: () 
//         }
//     }
// }


fn connection_handler(mut single_conn: &TcpStream) {

    let response = "HTTP/1.1 200 OK \r\n\r\n";
    let content_html = fs::read_to_string("hello.html").unwrap();
    let conent_err = fs::read_to_string("error.html").unwrap();
    let response_html = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content_html.len(),
        content_html,
);

    // work with requests
    let mut buffer:[u8;1024] = [0;1024];

    single_conn.read(&mut buffer).unwrap();
    println!("connection established\n");

    // println!("{}",String::from_utf8_lossy(&buffer));
    // // Ok(())
    // println!("{}",String::from_utf8_lossy(&buffer[0..15]));
    let check_get_request = "GET /hello HTTP/1.1".as_bytes();
    println!("{}",String::from_utf8_lossy(&buffer[0..15]));


    // if &buffer[0..15] == "GET / HTTP/1.1".as_bytes(){
if buffer.starts_with(check_get_request){
        single_conn.write(response_html.as_bytes());
        single_conn.flush();
    }else{
        single_conn.write(conent_err.as_bytes());
        single_conn.flush();
    }

}
    //write response



fn run() {
    let web_srv = match TcpListener::bind("0.0.0.0:8081"){
        Ok(listen_sock_ok) => {
            listen_sock_ok
        },
        Err(_) => return (),

    };

    loop {
        
        for conn in web_srv.incoming(){
            let mut single_conn = conn.unwrap();
            //
            connection_handler(&single_conn);
        }

    }
}

fn main() {

    // run();

    let mut srv = WebServer::new(None, None, "127.0.0.1".to_string(), "8080".to_string(), 1337);
    srv.start_listening("0.0.0.0".to_string(), "8081".to_string());

    println!("{:?}",srv.web_server_pages.index_page);
    println!("{:?}",srv.web_server_pages.error_page);

    println!("{:?}",srv);


    let mut buffer:[u8;1024] = [0;1024];
    let check_get_request = "GET /hello HTTP/1.1".as_bytes();

    loop{

        // let a = srv.listener.as_ref().unwrap().accept();
        // for conn in srv.listener{

        // }

        // for connection in srv.listener.as_ref().unwrap().incoming(){
        //     connection.unwrap().read(&mut buffer);
        //     if buffer.starts_with(check_get_request){
        //         connection.unwrap().write(srv.web_server_pages.index_page.as_ref().unwrap().as_bytes());
        //         connection.unwrap().flush();
        //     }else{
        //         connection.unwrap().write(srv.web_server_pages.error_page.as_ref().unwrap().as_bytes());
        //         connection.unwrap().flush();
        //     }
        // }

    }
}



