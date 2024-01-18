pub mod handle_client{ 
    use std::os::windows::{process, thread};
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use log::{debug, error, log_enabled, info, Level, trace};

    pub async fn handle_client( mut client_socket: TcpStream) { 

        debug!( "Thread id of spawned task {}", thread_id::get());
        let mut buf = [0; 1024]; 
    
        loop{
            match client_socket.read(&mut buf).await{
                Ok(n) if n == 0 => {
                    info!("Read {n} bytes of data"); 
                    return; 
                }, 
                Ok(n) => { 
                    debug!("Read { } byted of data", n );
                    trace!("Read { } bytes of data {:?}", 
                        n , &buf[0..n]);
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
}