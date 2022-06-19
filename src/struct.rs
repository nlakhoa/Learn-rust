#[derive(Debug)]

struct Member {
    name: String,
    email: String,
    age: u64,
    active: bool,
}
struct hinh_cn {
    dai: u32,
    rong: u32,
}
impl hinh_cn {
    fn dien_tich(&self) ->u32{
        self.dai * self.rong
    }

    fn chua(&self,hinhchunhatkhac: &hinh_cn) -> bool{
        self.dai > hinhchunhatkhac.dai && self.rong > hinhchunhatkhac.rong
    }
}


fn main() {
    // Phan 1
    let member1 = Member {
        name: String::from("Nguyen Le Anh Khoa 1"),
        email: String::from("anhkhoa1@gmail.com"),
        age: 21,
        active: true,
    };

    let member2 = crate_member(String::from("Khoa 2"), String::from("khoa2@gmail.com"), 21);

    let member3 = Member {
        name: String::from("Khoa 3"),
        ..member2
    };
    println!("{:#?}{:#?}", member1, member3);

    // Phan 2
    let hinh_chu_nhat = (30, 50);
    println!("Dien tich hinh chu nhat = {}", dien_tich(hinh_chu_nhat));
    
    let kichthuoc = hinh_cn { dai: 30, rong: 40 };

    println!("Dien tich hinh chu nhat = {}",kichthuoc.dien_tich());

    let kichthuoc1 = hinh_cn { dai: 20, rong: 30 };
    println!("Hinh chu nhat co the chua hinh 2 {}",kichthuoc.chua(&kichthuoc1));
    let kichthuoc2 = hinh_cn { dai: 40, rong: 50 };

    println!("Hinh chu nhat co the chua hinh 3 {}",kichthuoc.chua(&kichthuoc2));

    println!("Dien tich hinh chu nhat = {}", dien_tich_cn(&kichthuoc));



}

fn crate_member(name: String, email: String, age: u64) -> Member {
    Member {
        name,
        email,
        age,
        active: false,
    }
}

fn dien_tich(kichthuoc: (u32, u32)) -> u32 {
    kichthuoc.0 * kichthuoc.1
}

fn dien_tich_cn(kichthuoc: &hinh_cn) -> u32 {
    kichthuoc.dai * kichthuoc.rong
}
