use proc_macro::TokenStream;
use std::ffi::{CStr, CString};
use std::mem;
use std::str;
use std::sync::Once;

mod ffi {
    use libc::{c_char, c_int};

    #[repr(C)]
    pub struct ParseResult {
        pub success: c_int,
        pub error_message: *const c_char,
        pub index: c_int,
        pub num_params: c_int,
    }

    extern {
        pub fn init_parser();
        pub fn parse_query(query: *const c_char, result: *mut ParseResult);
    }
}

struct ParseInfo {
    #[allow(dead_code)]
    num_params: Option<usize>,
}

struct ParseError {
    message: String,
    index: usize,
}

impl std::string::ToString for ParseError {
    fn to_string(&self) -> String {
        format!("Invalid syntax at position {}: {}", self.index, self.message)
    }
}

static INITIALIZED: Once = Once::new();

fn parse<S: AsRef<str>>(query: S) -> Result<ParseInfo, ParseError> {
    INITIALIZED.call_once(|| unsafe { ffi::init_parser() });

    let mut result = mem::MaybeUninit::<ffi::ParseResult>::uninit();
    let query = CString::new(query.as_ref().as_bytes()).unwrap();
    
    let result = unsafe {
        ffi::parse_query(query.as_ptr(), result.as_mut_ptr());
        result.assume_init()
    };

    if result.success != 0 {
        let num_params = if result.num_params < 0 {
            None
        } else {
            Some(result.num_params as usize)
        };
        Ok(ParseInfo { num_params })
    } else {
        let message = unsafe { CStr::from_ptr(result.error_message) };
        Err(ParseError {
            message: str::from_utf8(message.to_bytes()).unwrap().to_string(),
            index: result.index as usize,
        })
    }
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let input2 = input.clone();
    let literal = syn::parse_macro_input!(input as syn::LitStr);
    match parse(literal.value()) {
        Ok(_) => input2,
        Err(err) => {
            let message = err.to_string();
            (quote::quote_spanned! {
                literal.span() => compile_error!(#message)
            }).into()
        }
    }
}
