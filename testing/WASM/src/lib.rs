use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test(inputData: &str) -> String {
  
    let test = String::from("Data from typescript: \n");

    return test + inputData;
}

