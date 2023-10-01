#[doc = "Register `DSTATUS` reader"]
pub type R = crate::R<DSTATUS_SPEC>;
#[doc = "Field `DATA0ZERO` reader - Data 0 Zero"]
pub type DATA0ZERO_R = crate::FieldReader<DATA0ZERO_A>;
#[doc = "Data 0 Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATA0ZERO_A {
    #[doc = "1: In DATA0 bits 0 to 31 are all zero."]
    ZERO0TO31 = 1,
    #[doc = "2: In DATA0 bits 32 to 63 are all zero."]
    ZERO32TO63 = 2,
    #[doc = "4: In DATA0 bits 64 to 95 are all zero."]
    ZERO64TO95 = 4,
    #[doc = "8: In DATA0 bits 96 to 127 are all zero."]
    ZERO96TO127 = 8,
}
impl From<DATA0ZERO_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA0ZERO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATA0ZERO_A {
    type Ux = u8;
}
impl DATA0ZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATA0ZERO_A> {
        match self.bits {
            1 => Some(DATA0ZERO_A::ZERO0TO31),
            2 => Some(DATA0ZERO_A::ZERO32TO63),
            4 => Some(DATA0ZERO_A::ZERO64TO95),
            8 => Some(DATA0ZERO_A::ZERO96TO127),
            _ => None,
        }
    }
    #[doc = "In DATA0 bits 0 to 31 are all zero."]
    #[inline(always)]
    pub fn is_zero0to31(&self) -> bool {
        *self == DATA0ZERO_A::ZERO0TO31
    }
    #[doc = "In DATA0 bits 32 to 63 are all zero."]
    #[inline(always)]
    pub fn is_zero32to63(&self) -> bool {
        *self == DATA0ZERO_A::ZERO32TO63
    }
    #[doc = "In DATA0 bits 64 to 95 are all zero."]
    #[inline(always)]
    pub fn is_zero64to95(&self) -> bool {
        *self == DATA0ZERO_A::ZERO64TO95
    }
    #[doc = "In DATA0 bits 96 to 127 are all zero."]
    #[inline(always)]
    pub fn is_zero96to127(&self) -> bool {
        *self == DATA0ZERO_A::ZERO96TO127
    }
}
#[doc = "Field `DDATA0LSBS` reader - LSBs in DDATA0"]
pub type DDATA0LSBS_R = crate::FieldReader;
#[doc = "Field `DDATA0MSBS` reader - MSB in DDATA0"]
pub type DDATA0MSBS_R = crate::FieldReader;
#[doc = "Field `DDATA1MSB` reader - MSB in DDATA1"]
pub type DDATA1MSB_R = crate::BitReader;
#[doc = "Field `CARRY` reader - Carry From Arithmetic Operation"]
pub type CARRY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Data 0 Zero"]
    #[inline(always)]
    pub fn data0zero(&self) -> DATA0ZERO_R {
        DATA0ZERO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - LSBs in DDATA0"]
    #[inline(always)]
    pub fn ddata0lsbs(&self) -> DDATA0LSBS_R {
        DDATA0LSBS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - MSB in DDATA0"]
    #[inline(always)]
    pub fn ddata0msbs(&self) -> DDATA0MSBS_R {
        DDATA0MSBS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - MSB in DDATA1"]
    #[inline(always)]
    pub fn ddata1msb(&self) -> DDATA1MSB_R {
        DDATA1MSB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Carry From Arithmetic Operation"]
    #[inline(always)]
    pub fn carry(&self) -> CARRY_R {
        CARRY_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTATUS_SPEC;
impl crate::RegisterSpec for DSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstatus::R`](R) reader structure"]
impl crate::Readable for DSTATUS_SPEC {}
#[doc = "`reset()` method sets DSTATUS to value 0"]
impl crate::Resettable for DSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
