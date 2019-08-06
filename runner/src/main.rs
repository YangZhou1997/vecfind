/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate aesm_client;
extern crate byteorder;
extern crate enclave_runner;
extern crate libc;
extern crate sgxs_loaders;
use std::time::{Duration, Instant};

use aesm_client::AesmClient;
use enclave_runner::usercalls::{SyncListener, SyncStream, UsercallExtension};
use enclave_runner::EnclaveBuilder;
use sgxs_loaders::isgx::Device as IsgxDevice;
use std::io;

use byteorder::{NetworkEndian, ReadBytesExt};
use std::io::{Error, ErrorKind, Read, Result as IoResult, Write};
use std::mem::size_of;
use std::net::Shutdown;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream};
use std::thread;

fn usage(name: String) {
    println!("Usage:\n{} <path_to_sgxs_file>", name);
}

fn parse_args() -> Result<String, ()> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => Ok(args[1].to_owned()),
        _ => {
            usage(args[0].to_owned());
            Err(())
        }
    }
}

fn run_server(file: String) -> Result<(), ()> {
    let mut device = IsgxDevice::new()
        .unwrap()
        .einittoken_provider(AesmClient::new())
        .build();

    let mut enclave_builder = EnclaveBuilder::new(file.as_ref());
    enclave_builder.dummy_signature();
    let enclave = enclave_builder.build(&mut device).unwrap();

    enclave.run().map_err(|e| {
        eprintln!("Error in running enclave {}", e);
    })
}

fn main() {
    let file = parse_args().unwrap();
    
    let start = Instant::now();
    
    let server = thread::spawn(move || run_server(file));
    let _ = server.join().unwrap();
    
    let duration = start.elapsed();
    println!("{:?}", duration);
}
