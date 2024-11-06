// pub fn mul(left: u64, right: u64) -> u64 {
//     let mut result = 0;
//     for _i in 0 .. left {
//         result = repo_A::add(result, right);
//     }
//     left * right
// }

// pub fn div(left: u64, right: u64) -> u64 {
//     left / right
// }

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn fake_macro(meta: TokenStream, input: TokenStream) -> TokenStream {

    if repo_A::add(1, 1) == 2 {
        return input;
    } else {
        return meta;
    }

}