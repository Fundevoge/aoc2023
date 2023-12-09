use std::{fmt::Display, ops::Div};

use itertools::Itertools;
use num::{Rational64, Signed, Zero};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynom {
    coefficients: Vec<Rational64>,
}
impl Polynom {
    pub fn phi_1() -> Self {
        Polynom {
            coefficients: vec![(-1).into(), 1.into()],
        }
    }

    pub fn x_n_1(n: usize) -> Self {
        let mut coefficients = vec![Rational64::zero(); n + 1];
        coefficients[0] = (-1).into();
        coefficients[n] = 1.into();
        Polynom { coefficients }
    }

    pub fn from_raw(coefficients: Vec<Rational64>) -> Self {
        Polynom { coefficients }
    }

    pub fn from_raw_ints(coefficients: Vec<i64>) -> Self {
        Polynom {
            coefficients: coefficients.into_iter().map(Rational64::from).collect(),
        }
    }

    pub fn zero() -> Self {
        Polynom {
            coefficients: vec![],
        }
    }

    pub fn new_cyclotomic(n: usize, cycl: &[Polynom]) -> Self {
        let mut poly = Self::x_n_1(n);
        for i in 1..=n / 2 {
            if n % i == 0 {
                poly = poly.div(&cycl[i - 1]).unwrap();
            }
        }
        poly
    }

    pub fn try_div(mut self, rhs: &Self) -> Option<Self> {
        let mut new_coefficients = vec![];
        while self.coefficients.len() >= rhs.coefficients.len() {
            let top_coeff = self.coefficients.pop().unwrap() / rhs.coefficients.last().unwrap();
            new_coefficients.push(top_coeff);
            for (num_c, den_c) in self
                .coefficients
                .iter_mut()
                .rev()
                .zip(rhs.coefficients.iter().rev().skip(1))
            {
                *num_c -= top_coeff * den_c;
            }
        }

        self.coefficients.iter().all(|c| c.is_zero()).then(|| {
            new_coefficients.reverse();
            Polynom {
                coefficients: new_coefficients,
            }
        })
    }

    pub fn is_unit_z2(&self) -> bool {
        self.len() == 1
    }

    pub fn factorize_into_parts_z2(mut self, target_degree: usize) -> Vec<Self> {
        let mut basics = vec![];
        let mut has_changed = true;
        while self.degree() > target_degree && has_changed {
            has_changed = false;
            // println!("Exploring {self}");
            let q_minus_i = self.q_matrix_minus_i_z2();
            // print_matrix(&q_minus_i);
            let kernel_basis = kernel_basis_z2_knuth(q_minus_i)
                .into_iter()
                .map(|row| row.into_iter().map(Rational64::from).collect_vec())
                .collect_vec();
            // print_matrix(&kernel_basis);

            for basis in kernel_basis.into_iter().skip(1) {
                let mut b = Polynom::from_raw(basis);
                b.truncate_coefficients();
                let b_prime = b.with_toggled_coeff_0();

                for b in [b, b_prime] {
                    let f = b.gcd_z2(&self);
                    // println!("b_{i} : {b}; gcd: {f}");

                    if !f.is_unit_z2() && !basics.contains(&f) {
                        if f.degree() == target_degree {
                            self = (self.div_z2(&f)).unwrap();
                            basics.push(f);
                        } else if f.degree() > target_degree && f.degree() % target_degree == 0 {
                            for b in (self.clone().div_z2(&f))
                                .unwrap()
                                .factorize_into_parts_z2(target_degree)
                            {
                                if !basics.contains(&b) {
                                    self = (self.div_z2(&b)).unwrap();
                                    basics.push(b);
                                }
                            }
                            for b in f.factorize_into_parts_z2(target_degree) {
                                if !basics.contains(&b) {
                                    self = (self.div_z2(&b)).unwrap();
                                    basics.push(b);
                                }
                            }
                        } else {
                            panic!(
                                "Don't know what to do with {f} for target degree {target_degree}"
                            );
                        }
                        has_changed = true;
                    }
                }
            }
        }
        if !has_changed {
            println!("Could not factor {self}");
        } else if self.degree() == target_degree {
            basics.push(self);
        }

        basics
    }

    pub fn div_z2(mut self, rhs: &Self) -> Result<Self, Self> {
        let mut new_coefficients = vec![];
        while self.coefficients.len() >= rhs.coefficients.len() {
            let top_coeff = self.coefficients.pop().unwrap() / rhs.coefficients.last().unwrap();
            new_coefficients.push(top_coeff);
            for (num_c, den_c) in self
                .coefficients
                .iter_mut()
                .rev()
                .zip(rhs.coefficients.iter().rev().skip(1))
            {
                *num_c -= top_coeff * den_c;
            }
        }
        self = self.into_z2();

        if self.coefficients.iter().all(|c| c.is_zero()) {
            new_coefficients.reverse();
            let new_poly = Polynom {
                coefficients: new_coefficients,
            }
            .into_z2();
            Ok(new_poly)
        } else {
            Err(self)
        }
    }

    pub fn len(&self) -> usize {
        self.coefficients.len()
    }

    pub fn is_empty(&self) -> bool {
        self.coefficients.len() == 0
    }

    pub fn gcd_z2(&self, other: &Self) -> Self {
        let (left, right) = if self.len() >= other.len() {
            (self, other)
        } else {
            (other, self)
        };
        let mut left = left.clone();
        let mut right = right.clone();
        loop {
            match left.div(&right) {
                Ok(_) => return right,
                Err(rem) => {
                    let rem = rem.into_z2();
                    if rem.is_empty() {
                        return right;
                    }
                    (left, right) = if right.len() >= rem.len() {
                        (right, rem)
                    } else {
                        (rem, right)
                    };
                }
            }
        }
    }

    pub fn into_z2(mut self) -> Self {
        for c in &mut self.coefficients {
            *c %= 2;
            if c < &mut Rational64::zero() {
                *c += 2;
            }
        }
        self.truncate_coefficients();
        self
    }

    pub fn cyclotomics(max: usize) -> Vec<Self> {
        let mut cyclotomic = vec![Polynom::phi_1()];
        for i in 2..=max {
            cyclotomic.push(Polynom::new_cyclotomic(i, &cyclotomic));
            println!("[{i}]: {}", cyclotomic.last().unwrap());
        }
        cyclotomic
    }

    pub fn truncate_coefficients(&mut self) {
        if let Some((max_i, _)) = self
            .coefficients
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.is_zero())
            .last()
        {
            self.coefficients.truncate(max_i + 1);
        } else {
            self.coefficients.clear();
        }
    }

    pub fn with_toggled_coeff_0(&self) -> Self {
        let mut r = self.clone();
        r.flip_one_zero(0);
        r
    }

    fn flip_one_zero(&mut self, idx: usize) {
        self.coefficients[idx] = if self.coefficients[idx].is_zero() {
            1.into()
        } else {
            Rational64::zero()
        };
    }

    pub fn degree(&self) -> usize {
        self.len() - 1
    }

    pub fn modulo_z2(self, other: &Self) -> Self {
        match self.div(other) {
            Ok(_) => Self::zero(),
            Err(rem) => rem.into_z2(),
        }
    }

    pub fn q_matrix_minus_i_z2(&self) -> Vec<Vec<i64>> {
        let mut q = Vec::new();
        let n = self.degree();
        for i in 0..self.degree() {
            let mut x_iq = vec![0; 2 * i];
            x_iq.push(1);
            let x_iq = Polynom::from_raw_ints(x_iq);

            let mut m = x_iq
                .modulo_z2(self)
                .coefficients
                .into_iter()
                .map(|c| c.to_integer())
                .collect_vec();
            m.extend((0..=0).cycle().take(n - m.len()));

            q.push(m);
        }
        // println!("Matrix without unit:");
        // print_matrix(&q);
        for (i, qr) in q.iter_mut().enumerate() {
            qr[i] ^= 1;
        }
        // println!("Matrix with unit:");
        // print_matrix(&q);
        q
    }

    pub fn multiply_z2(&self, rhs: &Self) -> Self {
        let mut results = vec![0; self.degree() + rhs.degree() + 1];
        for (i, v_l) in self.coefficients.iter().enumerate() {
            for (j, v_r) in rhs.coefficients.iter().enumerate() {
                let v_rl = v_r * v_l;
                results[i + j] = if results[i + j].is_zero() ^ v_rl.is_zero() {
                    1
                } else {
                    0
                };
            }
        }
        Polynom::from_raw_ints(results)
    }

    pub fn mod_pow_z2(&self, exponent: u128, modulus: &Self) -> Self {
        let mut mask = 1_u128;
        while mask <= exponent {
            mask <<= 1;
        }
        mask >>= 1;

        let mut result = Polynom::from_raw_ints(vec![1]);
        while mask > 1 {
            if mask & exponent > 0 {
                result = result.multiply_z2(self).modulo_z2(modulus);
            }
            result = result.multiply_z2(&result).modulo_z2(modulus);
            mask >>= 1;
        }
        if mask & exponent > 0 {
            result = result.multiply_z2(self).modulo_z2(modulus);
        }

        result
    }
}

impl Div<&Polynom> for Polynom {
    type Output = Result<Self, Self>;

    fn div(mut self, rhs: &Self) -> Result<Self, Self> {
        let mut new_coefficients = vec![];
        while self.coefficients.len() >= rhs.coefficients.len() {
            let top_coeff = self.coefficients.pop().unwrap() / rhs.coefficients.last().unwrap();
            new_coefficients.push(top_coeff);
            for (num_c, den_c) in self
                .coefficients
                .iter_mut()
                .rev()
                .zip(rhs.coefficients.iter().rev().skip(1))
            {
                *num_c -= top_coeff * den_c;
            }
        }

        if self.coefficients.iter().all(|c| c.is_zero()) {
            new_coefficients.reverse();
            Ok(Polynom {
                coefficients: new_coefficients,
            })
        } else {
            self.truncate_coefficients();
            Err(self)
        }
    }
}

impl Display for Polynom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "0")
        } else {
            let mut coeff_strings = self
                .coefficients
                .iter()
                .enumerate()
                .filter(|(_, c)| !c.is_zero())
                .map(|(i, c)| {
                    if i == 0 {
                        format!(" {} {}", if c >= &1.into() { "+" } else { "-" }, c.abs())
                    } else {
                        let mut v = if c >= &1.into() {
                            " + ".into()
                        } else {
                            " - ".into()
                        };
                        if c != &1.into() && c != &(-1).into() {
                            v += c.abs().to_string().as_str();
                        }
                        v += "x";
                        if i != 1 {
                            v + format!("^{i}").as_str()
                        } else {
                            v
                        }
                    }
                })
                .rev();
            let first = coeff_strings.next().unwrap();
            write!(
                f,
                "{}",
                if !self.coefficients.last().unwrap().is_negative() {
                    &first[3..]
                } else {
                    &first[1..]
                }
            )?;
            for cs in coeff_strings {
                write!(f, "{cs}",)?;
            }
            Ok(())
        }
    }
}

pub fn lcm(a: u128, b: u128) -> u128 {
    if a == 0 && b == 0 {
        return 0;
    }

    a * b / gcd(a, b)
}

pub fn gcd(mut a: u128, mut b: u128) -> u128 {
    (a, b) = if a >= b { (a, b) } else { (b, a) };

    while a > 1 || b > 1 {
        if a % b == 0 {
            return b;
        }

        (a, b) = (b, a - b * (a / b));
    }
    1
}

pub fn check_prime(n: u64, primes: &[u64]) -> bool {
    for &p in primes {
        if n % p == 0 {
            return false;
        }
        if p * p > n {
            break;
        }
    }
    true
}

pub fn gen_primes(max: u64) -> Vec<u64> {
    let mut primes = vec![2_u64, 3, 5];
    let mut candidate = 7;
    while candidate < max {
        for _ in 0..3 {
            if check_prime(candidate, &primes) {
                primes.push(candidate);
            }
            candidate += 2;
        }
        if check_prime(candidate, &primes) {
            primes.push(candidate);
        }
        candidate += 4;
    }
    while primes.last().unwrap() > &max {
        primes.pop();
    }
    primes
}

pub fn divisors(num: u128, primes: &[u64]) -> Vec<u128> {
    let factors = factorize_with_primes(num, primes);
    // for every divisor: take it 0..count times and multiply with every other divisor
    let mut divisors = vec![1_u128];
    for (factor, max_count) in factors {
        let mut new_divisors = Vec::new();
        for d in divisors {
            for amount in 0..max_count + 1 {
                new_divisors.push(d * factor.pow(amount as u32));
            }
        }
        divisors = new_divisors;
    }
    divisors
}

pub fn factorize_with_primes(mut num: u128, primes: &[u64]) -> Vec<(u128, usize)> {
    let mut factors = Vec::new();
    for &p in primes {
        let p = p as u128;
        if p > num {
            break;
        }
        if num % p == 0 {
            let mut count = 0;
            while num % p == 0 {
                count += 1;
                num /= p;
            }
            factors.push((p, count));
        }
    }
    let biggest_p = *primes.last().unwrap() as u128;
    if num > biggest_p && num < biggest_p * biggest_p {
        factors.push((num, 1));
        num = 1;
    }
    assert_eq!(num, 1);
    factors
}

pub fn s_ord_2(n: u128) -> (u128, u128) {
    let mut v = 2;
    let mut i = 1;
    loop {
        if v == 1 {
            return (i, i);
        }
        if v == n - 1 {
            return (2 * i, i);
        }
        v = (v * 2) % n;
        i += 1;
    }
}

pub fn kernel_basis_z2(mut q: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = q.len();
    let mut shadow_matrix = vec![vec![0; n]; n];
    // Start with identity matrix
    for (i, sm) in shadow_matrix.iter_mut().enumerate() {
        sm[i] = 1;
    }

    for row in 0..n {
        // First column that has a leading 1 in the given row
        if let Some(leading_col) =
            (0..n).find(|&col| q[row][col] == 1 && q[..row].iter().map(|r| r[col]).all(|e| e == 0))
        {
            // XOR all other columns that have a 1 in this row with it
            for col in 0..n {
                if col != leading_col && q[row][col] == 1 {
                    xor_columns(&mut q, col, leading_col);
                    xor_columns(&mut shadow_matrix, col, leading_col);
                }
            }
        }
    }
    // println!("After elimintation: [q]");
    // print_matrix(&q);

    // println!("After elimintation: [C]");
    // print_matrix(&shadow_matrix);

    let mut basis_vectors = vec![];
    for col in 0..n {
        if q.iter().map(|r| r[col]).all(|e| e == 0)
            && shadow_matrix.iter().skip(1).map(|r| r[col]).any(|e| e != 0)
        {
            basis_vectors.push(shadow_matrix.iter().map(|r| r[col]).collect())
        }
    }
    basis_vectors
}

pub fn kernel_basis_z2_knuth(mut q: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = q.len();
    let mut basis_vectors = vec![];
    let mut cs = vec![-1_i64; n];

    for row in 0..n {
        // First column that has a leading 1 in the given row
        if let Some(leading_col) = (0..n).find(|&col| q[row][col] == 1 && cs[col] < 0) {
            // XOR all other columns that have a 1 in this row with it
            for col in 0..n {
                if col != leading_col && q[row][col] == 1 {
                    xor_columns(&mut q, col, leading_col);
                }
            }
            cs[leading_col] = row as i64;
        } else {
            let mut v_i = vec![0; n];
            for (i, v_i) in v_i.iter_mut().enumerate() {
                if let Some((s, _)) = cs.iter().enumerate().find(|(_, c_s)| **c_s == i as i64) {
                    *v_i = q[row][s]
                } else if i == row {
                    *v_i = 1;
                }
            }
            basis_vectors.push(v_i);
        }
    }
    //println!("After elimintation: [q]");
    // print_matrix(&q);

    basis_vectors
}

pub fn xor_columns(matrix: &mut Vec<Vec<i64>>, target: usize, base: usize) {
    for r in matrix {
        r[target] ^= r[base];
    }
}

pub fn print_matrix(matrix: &[Vec<i64>]) {
    print!("[",);
    for r in matrix {
        print!("\n  {r:?},");
    }
    println!("\n]");
}

// pub fn lagrange(
//     xs: &[i64],
//     ys: &[i64],
//     tol: N::RealField,
// ) -> Result<Polynomial<N>, String>
// where
//     <N as ComplexField>::RealField: FromPrimitive + Copy,
// {
//     if xs.len() != ys.len() {
//         return Err("lagrange: slices have mismatched dimension".to_owned());
//     }

//     let mut qs = vec![Polynomial::with_tolerance(tol)?; xs.len() * xs.len()];
//     for (ind, y) in ys.iter().enumerate() {
//         qs[ind] = polynomial![*y];
//     }

//     for i in 1..xs.len() {
//         let mut poly_2 = polynomial![N::one(), -xs[i]];
//         poly_2.set_tolerance(tol)?;
//         for j in 1..=i {
//             let mut poly_1 = polynomial![N::one(), -xs[i - j]];
//             poly_1.set_tolerance(tol)?;
//             let idenom = N::one() / (xs[i] - xs[i - j]);
//             let numer =
//                 &poly_1 * &qs[i + xs.len() * (j - 1)] - &poly_2 * &qs[(i - 1) + xs.len() * (j - 1)];
//             qs[i + xs.len() * j] = numer * idenom;
//         }
//     }

//     for i in 0..=qs[xs.len() * xs.len() - 1].order() {
//         if qs[xs.len() * xs.len() - 1].get_coefficient(i).abs() < tol {
//             qs[xs.len() * xs.len() - 1].purge_coefficient(i);
//         }
//     }

//     qs[xs.len() * xs.len() - 1].purge_leading();
//     Ok(qs[xs.len() * xs.len() - 1].clone())
// }
