struct A {
    a:i64,
    b:u64,
}
//内存对齐，中间有填充u32
struct A1{
    a:i32,
    b:u64,
}
//变量顺序进行重拍，优化struct大小,ac排列在一起,b.d排列在一起
struct A2{
    a:i32,
    b:u64,
    c:i32,
    d:u64,
}
struct SimpleVec<T>{
    len:usize,
    capacity:usize,
    data: * const T //指针，指向元素
}

struct SimpleVec_2<T>{
    len:usize,
    capacity:usize,
    data:* mut T, //动态指针
}

enum Data {
    //tag,8
    I64(i32),
    F64(f64),
    Bytes(SimpleVec<u8>)
}

enum Option {
    Some,
    None,
}

type OptData = Option<Data>;
fn main() {
    println!("Hello, world!");
    println!("size of i32:{}",std::mem::size_of::<i32>());
    println!("size of i64:{}",std::mem::size_of::<i64>());
    println!("size of [i32:4]:{}",std::mem::size_of::<[i32;4]>());

}
