/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate aesm_client;
extern crate enclave_runner;
extern crate sgxs_loaders;
use aesm_client::AesmClient;
use enclave_runner::usercalls::{SyncListener, SyncStream, UsercallExtension};
use enclave_runner::EnclaveBuilder;
use sgxs_loaders::isgx::Device as IsgxDevice;
use std::thread;
use std::time::{Duration, Instant};

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
