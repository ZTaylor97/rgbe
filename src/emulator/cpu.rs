#[derive(Default, Debug)]
pub struct CPURegisters {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl CPURegisters {
    pub fn new() -> Self {
        CPURegisters::default()
    }
    pub fn get_af(&self) -> u16 {
        convert_two_u8s_to_u16(self.a, self.f)
    }
    pub fn get_bc(&self) -> u16 {
        convert_two_u8s_to_u16(self.b, self.c)
    }
    pub fn get_de(&self) -> u16 {
        convert_two_u8s_to_u16(self.d, self.e)
    }
    pub fn get_hl(&self) -> u16 {
        convert_two_u8s_to_u16(self.h, self.l)
    }

    pub fn set_af(&mut self, value: u16) {
        (self.a, self.f) = convert_u16_to_two_u8s(value);
    }
    pub fn set_bc(&mut self, value: u16) {
        (self.b, self.c) = convert_u16_to_two_u8s(value);
    }
    pub fn set_de(&mut self, value: u16) {
        (self.d, self.e) = convert_u16_to_two_u8s(value);
    }
    pub fn set_hl(&mut self, value: u16) {
        (self.h, self.l) = convert_u16_to_two_u8s(value);
    }
}

pub fn convert_two_u8s_to_u16(first: u8, second: u8) -> u16 {
    (first as u16) << 8 | second as u16
}

pub fn convert_u16_to_two_u8s(value: u16) -> (u8, u8) {
    (((value & 0xFF00) >> 8) as u8, (value & 0xFF) as u8)
}

#[cfg(test)]
mod cpu_tests {
    use super::CPURegisters;

    #[test]
    fn test_get_two_u8s_as_u16() {
        let first = 0x1A;
        let second = 0x00F1;
        assert_eq!(super::convert_two_u8s_to_u16(first, second), 0x1AF1 as u16);
    }
    #[test]
    fn test_get_two_u8s_from_u16() {
        let test = 0x1FF1;
        assert_eq!(super::convert_u16_to_two_u8s(test), (0x1F, 0x00F1));
    }

    #[test]
    fn test_get_af() {
        let mut test_reg = CPURegisters::new();
        test_reg.a = 0xBF;
        test_reg.f = 0xF1;

        assert_eq!(test_reg.get_af(), 0xBFF1 as u16);
    }
    #[test]
    fn test_set_af() {
        let mut test_reg = CPURegisters::new();

        assert_eq!(test_reg.get_af(), 0);
        test_reg.set_af(0x1371);
        assert_eq!(test_reg.get_af(), 0x1371 as u16);
    }

    #[test]
    fn test_get_bc() {
        let mut test_reg = CPURegisters::new();
        test_reg.b = 0x1F;
        test_reg.c = 0xF1;

        assert_eq!(test_reg.get_bc(), 0x1FF1 as u16);
    }
    #[test]
    fn test_set_bc() {
        let mut test_reg = CPURegisters::new();

        assert_eq!(test_reg.get_bc(), 0);
        test_reg.set_bc(0x1FF1);
        assert_eq!(test_reg.get_bc(), 0x1FF1 as u16);
    }

    #[test]
    fn test_get_de() {
        let mut test_reg = CPURegisters::new();
        test_reg.d = 0x1F;
        test_reg.e = 0xF1;

        assert_eq!(test_reg.get_de(), 0x1FF1 as u16);
    }
    #[test]
    fn test_set_de() {
        let mut test_reg = CPURegisters::new();

        assert_eq!(test_reg.get_de(), 0);
        test_reg.set_de(0x1FF1);
        assert_eq!(test_reg.get_de(), 0x1FF1 as u16);
    }

    #[test]
    fn test_get_hl() {
        let mut test_reg = CPURegisters::new();
        test_reg.h = 0x1F;
        test_reg.l = 0xF1;

        assert_eq!(test_reg.get_hl(), 0x1FF1 as u16);
    }
    #[test]
    fn test_set_hl() {
        let mut test_reg = CPURegisters::new();

        assert_eq!(test_reg.get_hl(), 0);
        test_reg.set_hl(0x1FF1);
        assert_eq!(test_reg.get_hl(), 0x1FF1 as u16);
    }
}
