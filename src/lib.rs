
use proc_macro2::Span;
use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, FnArg, TypeImplTrait, punctuated::Punctuated, TypeParamBound, TraitBound, Path, PathSegment, Ident, AngleBracketedGenericArguments, Type, Local, Pat, Stmt, PatIdent, Expr, ExprCall, ExprPath};

fn punctuated<const N: usize, T, P: Default>(v: [T; N]) -> Punctuated<T, P> {
	let mut p = Punctuated::new();
	for v in v { p.push(v); }
	p
}

fn make_impl_into_ty(ty: Type) -> Type {
	let mut bounds = Punctuated::new();
	bounds.push(
		TypeParamBound::Trait(
			TraitBound {
				paren_token: None,
				modifier: syn::TraitBoundModifier::None,
				lifetimes: None,
				path: Path {
					leading_colon: None,
					segments: punctuated([
						PathSegment {
							ident: Ident::new("Into", Span::call_site()),
							arguments: syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments {
								colon2_token: None,
								lt_token: Default::default(),
								args: punctuated([syn::GenericArgument::Type(ty)]),
								gt_token: Default::default(),
							}),
						},
					]),
				},
			}
		)
	);
	syn::Type::ImplTrait(TypeImplTrait { impl_token: Default::default(), bounds })
}

fn make_implicit_into_call(ident: Ident, pat: Pat, ty: Type) -> Stmt {
	let local = Local {
		attrs: Vec::new(),
		let_token: Default::default(),
		pat,
		init: Some((
			Default::default(),
			Box::new(Expr::Call(ExprCall {
				attrs: Vec::new(),
				func: Box::new(Expr::Path(ExprPath {
					attrs: Vec::new(),
					qself: None,
					path: Path {
						leading_colon: None,
						segments: punctuated([
							PathSegment { 
								ident: Ident::new("Into", Span::call_site()), 
								arguments: syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments {
									colon2_token: Some(Default::default()), 
									lt_token: Default::default(), 
									args: {
										let mut args = Punctuated::new();
										args.push(syn::GenericArgument::Type(ty));
										args
									}, 
									gt_token: Default::default(),
								}),
							},
							PathSegment {
        				ident: Ident::new("into", Span::call_site()),
        				arguments: syn::PathArguments::None,
							}
						]),
					},
				})),
				paren_token: Default::default(),
				args: punctuated([
					Expr::Path(syn::ExprPath {
						attrs: Vec::new(),
						qself: None,
						path: Path {
							leading_colon: None,
							segments: {
								let mut segs = Punctuated::new();
								segs.push(PathSegment { ident, arguments: syn::PathArguments::None });
								segs
							}
						},
					})
				])
			}))
		)),
		semi_token: Default::default(),
	};
	Stmt::Local(local)
}

#[proc_macro_attribute]
pub fn auto_into(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let mut fn_item = parse_macro_input!(item as ItemFn);

	let mut i = 0;
	for arg in fn_item.sig.inputs.iter_mut() {
		let FnArg::Typed(arg) = arg else { continue };
		let mut j = 0;
		if arg.attrs.iter().find(|attr| { j += 1; attr.path.get_ident().unwrap().to_string().eq("into") }).is_some() {
			let pat = arg.pat.clone();
			let ty = arg.ty.clone();

			let ident = Ident::new(&format!("auto_into{i}"), Span::call_site());

			arg.ty = Box::new(make_impl_into_ty(*ty.clone()));
			arg.pat = Box::new(Pat::Ident(PatIdent {
				attrs: Vec::new(),
				by_ref: None,
				mutability: None,
				ident: ident.clone(),
				subpat: None,
			}));

			fn_item.block.stmts.insert(0, make_implicit_into_call(ident.clone(), *pat, *ty));

			i += 1;
		}
		arg.attrs.remove(j - 1);
	}

	quote! { #fn_item }.into()
}
