use crate::protocol::{Addr, RegVal};
use crate::types::{Annotated, Array, Object, Value};

/// Holds information about a single stacktrace frame.
#[derive(Clone, Debug, Default, PartialEq, Empty, FromValue, ToValue, ProcessValue)]
#[metastructure(process_func = "process_frame", value_type = "Frame")]
pub struct Frame {
    /// Name of the frame's function. This might include the name of a class.
    #[metastructure(max_chars = "symbol")]
    #[metastructure(skip_serialization = "empty")]
    pub function: Annotated<String>,

    /// Potentially mangled name of the symbol as it appears in an executable.
    ///
    /// This is different from a function name by generally being the mangled
    /// name that appears natively in the binary.  This is relevant for languages
    /// like Swift, C++ or Rust.
    #[metastructure(max_chars = "symbol")]
    pub symbol: Annotated<String>,

    /// Name of the module the frame is contained in.
    ///
    /// Note that this might also include a class name if that is something the
    /// language natively considers to be part of the stack (for instance in Java).
    #[metastructure(pii = "true")]
    #[metastructure(skip_serialization = "empty")]
    // TODO: Cap? This can be a FS path or a dotted path
    pub module: Annotated<String>,

    /// Name of the package that contains the frame.
    ///
    /// For instance this can be a dylib for native languages, the name of the jar
    /// or .NET assembly.
    #[metastructure(pii = "true")]
    #[metastructure(skip_serialization = "empty")]
    // TODO: Cap? This can be a FS path or a dotted path
    pub package: Annotated<String>,

    /// The source file name (basename only).
    #[metastructure(pii = "true", max_chars = "short_path")]
    #[metastructure(skip_serialization = "empty")]
    pub filename: Annotated<String>,

    /// Absolute path to the source file.
    #[metastructure(pii = "true", max_chars = "path")]
    #[metastructure(skip_serialization = "empty")]
    pub abs_path: Annotated<String>,

    /// Line number within the source file.
    pub lineno: Annotated<u64>,

    /// Column number within the source file.
    pub colno: Annotated<u64>,

    /// Source code leading up to the current line.
    #[metastructure(skip_serialization = "empty")]
    pub pre_context: Annotated<Array<String>>,

    /// Source code of the current line.
    pub context_line: Annotated<String>,

    /// Source code of the lines after the current line.
    #[metastructure(skip_serialization = "empty")]
    pub post_context: Annotated<Array<String>>,

    /// Override whether this frame should be considered in-app.
    pub in_app: Annotated<bool>,

    /// Local variables in a convenient format.
    #[metastructure(pii = "true")]
    #[metastructure(skip_serialization = "empty")]
    pub vars: Annotated<Object<Value>>,

    /// Start address of the containing code module (image).
    pub image_addr: Annotated<Addr>,

    /// Absolute address of the frame's CPU instruction.
    pub instruction_addr: Annotated<Addr>,

    /// Start address of the frame's function.
    pub symbol_addr: Annotated<Addr>,

    /// Used for native crashes to indicate how much we can "trust" the instruction_addr
    #[metastructure(max_chars = "enumlike")]
    pub trust: Annotated<String>,

    /// Additional arbitrary fields for forwards compatibility.
    #[metastructure(additional_properties)]
    pub other: Object<Value>,
}

/// Holds information about an entirey stacktrace.
#[derive(Clone, Debug, Default, PartialEq, Empty, FromValue, ToValue, ProcessValue)]
#[metastructure(process_func = "process_stacktrace", value_type = "Stacktrace")]
pub struct Stacktrace {
    #[metastructure(required = "true", nonempty = "true")]
    pub frames: Annotated<Array<Frame>>,

    /// Register values of the thread (top frame).
    pub registers: Annotated<Object<RegVal>>,

    /// Additional arbitrary fields for forwards compatibility.
    #[metastructure(additional_properties)]
    pub other: Object<Value>,
}

#[test]
fn test_frame_roundtrip() {
    let json = r#"{
  "function": "main",
  "symbol": "_main",
  "module": "app",
  "package": "/my/app",
  "filename": "myfile.rs",
  "abs_path": "/path/to",
  "lineno": 2,
  "colno": 42,
  "pre_context": [
    "fn main() {"
  ],
  "context_line": "unimplemented!()",
  "post_context": [
    "}"
  ],
  "in_app": true,
  "vars": {
    "variable": "value"
  },
  "image_addr": "0x400",
  "instruction_addr": "0x404",
  "symbol_addr": "0x404",
  "trust": "69",
  "other": "value"
}"#;
    let frame = Annotated::new(Frame {
        function: Annotated::new("main".to_string()),
        symbol: Annotated::new("_main".to_string()),
        module: Annotated::new("app".to_string()),
        package: Annotated::new("/my/app".to_string()),
        filename: Annotated::new("myfile.rs".to_string()),
        abs_path: Annotated::new("/path/to".to_string()),
        lineno: Annotated::new(2),
        colno: Annotated::new(42),
        pre_context: Annotated::new(vec![Annotated::new("fn main() {".to_string())]),
        context_line: Annotated::new("unimplemented!()".to_string()),
        post_context: Annotated::new(vec![Annotated::new("}".to_string())]),
        in_app: Annotated::new(true),
        vars: {
            let mut vars = Object::new();
            vars.insert(
                "variable".to_string(),
                Annotated::new(Value::String("value".to_string())),
            );
            Annotated::new(vars)
        },
        image_addr: Annotated::new(Addr(0x400)),
        instruction_addr: Annotated::new(Addr(0x404)),
        symbol_addr: Annotated::new(Addr(0x404)),
        trust: Annotated::new("69".into()),
        other: {
            let mut vars = Object::new();
            vars.insert(
                "other".to_string(),
                Annotated::new(Value::String("value".to_string())),
            );
            vars
        },
    });

    assert_eq_dbg!(frame, Annotated::from_json(json).unwrap());
    assert_eq_str!(json, frame.to_json_pretty().unwrap());
}

#[test]
fn test_frame_default_values() {
    let json = "{}";
    let frame = Annotated::new(Frame::default());

    assert_eq_dbg!(frame, Annotated::from_json(json).unwrap());
    assert_eq_str!(json, frame.to_json_pretty().unwrap());
}

#[test]
fn test_stacktrace_roundtrip() {
    let json = r#"{
  "frames": [
    {
      "function": "foobar"
    }
  ],
  "registers": {
    "cspr": "0x20000000",
    "lr": "0x18a31aadc",
    "pc": "0x18a310ea4",
    "sp": "0x16fd75060"
  },
  "other": "value"
}"#;
    let stack = Annotated::new(Stacktrace {
        frames: Annotated::new(vec![Annotated::new(Frame {
            function: Annotated::new("foobar".to_string()),
            ..Default::default()
        })]),
        registers: {
            let mut registers = Object::new();
            registers.insert("cspr".to_string(), Annotated::new(RegVal(0x2000_0000)));
            registers.insert("lr".to_string(), Annotated::new(RegVal(0x1_8a31_aadc)));
            registers.insert("pc".to_string(), Annotated::new(RegVal(0x1_8a31_0ea4)));
            registers.insert("sp".to_string(), Annotated::new(RegVal(0x1_6fd7_5060)));
            Annotated::new(registers)
        },
        other: {
            let mut other = Object::new();
            other.insert(
                "other".to_string(),
                Annotated::new(Value::String("value".to_string())),
            );
            other
        },
    });

    assert_eq_dbg!(stack, Annotated::from_json(json).unwrap());
    assert_eq_str!(json, stack.to_json_pretty().unwrap());
}

#[test]
fn test_stacktrace_default_values() {
    // This needs an empty frame because "frames" is required
    let json = r#"{
  "frames": [
    {}
  ]
}"#;

    let stack = Annotated::new(Stacktrace {
        frames: Annotated::new(vec![Annotated::new(Frame::default())]),
        ..Default::default()
    });

    assert_eq_dbg!(stack, Annotated::from_json(json).unwrap());
    assert_eq_str!(json, stack.to_json_pretty().unwrap());
}

#[test]
fn test_stacktrace_invalid() {
    use crate::types::ErrorKind;

    let stack = Annotated::new(Stacktrace {
        frames: Annotated::from_error(ErrorKind::MissingAttribute, None),
        registers: Annotated::empty(),
        other: Default::default(),
    });

    assert_eq_dbg!(stack, Annotated::from_json("{}").unwrap());
}

#[test]
fn test_frame_vars_null_preserved() {
    let json = r#"{
  "vars": {
    "despacito": null
  }
}"#;
    let frame = Annotated::new(Frame {
        vars: Annotated::new({
            let mut vars = Object::new();
            vars.insert("despacito".to_string(), Annotated::new(Value::Null));
            vars
        }),
        ..Default::default()
    });

    assert_eq_dbg!(Annotated::from_json(json).unwrap(), frame);
    assert_eq_str!(json, frame.to_json_pretty().unwrap());
}

#[test]
fn test_frame_vars_empty_annotated_is_serialized() {
    let output = r#"{
  "vars": {
    "despacito": null,
    "despacito2": null
  }
}"#;
    let frame = Annotated::new(Frame {
        vars: Annotated::new({
            let mut vars = Object::new();
            vars.insert("despacito".to_string(), Annotated::new(Value::Null));
            vars.insert("despacito2".to_string(), Annotated::empty());
            vars
        }),
        ..Default::default()
    });

    assert_eq_str!(output, frame.to_json_pretty().unwrap());
}

#[test]
fn test_frame_empty_context_lines() {
    let json = r#"{
  "pre_context": [
    ""
  ],
  "context_line": "",
  "post_context": [
    ""
  ]
}"#;

    let frame = Annotated::new(Frame {
        pre_context: Annotated::new(vec![Annotated::new("".to_string())]),
        context_line: Annotated::new("".to_string()),
        post_context: Annotated::new(vec![Annotated::new("".to_string())]),
        ..Frame::default()
    });

    assert_eq_dbg!(frame, Annotated::from_json(json).unwrap());
    assert_eq_str!(json, frame.to_json_pretty().unwrap());
}
