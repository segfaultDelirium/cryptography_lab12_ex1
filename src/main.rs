use std::cmp::Reverse;
use std::fmt::format;


fn find_p_q(n: u32) -> (u32, u32) {

    let p = (100..200).into_iter().find(|x| {
        let res = n as f32/(*x as f32);
        let int_res = n/x;
        return (res - int_res as f32).abs() < 0.001
    }).unwrap();

    let q = n / p;
    (p, q)
    // (0, 0)
}

fn find_a(n: u32, x: u32) -> u32 {
    let res = odwrotnosc_multiplikatywna(x as i32, n as i32) as u32;

    return res;
}


// DOG = 3 * 26 ^ 2 + 14 * 26 ^ 1 + 6 * 26 ^ 0 = 2398
// trzeba znalezc liczby 2, 14, 6
// n = 18923 = liczba 3 cyfrowa zaczynajaca sie od 1 * inna liczba 3 cyfrowa zaczynajaca sie od 1
// b= 1261


fn to_char(c: u32) -> char {
    char::from_u32('a' as u32 + c as u32).unwrap()
}

// (char, char, char)
fn get_abc_from_number(x: u32) -> Vec<char>{
    let c = modulo_euclid(x as i32, 26);
    let c_char = to_char(c as u32);//char::from_u32('a' as u32 + c as u32).unwrap();
    let remaining = x - c as u32;


    let a = remaining / 26u32.pow(2);
    let b = (remaining - a * 26u32.pow(2)) / 26;
    // println!("b = {:?}", b);

    let a_char = to_char(a);
    let b_char = to_char(b);
    return vec![a_char, b_char, c_char];
    // return (a_char, b_char, c_char);
}

fn main() {
    let table = vec![
        12423, 11524, 7243, 7459, 14303, 6127, 10964, 16399,
        9792, 13629, 14407, 18817, 18830, 13556, 3159, 16647,
        5300, 13951,
        81, 8986, 8007, 13167, 10022, 17213,
        2264, 961, 17459, 4101, 2999, 14569, 17183, 15827,
        12693, 9553, 18194, 3830, 2664, 13998, 12501, 18873,
        12161, 13071, 16900, 7233, 8270, 17086, 9792, 14266,
        13236, 5300, 13951, 8850, 12129, 6091, 18110, 3332,
        15061, 12347, 7817, 7946, 11675, 13924, 13892, 18031,
        2620, 6276, 8500, 201, 8850, 11178, 16477, 10161,
        3533, 13842, 7537, 12259, 18110,
        44, 2364, 15570,
        3460, 9886, 8687, 4481, 11231, 7547, 11383, 17910,
        12867, 13203, 5102, 4742, 5053, 15407, 2976, 9330,
        12192,
        56, 2471, 15334, 841, 13995, 17592, 13297,
        2430, 9741, 11675, 424, 6686, 738, 13874, 8168,
        7913, 6246, 14301, 1144, 9056, 15967, 7328, 13203,
        796, 195, 9872, 16979, 15404, 14130, 9105, 2001,
        9792, 14251, 1498, 11296, 1105, 4502, 16979, 1105,
        56, 4118, 11302, 5988, 3363, 15827, 6928, 4191,
        4277, 10617, 874, 13211, 11821, 3090, 18110,
        44,
        2364, 15570, 3460, 9886, 9988, 3798, 1158, 9872,
        16979, 15404, 6127, 9872, 3652, 14838, 7437, 2540,
        1367, 2512, 14407, 5053, 1521, 297, 10935, 17137,
        2186, 9433, 13293, 7555, 13618, 13000, 6490, 5310,
        18676, 4782, 11374, 446, 4165, 11634, 3846, 14611,
        2364, 6789, 11634, 4493, 4063, 4576, 17955, 7965,
        11748, 14616, 11453, 17666, 925,
        56, 4118, 18031,
        9522, 14838, 7437, 3880, 11476, 8305, 5102, 2999,
        18628, 14326, 9175, 9061, 650, 18110, 8720, 15404,
        2951, 722, 15334, 841, 15610, 2443, 11056, 2186,
    ];


    let n = 18923;
    let b = 1261;
    let (p, q) = find_p_q(n);
    println!("p = {p}, q = {q}");
    let fi_n = (p-1) * (q-1);
    println!("fi_n = {fi_n}");

    let a = find_a(fi_n, b);
    println!("a = {a}");

    let decrypted_table = table.into_iter().map(|x|{
        let res = potegowanie(x, a, n);
        let letters = get_abc_from_number(res).into_iter().map(|c| format!("{c}"))
            .reduce(|acc, s| format!("{acc}{s}")).unwrap()
            ;
        return letters;
    }).collect::<Vec<_>>();
    println!("decrypted_table: {:?}", decrypted_table);

}

fn potegowanie(a: u32, e: u32, n: u32) -> u32 {
    let e_binary = reverse(create_binary(e));
    let mut d = 1;
    let mut i = e_binary.len() as i32 -1;
    while(i >= 0){
        d = modulo_euclid(d*d, n as i32);
        if e_binary[i as usize] == 1{
            d = modulo_euclid(d * a as i32, n as i32)
        }

        i -= 1;
    }
    return d as u32;
}

fn create_binary(value: u32) -> Vec<u32>{
    let binary_string = format!("{:b}", value);
    let res = binary_string.chars().into_iter().map(|c| if c == '0' {0} else {1}).collect();
    return res;
}

fn reverse(vec: Vec<u32>) -> Vec<u32>{
    let mut vec_clone = vec.clone();
    vec_clone.reverse();
    // vec_clone.sort_by_key(|&num| (false , Reverse(num)));
    return vec_clone;
}


fn modulo_euclid(j: i32, k: i32) -> i32 {
    let res =  j % k;
    if res < 0 {return res + k} else {return res}
}

fn rozNWD(j: i32, k: i32) -> (i32, i32, i32) {
    if j == 0 {
        return (k, 0, 1)
    }
    // let r = k % j;
    let r = modulo_euclid(k, j);
    // let r = k % j;
    let (d, x_prim, y_prim) = rozNWD(r, j);
    let x = y_prim - (k/j) * x_prim;
    let y = x_prim;
    return (d, x, y);

    // (k, 1, 0)
}


fn odwrotnosc_multiplikatywna(j: i32, k: i32) -> i32 {
    // a to jest 17 czyli j
    // n = 101 czyli k
    let (_d, x, _y) = rozNWD(j, k);
    // println!("d = {d}, x = {x}, y = {y}");
    return modulo_euclid(x, k);
}
