# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

# TODO(cjhopman): This was generated by scripts/hacks/rules_shim_with_docs.py,
# but should be manually edited going forward. There may be some errors in
# the generated docs, and so those should be verified to be accurate and
# well-formatted (and then delete this TODO)

def validate_uri(_s):
    return True

prelude_rule = record(
    name = field(str),
    docs = field([str, None], None),
    examples = field([str, None], None),
    further = field([str, None], None),
    attrs = field(dict[str, Attr]),
    impl = field([typing.Callable, None], None),
    uses_plugins = field([list[plugins.PluginKind], None], None),
    supports_incoming_transition = field([bool, None], None),
    is_toolchain_rule = field([bool, None], None),
    # TODO(mgd): should be `transition | None`, but `transition` does not work as type.
    cfg = field(typing.Any | None, None),
)

AbiGenerationMode = ["unknown", "class", "source", "migrating_to_source_only", "source_only", "unrecognized"]

AnnotationProcessingTool = ["kapt", "javac"]

CxxRuntimeType = ["dynamic", "static"]

CxxSourceType = ["c", "cxx", "cxx_thinlink", "objc", "objcxx", "cuda", "hip", "swift", "c_cpp_output", "cxx_cpp_output", "objc_cpp_output", "objcxx_cpp_output", "cuda_cpp_output", "hip_cpp_output", "assembler_with_cpp", "assembler", "asm_with_cpp", "asm", "pcm"]

ForkMode = ["none", "per_test"]

HeadersAsRawHeadersMode = ["required", "preferred", "disabled"]

IncludeType = ["local", "system", "raw"]

LinkableDepType = ["static", "static_pic", "shared"]

LogLevel = ["off", "severe", "warning", "info", "config", "fine", "finer", "finest", "all"]

OnDuplicateEntry = ["fail", "overwrite", "append"]

RawHeadersAsHeadersMode = ["enabled", "disabled"]

SourceAbiVerificationMode = ["off", "log", "fail"]

TestType = ["junit", "junit5", "testng"]

UnusedDependenciesAction = ["unknown", "fail", "warn", "ignore", "unrecognized"]

RuntimeDependencyHandling = ["none", "symlink_single_level_only", "symlink"]

def _name_arg(name_type):
    return {
        "name": name_type,
    }

def _deps_query_arg():
    return {
        "deps_query": attrs.option(attrs.query(), default = None, doc = """
    Status: **experimental/unstable**.
     The deps query takes a query string that accepts the following query
     functions, and appends the output of the query to the declared deps:

    * `attrfilter`
    * `deps`
    * `except`
    * `intersect`
    * `filter`
    * `kind`
    * `set`
    * `union`

    Some example queries:

    ```
      "filter({name_regex}, deps('//foo:foo'))".format(name_regex='//.*')
      "attrfilter(annotation_processors, com.foo.Processor, deps('//foo:foo'))"
      "deps('//foo:foo', 1)"
    ```
"""),
    }

def _provided_deps_query_arg():
    return {
        "provided_deps_query": attrs.option(attrs.query(), default = None, doc = """
    Status: **experimental/unstable**.
     The provided deps query functions in the same way as the deps query, but the
     results of the query are appended to the declared provided deps.
"""),
    }

def _platform_deps_arg():
    return {
        "platform_deps": attrs.list(attrs.tuple(attrs.regex(), attrs.set(attrs.dep(), sorted = True)), default = [], doc = """
    Platform specific dependencies.
     These should be specified as a list of pairs where the first element is an
     un-anchored regex (in java.util.regex.Pattern syntax) against which the
     platform name is matched, and the second element is a list of
     dependencies (same format as `deps`) that are exported
     if the platform matches the regex.
     See `deps` for more information.
"""),
    }

def _labels_arg():
    return {
        "labels": attrs.list(attrs.string(), default = [], doc = """
    Set of arbitrary strings which allow you to annotate a `build rule` with tags
     that can be searched for over an entire dependency tree using `buck query()`
    .
"""),
    }

def _visibility_arg(visibility_type):
    return {
        "visibility": visibility_type,
    }

def _tests_arg(tests_type):
    return {
        "tests": tests_type,
    }

def _tests_apple_arg(tests_type):
    return {
        "tests": tests_type,
    }

def _test_label_arg():
    return {
        "labels": attrs.list(attrs.string(), default = [], doc = """
    A list of labels to be applied to these tests. These labels are
     arbitrary text strings and have no meaning within buck itself. They
     can, however, have meaning for you as a test author
     (e.g., `smoke` or `fast`). A label can be
     used to filter or include a specific test rule
     when executing `buck test`
"""),
    }

def _run_test_separately_arg(run_test_separately_type):
    return {
        "run_test_separately": run_test_separately_type,
    }

def _fork_mode():
    return {
        "fork_mode": attrs.enum(ForkMode, default = "none", doc = """
    Controls whether tests will all be run in the same process or a process will be
     started for each set of tests in a class.

     (This is mainly useful when porting Java tests to Buck from Apache Ant which
     allows JUnit tasks to set a `fork="yes"` property. It should not be
     used for new tests since it encourages tests to not cleanup after themselves and
     increases the tests' computational resources and running time.)


    `none`
    All tests will run in the same process.
    `per_test`
    A process will be started for each test class in which all tests of that test class
     will run.
"""),
    }

def _test_rule_timeout_ms():
    return {
        "test_rule_timeout_ms": attrs.option(attrs.int(), default = None, doc = """
    If set specifies the maximum amount of time (in milliseconds) in which all of the tests in this
     rule should complete. This overrides the default `rule_timeout` if any has been
     specified in `.buckconfig`
    .
"""),
    }

def _target_os_type_arg() -> Attr:
    return attrs.default_only(attrs.dep(default = "prelude//os_lookup/targets:os_lookup"))

def _exec_os_type_arg() -> Attr:
    return attrs.default_only(attrs.exec_dep(default = "prelude//os_lookup/targets:os_lookup"))

def _allow_cache_upload_arg():
    return {
        "allow_cache_upload": attrs.option(
            attrs.bool(),
            default = None,
            doc = """
            Whether to allow uploading the output of this rule to be uploaded
            to cache when the action is executed locally if the configuration
            allows (i.e. there is a cache configured and the client has
            permission to write to it).
            """,
        ),
    }

def _inject_test_env_arg():
    return {
        # NOTE: We make this a `dep` not an `exec_dep` even though we'll execute
        # it, because it needs to execute in the same platform as the test itself
        # (we run tests in the target platform not the exec platform, since the
        # goal is to test the code that is being built!).
        "_inject_test_env": attrs.default_only(attrs.dep(default = "prelude//test/tools:inject_test_env")),
    }

buck = struct(
    name_arg = _name_arg,
    deps_query_arg = _deps_query_arg,
    exec_os_type_arg = _exec_os_type_arg,
    provided_deps_query_arg = _provided_deps_query_arg,
    platform_deps_arg = _platform_deps_arg,
    labels_arg = _labels_arg,
    visibility_arg = _visibility_arg,
    tests_arg = _tests_arg,
    tests_apple_arg = _tests_apple_arg,
    test_label_arg = _test_label_arg,
    run_test_separately_arg = _run_test_separately_arg,
    fork_mode = _fork_mode,
    test_rule_timeout_ms = _test_rule_timeout_ms,
    target_os_type_arg = _target_os_type_arg,
    allow_cache_upload_arg = _allow_cache_upload_arg,
    inject_test_env_arg = _inject_test_env_arg,
)
