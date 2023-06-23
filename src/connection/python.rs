use pyo3::prelude::*;

use std::io::{Read, Write};
use std::net::TcpStream;

use crate::routing::enums::Message;
use crate::routing::handle::Routes;

use super::interface::{
    ConnectionRoutes,
    Url,
    ConnectionId,
    EmptyState
};


/// Makes a connection to the database in an non-async manner.
/// 
/// # Arguments
/// * `url` - The URL for the connection
/// * `port` - The port for the connection
/// 
/// # Returns
/// * `Ok(String)` - The unique ID for the connection that was just made
#[pyfunction]
pub fn blocking_make_connection(url: String, port: i32) -> Result<String, PyErr> {
    let route = ConnectionRoutes::Create(Message::<Url, ConnectionId>::package_send(Url{url: url}));
    let message = Routes::Connection(route);

    let outgoing_json = serde_json::to_string(&message).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    stream.write_all(outgoing_json.as_bytes()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    let mut response_buffer = [0; 1024];
    // Read the response from the listener
    let bytes_read = stream.read(&mut response_buffer).unwrap();

    // Deserialize the response from JSON
    let response_json = &response_buffer[..bytes_read];
    let response_body: Routes = serde_json::from_slice(response_json).unwrap();

    let response = match response_body {
        Routes::Connection(message) => message,
        _ => return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Invalid response from listener"))
    };
    let unique_id = match response {
        ConnectionRoutes::Create(message) => {
            message.handle_recieve().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?
        },
        _ => return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Invalid response from listener"))
    };
    Ok(unique_id.connection_id)
}


/// Closes a connection to the database in an non-async manner.
/// 
/// # Arguments
/// * `connection_id` - The unique ID for the connection to be closed
#[pyfunction]
pub fn blocking_close_connection(connection_id: String, port: i32) -> Result<(), PyErr> {
    let route = ConnectionRoutes::Close(Message::<ConnectionId, EmptyState>::package_send(ConnectionId{connection_id: connection_id}));
    let message = Routes::Connection(route);

    let outgoing_json = serde_json::to_string(&message).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    stream.write_all(outgoing_json.as_bytes()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let mut response_buffer = [0; 1024];
    // Read the response from the listener
    let bytes_read = stream.read(&mut response_buffer).unwrap();

    // Deserialize the response from JSON
    let response_json = &response_buffer[..bytes_read];
    let response_body: Routes = serde_json::from_slice(response_json).unwrap();

    let response = match response_body {
        Routes::Connection(message) => message,
        _ => return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Invalid response from listener"))
    };
    let _ = match response {
        ConnectionRoutes::Close(message) => {
            message.handle_recieve().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?
        },
        _ => return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Invalid response from listener"))
    };
    Ok(())
}


/// Checks if a connection is still open.
/// 
/// # Arguments
/// * `connection_id` - The unique ID for the connection to be checked
/// 
/// # Returns
/// * `Ok(bool)` - Whether or not the connection is still open
#[pyfunction]
pub fn blocking_check_connection(connection_id: String, port: i32) -> Result<bool, PyErr> {
    let route = ConnectionRoutes::Check(Message::<ConnectionId, bool>::package_send(ConnectionId{connection_id: connection_id}));
    let message = Routes::Connection(route);

    let outgoing_json = serde_json::to_string(&message).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    stream.write_all(outgoing_json.as_bytes()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let mut response_buffer = [0; 1024];
    // Read the response from the listener
    let bytes_read = stream.read(&mut response_buffer).unwrap();

    // Deserialize the response from JSON
    let response_json = &response_buffer[..bytes_read];
    let response_body: Routes = serde_json::from_slice(response_json).unwrap();

    let response = match response_body {
        Routes::Connection(message) => message,
        _ => return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Invalid response from listener"))
    };
    let connection_state = match response {
        ConnectionRoutes::Check(message) => {
            message.handle_recieve().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?
        },
        _ => return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Invalid response from listener"))
    };
    Ok(connection_state)
}