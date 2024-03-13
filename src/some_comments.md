# Review of Part 2

Tuples, Structs,Arrays

Arrays can be iterated over
Tuples an structs cannot
Array elements must all have the same type
Tuple and struct fileds can be different types

In memory, they're all represented as adjacent bytes with no extra metadata

tuples          let foo:(i64, bool) = (1, true);
structs         struct Foo {x: i64, is_up: bool}
arrays          let arr: [u32; 3] = [1,2,3]
memory          u8 is 8 bits(1 byte), u16 is 16bits(2 bytes),...
