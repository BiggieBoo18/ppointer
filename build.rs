fn main() {
    cc::Build::new()
	.cpp(true)
	.warnings(true)
	.flag("-O1")
	.flag("-Wall")
	// .file("ppointer.c")
	.file("ppointer.cpp")
	.compile("ppointer");
}
