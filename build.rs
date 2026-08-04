fn main() {
    let mut settings = progenitor::GenerationSettings::new();
    settings.with_pre_hook_async(quote::quote! { crate::auth::inject });
    let mut gen = progenitor::Generator::new(&settings);
    let file = std::fs::File::open("api/openapi.rs.json").expect("preprocessed spec");
    let spec = serde_json::from_reader(file).expect("parse spec");
    let code = gen.generate_tokens(&spec).expect("generate client");
    let ast = syn::parse2(code).expect("parse tokens");
    let content = prettyplease::unparse(&ast);
    std::fs::write("src/lib_gen.rs", content).expect("write generated client");
}
