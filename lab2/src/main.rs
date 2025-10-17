// mod second;

#[derive(Debug, Clone, Default)]
struct NumberWithUnit {
    unit: String,
    value: f64,
}

#[allow(dead_code)]
impl NumberWithUnit {
    fn unitless(value: f64) -> Self {
        Self {
            unit: String::new(),
            value
        }
    }

    fn with_unit(value: f64, unit: String) -> Self {
        Self {
            unit,
            value
        }
    }

    fn with_unit_from(other: Self, value: f64) -> Self {
        Self {
            value,
            ..other
        }
    }

    fn add(self, other: Self) -> Self {
        assert_eq!(self.unit, other.unit);
        Self {
            unit: self.unit,
            value: self.value + other.value
        }
    }

    fn mul(self, other: Self) -> Self {
        let unit = if self.unit.is_empty() {
            other.unit
        } else {
            format!("{} * {}", self.unit, other.unit)
        };
        Self {
            unit,
            value: self.value * other.value
        }
    }

    fn div(self, other: Self) -> Self {
        assert_ne!(other.value, 0.0);
        Self {
            unit: format!("{} / {}", self.unit, other.unit),
            value: self.value / other.value
        }
    }

    fn add_in_place(&mut self, other: &Self) {
        assert_eq!(self.unit, other.unit);
        self.value += other.value;
    }

    fn mul_in_place(&mut self, other: &Self) {
        self.unit = if self.unit.is_empty() {
            other.unit.clone()
        } else {
            format!("{} * {}", self.unit, other.unit)
        };
        self.value *= other.value;
    }

    fn div_in_place(&mut self, other: &Self) {
        assert_ne!(other.value, 0.0);
        self.value /= other.value;
        self.unit = format!("{} / {}", self.unit, other.unit);
    }

}

fn mul_vals(vals: &[NumberWithUnit]) -> NumberWithUnit {
    let mut res = NumberWithUnit::unitless(1.0);
    for value in vals.iter() {
        res.mul_in_place(value);
    }
    res
}

fn mul_vals_vec(vals: Vec<NumberWithUnit>) -> NumberWithUnit {
    let mut res = NumberWithUnit::unitless(1.0);
    for value in vals.iter() {
        res.mul_in_place(value);
    }
    res
}


#[allow(dead_code)]
struct DoubleString(String, String);

#[allow(dead_code)]
impl DoubleString {
    fn from_strs(str_1: &str, str_2: &str) -> Self {
        DoubleString(str_1.to_string(), str_2.to_string())
    }

    fn from_strings(str_1: &String, str_2: &String) -> Self {
        DoubleString(str_1.clone(), str_2.clone())
    }

    fn show(&self) {
        println!("({}, {})", self.0, self.1);
    }
}

fn main() {
    let x = NumberWithUnit::unitless(21.0);
    println!("x: {:?}", x);
    let y = NumberWithUnit::with_unit(37.0, String::from("🍺"));
    println!("x: {:?}", y);
    let z = NumberWithUnit::with_unit_from(y, 23.0);
    println!("x: {:?}", z);

    let s1 = NumberWithUnit::with_unit(30.0, "km".to_string());
    let t1 = NumberWithUnit::with_unit(15.0, "min".to_string());
    let v1 = s1.div(t1);

    println!("v1 = {:?}", v1);
    
    let mut s2 = NumberWithUnit::with_unit(13.0, "astronimical unit".to_string());
    let t2 = NumberWithUnit::with_unit(2.0, "lecture duration".to_string());
    s2.div_in_place(&t2);

    println!("v2 = {:?}", s2);

    // 10.
    let values = vec![NumberWithUnit::with_unit(12.0, "🍌".to_string()), z.clone(), z];
    println!("Multiplied values = {:?}", mul_vals(&values));
    println!("Multiplied values = {:?}", mul_vals(&values));
    println!("Multiplied values = {:?}", mul_vals_vec(values.clone()));
    println!("Multiplied values = {:?}", mul_vals_vec(values));

    let string = String::from("🦀");
    let str_slice: &str = "🔫";
    let _double1 = DoubleString::from_strs(&string, str_slice); // Tutaj borrowed string zmieniamy na slice
    let _double2 = DoubleString::from_strings(&string, &str_slice.to_string()); // Tutaj trzeba stworzyć nowy string na stercie ze slice

    // Co robi to_owned()?
    // Czy to kopiuje obiekt?
}
