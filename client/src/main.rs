use commands::{create_page, get_pages};

#[cfg(test)]
mod tests;
// mod main_client;
mod commands;
mod requests;
mod sys;

pub mod vault;
pub const EMAIL: &str = "ssample@gmail.com";
pub const PASSWORD: &str = "samplePass123";
fn main() {
    // commandsxd::register("ssample@gmail.com", "samplePass123");
    // let token = commands::login("ssample@gmail.com", "samplePass123");
    // let vault = vault::Vault::create();
    // let mut vault = vault::Vault::load().unwrap();
    // vault.scan();
    // vault.save();
    // let xd = sys::page_data("./sample.md");
    // println!("xd: {:?}", xd.unwrap());
    // let id = commands::create_page("sample1123", "sample");
    // let mut vault = vault::Vault::load().unwrap();
    let (token, user_id) = commands::login(EMAIL, PASSWORD);
    let mut vault = vault::Vault::load().unwrap();
    vault.push();
    // commands::get_pages(&token, user_id);
    // vault.push();
    // println!("page id: {}", id);
    return;

    // print!("user {} token: {}", user_id, token.as_str().trim().len());
    //
    // let aa = "3c86a53f3ce3ecb8fee37dcb4a537376";
    // println!("-{}-", user_id);
    // println!("{:?} {}", &token[..token.len()], token.len());
    // dd9e9019a630064c8f6b1a4a80d8c432
    // "dd9e9019a630064c8f6b1a4a80d8c432"
    // get_pages(&token, user_id);
    // println!("Hello, world!{:?}", vault);
    // commandsxd::logout(&token);
    //
    // create_page(&token);
    //
    // let token = "1af8e96bd6225004ab131ee6ff7f34d8";
    // requests::get_page();
    // let user_id = requests::get_user_id(&token);
    //
    // print!("HAHHAuser_id: {}", user_id);
    // // let token =
    //requests::login("ssample@gmail.com", "samplePass123");
    // requests::unregister(&token);
}