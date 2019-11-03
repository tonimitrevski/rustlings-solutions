// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct<'a, 'b> {
    name: &'a str,
    hex: &'b str,
}

struct ColorTupleStruct<'a, 'b>(&'a str, &'b str);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let name = "green";
        let hex = "#00FF00";
         let green: ColorClassicStruct = ColorClassicStruct { name, hex };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // For more fun, use the field initialization shorthand.
         let green: ColorTupleStruct = ColorTupleStruct("green", "#00FF00");

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
         let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
