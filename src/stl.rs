// RGB Core Library: consensus layer for RGB smart contracts.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2023 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2019-2023 Dr Maxim Orlovsky. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use aluvm::stl::aluvm_stl;
pub use bp::bc::stl::bp_tx_stl;
pub use bp::stl::bp_core_stl;
use strict_types::stl::{std_stl, strict_types_stl};
use strict_types::typelib::LibBuilder;
use strict_types::{CompileError, TypeLib};

use crate::{Extension, Genesis, SubSchema, TransitionBundle, LIB_NAME_RGB};

/// Strict types id for the library providing data types for RGB consensus.
pub const LIB_ID_RGB: &str =
    "urn:ubideco:stl:4fGZWR5mH5zZzRZ1r7CSRe776zm3hLBUngfXc4s3vm3V#saturn-flash-emerald";

fn _rgb_core_stl() -> Result<TypeLib, CompileError> {
    LibBuilder::new(libname!(LIB_NAME_RGB), tiny_bset! {
        std_stl().to_dependency(),
        strict_types_stl().to_dependency(),
        bp_tx_stl().to_dependency(),
        bp_core_stl().to_dependency(),
        aluvm_stl().to_dependency()
    })
    .transpile::<SubSchema>()
    .transpile::<Genesis>()
    .transpile::<TransitionBundle>()
    .transpile::<Extension>()
    .compile()
}

/// Generates strict type library providing data types for RGB consensus.
pub fn rgb_core_stl() -> TypeLib { _rgb_core_stl().expect("invalid strict type RGB library") }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lib_id() {
        let lib = rgb_core_stl();
        assert_eq!(lib.id().to_string(), LIB_ID_RGB);
    }
}
