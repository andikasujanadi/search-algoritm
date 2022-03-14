use nanorand::{Rng, WyRand};
use std::io::{self, Write};
use std::process;

fn refresh(data: &mut [i32]) {
    let size = data.len();
    let mut rng = WyRand::new();

    for it in 0..size {
       data[it] = rng.generate::<i32>() % 100 + 1;
    }
}

fn sequence_search(key: i32, data: &[i32]) -> Option<usize> {
    for (it, item) in data.iter().enumerate() {
            if key == *item {
                return Some(it);
        }
    }

    None
}

fn sorted_sequence_search(key: i32, data: &[i32]) -> Option<usize> {
    for (it, item) in data.iter().enumerate() {
        if key == *item {
            return Some(it);
        }

        else if key < *item {
            break;
        }
    }

    None
}

fn biner(key: i32, data: &[i32]) -> Option<usize> {
    let mut left = 0;
    let mut right = data.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if key == data[mid] {
           return Some(mid);
        }

        else if key < data[mid] {
            right = mid - 1;
        }

        else {
            left = mid + 1;
        }
    }

    None
}

fn partition(part: &mut [i32], 
             left: usize, 
             right: usize) -> usize {
    let piv = part[left];
    let mut front = left;
    let mut rear = right;

    while front <= rear {
        while piv > part[front] {
            front += 1;
        }

        while piv < part[rear] {
            rear -= 1;
        }

        if rear > front {
            part[front] ^= part[rear];
            part[rear] ^= part[front];
            part[front] ^= part[rear];

            front += 1;
            rear -= 1;
        }
        else {
            return rear;
        }
    }
    rear
}

fn quick_sort(part: &mut [i32], 
              left: usize, 
              right: usize) {
    if left < right {
        let idx = partition(part, left, right);
        quick_sort(part, left, idx);
        quick_sort(part, idx + 1, right);
    }
}

fn user_input(msg: &str) -> i32 {
    print!("{msg}");
    io::stdout().flush().expect("Gagal flush");

    let mut st = String::new();
    io::stdin().read_line(&mut st).expect("Tidak dapat menerima masukan");
    let num: i32 = st.trim().parse().expect("Hanya masukan angka!");

    num
}


fn main() {
    const N: usize = 30; // array size
     
    let mut data = [0i32; N];
    refresh(&mut data);

    let mut key: i32 = 0;
    let mut res: Option<usize> = None;

    loop {
        let mut operatable = false;

        println!("Data: {data:?}");

        println!("\n1. Sekuensial");
        println!("2. Biner");
        println!("3. Sekuensial urut");
        println!("98. Segarkan"); // Bahasa Indonesia
        println!("99. Keluar");

        
        let opt = user_input("Masukan pilihan: ");

        if opt >= 1 && opt <= 3 {
            key = user_input("Masukan kunci: ");
            operatable = true;
        }

        else if opt == 98 {
            refresh(&mut data);
        }

        else if opt == 99 {
            process::exit(0);
        }

        if operatable {
            match opt {
                1 => {
                    res = sequence_search(key, &data);
                },

                2 => {
                    quick_sort(&mut data, 0, N-1);
                    res = biner(key, &data);
                },

                3 => {
                    quick_sort(&mut data, 0, N-1);
                    res = sorted_sequence_search(key, &data);
                },

                

                _ => {
                    println!("Pilihan tidak ada")
                },
            }

            match res {
                Some(x) => println!("Ditemukan pada indeks ke-{x}\n"),
                None => println!("Tidak ditemukan"),
            }
        }
    }
}
