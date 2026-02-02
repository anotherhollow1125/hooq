use std::rc::Rc;

use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::Flavor;
use crate::impls::method::Method;
use crate::impls::utils::unexpected_error_message::UNEXPECTED_ERROR_MESSAGE;

macro_rules! level_binding {
    ($level:expr) => {
        [("level".to_string(), Rc::new(parse_quote!($level)))].into()
    };
}

pub fn tracing_flavor() -> Flavor {
    let method: Method = tracing_method().try_into().expect(UNEXPECTED_ERROR_MESSAGE);

    Flavor {
        method: method.clone(),
        bindings: level_binding!(::tracing::Level::ERROR),
        sub_flavors: [
            ("error", level_binding!(::tracing::Level::ERROR)),
            ("warn", level_binding!(::tracing::Level::WARN)),
            ("info", level_binding!(::tracing::Level::INFO)),
            ("debug", level_binding!(::tracing::Level::DEBUG)),
            ("trace", level_binding!(::tracing::Level::TRACE)),
        ]
        .into_iter()
        .map(|(s, bindings)| {
            (
                s.to_string(),
                Flavor {
                    method: method.clone(),
                    bindings,
                    ..Default::default()
                },
            )
        })
        .collect(),
        ..Default::default()
    }
}

fn tracing_method() -> TokenStream {
    let excerpted_helpers_path = crate::impls::utils::get_source_excerpt_helpers_name_space();

    parse_quote! {
        .inspect_err(|e| {
            let path = $path;
            let line = $line;
            let col = $col;
            let expr = #excerpted_helpers_path ::one_line_stringify!($source);

            ::tracing::event!(
                $level,
                path,
                line,
                col,
                error = %e,
                expr,
            );
        })
    }
}
