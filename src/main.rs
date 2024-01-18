use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use log::{debug, error, log_enabled, info, Level};
use env_logger;
#[tokio::main]

async fn main () -> Result<(), Box<dyn std::error::Error>>{
    env_logger::init(); 
    /* Open Listener  */
    let lt = 
        TcpListener::bind("127.0.0.1:8080").await?;
    
    /* Loop to accept connection and 
        spawn task to handle I/O  */
    loop { 
        match lt.accept().await {
            /* TO-DO : the connection should be
                     logged instead of printed  */
            Ok((socket, addr)) => {
                info!("Got connection from Client {addr}"); 

                tokio::spawn(async move { 
                    handle_client(socket).await; 
                });


            }, 
            Err(_e) => error!("Error getting connection"), 
        };
    }

}

async fn handle_client( mut client_socket: TcpStream) { 
    let mut buf = [0; 1024]; 

    loop{
        match client_socket.read(&mut buf).await{
            Ok(n) if n == 0 => {
                debug!("Read {n} bytes of data"); 
                return; 
            }, 
            Ok(n) => { 
                info!("Read { } bytes of data {:?}", n , &buf[0..n]);
            }, 
            Err(e) => { 
                error!("Failed to read from socket: {}", e);
                return;                
            }
        }

        let write_buf = "Hello, World"; 
        let send_buf = write_buf.as_bytes(); 
        match client_socket.write(send_buf ).await{ 
            Ok(n) => { 
                info!("Written {n} bytes of data to client"); 
            }, 
            Err(e) => {
                error!("Failed to write to socket: {}", e)
            }
        };
    }
}

