use tokenizers::Tokenizer;
use cxx::CxxString;

pub struct TokenizerWrapper {
    inner: Tokenizer,
}

impl TokenizerWrapper {
    pub fn new(file: &str) -> Self {
        let tokenizer = Tokenizer::from_file(file)
            .expect("failed to load tokenizer");
        Self { inner: tokenizer }
    }

    pub fn encode(&self, text: &CxxString , add_special_tokens: bool  ) -> ffi::EncodeResult {
    let encoding = self.inner.encode(
            text.to_str().unwrap(), add_special_tokens
        ).unwrap();
        
        ffi::EncodeResult {
            ids: encoding.get_ids().to_vec(),
            tokens: encoding.get_tokens().to_vec(),
            attention_mask: encoding.get_attention_mask().to_vec(),
        }
    }
    pub fn decode(&self, ids: &Vec<u32>, skip_special_tokens: bool) -> String {
        self.inner
            .decode(ids, skip_special_tokens)
            .unwrap_or_else(|_| "".to_string())
    }
}

#[cxx::bridge]
mod ffi {

    struct EncodeResult {
        ids: Vec<u32>,
        tokens: Vec<String>,
        attention_mask : Vec<u32>
    }

    extern "Rust" {
        type TokenizerWrapper;

        fn make_tokenizer(file: &CxxString) -> Box<TokenizerWrapper>;
        fn encode(self: &TokenizerWrapper,text: &CxxString , add_special_tokens: bool ) -> EncodeResult;
        fn decode(self: &TokenizerWrapper, ids: &Vec<u32> , skip_special_tokens: bool ) -> String;
    }
}

pub fn make_tokenizer(file: &CxxString) -> Box<TokenizerWrapper> {
    Box::new(TokenizerWrapper::new(file.to_str().unwrap()))
}
