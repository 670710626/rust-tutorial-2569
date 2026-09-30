# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 15
> **Topic No.:** 15
> **Topic Name:** Generics & Type System
> **ประเด็นหลักที่ควรครอบคลุม:** generic functions, generic structs, type parameters, monomorphization เบื้องต้น

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายณัฐวุฒิ โตเมือง | 670710623 | `@[กรอก GitHub username]` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวณัฐสุดา ลานตวน | 670710624 | `@[กรอก GitHub username]` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นางสาวณัฐสุรางค์ ชาติทองคำ | 670710625 | `@670710625` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายธนเทพ นาสวน | 670710626 | `@[กรอก GitHub username]` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

`[เขียนเนื้อหาที่นี่ — ใช้โครงสร้างเดียวกับ rust_tutorial_template.md ฉบับเต็มที่ผู้สอนแจกให้]`

---
## 9. PPL Perspective

>**ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax
`Rust ใช้ `<T>` ประกาศ Type Parameter ได้ที่ function, struct, enum, method, trait เพื่อให้รองรับได้หลาย Type กล่าวได้ว่าภาษานี้ออกแบบ Syntax รองรับ Parametric Polymorphism โดยให้ระบุ Type Parameter ผ่าน syntax และกำหนดข้อจำกัดผ่าน Trait Bounds`
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
fn id<T>(x: T) -> T { x }
```
##### Bounded Polymorphism เมื่อดำเนินการโดยถูกจำกัดด้วย Trait Bound
```rust
fn largest<T: PartialOrd + Copy>(xs: &[T]) -> T { ... }
```

### 9.4 Memory / Resource Management
`Generics ของ Rust ทำงานร่วมกับระบบ Ownership, Borrowing และ Lifetime โดย Generic Type ยังคงอยู่ภายใต้กฎการจัดการ Memory ของ Rust ซึ่งเป็นกลไกหลักในการจัดการ Memory ของภาษา โดย Compiler จะตรวจสอบการเป็นเจ้าของ การยืม และอายุการใช้งานของข้อมูลตั้งแต่ Compile Time ทำให้ Generic Code ยังคงอยู่ภายใต้กฎของ Memory Safety โดยสามารถป้องกันปัญหาด้าน Memory ผ่านการตรวจสอบแบบ Static ได้ ลดความจำเป็นในการจัดการ Generic Type แบบ Dynamic
ในด้านการจัดการ Resource Rust ใช้ Monomorphization ในการ Compile Generic Code ไปสร้างเป็นรูปแบบที่สอดคล้องกับ Concrete Type ที่ถูกใช้งานในขั้น Compile Time แทนการต้องจัดการ Generic Type แบบ Dynamic ระหว่าง Runtime แนวทางนี้ช่วยให้การใช้ Generics ไม่จำเป็นต้องเพิ่มกลไก Runtime สำหรับการตรวจสอบหรือเลือก Type
จึงแสดงให้เห็นความสัมพันธ์ระหว่าง Type System, Memory Management และ Compilation อย่างชัดเจน เพราะข้อกำหนดของ Type และกฎด้าน Memory ถูกตรวจสอบก่อนโปรแกรมทำงาน ขณะที่ Generic Abstraction สามารถถูกแปลงเป็นโค้ดที่เหมาะสมก่อน Runtime (ไม่ต้องเพิ่มต้นทุน Runtime ที่ไม่จำเป็น)`

### 9.5 Abstraction / Other PPL Concepts
`การ Abstraction ทำผ่าน trait ซึ่งกำหนดว่าชนิดข้อมูลหนึ่งสามารถทำอะไรได้ โดยไม่ระบุว่าเก็บข้อมูลอย่างไร ทำให้แยกพฤติกรรมออกจากข้อมูลได้อย่างชัดเจน นอกจากนี้ยังสามารถเพิ่มการ implement trait ให้กับชนิดที่มีอยู่แล้วภายหลังภายใต้ orphan rule ได้อีกด้วย ต่างจากภาษาเชิงวัตถุแบบดั้งเดิมที่ต้องประกาศความสัมพันธ์ไว้ตั้งแต่ตอนนิยามคลาส
Rust ไม่มี Class Inheritance แต่ใช้แนวคิด composition และ trait แทน คือ สร้างความสามารถใหม่จากการประกอบชนิดข้อมูลเข้าด้วยกัน และกำหนดพฤติกรรมร่วมผ่าน trait จึงหลีกเลี่ยงปัญหาที่พบในระบบสืบทอดได้
เมื่อเปรียบเทียบกับภาษาอื่น จะมีการทำ Generics ต่างกัน เช่น `Rust` ใช้ Monomorphization ร่วมกับ trait bound ที่ประกาศชัดเจน ไม่มีต้นทุนขณะรัน แต่ `C++` ใช้ templates ซึ่งให้ประสิทธิภาพใกล้เคียงกัน แต่เดิมตรวจเงื่อนไขแบบโดยนัยตอน instantiate (ปัจจุบันมี Concepts ใน C++20 ช่วย) ในขณะที่ `Java` ใช้ type erasure ลบข้อมูลชนิดทิ้งหลัง compile ทำให้ต้องใช้ boxing และ cast ขณะรัน หรือ `C#` ใช้ reified generics คือคงข้อมูลชนิดไว้ขณะรัน มีต้นทุนเล็กน้อย และ `Python` เป็นภาษา dynamically typed จึงไม่ต้องใช้ generics เพื่อให้โค้ดรับได้หลายชนิด ใช้ duck typing เป็นหลัก และมี generics เพียงในระดับ type hint ที่ให้เครื่องมืออย่าง mypy ตรวจสอบ ตัวภาษาเองไม่บังคับและ type hint ถูกละเว้นขณะรัน ข้อผิดพลาดด้านชนิดจึงเกิดขณะรัน และทุกการเรียกเป็น dynamic dispatch จึงมีต้นทุนขณะรันสูงกว่า เป็นต้น`

### 9.6 Why Rust?
`1. รองรับการสร้างโค้ดที่มีประสิทธิภาพ : Generics ของ Rust ใช้ Monomorphization ทำให้ Generic Code สามารถถูก specialize ตั้งแต่ Compile Time และหลีกเลี่ยง Runtime Generic Dispatch ในกรณีดังกล่าว`
`2. Generics ผสานกับ ownership และ lifetime ในระบบเดียว : Type parameter, trait bound และ lifetime อยู่ในกลไกเดียวกัน เช่น T: Send + 'static บอกทั้งความปลอดภัยเมื่อใช้ข้ามเธรดและอายุของข้อมูลในบรรทัดเดียว`
`3. Signature เป็นสัญญาที่ตรวจสอบได้ : อ่านเพียงหัวฟังก์ชันก็ทราบว่าชนิดที่ส่งเข้ามาต้องมีความสามารถอะไร ผู้ใช้ไม่ต้องอ่าน body และ compiler รับประกันว่า body ไม่ใช้ความสามารถเกินที่ประกาศ`
`4. Zero-cost abstraction : เขียนโค้ดระดับสูง เช่น iterator chain ได้โดยไม่เสียประสิทธิภาพเมื่อเทียบกับการเขียนระดับต่ำด้วยมือ จึงเหมาะกับ systems programming ที่ต้องการทั้งควบคุมทรัพยากรและโค้ดที่ดูแลง่าย`
`5. ไม่ต้องพึ่ง garbage collector : Rust ใช้ Ownership, Borrowing และ Drop โดยไม่ต้องพึ่ง Garbage Collector เพราะการไม่มี GC เป็นเรื่องของระบบ Resource Management ไม่ใช่ Generics โดยตรง`

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
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
fn main() {
    println!("{}", max(3, 7));
}
```

### Java Example

```Java
public class Main {
    static <T extends Comparable<T>> T max(T a, T b) {
        return (a.compareTo(b) > 0) ? a : b;
    }

    public static void main(String[] args) {
        System.out.println(max(3, 7));
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
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
fn main() {
    println!("{}", max(3, 7));
}
```

### Python Example

```Python
from typing import TypeVar

T = TypeVar('T')

def max_value(a: T, b: T) -> T:
    return a if a > b else b

print(max_value(3, 7))
```

### Analysis
- ทั้งสองภาษามีวิธีเขียนโค้ดที่ทำงานร่วมกับหลาย Type ได้โดยไม่ต้องเขียนฟังก์ชันซ้ำสำหรับแต่ละ Type (Rust ผ่าน Generic จริงในภาษา ส่วน Python ผ่าน Duck Typing ที่ทำได้อยู่แล้วโดยธรรมชาติ บวกกับ Type Hint ที่เพิ่มเข้ามาทีหลังเพื่อช่วยด้านเอกสารและเครื่องมือ)

- Rust เป็น Static Type System ที่ Compiler บังคับตรวจสอบ Generic และ Trait Bound ก่อนโปรแกรมจะรันได้เลย ทำให้ข้อผิดพลาดเรื่อง Type ถูกจับตั้งแต่ Compile Time ทั้งหมด ในขณะที่ Python ไม่มี Compile-time Type Checking ในตัวภาษาเลย จึงต้องพึ่งเครื่องมือภายนอกอย่าง mypy ในการตรวจสอบแทน Compiler ของภาษาเอง

Comparison Language: C

| Aspect | Rust | C |
|---|---|---|
| Syntax | ใช้ `<T>` ระบุ Type Parameter พร้อม Trait Bound เป็น Syntax หลักของภาษาที่ออกแบบมาเพื่อ Generic โดยเฉพาะ | ไม่มี Syntax สำหรับ Generic โดยตรง ภาษา C ไม่รองรับ Parametric Polymorphism เลย วิธีที่ใกล้เคียงที่สุดคือ `_Generic` keyword (เพิ่มเข้ามาใน C11) ที่ใช้เลือก expression ตาม Type ของ argument เช่น `#define max(a,b) _Generic((a), int: max_int, double: max_double, default: max_int)(a,b)` |
| Semantics / Behavior | Compiler ตรวจสอบและ Monomorphize ตั้งแต่ Compile Time สร้างโค้ดจริงแยกสำหรับแต่ละ Type ที่ใช้งาน ไม่มี Runtime Overhead | Generic ถูก Resolve ตอน Compile Time เช่นกัน แต่เป็นการเลือก function ที่มีอยู่แล้วจาก Type ที่ระบุไว้ตายตัว ไม่ใช่การสร้างโค้ดใหม่จาก Template แบบ Rust ถ้า Type ไม่อยู่ใน list ที่ระบุไว้ จะ fallback ไป `default` หรือ error ตอน Compile |
| Type System | Static Type System ที่ผูกกับ Trait System บังคับให้ Type ต้อง implement พฤติกรรมที่ต้องการก่อน | Static Type System แต่ไม่มีกลไก Trait Bound แบบ Rust โดย Generic สามารถเลือก expression ตาม Type ที่ระบุไว้ล่วงหน้า จึงไม่สามารถกำหนดข้อกำหนดเชิงพฤติกรรม |
| Memory Management | ใช้ Ownership และ Borrowing จัดการหน่วยความจำ ไม่มี Garbage Collector ปล่อย Resource แบบกำหนดเวลาแน่นอนโดย Compiler ตรวจสอบให้ | จัดการหน่วยความจำเองทั้งหมดผ่าน `malloc`/`free` ไม่มี Compiler หรือ Runtime ใดๆ คอยตรวจสอบว่าปล่อย Memory ถูกที่ถูกเวลาหรือไม่ ผู้เขียนต้องรับผิดชอบเองทั้งหมด |
| Safety | Compiler ตรวจสอบ Type และกฎ Ownership ก่อนโปรแกรมทำงาน ป้องกัน Memory Bug ได้ตั้งแต่ Compile Time | ไม่มีการตรวจสอบ Memory Safety ใดๆ ทั้งจาก Compiler หรือ Runtime การใช้ `void*` ร่วมกับ Generic แบบ Manual อาจทำให้เกิด Undefined Behavior ได้ง่าย เช่น cast ผิด Type แล้วอ่านค่าผิดเพี้ยนโดยไม่มี error ใดๆ เตือน |

### Rust Example
```rust
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", max(3, 7));
}
```

### C Example

```c
#include <stdio.h>

int max_int(int a, int b) { return a > b ? a : b; }
double max_double(double a, double b) { return a > b ? a : b; }

#define max(a, b) _Generic((a), \
    int: max_int, \
    double: max_double, \
    default: max_int \
)(a, b)

int main(void) {
    printf("%d\n", max(3, 7));
    return 0;
}
```

### Analysis
- ทั้งสองภาษาต้องการโค้ดที่ทำงานกับหลาย Type ได้เหมือนกัน และการ Resolve ว่าจะใช้ Type ไหนก็เกิดขึ้นตอน Compile Time เหมือนกันทั้งคู่ (Rust ผ่าน Monomorphization, C ผ่าน Generic ที่ Resolve ตอน Compile เช่นกัน)

- ส่วนที่ต่างกันคือ Rust รองรับ Parametric Polymorphism คือเขียนฟังก์ชันครั้งเดียวใช้ได้กับ Type ใดก็ได้ที่ผ่าน Trait Bound (Open Set ไม่จำกัดจำนวน Type) ส่วน Generic ของ C เป็นเพียง Compile-time Dispatch ตาม Type ที่ระบุไว้ล่วงหน้า ต้องเขียนฟังก์ชันแยกสำหรับแต่ละ Type เองด้วยมือแล้วใช้ Generic (`max_int`, `max_double`) แค่เลือกว่าจะเรียกอันไหน ไม่ได้ generate โค้ดให้อัตโนมัติเหมือน Rust หรือก็คือ C ไม่มี Generics ในความหมายที่ Rust/Java/C++ เข้าใจ มีแค่กลไกจำลองพฤติกรรมบางส่วนเท่านั้น และเรื่อง Memory Safety ก็ต่างกัน เพราะ C ไม่มี Compiler ช่วยตรวจสอบต่างจาก Rust ที่ผูก Type System เข้ากับ Ownership เพื่อรับประกัน Safety ตั้งแต่ Compile Time

## 12. References

1. The Rust Programming Language
2. Oracle Java Tutorials
3. Python official docs

4. cppreference
5. JetBrains : Rust vs Java

*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
