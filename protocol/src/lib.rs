//  Copyright (c) Facebook, Inc. and its affiliates.
//  SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![crate_name = "protocol"]

#[macro_use]
extern crate log;

#[cfg(not(target_arch = "wasm32"))]  pub mod cross_psi;
pub mod fileio;
#[cfg(not(target_arch = "wasm32"))]  pub mod pjc;
pub mod private_id;

pub mod shared {
    extern crate crypto;

    use crypto::prelude::*;
    use std::path::Path;

    /// Type of the input expected right now
    pub type TDomain = u64;
    /// Feature matrix type
    pub type TFeatures = Vec<Vec<TDomain>>;

    /// trait to get the encryption key
    #[cfg(not(target_arch = "wasm32"))]  /// Omit HE becasue wasm32 only supports Private-ID+ECCipher
    pub trait ShareableEncKey {
        fn get_he_public_key(&self) -> EncryptionKey;
    }

    pub trait LoadData {
        fn load_data<T>(&self, input_path: T)
        where
            T: AsRef<Path>;
    }

    pub trait Reveal {
        fn reveal<T: AsRef<Path>>(&self, path: T);
    }
}
