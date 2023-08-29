// NOTE: generate fully expanded version with `cargo expand`.
//
//       cargo expand --example doc_example


use derive_builder::Builder;

/// input `&mut self`, output `&mut Self`
#[allow(dead_code)]
#[derive(Builder)]
#[builder(pattern = "mutable")]
struct LoremMutableNonOwned {
    ipsum: u32,
    name: String,
}


/// input `self`, output `Self`
#[allow(dead_code)]
#[derive(Builder)]
#[builder(pattern = "owned")]
struct LoremImmutableOwned {
    ipsum: u32,
    name: String,
}

/// input `&self`, output `Self`
#[allow(dead_code)]
#[derive(Builder)]
#[builder(pattern = "immutable")]
struct LoremImmutableNonOwned {
    ipsum: u32,
    name: String,
}