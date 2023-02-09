use serde::{Serialize};

#[derive(Serialize)]
enum EnumOne {
  A(EnumTwo),
}

#[derive(Serialize)]
enum EnumTwo {
  Z(StructOne),
}

#[derive(Serialize)]
struct StructOne {
}

fn main() {
  let obj = EnumOne::A(
    EnumTwo::Z (
      StructOne {
      }
    )
  );
  serde_xml_rs::to_string(&obj).unwrap();

}
