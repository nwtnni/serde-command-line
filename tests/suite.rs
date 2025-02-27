#![allow(clippy::disallowed_names)]

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
    ($deserialized:expr, [ $($serialized:expr),* $(,)? ] $(,)?) => {
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

#[derive(Serialize)]
struct UnitEnumOuter {
    b: UnitEnum,
}

#[derive(Serialize)]
enum UnitEnum {
    A,
    B,
}

#[test]
fn enum_unit() {
    compare!(UnitEnumOuter { b: UnitEnum::A }, ["--b=A"]);
    compare!(UnitEnumOuter { b: UnitEnum::B }, ["--b=B"]);
}

#[derive(Serialize)]
struct Command {
    path: String,
    subcommand: Subcommand,
}

#[derive(Serialize)]
enum Subcommand {
    Load {},
    Run { time: i64 },
}

#[test]
fn subcommand_empty() {
    compare!(
        Command {
            path: String::from("a/b"),
            subcommand: Subcommand::Load {}
        },
        ["--path=a/b", "Load"]
    );
}

#[test]
fn subcommand_field() {
    compare!(
        Command {
            path: String::from("a/b"),
            subcommand: Subcommand::Run { time: 2 }
        },
        ["--path=a/b", "Run", "--time=2"]
    );
}
