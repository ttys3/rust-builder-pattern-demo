
#[macro_use]
extern crate derive_builder;

mod mutable_owned;
mod mutable_non_owned;
mod immutable_owned;
mod immutable_i_non_owned_o_owned;
mod derive_builder_demo;

fn main() {
    println!("chain_call_demo() ========================================>");
    chain_call_demo();

    println!("non_chain_call_demo() ========================================>");
    non_chain_call_demo();
}

fn chain_call_demo() {
    /// Setters take and return `&mut self`.
    /// PRO: Setter calls and final build method can be chained.
    /// CON: The build method must clone or copy data to create something owned out of a mutable reference. Otherwise it could not be used in a chain. (*)
    /// see https://docs.rs/derive_builder/latest/derive_builder/#mutable-aka-non-consuming-recommended
    let mut binding = mutable_non_owned::OpenAIConfig::new();
    let mno = binding.with_api_key("sk-xxxx").with_org_id("org-xxxx");
    println!("mutable_non_owned: {:?}", mno);

    /// Setters take and return `mut self`.
    let mo = mutable_owned::OpenAIConfig::new().with_api_key("sk-xxxx").with_org_id("org-xxxx");
    println!("mutable_owned: {:?}", mo);

    /// Setters take and return `self`.
    /// see https://docs.rs/derive_builder/latest/derive_builder/#owned-aka-consuming
    let io = immutable_owned::OpenAIConfig::new().with_api_key("sk-xxxx").with_org_id("org-xxxx");
    println!("immutable_owned: {:?}", io);

    /// Setters take `&self` and return `self`.
    /// see https://docs.rs/derive_builder/latest/derive_builder/#immutable
    let ino = immutable_i_non_owned_o_owned::OpenAIConfig::new().with_api_key("sk-xxxx").with_org_id("org-xxxx");
    println!("immutable_i_non_owned_o_owned: {:?}", ino);
}

fn non_chain_call_demo() {
    /// Setters take and return &mut self.
    /// PRO: Setter calls and final build method can be chained.
    /// CON: The build method must clone or copy data to create something owned out of a mutable reference. Otherwise it could not be used in a chain. (*)
    let mut mno = mutable_non_owned::OpenAIConfig::new();
    mno.with_api_key("sk-xxxx");
    mno.with_org_id("org-xxxx");
    println!("mutable_non_owned: {:?}", mno);

    /// Setters take and return mut self.
    let mut mo = mutable_owned::OpenAIConfig::new();
    mo = mo.with_api_key("sk-xxxx");
    mo = mo.with_org_id("org-xxxx");
    println!("mutable_owned: {:?}", mo);


    let mut io = immutable_owned::OpenAIConfig::new();
    io = io.with_api_key("sk-xxxx");
    io = io.with_org_id("org-xxxx");
    println!("immutable_owned: {:?}", io);

    let mut ino = immutable_i_non_owned_o_owned::OpenAIConfig::new();
    // buggy
    ino.with_api_key("sk-xxxx");
    ino.with_org_id("org-xxxx");
    println!("immutable_i_non_owned_o_owned buggy: {:?}", ino);

    // buggy
    ino = ino.with_api_key("sk-xxxx");
    ino = ino.with_org_id("org-xxxx");
    println!("immutable_i_non_owned_o_owned ok: {:?}", ino);
}