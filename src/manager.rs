use std::os::windows::{process, thread};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, mpsc};
use tokio::sync::{Mutex};
use tokio::signal;
use log::{debug, error, log_enabled, info, Level, trace};
use env_logger;
use thread_id;

/* 
TO:DO 
    Use the this data structure to pass between 
    client handler and the manager 
    Should be modified in main.rs where the channel
    must be created with message type 
    
    This should be sent by the sender
    
    Implement a  file handler in the manager where the final 
    result can be written in CSV
*/
pub struct DataClient{ 
    pub threadid : usize , 
    pub av_latency: f64, 
    pub no_request: u32, 
    pub bytes_data: usize,
}

impl DataClient { 

    pub fn new(tid: usize, al: f64, nr: u32, by: usize) -> DataClient {
        DataClient {threadid: tid, av_latency: al, no_request: nr, bytes_data: by}
    }
}
pub fn start_manager_t(rx: std::sync::mpsc::Receiver<u128>) { 

    debug!( "Manager thread  starting ... {}"
                        , thread_id::get()); 

    /* loop and keep checking for updates 
                            on the channel */
    let mut total_latency = 0f64;
    let mut count = 0u64 ;  
    loop { 
        // check for latency updates from the 
        let lat = rx.recv(); 
        match lat{ 
            /* if received latency add to total latency  */
            Ok(lat) => { 
                total_latency += lat as f64; 
                count += 1; 

                let avg_latency = total_latency / count as f64; 
                info!( "Average latency { } for { } requests", 
                            avg_latency, count);  
            }, 
            Err(e)  => {
                debug!("Received something fishy { }", e); 
                break; 
            }
        }

    }


}