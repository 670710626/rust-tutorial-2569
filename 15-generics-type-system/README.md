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
| 4 | นายธนเทพ นาสวน | 670710626 | `@670710626` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

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
