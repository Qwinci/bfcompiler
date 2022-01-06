use std::path::Path;
use std::thread::Builder;
use crate::lexer::Token;
use inkwell::{IntPredicate, OptimizationLevel};
use inkwell::basic_block::BasicBlock;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::context::Context;
use inkwell::module::Linkage;

pub fn codegen(tokens: Vec<Token>) {
	let context = Context::create();
	let module = context.create_module("main");

	Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialize native target.");
	module.set_triple(&TargetMachine::get_default_triple());

	let target = Target::from_triple(&TargetMachine::get_default_triple()).unwrap();
	let target_machine = target.create_target_machine(
		&TargetMachine::get_default_triple(),
		"x86-64",
		"",
		OptimizationLevel::Aggressive,
		RelocMode::Static,
		CodeModel::Small
	).unwrap();

	let i32_type = context.i32_type();
	let i64_type = context.i64_type();
	let char_type = context.i8_type();
	let void_type = context.void_type();
	let fn_type = void_type.fn_type(&[], false);
	let function = module.add_function("main", fn_type, Option::from(Linkage::External));
	let entry_block = context.append_basic_block(function, "entry");

	let builder = context.create_builder();

	builder.position_at_end(entry_block);

	let cells = builder.build_array_malloc(char_type, i64_type.const_int(256, false), "").unwrap();
	let ptr = builder.build_alloca(i64_type, "");
	builder.build_store(ptr, i64_type.const_int(0, false));

	let putchar_type = i32_type.fn_type(&[char_type.into()], false);
	let putchar = module.add_function("putchar", putchar_type, Option::from(Linkage::External));

	let getchar_type = char_type.fn_type(&[], false);
	let getchar = module.add_function("getchar", getchar_type, Option::from(Linkage::External));

	for token in tokens.iter() {
		match token {
			Token::PtrRight => {
				let old_value = builder.build_load(ptr, "");
				let new_value = builder.build_int_add(old_value.into_int_value(), i64_type.const_int(1, false), "");
				builder.build_store(ptr, new_value);
			}
			Token::PtrLeft => {
				let old_value = builder.build_load(ptr, "");
				let new_value = builder.build_int_sub(old_value.into_int_value(), i64_type.const_int(1, false), "");
				builder.build_store(ptr, new_value);
			}
			Token::Inc => {
				let index = builder.build_load(ptr, "");
				let address = unsafe { builder.build_gep(cells, &[index.into_int_value().into()], "") };
				let value = builder.build_load(address, "");
				let new_value = builder.build_int_add(value.into_int_value(), char_type.const_int(1, false), "");
				builder.build_store(address, new_value);
			}
			Token::Dec => {
				let index = builder.build_load(ptr, "");
				let address = unsafe { builder.build_gep(cells, &[index.into_int_value().into()], "") };
				let value = builder.build_load(address, "");
				let new_value = builder.build_int_sub(value.into_int_value(), char_type.const_int(1, false), "");
				builder.build_store(address, new_value);
			}
			Token::Out => {
				let index = builder.build_load(ptr, "");
				let address = unsafe { builder.build_gep(cells, &[index.into_int_value().into()], "") };
				let value = builder.build_load(address, "");
				builder.build_call(putchar, &[value.into_int_value().into()], "");
			}
			Token::In => {
				let index = builder.build_load(ptr, "");
				let address = unsafe { builder.build_gep(cells, &[index.into_int_value().into()], "") };

				let call = builder.build_call(getchar, &[], "");
				builder.build_store(address, call.try_as_basic_value().unwrap_left().into_int_value());
			}
			Token::JmpPast => {
				let loop_block = context.append_basic_block(function, "");
				builder.build_unconditional_branch(loop_block);
				builder.position_at_end(loop_block);
			}
			Token::JmpBack => {
				let index = builder.build_load(ptr, "");
				let address = unsafe { builder.build_gep(cells, &[index.into_int_value().into()], "") };

				let value = builder.build_load(address, "").into_int_value();

				let comparison = builder.build_int_compare(IntPredicate::NE, value, char_type.const_int(0, false).into(), "");

				let after_block = context.append_basic_block(function, "");

				builder.build_conditional_branch(comparison, builder.get_insert_block().unwrap(), after_block);

				builder.position_at_end(after_block);
			}
		}
	}

	builder.build_return(None);

	let result = module.verify();
	if result.is_err() {
		println!("{}", result.unwrap_err().to_str().unwrap());
		panic!("LLVM module verification failed.");
	}
	module.write_bitcode_to_path(Path::new("output.bc"));
	target_machine.write_to_file(&module, FileType::Object, Path::new("output.o")).expect("failed to write result output to a file.");
}