#[macro_use]
extern crate quote;

use bit_vec::BitVec;
use convert_case::{Case, Casing};
use rayon::prelude::*;
use resvg::tiny_skia::{Pixmap, Transform};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use usvg::{FitTo, Tree};

const ALPHA_CUTOFF: u8 = 0x60;
const EXTENSION: &str = "bits";

fn render_icon_to_bits(path: &Path, size: usize) -> BitVec<u8> {
    assert!(size > 0, "BUG: Cannot render icon for size 0");

    let size: u32 = size.try_into().unwrap();
    if !path.exists() {
        panic!("No file at path {path:?}");
    }

    let mut pixmap = Pixmap::new(size, size).unwrap();

    resvg::render(
        &Tree::from_str(&fs::read_to_string(path).unwrap(), &Default::default()).unwrap(),
        FitTo::Size(size, size),
        Transform::default(),
        pixmap.as_mut(),
    );

    let result: BitVec<u8> = pixmap
        .data()
        .iter()
        .enumerate()
        .filter(|(a, _)| a % 4 == 3 /* select alpha channel */)
        .map(|(_, b)| *b) // discard index
        .map(|alpha| alpha > ALPHA_CUTOFF)
        .collect();

    result
}

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

use std::io::Write;

pub struct Module {
    name: String,
    svg_names: Vec<String>,
    sizes: Vec<usize>,
}

fn starts_with_digit(s: &str) -> bool {
    s.chars().next().and_then(|c| c.to_digit(10)).is_some()
}

pub fn generate_mod(module: &Module, output_dir: &PathBuf) {
    let module_ident = quote::format_ident!("{}", module.name.clone());
    let file_name = output_dir.join("mod.rs");
    let names = module.svg_names.clone();

    println!("cargo:warning=Bits {names:#?}");

    let sizes_tokens: Vec<proc_macro2::TokenStream> = module
        .sizes
        .iter()
        .map(|size| {
            let size_feature_ident = format!("{size}px");
            let size_module_ident = quote::format_ident!("size{size}px");
            // Remove type from size variable, otherwise it'll be 12u32
            let size_ident = proc_macro2::Literal::usize_unsuffixed(*size);
            println!("cargo:warning=Mod for {size}");
            let icons_tokens: Vec<proc_macro2::TokenStream> = names
                .iter()
                .map(|icon| {
                    let icon_ident = if starts_with_digit(icon) {
                        quote::format_ident!("Icon{}", icon.clone().to_case(Case::Pascal))
                    } else {
                        quote::format_ident!("{}", icon.clone().to_case(Case::Pascal))
                    };
                    let icon_name = format!("{icon}");
                    quote! {
                        include_icon!(#icon_ident, #module_ident, #icon_name, #size_ident);
                    }
                })
                .collect();

            quote! {
                #[cfg(feature = #size_feature_ident)]
                pub mod #size_module_ident {
                    use crate::include_icon;

                    #(#icons_tokens)*
                }
            }
        })
        .collect();

    let token = quote! {
        #[allow(non_camel_case_types)]

        #(#sizes_tokens)*
    };

    dbg!("oi");
    let mut out_file = File::create(&file_name).unwrap();
    writeln!(out_file, "{}", token).unwrap();
}

pub fn generate_main_mod(libraries: &Vec<Library>, output_dir: &PathBuf) {
    let file_name = output_dir.join("mod.rs");

    let tokens: Vec<proc_macro2::TokenStream> = libraries
        .iter()
        .map(|library| {
            let feature_ident = format!("{}", library.name);
            let library_ident = quote::format_ident!("{}", library.name);
            quote! {
                #[cfg(feature = #feature_ident)]
                pub mod #library_ident;
            }
        })
        .collect();

    let token = quote! {
        #(#tokens)*
    };

    let mut out_file = File::create(&file_name).unwrap();
    writeln!(out_file, "{}", token).unwrap();
}

pub struct Library {
    name: String,
    svgs: Vec<PathBuf>,
    sizes: Vec<usize>,
}

fn get_all_svgs_from_path(path: &Path) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .map(|res| res.path())
        .filter(|path| {
            path.is_file() && path.extension().unwrap_or_default().to_ascii_lowercase() == "svg"
        })
        .collect()
}

fn create_library(library: &Library, output_dir: &PathBuf) -> Module {
    let mut names: Vec<String> = library
        .svgs
        .iter()
        .map(|svg| {
            assert!(
                svg.as_path().file_stem().is_some(),
                "SVG path {:?} is invalid!",
                svg
            );

            svg.file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_case(Case::Snake)
        })
        .collect();
    names.sort();

    library.sizes.iter().for_each(|size| {
        println!("cargo:warning=Create {size}");
        let folder = output_dir.join(&format!("{size}px"));
        fs::create_dir_all(&folder).unwrap();

        library.svgs.iter().for_each(|file| {
            let bits = render_icon_to_bits(file.as_path(), *size);
            let mut target_file =
                folder.join(file.as_path().file_stem().unwrap().to_str().unwrap());
            target_file.set_extension(EXTENSION);
            let new_name = target_file
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_case(Case::Snake);
            target_file.set_file_name(&new_name);
            fs::write(&target_file, bits.blocks().collect::<Vec<_>>()).unwrap();
        });
    });

    Module {
        name: library.name.clone(),
        svg_names: names,
        sizes: library.sizes.clone(),
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs");

    let project_dir = Path::new(PROJECT_DIR);
    let libraries_dir = project_dir.join("build/libraries");
    let rendered_dir = project_dir.join("rendered");
    let default_sizes = vec![12, 18, 24, 32, 48, 96, 144];

    let libraries = vec![
        Library {
            name: "iconoir".into(),
            svgs: get_all_svgs_from_path(&libraries_dir.join("iconoir/icons")),
            sizes: default_sizes.clone(),
        },
        Library {
            name: "ionic".into(),
            svgs: get_all_svgs_from_path(&libraries_dir.join("ionicons/src/svg")),
            sizes: default_sizes.clone(),
        },
        Library {
            name: "mdi".into(),
            svgs: get_all_svgs_from_path(&libraries_dir.join("MaterialDesign/svg")),
            sizes: default_sizes.clone(),
        },
        Library {
            name: "simple".into(),
            svgs: get_all_svgs_from_path(&libraries_dir.join("simple-icons/icons")),
            sizes: default_sizes.clone(),
        },
    ];

    for library in &libraries {
        println!("cargo:warning=This is a warning message");

        let output = &rendered_dir.join(&library.name);
        let module = create_library(&library, output);
        generate_mod(&module, output);
    }
    generate_main_mod(&libraries, &rendered_dir);
    println!("cargo:warning=Done!");
}
