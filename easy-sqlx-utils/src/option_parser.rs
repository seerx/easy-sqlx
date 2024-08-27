static OPTIONS_TYPE: [&str; 3] = ["Option", "std::option::Option", "core::option::Option"];

pub fn is_option(ty: &str) -> bool {
    OPTIONS_TYPE.contains(&ty)
}

// pub fn parse_size(lit: &Option<Lit>) -> Option<isize> {
//      if let Some(len) = lit {
//         match len {
//             Lit::Int(l) => {
//                 let res = l.base10_parse::<isize>();
//                 if let Ok(sz) = res {
//                     Some(sz)
//                 } else {
//                     None
//                 }
//             },
//             _ => {
//                 None
//             }
//         }
//     } else { None }
// }

// pub fn parse_type_as_string(ty: &syn::Type, add_surffix: bool) -> String {
//     if let syn::Type::Path(p) = ty {
//         // let paths = p.path.segments.iter().map(|v| v.ident.to_string()).collect::<Vec<String>>();
//         // return paths.join("::");
//         let idents_of_path =
//             p.path
//                 .segments
//                 .iter()
//                 .into_iter()
//                 .fold(String::new(), |mut acc, v| {
//                     acc.push_str(&v.ident.to_string());
//                     if add_surffix {
//                         acc.push('|');
//                     }
//                     acc
//                 });
//         return idents_of_path;
//     }
//     "".to_string()
// }

pub fn parse_type_options(ty: &syn::Type) -> (isize, String, &syn::Type) {
    if let syn::Type::Path(p) = ty {
        let idents_path = p
            .path
            .segments
            .iter()
            .map(|v| v.ident.to_string())
            .collect::<Vec<String>>()
            .join("::");
        if is_option(&idents_path.as_str()) {
            // 是 Option
            if let Some(p) = p.path.segments.first() {
                if let syn::PathArguments::AngleBracketed(ref params) = p.arguments {
                    if let syn::GenericArgument::Type(ref ty) = params.args.first().unwrap() {
                        // 解析泛型
                        // 继续下一层解析
                        let (option_count, path, typ) = parse_type_options(ty);
                        return (option_count + 1, path, typ);
                    }
                }
            }
        }
        // 不是 Option
        return (0, idents_path, ty);
    }

    panic!("无法解析类型")
}

// pub fn parse_types(paths: &mut Vec<&syn::Type>) {
//     if let syn::Type::Path(p) = paths.last().unwrap() {
//         let idents_of_path =
//             p.path
//                 .segments
//                 .iter()
//                 .into_iter()
//                 .fold(String::new(), |mut acc, v| {
//                     acc.push_str(&v.ident.to_string());
//                     acc.push('|');
//                     acc
//                 });

//         if is_option(&idents_of_path.as_str()) {
//             // count += 1;
//             if let Some(p) = p.path.segments.first() {
//                 if let syn::PathArguments::AngleBracketed(ref params) = p.arguments {
//                     if let syn::GenericArgument::Type(ref ty) = params.args.first().unwrap() {
//                         paths.push(ty);
//                         parse_types(paths);
//                         // count = find_option(count, ty);
//                     }
//                 }
//             }
//         }
//     }
//     // count
// }
