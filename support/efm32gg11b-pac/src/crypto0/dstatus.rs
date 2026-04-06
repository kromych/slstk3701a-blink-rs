#[doc = "Register `DSTATUS` reader"]
pub type R = crate::R<DstatusSpec>;
#[doc = "Data 0 Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Data0zero {
    #[doc = "1: In DATA0 bits 0 to 31 are all zero."]
    Zero0to31 = 1,
    #[doc = "2: In DATA0 bits 32 to 63 are all zero."]
    Zero32to63 = 2,
    #[doc = "4: In DATA0 bits 64 to 95 are all zero."]
    Zero64to95 = 4,
    #[doc = "8: In DATA0 bits 96 to 127 are all zero."]
    Zero96to127 = 8,
}
impl From<Data0zero> for u8 {
    #[inline(always)]
    fn from(variant: Data0zero) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Data0zero {
    type Ux = u8;
}
impl crate::IsEnum for Data0zero {}
#[doc = "Field `DATA0ZERO` reader - Data 0 Zero"]
pub type Data0zeroR = crate::FieldReader<Data0zero>;
impl Data0zeroR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Data0zero> {
        match self.bits {
            1 => Some(Data0zero::Zero0to31),
            2 => Some(Data0zero::Zero32to63),
            4 => Some(Data0zero::Zero64to95),
            8 => Some(Data0zero::Zero96to127),
            _ => None,
        }
    }
    #[doc = "In DATA0 bits 0 to 31 are all zero."]
    #[inline(always)]
    pub fn is_zero0to31(&self) -> bool {
        *self == Data0zero::Zero0to31
    }
    #[doc = "In DATA0 bits 32 to 63 are all zero."]
    #[inline(always)]
    pub fn is_zero32to63(&self) -> bool {
        *self == Data0zero::Zero32to63
    }
    #[doc = "In DATA0 bits 64 to 95 are all zero."]
    #[inline(always)]
    pub fn is_zero64to95(&self) -> bool {
        *self == Data0zero::Zero64to95
    }
    #[doc = "In DATA0 bits 96 to 127 are all zero."]
    #[inline(always)]
    pub fn is_zero96to127(&self) -> bool {
        *self == Data0zero::Zero96to127
    }
}
#[doc = "Field `DDATA0LSBS` reader - LSBs in DDATA0"]
pub type Ddata0lsbsR = crate::FieldReader;
#[doc = "Field `DDATA0MSBS` reader - MSB in DDATA0"]
pub type Ddata0msbsR = crate::FieldReader;
#[doc = "Field `DDATA1MSB` reader - MSB in DDATA1"]
pub type Ddata1msbR = crate::BitReader;
#[doc = "Field `CARRY` reader - Carry From Arithmetic Operation"]
pub type CarryR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Data 0 Zero"]
    #[inline(always)]
    pub fn data0zero(&self) -> Data0zeroR {
        Data0zeroR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - LSBs in DDATA0"]
    #[inline(always)]
    pub fn ddata0lsbs(&self) -> Ddata0lsbsR {
        Ddata0lsbsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - MSB in DDATA0"]
    #[inline(always)]
    pub fn ddata0msbs(&self) -> Ddata0msbsR {
        Ddata0msbsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - MSB in DDATA1"]
    #[inline(always)]
    pub fn ddata1msb(&self) -> Ddata1msbR {
        Ddata1msbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Carry From Arithmetic Operation"]
    #[inline(always)]
    pub fn carry(&self) -> CarryR {
        CarryR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstatusSpec;
impl crate::RegisterSpec for DstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstatus::R`](R) reader structure"]
impl crate::Readable for DstatusSpec {}
#[doc = "`reset()` method sets DSTATUS to value 0"]
impl crate::Resettable for DstatusSpec {}
