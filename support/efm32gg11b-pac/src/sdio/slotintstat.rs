#[doc = "Register `SLOTINTSTAT` reader"]
pub type R = crate::R<SLOTINTSTAT_SPEC>;
#[doc = "Field `INTSLOT0` reader - Interrupt Signal for Slot#0"]
pub type INTSLOT0_R = crate::BitReader;
#[doc = "Field `SPECVERNUM` reader - Host Controller Compliant Spec Version Number"]
pub type SPECVERNUM_R = crate::FieldReader;
#[doc = "Field `VENDVERNUM` reader - Vendor Version Number"]
pub type VENDVERNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Interrupt Signal for Slot#0"]
    #[inline(always)]
    pub fn intslot0(&self) -> INTSLOT0_R {
        INTSLOT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Host Controller Compliant Spec Version Number"]
    #[inline(always)]
    pub fn specvernum(&self) -> SPECVERNUM_R {
        SPECVERNUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vendor Version Number"]
    #[inline(always)]
    pub fn vendvernum(&self) -> VENDVERNUM_R {
        VENDVERNUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Slot Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slotintstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLOTINTSTAT_SPEC;
impl crate::RegisterSpec for SLOTINTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slotintstat::R`](R) reader structure"]
impl crate::Readable for SLOTINTSTAT_SPEC {}
#[doc = "`reset()` method sets SLOTINTSTAT to value 0x1002_0000"]
impl crate::Resettable for SLOTINTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x1002_0000;
}
