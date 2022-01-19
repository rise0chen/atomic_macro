use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::DeriveInput;

#[proc_macro_attribute]
pub fn atomic(attr: TokenStream, mut input: TokenStream) -> TokenStream {
    let atomic_int = Ident::new(&format!("AtomicU{}", attr), Span::call_site());
    let ast: DeriveInput = syn::parse(input.clone()).unwrap();
    let (vis, ident) = (&ast.vis, &ast.ident);
    let atomic_ident = Ident::new(&format!("Atomic{}", ident), Span::call_site());

    let tokens = quote! {
        #vis struct #atomic_ident (core::sync::atomic::#atomic_int);
        impl #atomic_ident {
            #[inline(always)]
            pub const ZERO: Self = Self(core::sync::atomic::#atomic_int::new(0));
            #[inline(always)]
            pub fn new(v: #ident) -> Self {
                Self(core::sync::atomic::#atomic_int::new(v.into()))
            }
            #[inline(always)]
            pub fn load(&self, order: core::sync::atomic::Ordering) -> #ident {
                self.0.load(order).into()
            }
            #[inline(always)]
            pub fn store(&self, val: #ident, order: core::sync::atomic::Ordering) {
                self.0.store(val.into(), order)
            }
            #[inline(always)]
            pub fn swap(&self, val: #ident, order: core::sync::atomic::Ordering) -> #ident {
                self.0.swap(val.into(), order).into()
            }
            #[inline(always)]
            pub fn compare_exchange(
                &self,
                current: #ident,
                new: #ident,
                success: core::sync::atomic::Ordering,
                failure: core::sync::atomic::Ordering,
            ) -> Result<#ident, #ident> {
                match self.0.compare_exchange(current.into(), new.into(), success, failure) {
                    Ok(x) => Ok(x.into()),
                    Err(x) => Err(x.into()),
                }
            }
            #[inline(always)]
            pub fn compare_exchange_weak(
                &self,
                current: #ident,
                new: #ident,
                success: core::sync::atomic::Ordering,
                failure: core::sync::atomic::Ordering,
            ) -> Result<#ident, #ident> {
                match self.0.compare_exchange_weak(current.into(), new.into(), success, failure) {
                    Ok(x) => Ok(x.into()),
                    Err(x) => Err(x.into()),
                }
            }
            #[inline(always)]
            pub fn fetch_update<F>(
                &self,
                set_order: core::sync::atomic::Ordering,
                fetch_order: core::sync::atomic::Ordering,
                mut f: F,
            ) -> Result<#ident, #ident>
            where
                F: FnMut(#ident) -> Option<#ident>,
            {
                match self.0.fetch_update(set_order, fetch_order, |x| {
                    match f(x.into()) {
                        Some(x) => Some(x.into()),
                        None => None,
                    }
                }) {
                    Ok(x) => Ok(x.into()),
                    Err(x) => Err(x.into()),
                }
            }
        }
    };
    let tokens = TokenStream::from(tokens);
    input.extend([tokens]);
    input
}
