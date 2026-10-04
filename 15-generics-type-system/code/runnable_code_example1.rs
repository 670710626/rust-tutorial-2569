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