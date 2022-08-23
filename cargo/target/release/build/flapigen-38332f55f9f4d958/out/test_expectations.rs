
#[test]
fn test_expectation_bool_in_out_interface() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("bool_in_out_interface.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_bool_in_out() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("bool_in_out.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_circular_deps() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("circular_deps.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_class_with_dummy_constructor() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("class_with_dummy_constructor.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_cpp_include_custom_rule() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("cpp_include_custom_rule.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_cpp_include_return_only_result_vec() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("cpp_include_return_only_result_vec.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_cpp_qdate_typemap() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("cpp_qdate_typemap.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_cpp_return_option() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("cpp_return_option.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_cpp_return_tuple() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("cpp_return_tuple.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_cpp_self_ref() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("cpp_self_ref.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_document_generated_code() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("document_generated_code.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_foreign_class_as_arg_type_simple() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("foreign_class_as_arg_type_simple.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_foreign_class_as_return_type_simple() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("foreign_class_as_return_type_simple.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_foreign_class_static_only_methods() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("foreign_class_static_only_methods.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_foreign_enum_plus_interface() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("foreign_enum_plus_interface.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_foreign_interface_cpp_return_not_void() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("foreign_interface_cpp_return_not_void.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_foreign_interface() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("foreign_interface.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_foreign_vec_as_arg_cpp() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("foreign_vec_as_arg_cpp.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_foreign_vec_return() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("foreign_vec_return.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_generated_ref_classes() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("generated_ref_classes.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_generic() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("generic.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_int_array() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("int_array.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_interface_with_str() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("interface_with_str.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_javadoc() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("javadoc.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_lifetime_param_in_result() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("lifetime_param_in_result.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_option_arg_cpp() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("option_arg_cpp.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_option_java() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("option_java.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_own_objects_creation() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("own_objects_creation.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_pass_foreign_trait_cpp() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("pass_foreign_trait_cpp.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_pass_objects_as_param() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("pass_objects_as_param.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_pass_objects_as_param_simple() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("pass_objects_as_param_simple.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_pass_slice_as_args() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("pass_slice_as_args.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_references() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("references.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_return_foreign_class1() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("return_foreign_class1.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_return_foreign_class2() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("return_foreign_class2.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_return_foreign_class3() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("return_foreign_class3.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_return_foreign_class_arc() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("return_foreign_class_arc.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_return_foreign_enum_as_err() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("return_foreign_enum_as_err.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_return_foreign_interface_opt() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("return_foreign_interface_opt.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_return_result_i64_object() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("return_result_i64_object.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_return_result_with_object_as_value_and_err() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("return_result_with_object_as_value_and_err.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_return_result_with_object_as_value_and_err_cpp() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("return_result_with_object_as_value_and_err_cpp.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_vec_with_basic_types() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("vec_with_basic_types.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_return_slice() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("return_slice.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_static_func_with_foreign_class_as_param1() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("static_func_with_foreign_class_as_param1.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_static_func_with_foreign_class_as_param2() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("static_func_with_foreign_class_as_param2.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_static_func_with_foreign_class_full() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("static_func_with_foreign_class_full.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_string_containers() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("string_containers.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_string_handling() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("string_handling.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_work_with_rc() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("work_with_rc.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_inline_function() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("inline_function.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_test_bare_fn_match() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("test_bare_fn_match.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_cpp_generic_ptr_rule() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("cpp_generic_ptr_rule.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_smart_ptr_copy_derived() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("smart_ptr_copy_derived.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_smart_ptr_copy_derived_arc() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("smart_ptr_copy_derived_arc.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_null_annotation_java() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("null_annotation_java.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_callback_with_several_traits() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("callback_with_several_traits.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_reachability_fence_java() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("reachability_fence_java.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_inline_dyn() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("inline_dyn.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_fenum_with_comments() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("fenum_with_comments.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_cpp_plain_class() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("cpp_plain_class.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_access() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("access.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_import_null_annotation_java() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("import_null_annotation_java.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_parse_errors() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("parse_errors.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_result_in_callback() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("result_in_callback.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_cpp_ret_opt_qstring() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("cpp_ret_opt_qstring.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}

#[test]
fn test_expectation_callback_in_callback_args() {
   let _ = env_logger::try_init();

   let test_case = Path::new("tests").join("expectations").join("callback_in_callback_args.rs");
   let base_name = test_case.file_stem().expect("name without extenstion");
   let test_name = base_name.to_string_lossy();

   let mut test_something = false;
   for lang in &[ForeignLang::Cpp, ForeignLang::Java] {
       if check_expectation(&test_name, &test_case, *lang) {
           test_something = true;
       }
   }
   assert!(test_something, "empty test");
}
