use serde::Serialize;

macro_rules! expect {
    ($($value:expr),* $(,)?) => {
        {
            let args: Vec<String> = vec![
                $(
                    $value.to_string()
                ),*
            ];
            args
        }
    }
}

macro_rules! compare {
    ($deserialized:expr, [ $($serialized:expr),* $(,)? ]) => {
        assert_eq!(serialize($deserialized), expect!($($serialized),*))
    }
}

#[derive(Serialize)]
struct Flag {
    abc: bool,
}

fn serialize<T: Serialize>(value: T) -> Vec<String> {
    serde_command_line::to_vec(&value).unwrap()
}

#[test]
fn flag_true() {
    compare!(Flag { abc: true }, ["--abc"])
}

#[test]
fn flag_false() {
    compare!(Flag { abc: false }, [])
}

#[derive(Serialize)]
struct Optional {
    foo: Option<u64>,
}

#[test]
fn option_some() {
    compare!(Optional { foo: Some(32) }, ["--foo=32"])
}

#[test]
fn option_none() {
    compare!(Optional { foo: None }, [])
}

#[derive(Serialize)]
struct OptionalFlag {
    foo: Option<bool>,
}

#[test]
fn option_some_flag() {
    compare!(OptionalFlag { foo: Some(true) }, ["--foo"])
}

#[test]
fn option_none_flag() {
    compare!(OptionalFlag { foo: None }, [])
}

#[derive(Serialize)]
struct Multiple {
    a: u64,
    b: String,
}

#[test]
fn multiple() {
    compare!(
        Multiple {
            a: 42,
            b: String::from("test"),
        },
        ["--a=42", "--b=test"]
    )
}
