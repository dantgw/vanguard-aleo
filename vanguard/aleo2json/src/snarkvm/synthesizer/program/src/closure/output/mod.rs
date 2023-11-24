// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde_json::json;

mod bytes;
mod parse;

use crate::Operand;

use console::{network::prelude::*, program::RegisterType};

/// An output statement defines an output of a closure.
/// An output statement is of the form `output {operand} as {register_type};`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Output<N: Network> {
    /// The output operand.
    operand: Operand<N>,
    /// The output register type.
    register_type: RegisterType<N>,
}

impl<N: Network> Output<N> {
    /// ** Vanguard JSON serialization helper ** ///
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "type": "Output",
            "operand": self.operand.to_json(),
            "register_type": self.register_type.to_json(),
            "str": format!("{}", self),
        })
    }

    /// Returns the output register.
    #[inline]
    pub const fn operand(&self) -> &Operand<N> {
        &self.operand
    }

    /// Returns the output register type.
    #[inline]
    pub const fn register_type(&self) -> &RegisterType<N> {
        &self.register_type
    }
}

impl<N: Network> TypeName for Output<N> {
    /// Returns the type name as a string.
    #[inline]
    fn type_name() -> &'static str {
        "output"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_output_type_name() {
        assert_eq!(Output::<CurrentNetwork>::type_name(), "output");
    }
}