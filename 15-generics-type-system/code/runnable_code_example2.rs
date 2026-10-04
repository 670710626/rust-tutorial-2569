use std::any::type_name;
use std::mem::size_of;

// Generic Function สำหรับตรวจสอบตำแหน่งโค้ดในหน่วยความจำ
fn inspect_execution<T: std::fmt::Display>(val: T) {
    println!(
        "Type: {:<5} | Value: {:<5} | Function Address: {:p}",
        type_name::<T>(),
        val,
        inspect_execution::<T> as *const ()
    );
}

// Generic Struct เพื่อตรวจสอบขนาดหน่วยความจำจริง
struct Storage<T> {
    data: T,
}

impl<T> Storage<T> {

    fn get(&self) -> &T {
        &self.data
    }

    fn byte_size(&self) -> usize {
        size_of::<T>()
    }
}

fn main() {
    println!("=== 1. Function Pointer Monomorphization ===");
    // เรียกใช้ Type เดียวกันซ้ำ (i32)
    inspect_execution(100_i32);
    inspect_execution(200_i32);

    // เรียกใช้ Type อื่นๆ (f64 และ bool)
    inspect_execution(3.14_f64);
    inspect_execution(true);

    println!("\n=== 2. Struct Memory Layout ===");
    let int_store = Storage { data: 42_i32 };
    let byte_store = Storage { data: 42_u8 };
    let float_store = Storage { data: 42.0_f64 };

    println!(
        "Storage<i32> holds value: {:<4} | consumes: {} bytes",
        int_store.get(),
        int_store.byte_size()
    );
    println!(
        "Storage<u8>  holds value: {:<4} | consumes: {} bytes",
        byte_store.get(),
        byte_store.byte_size()
    );
    println!(
        "Storage<f64> holds value: {:<4} | consumes: {} bytes",
        float_store.get(),
        float_store.byte_size()
    );
}