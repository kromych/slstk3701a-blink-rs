#[doc = "Register `CSTATUS` reader"]
pub type R = crate::R<CSTATUS_SPEC>;
#[doc = "Field `V0` reader - Selected ALU Operand 0"]
pub type V0_R = crate::FieldReader<V0_A>;
#[doc = "Selected ALU Operand 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum V0_A {
    #[doc = "0: `0`"]
    DDATA0 = 0,
    #[doc = "1: `1`"]
    DDATA1 = 1,
    #[doc = "2: `10`"]
    DDATA2 = 2,
    #[doc = "3: `11`"]
    DDATA3 = 3,
    #[doc = "4: `100`"]
    DDATA4 = 4,
    #[doc = "5: `101`"]
    DATA0 = 5,
    #[doc = "6: `110`"]
    DATA1 = 6,
    #[doc = "7: `111`"]
    DATA2 = 7,
}
impl From<V0_A> for u8 {
    #[inline(always)]
    fn from(variant: V0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for V0_A {
    type Ux = u8;
}
impl V0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V0_A {
        match self.bits {
            0 => V0_A::DDATA0,
            1 => V0_A::DDATA1,
            2 => V0_A::DDATA2,
            3 => V0_A::DDATA3,
            4 => V0_A::DDATA4,
            5 => V0_A::DATA0,
            6 => V0_A::DATA1,
            7 => V0_A::DATA2,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == V0_A::DDATA0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == V0_A::DDATA1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ddata2(&self) -> bool {
        *self == V0_A::DDATA2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_ddata3(&self) -> bool {
        *self == V0_A::DDATA3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_ddata4(&self) -> bool {
        *self == V0_A::DDATA4
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == V0_A::DATA0
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == V0_A::DATA1
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == V0_A::DATA2
    }
}
#[doc = "Field `V1` reader - Selected ALU Operand 1"]
pub type V1_R = crate::FieldReader<V1_A>;
#[doc = "Selected ALU Operand 1\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum V1_A {
    #[doc = "0: `0`"]
    DDATA0 = 0,
    #[doc = "1: `1`"]
    DDATA1 = 1,
    #[doc = "2: `10`"]
    DDATA2 = 2,
    #[doc = "3: `11`"]
    DDATA3 = 3,
    #[doc = "4: `100`"]
    DDATA4 = 4,
    #[doc = "5: `101`"]
    DATA0 = 5,
    #[doc = "6: `110`"]
    DATA1 = 6,
    #[doc = "7: `111`"]
    DATA2 = 7,
}
impl From<V1_A> for u8 {
    #[inline(always)]
    fn from(variant: V1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for V1_A {
    type Ux = u8;
}
impl V1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V1_A {
        match self.bits {
            0 => V1_A::DDATA0,
            1 => V1_A::DDATA1,
            2 => V1_A::DDATA2,
            3 => V1_A::DDATA3,
            4 => V1_A::DDATA4,
            5 => V1_A::DATA0,
            6 => V1_A::DATA1,
            7 => V1_A::DATA2,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == V1_A::DDATA0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == V1_A::DDATA1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ddata2(&self) -> bool {
        *self == V1_A::DDATA2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_ddata3(&self) -> bool {
        *self == V1_A::DDATA3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_ddata4(&self) -> bool {
        *self == V1_A::DDATA4
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == V1_A::DATA0
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == V1_A::DATA1
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == V1_A::DATA2
    }
}
#[doc = "Field `SEQPART` reader - Sequence Part"]
pub type SEQPART_R = crate::BitReader;
#[doc = "Field `SEQSKIP` reader - Sequence Skip Next Instruction"]
pub type SEQSKIP_R = crate::BitReader;
#[doc = "Field `SEQIP` reader - Sequence Next Instruction Pointer"]
pub type SEQIP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Selected ALU Operand 0"]
    #[inline(always)]
    pub fn v0(&self) -> V0_R {
        V0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Selected ALU Operand 1"]
    #[inline(always)]
    pub fn v1(&self) -> V1_R {
        V1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Sequence Part"]
    #[inline(always)]
    pub fn seqpart(&self) -> SEQPART_R {
        SEQPART_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sequence Skip Next Instruction"]
    #[inline(always)]
    pub fn seqskip(&self) -> SEQSKIP_R {
        SEQSKIP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:24 - Sequence Next Instruction Pointer"]
    #[inline(always)]
    pub fn seqip(&self) -> SEQIP_R {
        SEQIP_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
#[doc = "Control Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSTATUS_SPEC;
impl crate::RegisterSpec for CSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cstatus::R`](R) reader structure"]
impl crate::Readable for CSTATUS_SPEC {}
#[doc = "`reset()` method sets CSTATUS to value 0x0201"]
impl crate::Resettable for CSTATUS_SPEC {
    const RESET_VALUE: u32 = 0x0201;
}
