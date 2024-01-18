use std::os::windows::{process, thread};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::signal;
use log::{debug, error, log_enabled, info, Level, trace};
use env_logger;

use thread_id;

mod client;
#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>>{
    env_logger::init(); 

    debug!( "Thread id of async main {}", thread_id::get());

    /* Open Listener  */
    let lt = 
        TcpListener::bind("127.0.0.1:8080").await?;

    /* after binding the socket 
        create shared listener  */ 
    let lt = Arc::new(&lt); 
    let shutdown_signal = Arc::new(Mutex::new(false)); 

    /* Loop to accept connection and spawn task to 
    handle I/O  concurrently check for for interrupts 
    such as ctrl^c OR if there are I/O event 
    do that. Which is ever is first execute that*/ 
    loop{
        /* Since we need to share the listener 
        and the shut down between diffrent iterations */
        let lt = Arc::clone(&lt); 
        let shutdown_signal = Arc::clone(&shutdown_signal); 

        /* Using tokio select! to switch between 
            and monitor SIGNALS and IO  */
        tokio::select! {
            /* Check for SIGNAL */
            _ = signal::ctrl_c() => { 
                info!("Ctrl Received. Shutting down ....");
                /* Since lock might be held by others wait */
                *shutdown_signal.lock().await = true ; 
                break;  
            },
            /* Or else accept connections  */
            result = lt.accept() => { 
                match result { 
                    Ok((socket, addr)) => { 
                        info!("Got connection from a client {addr}"); 
                        tokio::spawn(async move {
                            client::handle_client::handle_client(socket).await; 
                        }); 
                    }
                    Err(e) => { 
                        error!("Error accepting connection : { }", e);
                        /* if error occurs  and check if shutdown 
                            signal is enabled then break the loop 
                            and exit() the main */
                        if *shutdown_signal.lock().await { 
                            break; 
                        }    
                    }
                }
            }
        }
    }

    Ok(())

}

