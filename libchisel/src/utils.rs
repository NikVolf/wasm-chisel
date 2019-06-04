//! These are helpers to be used internally.

use parity_wasm::elements::{deserialize_buffer, serialize, Module};

pub trait HasNamesSection {
    /// Returns true if the module has a NamesSection.
    fn has_names_section(&self) -> bool;
}

impl HasNamesSection for Module {
    fn has_names_section(&self) -> bool {
        // Lets forcefully parse in case it wasn't yet.
        let mut module = self
            .clone()
            .parse_names()
            .expect("parsing the names section failed");
        module.names_section().is_some()
    }
}

pub trait SerializationHelpers {
    /// Deserialize bytecode to a Module.
    fn from_slice(input: &[u8]) -> Module;

    /// Serialize Module to bytecode. Serialization consumes the input.
    fn to_vec(self) -> Vec<u8>;
}

impl SerializationHelpers for Module {
    fn from_slice(input: &[u8]) -> Self {
        deserialize_buffer::<Module>(&input).expect("invalid Wasm bytecode")
    }

    fn to_vec(self) -> Vec<u8> {
        serialize::<Module>(self).expect("invalid Wasm module")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_hex::FromHex;

    #[test]
    fn module_roundtrip() {
        let input = FromHex::from_hex(
            "0061736d01000000010401600000030201000405017001010105030100100619
037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d
6f727902000b5f5f686561705f6261736503010a5f5f646174615f656e640302
0a040102000b",
        )
        .unwrap();
        let module = Module::from_slice(&input);
        let output = module.to_vec();
        assert_eq!(input, output);
    }

    #[test]
    fn bytecode_has_names_section() {
        let input = FromHex::from_hex(
            "0061736d010000000104016000000303020000070801046d61696e00010a
0a020300010b040010000b0014046e616d65010d0200047465737401046d
61696e",
        )
        .unwrap();
        let module = Module::from_slice(&input);
        assert_eq!(module.has_names_section(), true);
    }

    #[test]
    fn bytecode_has_no_names_section() {
        let input = FromHex::from_hex(
            "0061736d010000000104016000000303020000070801046d61696e00010a
0a020300010b040010000b",
        )
        .unwrap();
        let module = Module::from_slice(&input);
        assert_eq!(module.has_names_section(), false);
    }
}