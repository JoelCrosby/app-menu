fn main() {
    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "composite_templates.gresource",
    );
}
