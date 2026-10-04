# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 15
> **Topic No.:** 15
> **Topic Name:** Generics & Type System
> **ประเด็นหลักที่ควรครอบคลุม:** generic functions, generic structs, type parameters, monomorphization เบื้องต้น

---

## 1. Members

| # | Name | Student ID | Email | GitHub Username | Main Responsibility |
|---|---|---|---|---|---|
| 1 | นายณัฐวุฒิ โตเมือง | 670710623 | tomueang_n@silpakorn.edu | `@670710623` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวณัฐสุดา ลานตวน | 670710624 | lantuan_n@silpakorn.edu | `@670710624` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นางสาวณัฐสุรางค์ ชาติทองคำ | 670710625 | chatthongkham_n@silpakorn.edu | `@670710625` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายธนเทพ นาสวน | 670710626 | nasuan_t@silpakorn.edu | `@670710626` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |


---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. อธิบายแนวคิดของ Generics และ Generic Type Parameters ในภาษา Rust
   และบอกได้ว่า Generics ช่วยลดการเขียนโค้ดซ้ำและเพิ่มความสามารถในการนำโค้ดกลับมาใช้ซ้ำอย่างไร

2. เขียนและอ่านโค้ด Rust ที่ใช้ Generic Functions, Structs และ Enums
   พร้อมอธิบายบทบาทของ Type Parameter เช่น `T` และการที่ Compiler อนุมานชนิดข้อมูลจากบริบทได้

3. อธิบายความสัมพันธ์ระหว่าง Generics, Traits และ Trait Bounds
   รวมถึงเหตุผลที่ Rust ใช้ Trait เพื่อกำหนดความสามารถที่ Generic Type ต้องมี

4. เชื่อมโยงการใช้ Generics และ Type System ของ Rust
   กับแนวคิดด้าน Programming Languages เช่น Type Safety, Polymorphism และการตรวจสอบชนิดข้อมูลใน Compile Time


---

## 3. Introduction

ในการเขียนโปรแกรม เรามักพบกรณีที่ต้องใช้โค้ดแบบเดียวกันกับข้อมูลหลายชนิด เช่น ฟังก์ชันที่ต้องทำงานกับตัวเลขหลายชนิด หรือโครงสร้างข้อมูลที่ต้องเก็บข้อมูลมากกว่าหนึ่งชนิด หากเขียนโค้ดแยกสำหรับแต่ละชนิดข้อมูล จะทำให้เกิดการเขียนโค้ดซ้ำ และทำให้การแก้ไขโปรแกรมในภายหลังทำได้ยากขึ้น

Generics ในภาษา Rust เป็นแนวทางที่ช่วยแก้ปัญหานี้ โดยใช้ Generic Type Parameter เช่น `T` เป็นตัวแทนของชนิดข้อมูลที่ยังไม่ระบุแบบตายตัว ทำให้สามารถเขียน Function, Struct หรือ Enum เพียงชุดเดียวและนำไปใช้กับหลายชนิดข้อมูลได้ โดยยังคงให้ Compiler ตรวจสอบความถูกต้องของชนิดข้อมูลก่อนโปรแกรมทำงาน

นอกจาก Generics แล้ว Rust ยังใช้ Traits เพื่ออธิบายพฤติกรรมที่ Type หนึ่งต้องมี และสามารถใช้ Trait Bounds เพื่อกำหนดว่า Generic Type จะต้องมีความสามารถใดบ้าง ก่อนที่จะนำไปใช้กับการดำเนินการหรือ Method ที่เกี่ยวข้อง แนวคิดนี้ทำให้การเขียน Generic Code มีความยืดหยุ่นโดยยังคงมีข้อกำหนดด้าน Type ที่ชัดเจน

ดังนั้น Generics และ Type System จึงเป็นส่วนสำคัญของการออกแบบภาษา Rust เพราะช่วยให้เราสร้างโค้ดที่นำกลับมาใช้ได้หลายกรณี โดยให้ Compiler ตรวจสอบข้อผิดพลาดด้าน Type ตั้งแต่ Compile Time และช่วยให้ความสัมพันธ์ระหว่างข้อมูลกับพฤติกรรมของโปรแกรมถูกกำหนดไว้อย่างชัดเจน

---

## 4. Key Concepts

### 4.1 Generics and Type Parameters

**คำอธิบาย**

Generics คือแนวทางในการเขียนโค้ดโดยใช้ตัวแทนของชนิดข้อมูลแทนการกำหนด
ชนิดข้อมูลแบบตายตัว เช่น `T` หรือ `U` ทำให้โค้ดชุดเดียวสามารถนำไปใช้กับ
ข้อมูลหลายชนิดได้

ในภาษา Rust Generic Type Parameter สามารถใช้กับหลายองค์ประกอบของภาษา
เช่น Functions, Structs, Enums และ Implementations โดยชนิดจริงของ `T`
จะถูกกำหนดเมื่อมีการนำ Generic นั้นไปใช้งาน

แนวคิดนี้ช่วยลดการเขียนโค้ดที่มีตรรกะเหมือนกันซ้ำหลายครั้ง โดยยังคงให้
Compiler ตรวจสอบชนิดข้อมูลของโปรแกรมตามกฎของ Type System ของ Rust

**ตัวอย่าง**

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 4.0 };

    println!("Integer point: {:?}", integer_point);
    println!("Float point: {:?}", float_point);
}
```
**Expected Output**

```
Integer point: Point { x: 5, y: 10 }
Float point: Point { x: 1.5, y: 4.0 }
```

**Explanation**

ในตัวอย่างนี้ `Point<T>` เป็น Struct ที่ใช้ Generic Type Parameter `T`
เพื่อให้สามารถเก็บข้อมูลประเภทเดียวกันได้หลายชนิด โดย `x` และ `y`
จะมีชนิดข้อมูลเดียวกับ `T`

เมื่อสร้าง `integer_point` ด้วยค่า `5` และ `10` ซึ่งเป็นจำนวนเต็ม
Compiler สามารถอนุมานได้ว่า `T` คือ `i32` ทำให้ตัวแปรนี้มีชนิดเป็น
`Point<i32>`

ส่วน `float_point` มีค่า `1.5` และ `4.0` ซึ่งเป็น Floating-Point
Compiler จึงอนุมานว่า `T` คือ `f64` และตัวแปรนี้มีชนิดเป็น `Point<f64>`

ดังนั้น Struct `Point<T>` เพียงรูปแบบเดียวสามารถนำไปใช้กับข้อมูลต่างชนิดกันได้
โดยไม่ต้องสร้าง Struct แยกสำหรับแต่ละชนิดข้อมูล

---

### 4.2 Generic Functions

**คำอธิบาย**

Generic Function คือฟังก์ชันที่สามารถรับหรือคืนค่าที่มีชนิดข้อมูลแบบ Generic
โดยใช้ Type Parameter เช่น `T` แทนการกำหนดชนิดข้อมูลแบบตายตัว

การประกาศ Generic Function จะระบุ Type Parameter ไว้หลังชื่อฟังก์ชัน
เช่น `fn identity<T>(value: T) -> T` โดย `T` ในตัวอย่างนี้สามารถแทน
ชนิดข้อมูลที่แตกต่างกันได้ ทำให้ไม่จำเป็นต้องสร้างฟังก์ชันแยกสำหรับแต่ละ Type

แนวคิดนี้ช่วยลดการเขียนโค้ดซ้ำ โดยยังคงเป็นฟังก์ชันที่มีชนิดข้อมูลชัดเจน
ตาม Type System ของ Rust

**ตัวอย่าง**

```rust
fn identity<T>(value: T) -> T {
    value
}

fn main() {
    let number = identity(10);
    let text = identity("Rust");

    println!("Number: {}", number);
    println!("Text: {}", text);
}
```

---

### 4.3 Generic Structs and Enums

**คำอธิบาย**

นอกจาก Function แล้ว Generics ยังสามารถใช้กับ Struct และ Enum ได้
โดยกำหนด Generic Type Parameter เช่น `T` เพื่อให้โครงสร้างข้อมูลเดียว
สามารถรองรับข้อมูลหลายชนิดได้

สำหรับ Struct เราสามารถกำหนด `T` ให้กับ field เพื่อระบุว่าข้อมูลภายใน
สามารถเป็นชนิดใดก็ได้ตามที่กำหนดเมื่อสร้าง Struct

ส่วน Enum สามารถใช้ Generic Type Parameter เพื่อกำหนดชนิดข้อมูลที่เก็บอยู่ในแต่ละ variant ได้ ตัวอย่างเช่น `Option<T>` ซึ่งใช้แทนกรณีที่ค่าหนึ่งอาจมีข้อมูลหรือไม่มีข้อมูล โดย Type ของ `T` จะถูกกำหนดเมื่อมีการใช้งาน Enum

**ตัวอย่าง**

```rust
#[derive(Debug)]
struct Container<T> {
    value: T,
}

#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    let number = Container { value: 100 };
    let text = Container {
        value: "Rust",
    };

    let some_number = MyOption::Some(42);
    let no_value: MyOption<i32> = MyOption::None;

    println!("Number container: {:?}", number);
    println!("Text container: {:?}", text);
    println!("Some value: {:?}", some_number);
    println!("No value: {:?}", no_value);
}
```

---

### 4.4 Traits as Shared Behavior

**คำอธิบาย**

Trait คือการกำหนดพฤติกรรม (Behavior) ที่ Type สามารถนำไปใช้งานร่วมกันได้ โดยภายใน Trait สามารถประกาศ Method ที่ Type ที่นำ Trait ไปใช้ต้องกำหนดการทำงานให้

แนวคิดของ Trait ช่วยให้เราสามารถกำหนด Interface ของพฤติกรรมร่วมกัน โดยไม่จำเป็นต้องกำหนดว่า Type นั้นต้องเป็น Struct หรือ Enum ชนิดใด

```rust
trait Describe {
    fn describe(&self) -> String;
}

struct User {
    name: String,
}

impl Describe for User {
    fn describe(&self) -> String {
        format!("User: {}", self.name)
    }
}

fn main() {
    let user = User {
        name: String::from("Alice"),
    };

    println!("{}", user.describe());
}
```

---

### 4.5 Trait Bounds

**คำอธิบาย**

Trait Bound คือการกำหนดเงื่อนไขให้ Generic Type ว่า Type ที่นำมาใช้งานต้องมี Trait ที่กำหนดไว้ก่อน จึงจะสามารถใช้ความสามารถของ Trait นั้นภายในฟังก์ชันได้

Trait Bound สามารถเขียนในรูปแบบ T: Trait เพื่อระบุว่า Generic Type T ต้อง Implement Trait ที่กำหนด

ตัวอย่าง**

```rust
use std::fmt::Display;

fn print_value<T: Display>(value: T) {
    println!("Value: {}", value);
}

fn main() {
    print_value(100);
    print_value("Rust");
}
```
**Monomorphization เบื้องต้น**

เมื่อ Generic Code ถูกนำไปใช้งานกับ Concrete Type ต่าง ๆ Compiler ของ Rust สามารถสร้างโค้ดที่เหมาะกับแต่ละ Type ที่ใช้งานจริงในขั้นตอน Compile Time แนวทางนี้เรียกว่า Monomorphization ซึ่งเป็นส่วนหนึ่งที่ช่วยให้ Generic Code ของ Rust สามารถนำกลับมาใช้ซ้ำได้โดยไม่จำเป็นต้องเลือกชนิดข้อมูลแบบ Dynamic ระหว่าง Runtime

---

## 5. Important Syntax / Rules

| Syntax / Rule         | Meaning                                                        | Example                                  |
| --------------------- | -------------------------------------------------------------- | ---------------------------------------- |
| `<T>`                 | ใช้ประกาศ Generic Type Parameter                               | `fn identity<T>(value: T) -> T`          |
| `Struct<T>`           | ใช้กำหนด Struct ให้รองรับ Generic Type                         | `struct Container<T> { value: T }`       |
| `Enum<T>`             | ใช้กำหนด Enum ให้รองรับ Generic Type                           | `enum MyOption<T> { Some(T), None }`     |
| `impl Trait for Type` | ใช้กำหนดให้ Type นั้น Implement Trait                          | `impl Describe for User`                 |
| `T: Trait`            | กำหนดว่า Generic Type `T` ต้องมี Trait ที่ระบุ                 | `fn print<T: Display>(value: T)`         |
| `where T: ...`        | ใช้เขียน Trait Bound แยกออกจากส่วนหัวของ Function              | `fn check<T>(value: T) where T: Display` |
| `#[derive(...)]`      | ให้ Compiler สร้าง Implementation ของ Trait บางตัวให้อัตโนมัติ | `#[derive(Debug)]`                       |
| `impl Trait` | ใช้ระบุ Trait ที่ Type ต้อง Implement โดยใน Return Position สามารถใช้เป็น Opaque Type ได้ | `fn make() -> impl Display` |
| `dyn Trait` | ใช้สร้าง Trait Object สำหรับ Dynamic Dispatch | `let value = 10; let x: &dyn Display = &value;` |

### Important Rules

1. Generic Type ต้องถูกระบุหรือสามารถอนุมานได้
Rust ต้องรู้ว่า Generic Type เช่น T เป็น Type อะไรจากค่าที่ใช้งาน หรือจากการระบุ Type โดยตรง
2. Trait Bound ต้องตรงกับความสามารถที่ต้องการใช้
ถ้า Function ต้องเรียกใช้ความสามารถจาก Trait ใด Generic Type ที่นำมาใช้ก็ต้องมี Trait นั้น เช่น T: Display เมื่อ Function ต้องการแสดงค่าด้วย Display
3. Type ที่ Implement Trait ต้องกำหนดพฤติกรรมของ Trait ให้ครบตามที่ Trait ต้องการ
เช่น impl Describe for User ต้องกำหนดการทำงานของ describe() ให้กับ User
4. where เป็นอีกวิธีในการเขียน Trait Bound
เหมาะกับกรณีที่มีเงื่อนไขหลายตัว เพราะทำให้ส่วนหัวของ Function อ่านง่ายขึ้น โดย Draft ของกลุ่มยกตัวอย่าง where T: PartialOrd + Display ไว้

---

## 6. Runnable Code Examples

### Example 1 — `Generic Functions and Structs with Type Parameters`

**Purpose:** `สาธิตการสร้าง Generic Struct ที่มี Type Parameters หลายตัว (<K, V>), การทำ Specialized Method และการสร้าง Generic Function ที่ใช้ Trait Bounds (PartialOrd, Copy) เพื่อทำงานกับข้อมูลหลายชนิดได้อย่างปลอดภัย`

```rust
// 1. Generic Struct ที่มี 2 Type Parameters
#[derive(Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

// Implementation ทั่วไป: ใช้งานได้กับ KeyValue ทุกประเภท
impl<K, V> KeyValue<K, V> {
    fn new(key: K, value: V) -> Self {
        KeyValue { key, value }
    }

    fn unpack(self) -> (K, V) {
        (self.key, self.value)
    }
}

// Specialized Implementation: จำกัดเฉพาะกรณีที่ V เป็น f64 เท่านั้น
impl<K> KeyValue<K, f64> {
    fn is_positive_metric(&self) -> bool {
        self.value > 0.0
    }
}

// 2. Generic Function with Trait Bounds
// T ต้องเปรียบเทียบค่าได้ (PartialOrd) และคัดลอกค่าระดับบิตได้ (Copy)
fn find_min<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }

    let mut min = list[0];
    for &item in list.iter().skip(1) {
        if item < min {
            min = item;
        }
    }
    Some(min)
}

fn main() {
    // --- ใช้งาน Generic Struct ---
    let sensor = KeyValue::new("Temperature", 36.6);
    println!("Sensor Record: {:?}", sensor);
    println!("Is positive metric: {}", sensor.is_positive_metric());

    let user_flag = KeyValue::new(1001, true);
    println!("User Flag: {:?}", user_flag);
    let (uid, flag) = user_flag.unpack();
    println!("Unpacked -> UID: {}, Status: {}", uid, flag);

    // --- ใช้งาน Generic Function ---
    let scores = [85, 92, 78, 90];
    let temps = [36.6, 37.2, 35.8, 38.0];

    // คอมไพเลอร์ทำ Type Inference ให้อัตโนมัติ (i32 และ f64)
    println!("Min Score: {:?}", find_min(&scores));
    println!("Min Temp : {:?}", find_min(&temps));
}
```

**Expected Output**

```text
Sensor Record: KeyValue { key: "Temperature", value: 36.6 }
Is positive metric: true
User Flag: KeyValue { key: 1001, value: true }
Unpacked -> UID: 1001, Status: true
Min Score: Some(78)
Min Temp : Some(35.8)
```

**Explanation**

1. struct KeyValue<K, V>: กำหนด Type Parameters สองตัวที่ไม่จำเป็นต้องเหมือนกัน เช่น &str กับ f64 หรือ i32 กับ bool
2. impl<K, V> vs impl<K> KeyValue<K, f64>:
   - บล็อกแรกทำให้ new และ unpack ใช้ได้กับทุก Type
   - บล็อกที่สองจำกัดให้เรียก is_positive_metric() ได้เฉพาะกรณีที่ V เป็น f64 เท่านั้น หากนำ user_flag ไปเรียกจะเกิด Compile Error ทันที
3. find_min<T: PartialOrd + Copy>: การใช้ Trait Bounds บังคับว่า Type T ใดๆ ที่ส่งเข้ามาต้องสามารถนำมาเปรียบเทียบด้วยเครื่องหมาย < ได้ (PartialOrd) และสามารถคัดลอกค่าออกจาก Reference เพื่อเก็บในตัวแปร min ได้ (Copy)

---

### Example 2 — `Verification of Monomorphization & Type System Mechanics`

**Purpose:** `พิสูจน์กระบวนการ Monomorphization ผ่านการตรวจสอบตำแหน่ง Function Pointer และขนาดหน่วยความจำ (Memory Layout) เพื่อแสดงว่า Rust สร้าง Machine Code และ Struct ที่เฉพาะเจาะจงแยกตาม Type จริงในขั้นตอนคอมไพล์`

```rust
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
```

**Expected Output**
(หมายเหตุ: แอดเดรสฐาน 0x... จะเปลี่ยนไปตามการจัดสรร Memory ของ OS แต่ความสัมพันธ์ของตำแหน่งจะเหมือนกัน)
```text
=== 1. Function Pointer Monomorphization ===
Type: i32   | Value: 100   | Function Address: 0x7ff6521911e0
Type: i32   | Value: 200   | Function Address: 0x7ff6521911e0
Type: f64   | Value: 3.14  | Function Address: 0x7ff6521910f0
Type: bool  | Value: true  | Function Address: 0x7ff652191000

=== 2. Struct Memory Layout ===
Storage<i32> holds value: 42   | consumes: 4 bytes
Storage<u8>  holds value: 42   | consumes: 1 bytes
Storage<f64> holds value: 42   | consumes: 8 bytes
```

**Explanation**

1. Monomorphization in Functions:
   - สังเกตว่า inspect_execution(100_i32) และ inspect_execution(200_i32) ชี้ไปยัง Function Address เดียวกัน (0x...11e0) เพราะคอมไพเลอร์มองว่าเป็นโค้ดฟังก์ชันรูปธรรมชุดเดียวกัน (Concrete Function)
   - เมื่อเปลี่ยน Type เป็น f64 และ bool ตัวชี้จะกลายเป็น Address ใหม่ทันที (0x...10f0 และ 0x...1000) พิสูจน์ว่า Rust แตกโค้ด Generic ออกเป็นฟังก์ชันเฉพาะของแต่ละ Type ให้โดยอัตโนมัติในระดับ Machine Code (Static Dispatch)
2. Monomorphization in Structs:
   - Storage<i32>, Storage<u8>, และ Storage<f64> มีขนาดหน่วยความจำตาม Type ข้อมูลจริงที่บรรจุอยู่ภายใน (4, 1, และ 8 ไบต์ตามลำดับ)
   - มีการเรียกใช้ Method .get() เพื่อดึงค่าจริงออกมาใช้งานโดยตรง และไม่มีการใช้ Pointer ห่อหุ้ม (Box) หรือแอบแฝง Metadata/Type Tag ใดๆ เพิ่มเติม ทำให้ได้ประสิทธิภาพสูงสุดในระดับฮาร์ดแวร์เทียบเท่าภาษา C/C++ (Zero-Cost Abstraction)

---

## 7. Common Mistakes

### Mistake 1 — การคำนวณหรือเปรียบเทียบค่าบน Generic Type โดยไม่มี Trait Bounds

**Problem**

ผู้เริ่มต้นเขียน Rust มักคุ้นชินกับ Template ใน C++ ที่คอมไพเลอร์ยอมให้เขียนตัวดำเนินการคณิตศาสตร์ได้โดยตรง แต่ในภาษา Rust หากไม่ระบุ Trait Bound คอมไพเลอร์จะปฏิเสธการคอมไพล์ทันที

**Incorrect Code**

```rust
// คอมไพล์ไม่ผ่าน!
fn add_numbers<T>(a: T, b: T) -> T {
    a + b // ERROR: cannot add `T` to `T`
}
```

**Correct Code**

```rust
use std::ops::Add;

// ระบุ Trait Bound ว่า T ต้อง Implement std::ops::Add
fn add_numbers<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}
```

**Why?**

ภาษา Rust ใช้ระบบตรวจสอบแบบ Nominal Contract Enforcement at Definition Site (การบังคับตรวจสัญญาความถูกต้องตามชื่อชนิดข้อมูล ณ จุดนิยามฟังก์ชัน เพื่อดักจับข้อผิดพลาดทั้งหมดตั้งแต่ขั้นตอนคอมไพล์โดยไม่รอให้เกิดการเรียกใช้จริง) หมายความว่าคอมไพเลอร์จะตรวจสอบความถูกต้องของตัวฟังก์ชันทันทีที่ประกาศ โดยไม่รอให้มีการเรียกใช้งานจริง หากฟังก์ชันไม่ระบุว่า `T` ทำการ Implement Trait `Add` คอมไพเลอร์จะไม่ยอมให้คอมไพล์เด็ดขาด เพื่อป้องกันความผิดพลาดขณะรันไทม์

---

### Mistake 2 — การฝ่าฝืนกฎ Object Safety เมื่อสร้าง Trait Object

**Problem**

พยายามใช้คีย์เวิร์ด `dyn Trait` (Trait Object สำหรับ Dynamic Dispatch) กับ Trait ที่มี Method คืนค่าเป็น `Self` หรือมี Generic Type Parameters ในตัว Method ส่งผลให้คอมไพเลอร์แจ้งข้อผิดพลาด `the trait cannot be made into an object`

**Incorrect Code**

```rust
pub trait Transformer {
    fn transform<U>(&self, input: U); // ผิดกฎ: Method มี Generic Parameter
    fn duplicate(&self) -> Self;       // ผิดกฎ: คืนค่าเป็น Self
}

// ERROR: the trait `Transformer` cannot be made into an object
// let obj: Box<dyn Transformer>;
```

**Correct Code**

```rust
pub trait SafeTransformer {
    fn transform_str(&self, input: &str) -> String;
}

// ใช้งานผ่าน Trait Object ได้อย่างถูกต้อง
let obj: Box<dyn SafeTransformer>;
```

**Why?**

ตามหลักการของตารางฟังก์ชันเสมือน (Vtable) ตัวคอมไพเลอร์ต้องทราบขนาดของ Offset และ Function Pointer ที่แน่นอนล่วงหน้าตั้งแต่ตอนคอมไพล์ หาก Method มี Generic Parameter หรือมีการคืนค่าเป็น `Self` ตัวคอมไพเลอร์จะไม่สามารถทราบขนาดข้อมูลและจำนวนช่องฟังก์ชันในตาราง Vtable ที่แน่นอนได้ จึงต้องปฏิเสธการคอมไพล์เพื่อรักษาความปลอดภัยของหน่วยความจำ

---

## 8. Exercises

### Exercise 1 — Generic Key-Value Cache with Capacity & Eviction

**Problem**

จงออกแบบและเขียนโค้ดโครงสร้างข้อมูล `SimpleCache<K, V>` ที่เป็น Generic รองรับ Key ชนิดใดก็ได้ที่สามารถเปรียบเทียบความเท่ากันได้ (`PartialEq + Display`) และ Value ชนิดใดก็ได้ที่สามารถคัดลอกและแสดงผลได้ (`Clone + Display`) โดยให้มี Method `put(key, value)` (เพิ่มหรืออัปเดตข้อมูล) และ `get(&key)` (ค้นหาและอ่านข้อมูล) พร้อมระบุความจุสูงสุด (Capacity)

**Hint**

ใช้ `Vec<CacheEntry<K, V>>` และกำหนด Trait Bounds บน `impl<K, V> SimpleCache<K, V> where K: PartialEq + Display, V: Clone + Display`

**Solution**

```rust
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct CacheEntry<K, V> {
    pub key: K,
    pub value: V,
    pub access_count: usize,
}

pub struct SimpleCache<K, V> {
    entries: Vec<CacheEntry<K, V>>,
    capacity: usize,
}

impl<K, V> SimpleCache<K, V>
where
    K: PartialEq + Display,
    V: Clone + Display,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::new(),
            capacity,
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        for entry in &mut self.entries {
            if entry.key == key {
                entry.value = value;
                entry.access_count += 1;
                return;
            }
        }
        if self.entries.len() >= self.capacity {
            self.entries.remove(0); // ลบตัวเก่าสุดออกเมื่อเต็มตามนโยบาย FIFO
        }
        self.entries.push(CacheEntry {
            key,
            value,
            access_count: 1,
        });
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        for entry in &mut self.entries {
            if &entry.key == key {
                entry.access_count += 1;
                return Some(entry.value.clone());
            }
        }
        None
    }
}
```

**Explanation**

โครงสร้างนี้แสดงการประยุกต์ใช้ Parametric Polymorphism ร่วมกับ Trait Bounds และหลักการออกแบบซอฟต์แวร์:
1. `#[derive(Debug, Clone)]`: Macro สั่งให้คอมไพเลอร์สร้าง Implementation สำหรับแสดงผลและคัดลอกข้อมูลให้อัตโนมัติ
2. `CacheEntry<K, V>`: แยกการเก็บข้อมูลออกจากพฤติกรรม (Data-Behavior Separation)
3. การใช้ `usize`: ตัวแปร `access_count` และ `capacity` ใช้ชนิดข้อมูล `usize` ซึ่งเป็นชนิดตัวเลขจำนวนเต็มบวกมาตรฐานสำหรับดัชนีและขนาดหน่วยความจำในภาษา Rust ช่วยป้องกันปัญหาค่าติดลบได้อย่างสมบูรณ์
4. หลัก Encapsulation: ฟิลด์ `entries` และ `capacity` ไม่ได้ใส่ `pub` ทำให้เป็น Private ป้องกันการแก้ไขโดยตรงจากภายนอก บังคับให้เรียกผ่านเมธอด `put()` และ `get()` เท่านั้น
5. Trait Bounds:
   - `K: PartialEq`: เพื่อเปรียบเทียบ `entry.key == key` ได้
   - `K: Display`: เพื่อแสดงผล Key ออกทางหน้าจอ
   - `V: Clone`: เพื่อให้ `get()` สามารถส่งสำเนาข้อมูลออกไปได้โดยไม่แย่ง Ownership ของข้อมูลในแคช
   - `V: Display`: เพื่อแสดงผล Value ออกทางหน้าจอ

---

### Exercise 2 — Polymorphic Notification Engine

**Problem**

จงออกแบบระบบส่งการแจ้งเตือนที่มี Trait `Notifier` ซึ่งมี method `channel_name(&self) -> &'static str` และ `send(&self, recipient: &str, message: &str) -> Result<(), String>` จากนั้นสร้าง Struct สำหรับ `EmailNotifier`, `SmsNotifier`, และ `DiscordWebhookNotifier` พร้อมทั้ง:
1. เขียนฟังก์ชัน `send_urgent_static<N: Notifier>` สำหรับส่งด่วนด้วย Static Dispatch (Early Binding)
2. เขียน Struct `NotificationBroadcaster` ที่เก็บ `Vec<Box<dyn Notifier>>` สำหรับกระจายข้อความทุกช่องทางแบบ Dynamic Dispatch (Late Binding)

**Hint**

สำหรับส่วน Dynamic Dispatch ให้ใช้ Vector เก็บ `Box<dyn Notifier>` และวนลูปเรียก `channel.send(...)`

**Solution**

```rust
pub trait Notifier {
    fn channel_name(&self) -> &'static str;
    fn send(&self, recipient: &str, message: &str) -> Result<(), String>;
}

// 1. Static Dispatch (Early Binding / Monomorphization)
pub fn send_urgent_static<N: Notifier>(notifier: &N, recipient: &str, alert: &str) {
    let _ = notifier.send(recipient, alert);
}

// 2. Dynamic Dispatch (Late Binding / Trait Objects)
pub struct NotificationBroadcaster {
    channels: Vec<Box<dyn Notifier>>,
}

impl NotificationBroadcaster {
    pub fn new() -> Self {
        Self { channels: Vec::new() }
    }

    pub fn register_channel(&mut self, ch: Box<dyn Notifier>) {
        self.channels.push(ch);
    }

    pub fn broadcast(&self, recipient: &str, msg: &str) {
        for ch in &self.channels {
            let _ = ch.send(recipient, msg);
        }
    }
}
```

**Explanation**

แบบฝึกหัดนี้แสดงถึงการตัดสินใจเชิงสถาปัตยกรรมภาษาโปรแกรม:
1. Static Dispatch (`send_urgent_static`): คอมไพเลอร์ทำ Monomorphization และ Inlining ทำให้เรียกคำสั่งตรงโดยไม่มี Overhead ของตาราง Vtable เหมาะสำหรับงานที่ต้องการประสิทธิภาพและความเร็วสูงสุด
2. Dynamic Dispatch (`NotificationBroadcaster`): ใช้ Trait Object `Box<dyn Notifier>` ร่วมกับ Heterogeneous Collection ทำให้สามารถรวบรวมช่องทางแจ้งเตือนต่างชนิดกันไว้ใน Vector เดียวกันได้ เหมาะสำหรับระบบที่ต้องการความยืดหยุ่นในการขยายโมดูลขณะรันไทม์

---

### Challenge — คำถามท้าทาย (Challenge Question)

**Question:**  
หากเราสร้างฟังก์ชันแบบ Generic ขึ้นมา 1 ฟังก์ชัน แต่ในระบบจริงมีการเรียกใช้งานฟังก์ชันนี้ด้วยชนิดข้อมูล (Concrete Types) ที่แตกต่างกันถึง 100 ชนิด จะส่งผลกระทบต่อประสิทธิภาพการทำงาน (Runtime Performance) และขนาดของไฟล์โปรแกรม (.exe / Binary Size) อย่างไร?

**Answer:**  
1. **ด้านความเร็ว (Runtime Performance):** ยังคงทำงานด้วยความเร็วสูงสุดระดับ Zero-Cost Abstraction เท่าเดิม เพราะคอมไพเลอร์สร้าง Machine Code แบบ Direct Call เฉพาะทางสำหรับแต่ละ Type ล่วงหน้า
2. **ด้านขนาดไฟล์ (Binary Size):** ขนาดของไฟล์โปรแกรมจะขยายใหญ่ขึ้นอย่างมีนัยสำคัญ (เรียกว่าปรากฏการณ์ **Code Bloat**) เนื่องจากกลไก Monomorphization จะโคลนและคอมไพล์โค้ดเครื่องออกมาถึง 100 ชุดตามจำนวน Type ที่เรียกใช้จริง

---

## 9. PPL Perspective

>**ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax
`Rust ใช้ <T> ประกาศ Type Parameter ได้ที่ function, struct, enum, method, trait เพื่อให้รองรับได้หลาย Type กล่าวได้ว่าภาษานี้ออกแบบ Syntax รองรับ Parametric Polymorphism โดยให้ระบุ Type Parameter ผ่าน syntax และกำหนดข้อจำกัดผ่าน Trait Bounds`
```rust
fn compare<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}
```
| องค์ประกอบ | ความหมาย | หน้าที่ |
|---|---|---|
| `fn` | Function Keyword | ประกาศฟังก์ชัน |
| `compare` | Function Name | ชื่อฟังก์ชัน |
| `<T>` | Generic Type Parameter | กำหนด Type ที่สามารถเปลี่ยนแปลงได้ |
| `: PartialOrd` | Trait Bound | กำหนดความสามารถที่ Type ต้องมี |
| `a: T, b: T` | Parameters | กำหนดให้พารามิเตอร์ทั้งสองมี Type เดียวกัน |
| `-> bool` | Return Type | กำหนดชนิดข้อมูลที่ส่งกลับ |
| `a > b` | Expression | เปรียบเทียบค่าผ่านความสามารถของ PartialOrd |

### 9.2 Semantics
`Rust รองรับการเขียน Generic Code ที่สามารถทำงานกับหลาย Type โดยกำหนดพฤติกรรมร่วมผ่าน Type Parameter และสามารถจำกัดความสามารถของ Type Parameter ด้วย Trait Bounds จึงสามารถมองได้ว่าเป็นการใช้ Parametric Polymorphism ร่วมกับ Bounded Polymorphism โดย Type ที่นำมาใช้ต้องมีความสามารถตาม Trait Bound ที่กำหนดไว้`

### 9.3 Type System
`ระบบชนิดข้อมูลของ Rust รวม Parametric Polymorphism เข้ากับ Bounded Polymorphism ผ่าน trait โดยกำหนดให้ความสามารถของ type parameter ต้องถูกประกาศอย่างชัดเจนและ Compiler ใช้ Trait Bound ที่ประกาศไว้ในการตรวจสอบการดำเนินการกับ Type Parameter และตรวจสอบว่า Concrete Type ที่นำมาใช้ตรงตามข้อกำหนดใน Compile Time (การดำเนินการกับค่าของ Type Parameter ต้องระบุ Trait Bound จึงจะใช้ตัวดำเนินการเปรียบเทียบได้) ส่งผลให้ได้ทั้งความปลอดภัยของชนิดข้อมูลและสัญญาที่อ่านได้จาก signature โดยไม่ต้องพึ่งการตรวจสอบขณะรันโปรแกรม กล่าวคือ ฟังก์ชันหรือโครงสร้างข้อมูลชนิดหนึ่งสามารถนิยามครั้งเดียวแต่ใช้งานได้กับหลายชนิดข้อมูล`
##### Parametric Polymorphism โดยโค้ดเดียวใช้ได้กับหลายชนิดและมีพฤติกรรมเดียวกัน
```rust
fn identity<T>(value: T) -> T {
    value
}

fn main() {
    println!("{}", identity(10));
    println!("{}", identity(3.14));
}
```
##### Bounded Polymorphism เมื่อดำเนินการโดยถูกจำกัดด้วย Trait Bound
```rust
fn compare<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}

fn main() {
    println!("{}", compare(10, 5));
    println!("{}", compare(3.5, 2.5));
}
```

### 9.4 Memory / Resource Management
`Generics ของ Rust ทำงานร่วมกับระบบ Ownership, Borrowing และ Lifetime โดย Generic Type ยังคงอยู่ภายใต้กฎการจัดการ Memory ของ Rust ซึ่งเป็นกลไกหลักในการจัดการ Memory ของภาษา โดย Compiler จะตรวจสอบการเป็นเจ้าของ การยืม และอายุการใช้งานของข้อมูลตั้งแต่ Compile Time ทำให้ Generic Code ยังคงอยู่ภายใต้กฎของ Memory Safety โดยสามารถป้องกันปัญหาด้าน Memory ผ่านการตรวจสอบแบบ Static ได้ ลดความจำเป็นในการจัดการ Generic Type แบบ Dynamic
ในด้านการจัดการ Resource Rust ใช้ Monomorphization ในการ Compile Generic Code ไปสร้างเป็นรูปแบบที่สอดคล้องกับ Concrete Type ที่ถูกใช้งานในขั้น Compile Time แทนการต้องจัดการ Generic Type แบบ Dynamic ระหว่าง Runtime แนวทางนี้ช่วยให้การใช้ Generics ไม่จำเป็นต้องเพิ่มกลไก Runtime สำหรับการตรวจสอบหรือเลือก Type
จึงแสดงให้เห็นความสัมพันธ์ระหว่าง Type System, Memory Management และ Compilation อย่างชัดเจน เพราะข้อกำหนดของ Type และกฎด้าน Memory ถูกตรวจสอบก่อนโปรแกรมทำงาน ขณะที่ Generic Abstraction สามารถถูกแปลงเป็นโค้ดที่เหมาะสมก่อน Runtime (ไม่ต้องเพิ่มต้นทุน Runtime ที่ไม่จำเป็น)`

### 9.5 Abstraction / Other PPL Concepts
`การ Abstraction ทำผ่าน trait ซึ่งกำหนดว่าชนิดข้อมูลหนึ่งสามารถทำอะไรได้ โดยไม่ระบุว่าเก็บข้อมูลอย่างไร ทำให้แยกพฤติกรรมออกจากข้อมูลได้อย่างชัดเจน นอกจากนี้ยังสามารถเพิ่มการ implement trait ให้กับชนิดที่มีอยู่แล้วภายหลังภายใต้ orphan rule ได้อีกด้วย ต่างจากภาษาเชิงวัตถุแบบดั้งเดิมที่ต้องประกาศความสัมพันธ์ไว้ตั้งแต่ตอนนิยามคลาส
Rust ไม่มี Class Inheritance แต่ใช้แนวคิด composition และ trait แทน คือ สร้างความสามารถใหม่จากการประกอบชนิดข้อมูลเข้าด้วยกัน และกำหนดพฤติกรรมร่วมผ่าน trait จึงหลีกเลี่ยงปัญหาที่พบในระบบสืบทอดได้
เมื่อเปรียบเทียบกับภาษาอื่น จะมีการทำ Generics ต่างกัน เช่น Rust ใช้ Monomorphization ร่วมกับ trait bound ที่ประกาศชัดเจน ไม่มีต้นทุนขณะรัน แต่ C++ ใช้ templates ซึ่งให้ประสิทธิภาพใกล้เคียงกัน แต่เดิมตรวจเงื่อนไขแบบโดยนัยตอน instantiate (ปัจจุบันมี Concepts ใน C++20 ช่วย) ในขณะที่ Java ใช้ type erasure ลบข้อมูลชนิดทิ้งหลัง compile ทำให้ต้องใช้ boxing และ cast ขณะรัน หรือ C# ใช้ reified generics คือคงข้อมูลชนิดไว้ขณะรัน มีต้นทุนเล็กน้อย และ Python เป็นภาษา dynamically typed จึงไม่ต้องใช้ generics เพื่อให้โค้ดรับได้หลายชนิด ใช้ duck typing เป็นหลัก และมี generics เพียงในระดับ type hint ที่ให้เครื่องมืออย่าง mypy ตรวจสอบ ตัวภาษาเองไม่บังคับและ type hint ถูกละเว้นขณะรัน ข้อผิดพลาดด้านชนิดจึงเกิดขณะรัน และทุกการเรียกเป็น dynamic dispatch จึงมีต้นทุนขณะรันสูงกว่า เป็นต้น`

### 9.6 Why Rust?
1. Compiler สามารถตรวจสอบ Type Parameter และ Trait Bound ตั้งแต่ Compile Time ทำให้การใช้ Generic Type ต้องเป็นไปตามข้อกำหนดที่ประกาศไว้
2. Trait Bound ทำให้ Generic Function ระบุความสามารถที่ Type ต้องมีไว้อย่างชัดเจนใน Function Signature และ Compiler สามารถตรวจสอบการใช้งานภายใน Function ตามข้อกำหนดนั้น
3. Rust ใช้ Monomorphization ในการ Compile Generic Code โดยสร้าง Code สำหรับ Concrete Type ที่นำมาใช้ ทำให้ไม่จำเป็นต้องจัดการ Generic Type แบบ Dynamic ใน Runtime ในกรณีที่ใช้วิธีนี้
4. Rust ออกแบบ Generics และ Traits ให้สามารถใช้ Abstraction ได้โดยไม่จำเป็นต้องเพิ่ม Runtime Overhead ที่ไม่จำเป็น เมื่อ Compiler สามารถสร้าง Code ที่เหมาะสมได้

---

## 10. Rust vs. Other Language
Comparison Language: Java

| Aspect | Rust | Java |
|---|---|---|
| Syntax | ใช้ `<T>` ระบุ Type Parameter และใช้ Trait Bound กำหนดข้อจำกัด | ใช้ `<T>` เช่นกัน แต่กำหนดข้อจำกัดผ่าน `extends` แบบ OOP |
| Semantics / Behavior | Compiler สร้างโค้ดจริงแยกสำหรับแต่ละ Type ที่ใช้งาน (Monomorphization) ตรวจสอบความถูกต้องตั้งแต่ Compile Time ได้ Native Machine Code ที่ไม่มี Runtime Overhead | ใช้ Type Erasure โดยข้อมูล Generic Type ส่วนใหญ่ไม่คงอยู่ในรูป Generic Type ที่ Runtime และ Compiler อาจแทรก Cast ตามบริบท ขณะที่ Primitive Type อาจต้องผ่าน Boxing/Unboxing เมื่อใช้กับ Generics |
| Type System | Static Type System ผูกกับ Trait System โดยตรง Trait คือสัญญาที่ตรวจสอบตั้งแต่จุดนิยาม Generic ไม่ต้องรอ Instantiate | Static Type System เช่นกัน แต่ Bound ถูกเช็คแค่ตอน Compile แล้วข้อมูลจริงหายไปตอน Runtime (ต่างจาก Rust ที่แปลง Generic เป็นโค้ดเฉพาะชนิดตอน Compile จึงไม่ต้องใช้ข้อมูลชนิดตอนรัน) |
| Memory Management | ใช้ Ownership และ Borrowing จัดการหน่วยความจำโดยไม่มี Garbage Collector ปล่อย Resource แบบกำหนดเวลาแน่นอน (Deterministic) | ใช้ Garbage Collector จัดการหน่วยความจำอัตโนมัติ แต่เวลาที่เก็บกวาดไม่แน่นอน (Non-deterministic) |
| Safety | ป้องกันปัญหาอย่าง Null Reference ได้ตั้งแต่ Compile Time ผ่าน `Option<T>` แทนการมีค่า Null ในระบบ | ยังมีโอกาสเกิด `NullPointerException` ตอน Runtime ได้ เพราะทุก Reference Type สามารถเป็น `null` ได้โดยไม่ถูกบังคับตรวจสอบตอน Compile |


### Rust Example
```rust
use std::fmt::Display;

fn max_value<T: PartialOrd + Display>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// Generic Struct
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: PartialOrd + Display> Pair<T> {
    fn max(&self) -> &T {
        if self.first > self.second {
            &self.first
        } else {
            &self.second
        }
    }

    fn print(&self) {
        println!(
            "First: {}, Second: {}, Max: {}",
            self.first,
            self.second,
            self.max()
        );
    }
}

fn main() {
    // Generic Function
    println!("Max Integer: {}", max_value(10, 20));
    println!("Max Float: {}", max_value(3.5, 2.8));

    // Generic Struct
    let numbers = Pair {
        first: 15,
        second: 25,
    };

    let scores = Pair {
        first: 85.5,
        second: 92.0,
    };

    numbers.print();
    scores.print();
}
```

### Java Example

```Java
public class Main {

    static <T extends Comparable<T>> T maxValue(T a, T b) {
        return (a.compareTo(b) > 0) ? a : b;
    }

    // Generic Class
    static class Pair<T extends Comparable<T>> {
        private T first;
        private T second;

        Pair(T first, T second) {
            this.first = first;
            this.second = second;
        }

        T max() {
            return (first.compareTo(second) > 0)
                    ? first
                    : second;
        }

        void print() {
            System.out.println(
                "First: " + first +
                ", Second: " + second +
                ", Max: " + max()
            );
        }
    }

    public static void main(String[] args) {

        // Generic Function
        System.out.println("Max Integer: "
                + maxValue(10, 20));

        System.out.println("Max Double: "
                + maxValue(3.5, 2.8));

        // Generic Class
        Pair<Integer> numbers =
                new Pair<>(15, 25);

        Pair<Double> scores =
                new Pair<>(85.5, 92.0);

        numbers.print();
        scores.print();
    }
}
```

### Analysis
- ส่วนที่เหมือนกันคือ Rust และ Java เป็นภาษาที่มี Static Type System และรองรับ Generics ทั้งคู่ จึงทำให้เขียนฟังก์ชันหรือ Class เดียวใช้ได้กับหลาย Type และตรวจสอบความถูกต้องของ Type ช่วง Compile Time เหมือนกันอีกด้วย

- ส่วนที่ต่างกันคือกลไกเบื้องหลัง Generic: Rust ใช้ Monomorphization สร้างโค้ดแยกสำหรับแต่ละ Type จริง ทำให้ไม่มี Runtime Overhead แต่ Java ใช้ Type Erasure ลบข้อมูล Type ทิ้งหลัง Compile ทำให้มี Overhead จาก Boxing/Unboxing เล็กน้อย นอกจากนี้ Rust ยังผูก Generic เข้ากับ Ownership เพื่อจัดการหน่วยความจำโดยไม่ต้องมี Garbage Collector ต่างจาก Java ที่พึ่งพา GC และ JVM ในการจัดการ Runtime ทั้งหมด
---

Comparison Language: Python

| Aspect | Rust | Python |
|---|---|---|
| Syntax | ใช้ `<T>` ระบุ Type Parameter พร้อม Trait Bound เช่น `fn max<T: PartialOrd>(a: T, b: T) -> T` — เป็นส่วนหนึ่งของภาษาที่ Compiler บังคับตรวจสอบ | ใช้ `TypeVar` และ `Generic` จาก module `typing` เช่น `T = TypeVar('T')` แล้วเขียน `def max(a: T, b: T) -> T` — เป็นเพียง Type Hint ที่แนบไว้เฉย ๆ ไม่ใช่ Syntax หลักของภาษา |
| Semantics / Behavior | Compiler ตรวจสอบและ Monomorphize ตั้งแต่ Compile Time ได้ Native Machine Code ที่ไม่มี Runtime Overhead จาก Generic | Python Interpreter ไม่ได้บังคับ Static Type Checking แบบ Rust แต่สามารถใช้เครื่องมือภายนอก เช่น mypy เพื่อตรวจสอบ Type ก่อน Runtime ได้ |
| Type System | Static Type System ที่ผูกกับ Trait System โดยตรง ตรวจสอบ Type ตั้งแต่จุดนิยาม Generic ก่อนโปรแกรมทำงาน | Dynamically-typed ใช้ Duck Typing เป็นหลัก ตัวแปรไม่มี Type ตายตัว ค่าทุกตัวเป็น Object ที่ตรวจสอบ Type จริงได้ตอน Runtime เท่านั้น (ผ่าน `type()` หรือ `isinstance()`) |
| Memory Management | ใช้ Ownership และ Borrowing จัดการหน่วยความจำโดยไม่มี Garbage Collector ปล่อย Resource แบบกำหนดเวลาแน่นอน | ใช้ Reference Counting ร่วมกับ Garbage Collector (สำหรับเก็บ Cyclic Reference) จัดการหน่วยความจำอัตโนมัติทั้งหมด ผู้เขียนไม่ต้องยุ่งเกี่ยวกับ Memory เลย |
| Safety | Compiler ตรวจสอบ Type และกฎ Ownership ก่อนโปรแกรมทำงาน ทำให้ข้อผิดพลาดเรื่อง Type หรือ Memory ส่วนใหญ่ถูกจับได้ตั้งแต่ Compile Time | ข้อผิดพลาดเรื่อง Type (เช่นส่ง `str` เข้าไปในฟังก์ชันที่ตั้งใจให้รับ `int`) จะไม่ถูกจับจนกว่าโปรแกรมจะรันถึงบรรทัดนั้นจริง แล้วเกิด Exception เช่น `TypeError` ขึ้นตอน Runtime |



### Rust Example
```rust
use std::fmt::Display;

fn max_value<T: PartialOrd + Display>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// Generic Struct
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: PartialOrd + Display> Pair<T> {
    fn max(&self) -> &T {
        if self.first > self.second {
            &self.first
        } else {
            &self.second
        }
    }

    fn print(&self) {
        println!(
            "First: {}, Second: {}, Max: {}",
            self.first,
            self.second,
            self.max()
        );
    }
}

fn main() {
    // Generic Function
    println!("Max Integer: {}", max_value(10, 20));
    println!("Max Float: {}", max_value(3.5, 2.8));

    // Generic Struct
    let numbers = Pair {
        first: 15,
        second: 25,
    };

    let scores = Pair {
        first: 85.5,
        second: 92.0,
    };

    numbers.print();
    scores.print();
}
```

### Python Example

```Python
from typing import TypeVar, Generic

T = TypeVar('T')

# Generic Function
def max_value(a: T, b: T) -> T:
    return a if a > b else b


# Generic Class
class Pair(Generic[T]):
    def __init__(self, first: T, second: T):
        self.first = first
        self.second = second

    def max(self) -> T:
        return self.first if self.first > self.second else self.second

    def print_pair(self):
        print(
            f"First: {self.first}, "
            f"Second: {self.second}, "
            f"Max: {self.max()}"
        )


def main():
    # Generic Function
    print("Max Integer:", max_value(10, 20))
    print("Max Float:", max_value(3.5, 2.8))

    # Generic Class
    numbers = Pair(15, 25)
    scores = Pair(85.5, 92.0)

    numbers.print_pair()
    scores.print_pair()


main()
```

### Analysis
- ทั้งสองภาษามีวิธีเขียนโค้ดที่ทำงานร่วมกับหลาย Type ได้โดยไม่ต้องเขียนฟังก์ชันซ้ำสำหรับแต่ละ Type (Rust ผ่าน Generic จริงในภาษา ส่วน Python ผ่าน Duck Typing ที่ทำได้อยู่แล้วโดยธรรมชาติ บวกกับ Type Hint ที่เพิ่มเข้ามาทีหลังเพื่อช่วยด้านเอกสารและเครื่องมือ)

- Rust เป็น Static Type System ที่ Compiler บังคับตรวจสอบ Generic และ Trait Bound ก่อนโปรแกรมจะรันได้เลย ทำให้ข้อผิดพลาดเรื่อง Type ถูกจับตั้งแต่ Compile Time ทั้งหมด ในขณะที่ Python ไม่มี Compile-time Type Checking ในตัวภาษาเลย จึงต้องพึ่งเครื่องมือภายนอกอย่าง mypy ในการตรวจสอบแทน Compiler ของภาษาเอง

Comparison Language: C++

| Aspect | Rust | C++ |
|---|---|---|
| Syntax | ใช้ `<T>` ระบุ Type Parameter พร้อม Trait Bound เป็น Syntax หลักของภาษาที่ออกแบบมาเพื่อ Generic โดยเฉพาะ | ใช้ template <typename T> หรือ template <class T> เพื่อประกาศ Type Parameter และสามารถใช้ Concepts เช่น std::totally_ordered ใน C++20 เพื่อกำหนดข้อจำกัด |
| Semantics / Behavior | Compiler ตรวจสอบและ Monomorphize ตั้งแต่ Compile Time สร้างโค้ดจริงแยกสำหรับแต่ละ Type ที่ใช้งาน ไม่มี Runtime Overhead | Compiler สร้าง/instantiate โค้ดจาก Template ตาม Type ที่ใช้งาน โดยทั่วไปเกิดขึ้นตอน Compile Time และสามารถใช้ Concepts ช่วยตรวจสอบข้อกำหนดของ Type |
| Type System | Static Type System ที่ผูกกับ Trait System บังคับให้ Type ต้อง implement พฤติกรรมที่ต้องการก่อน | Static Type System และใช้ Template เพื่อกำหนดข้อกำหนดของ Type แต่ไม่มี Trait System แบบ Rust |
| Memory Management | ใช้ Ownership และ Borrowing จัดการหน่วยความจำ ไม่มี Garbage Collector ปล่อย Resource แบบกำหนดเวลาแน่นอนโดย Compiler ตรวจสอบให้ | ไม่มี Garbage Collector เป็นกลไกหลักของภาษา สามารถจัดการ Resource ด้วย RAII, smart pointers และการจัดการหน่วยความจำแบบ manual ได้ โดย Template ไม่ได้กำหนดรูปแบบการจัดการ Memory โดยตรง |
| Safety | Compiler ตรวจสอบ Type และกฎ Ownership ก่อนโปรแกรมทำงาน ป้องกัน Memory Bug ได้ตั้งแต่ Compile Time | มี Static Type Checking และเครื่องมืออย่าง RAII/Smart Pointer ช่วยเพิ่มความปลอดภัย แต่ยังสามารถเกิด Memory Bug อย่าง dangling pointer, use-after-free หรือ undefined behavior ได้ |

### Rust Example
```rust
use std::fmt::Display;

fn max_value<T: PartialOrd + Display>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// Generic Struct
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: PartialOrd + Display> Pair<T> {
    fn max(&self) -> &T {
        if self.first > self.second {
            &self.first
        } else {
            &self.second
        }
    }

    fn print(&self) {
        println!(
            "First: {}, Second: {}, Max: {}",
            self.first,
            self.second,
            self.max()
        );
    }
}

fn main() {
    // Generic Function
    println!("Max Integer: {}", max_value(10, 20));
    println!("Max Float: {}", max_value(3.5, 2.8));

    // Generic Struct
    let numbers = Pair {
        first: 15,
        second: 25,
    };

    let scores = Pair {
        first: 85.5,
        second: 92.0,
    };

    numbers.print();
    scores.print();
}
```

### C++ Example

```C++
#include <iostream>

template <typename T>
T max_value(T a, T b) {
    return a > b ? a : b;
}

// Generic Struct
template <typename T>
struct Pair {
    T first;
    T second;

    T max() const {
        return first > second ? first : second;
    }

    void print() const {
        std::cout
            << "First: " << first
            << ", Second: " << second
            << ", Max: " << max()
            << '\n';
    }
};

int main() {

    // Generic Function
    std::cout
        << "Max Integer: "
        << max_value(10, 20)
        << '\n';

    std::cout
        << "Max Double: "
        << max_value(3.5, 2.8)
        << '\n';

    // Generic Struct
    Pair<int> numbers{15, 25};
    Pair<double> scores{85.5, 92.0};

    numbers.print();
    scores.print();

    return 0;
}
```

### Analysis
- ทั้งสองภาษารองรับ Generic/Template Code ที่สามารถนำไปใช้กับหลาย Type และ Compiler ตรวจสอบความถูกต้องของ Type ก่อนโปรแกรมทำงาน
- ส่วนที่ต่างกันคือ Rust ใช้ Generics ร่วมกับ Trait Bounds และมี Monomorphization เป็นกลไกสำคัญ ส่วน C++ ใช้ Templates ซึ่งเป็นกลไก Compile-time Polymorphism และสามารถใช้ Concepts (C++20) เพื่อกำหนดข้อจำกัดของ Type ได้อย่างชัดเจน
- Rust ผูก Generic เข้ากับ Ownership และ Borrowing เพื่อช่วยรับประกัน Memory Safety ส่วน C++ มีเครื่องมืออย่าง RAII และ Smart Pointer ช่วยจัดการ Resource แต่ไม่ได้รับประกัน Memory Safety แบบ Rust

---

## 11. Teach Your Topic

การนำเสนอในชั้นเรียนจัดสรรเวลา **สมาชิก 4 คน คนละประมาณ 5 นาที (รวม 20 นาที)** โดยแบ่งบทบาทและความรับผิดชอบอย่างชัดเจน:

| Member | Responsibility | Time | Presentation Scope |
|---|---|---:|---|
| **Member 1** (นายณัฐวุฒิ โตเมือง) | Concept + Short Code Illustration | 5 min | สรุปภาพรวม ปัญหาที่ Generics มาแก้, Parametric Polymorphism, Traits, โค้ดตัวอย่างสั้น (`Point<T>`, `Option<T>`) |
| **Member 2** (นางสาวณัฐสุดา ลานตวน) | Detailed Code + Live Demo | 5 min | โค้ดตัวอย่างเต็ม `KeyValue` (Example 1), Static vs Dynamic Dispatch (Example 2), สาธิต Live Demo |
| **Member 3** (นางสาวณัฐสุรางค์ ชาติทองคำ) | Rust vs Other Language + PPL Analysis | 5 min |  วิเคราะห์เชิงลึก 4 เสาหลัก PPL, เปรียบเทียบ Rust Monomorphization vs Java Type Erasure vs Python Duck Typing vs C++ |
| **Member 4** (นายธนเทพ นาสวน) | Exercises + Common Mistakes + Challenge | 5 min | สไลด์ 12–15: ข้อผิดพลาดที่พบบ่อย 2 กรณี (Trait Bounds, Object Safety), แบบฝึกหัด Cache & Notifier, คำถามท้าทาย Code Bloat |

### Individual Contribution & Script Cues

- **Member 1 (00:00 - 05:00):**  
  เปิดการบรรยาย นำเสนอแนวคิดพื้นฐาน ปัญหาเรื่อง Code Duplication ที่ Generics เข้ามาแก้ไข และหลักการ Parametric Polymorphism เบื้องต้น 

- **Member 2 (05:00 - 10:00):**  
  บรรยายโครงสร้างโค้ดเชิงลึก อธิบาย Trait Bounds ในทางปฏิบัติ และทำการสาธิต Live Demo เปรียบเทียบพฤติกรรมการทำงานของระบบ Static vs Dynamic Dispatch 
- **Member 3 (10:00 - 15:00):**  
  วิเคราะห์ระบบ Generics ผ่านแว่นตาของวิชา PPL (Syntax, Semantics, Type System, Memory Management) และเปรียบเทียบกลไก Monomorphization กับ Type Erasure ของ Java และ Templates ของภาษา C++

- **Member 4 (15:00 - 20:00):**  
  ชี้จุดข้อผิดพลาดทางเทคนิคที่พบบ่อยพร้อมเหตุผลทางคอมไพเลอร์ นำเสนอแบบฝึกหัดประยุกต์โครงสร้างข้อมูล Cache และ Dynamic Notification Engine เปิดคำถามท้าทายเชิงสถาปัตยกรรม (Code Bloat)

---

## 12. References

1. **The Rust Programming Language (The Rust Book):**  
   *Chapter 10: Generic Types, Traits, and Lifetimes*  
   Official Documentation: https://doc.rust-lang.org/book/ch10-00-generics.html
2. **The Rust Reference:**  
   *Traits, Trait Objects, and Dynamic Dispatch Specifications*  
   Official Documentation: https://doc.rust-lang.org/reference/types/trait-object.html
3. **Rust by Example:**  
   *Generics, Bounds, and Where Clauses*  
   Official Tutorial: https://doc.rust-lang.org/rust-by-example/generics.html
4. **Concepts of Programming Languages (12th Edition):**  
   *Robert W. Sebesta — Chapter 9: Subprograms & Chapter 12: Support for Object-Oriented Programming (Parametric Polymorphism & Type Erasure)*
5. **Rust RFC 0255 — Object Safety:**  
   *Formalizing object safety rules for dynamic dispatch in Rust*  
   Official RFC Archive: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
6. **cppreference:**  
   Official Documentation: https://cppreference.com/
7. **Oracle Java Tutorials:**  
   Official Tutorial: https://docs.oracle.com/javase/tutorial/
8. **JetBrains — Rust vs Java:**  
   Official Article: https://blog.jetbrains.com/rust/2025/08/01/rust-vs-java/

---

## 13. AI Usage Declaration

การจัดทำบทเรียนชุดนี้มีการใช้ปัญญาประดิษฐ์เพื่อช่วยค้นคว้า ร่างแบบทดสอบ และตรวจสอบความครบถ้วนของเอกสาร โดยสมาชิกทุกคนในกลุ่มได้ตรวจสอบและเข้าใจโค้ดทุกบรรทัดอย่างแท้จริง

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| **ChatGPT (OpenAI)** | ช่วยค้นหาข้อมูลไวยากรณ์ ช่วยค้นคว้าเอกสารอ้างอิง และช่วยเรียบเรียงคำอธิบายเชิงเปรียบเทียบภาษา | สมาชิกในกลุ่มร่วมกันตรวจสอบความถูกต้องกับ The Rust Book และทดสอบโค้ด |
| **Claude (Anthropic)** | ช่วยวิเคราะห์ทฤษฎี PPL เชิงลึก (Type System, Monomorphization) และช่วยเกลาภาษาเชิงวิชาการ | สมาชิกในกลุ่มร่วมกันทบทวนความถูกต้องตามหลักการวิชา PPL ทุกหัวข้อย่อย |
| **Google Antigravity / Gemini** | ช่วยสังเคราะห์โครงสร้างเนื้อหาตาม Template 15 หัวข้อ และร่างแนวคิดโจทย์แบบฝึกหัด | สมาชิกในกลุ่มร่วมกันตรวจสอบความถูกต้องของไวยากรณ์และข้อกำหนดของโครงงาน |
| **Rust Playground / rustc** | ใช้ตรวจสอบความถูกต้องของการคอมไพล์โค้ดตัวอย่างทุกไฟล์ | คอมไพล์และรันโค้ดจริง ตรวจสอบผลลัพธ์ตรงตาม Expected Output ในเอกสาร 100% |

### Declaration

- [x] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [x] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [x] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [x] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**  
กลุ่มได้ใช้เครื่องมือ AI (ChatGPT, Claude, Google Gemini) เพื่อช่วยในการสืบค้นข้อมูล ร่างไอเดียโจทย์แบบฝึกหัด และการจัดรูปเล่มตารางเปรียบเทียบตามมุมมองของวิชา PPL หลังจากนั้นสมาชิกทั้ง 4 คนได้นำโค้ดมาปรับปรุง ทดสอบการคอมไพล์จริงด้วยตนเอง และร่วมกันฝึกซ้อมเพื่อเตรียมพร้อมสำหรับการนำเสนอและตอบคำถามสดหน้าชั้นเรียน

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| **Member 1 (นายณัฐวุฒิ โตเมือง)** | 3 | 2 | 1 | 2 | วิจัยเนื้อหา Concept, ออกแบบ Learning Objectives, เขียนโค้ดสั้น |
| **Member 2 (นางสาวณัฐสุดา ลานตวน)** | 4 | 5 | 1 | 2 | พัฒนาโค้ดตัวอย่าง Example 1, 2 และจัดเตรียม Live Demo Script |
| **Member 3 (นางสาวณัฐสุรางค์ ชาติทองคำ)** | 5 | 7 | 1 | 2 | วิเคราะห์มุมมอง PPL, สรุปตาราง Rust vs Java/Python/C, ตรวจสอบทฤษฎี |
| **Member 4 (นายธนเทพ นาสวน)** | 4 | 6 | 1 | 2 | ออกแบบแบบฝึกหัด Cache & Notifier, รวบรวม Common Mistakes และสรุป Checklist |

### Teamwork Reflection

**How did your team collaborate?**  
ทีมงานแบ่งงานผ่าน GitHub Projects และ Issue Tracking โดยใช้การทำงานแบบ Feature Branches เมื่อสมาชิกแต่ละคนพัฒนาเนื้อหาหรือโค้ดในส่วนของตนเองเสร็จสิ้น จะทำการเปิด Pull Request เพื่อให้เพื่อนร่วมกลุ่มช่วยกัน Code Review และตรวจสอบความถูกต้องของทฤษฎีก่อน Merge เข้าสู่ Main Branch

**Problems encountered**  
ในช่วงแรกสมาชิกมีความสับสนระหว่างพฤติกรรมของ Static Dispatch (Generics) กับ Dynamic Dispatch (`dyn Trait`) โดยเฉพาะเงื่อนไขว่าเมื่อใดที่จำเป็นต้องใช้ `Box<dyn Trait>` และทำไม Trait บางตัวจึงไม่สามารถแปลงเป็น Trait Object ได้

**How did you solve them?**  
ทีมได้ร่วมกันสืบค้นเอกสาร RFC 0255 เรื่อง Object Safety และทดลองเขียนโค้ดเพื่อดูข้อความ Error จากคอมไพเลอร์ของ Rust จนเข้าใจกระจ่างว่า Vtable ต้องการขนาด Method ที่แน่นอน ทำให้สามารถนำประสบการณ์ข้อผิดพลาดนี้มาเขียนเป็นหัวข้อ Common Mistakes ที่มีประโยชน์ในเอกสาร

---

## 15. Final Checklist

- [x] Learning Objectives ครบ 4 ข้อ ชัดเจน สอดคล้องกับหัวข้อ
- [x] Key Concepts ครบถ้วน (Generics, Traits, Bounds, Static vs Dynamic Dispatch, Monomorphization)
- [x] Syntax / Rules และกฎสำคัญ
- [x] Runnable Code Examples มีครบทั้ง 2 ตัวอย่าง
- [x] Code Compile และ Run ได้จริง ผลลัพธ์ถูกต้อง 100%
- [x] Common Mistakes พร้อมวิธีแก้และคำอธิบายเชิงลึก 2 ข้อ
- [x] Exercises 2 ข้อ พร้อม Solution ละเอียดและคำอธิบาย
- [x] PPL Perspective ครบ 6 มิติ (Syntax, Semantics, Type System, Memory, Abstraction, Why Rust)
- [x] Rust vs Other Language (ตารางเปรียบเทียบ Rust vs Java, Python, C พร้อมโค้ดตัวอย่าง)
- [x] References ครบ 5 แหล่งอ้างอิงทางการ
- [x] AI Usage Declaration ครบถ้วน โปร่งใส
- [x] GitHub Contribution และ Teamwork Reflection ครบถ้วน
- [x] สมาชิกทั้ง 4 คนมีส่วนร่วมและแบ่งหน้าที่ชัดเจน
- [x] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ ~5 นาที (~20 นาทีรวม)
- [x] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้ทุกส่วน

---

## Submission Information

**Repository:** `https://github.com/soonklang/rust-tutorial-2569`  
**Chapter Path:** `15-generics-type-system/`  
**Final PR:** `#43`  
**Submitted by:** `Group 15`  
**Date:** `2026-10-04`

