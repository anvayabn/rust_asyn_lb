pub mod handle_client{ 
    use std::os::windows::{process, thread};
    use std::time::{SystemTime, Duration};
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use log::{debug, error, log_enabled, info, Level, trace};
    use std::sync::{Arc, mpsc};

    use crate::manager; 

    pub async fn handle_client( mut client_socket: TcpStream, tx: std::sync::mpsc::Sender<manager::DataClient>) { 
        let tid = thread_id::get(); 
        debug!( "Thread id of spawned task {}", tid);

        let mut buf = [0; 1024]; 

        let mut total_bytes_sent = 0usize;
        let mut count = 0u32 ; 

        loop{
            let start = SystemTime::now(); 
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
                    total_bytes_sent += n ; 
                }, 
                Err(e) => {
                    error!("Failed to write to socket: {}", e)
                }
            };
            let stop = SystemTime::now(); 

            /* Did a request respose */
            count += 1 ; 
            
            /* after each rr we send a report to manager */
            let latency = stop.duration_since(start)
                            .expect("System Time failed")
                            .as_nanos();
            
            /* Create a Data Client */
            let report = manager::DataClient::new(tid, latency, 
                                        count, total_bytes_sent); 

            match tx.send(report) { 
                Ok(_) =>{ 
                    debug!( "Sent latency info to manager {latency} ms"); 
                },
                Err(e) => { 
                    error!("Failed to send latency info {e}");
                    break; 
                }
            }

        }
    }
}