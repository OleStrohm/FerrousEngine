#![recursion_limit = "128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();

    let gen = generate_impl(&ast);

    gen.parse().unwrap()
}

fn generate_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;

    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&ast.body);

    quote! {
        impl #ident #generics #where_clause {
            #[allow(unused_variables)]
            pub fn vertex_attrib_pointers() {
                let stride = std::mem::size_of::<Self>();
                let offset = 0;

                #(#fields_vertex_attrib_pointer)*
            }
        }
    }
}

fn generate_vertex_attrib_pointer_calls(body: &syn::Body) -> Vec<quote::Tokens> {
    match body {
        &syn::Body::Enum(_) => panic!("VertexAttribPointers can not be implemented for enums"),
        &syn::Body::Struct(syn::VariantData::Unit) => {
            panic!("VertexAttribPointers can not be implemented for unit structs")
        }
        &syn::Body::Struct(syn::VariantData::Tuple(_)) => {
            panic!("VertexAttribPointers can not be implemented for tuple structs")
        }
        &syn::Body::Struct(syn::VariantData::Struct(ref s)) => s
            .iter()
            .map(generate_struct_field_vertex_attrib_pointer_call)
            .collect(),
    }
}

fn generate_struct_field_vertex_attrib_pointer_call(f: &syn::Field) -> quote::Tokens {
    let ident = match f.ident {
        Some(ref i) => format!("{}", i),
        None => String::from(""),
    };

    let loc_attr = f
        .attrs
        .iter()
        .filter(|a| a.value.name() == "location")
        .next()
        .unwrap_or_else(|| panic!("Field {:?} is missing #[location = ?] attribute", ident));

    let loc = match loc_attr.value {
        syn::MetaItem::NameValue(_, ref literal @ syn::Lit::Int(_, _)) => literal,
        _ => panic!(
            "Field {} location attribute must be an integer literal",
            ident
        ),
    };

    let ty = &f.ty;
    quote! {
        let location = #loc;
        unsafe {
            #ty::vertex_attrib_pointer(stride, location, offset);
        }
        let offset = offset + std::mem::size_of::<#ty>();
    }
}

//impl Vertex {
//    pub fn vertex_attrib_pointers() {
//        let stride = std::mem::size_of::<Self>();
//
//        let location = 0;
//        let offset = 0;
//
//        unsafe {
//            Vec3::vertex_attrib_pointer(stride, location, offset);
//        }
//
//        let location = 1;
//        let offset = offset + std::mem::size_of::<Vec3>();
//
//        unsafe {
//            Vec3::vertex_attrib_pointer(stride, location, offset);
//        }
//    }
//}
//
