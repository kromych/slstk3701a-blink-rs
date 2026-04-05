#[doc = "Register `SLOTINTSTAT` reader"]
pub type R = crate::R<SlotintstatSpec>;
#[doc = "Field `INTSLOT0` reader - Interrupt Signal for Slot#0"]
pub type Intslot0R = crate::BitReader;
#[doc = "Field `SPECVERNUM` reader - Host Controller Compliant Spec Version Number"]
pub type SpecvernumR = crate::FieldReader;
#[doc = "Field `VENDVERNUM` reader - Vendor Version Number"]
pub type VendvernumR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Interrupt Signal for Slot#0"]
    #[inline(always)]
    pub fn intslot0(&self) -> Intslot0R {
        Intslot0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Host Controller Compliant Spec Version Number"]
    #[inline(always)]
    pub fn specvernum(&self) -> SpecvernumR {
        SpecvernumR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vendor Version Number"]
    #[inline(always)]
    pub fn vendvernum(&self) -> VendvernumR {
        VendvernumR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Slot Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slotintstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlotintstatSpec;
impl crate::RegisterSpec for SlotintstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slotintstat::R`](R) reader structure"]
impl crate::Readable for SlotintstatSpec {}
#[doc = "`reset()` method sets SLOTINTSTAT to value 0x1002_0000"]
impl crate::Resettable for SlotintstatSpec {
    const RESET_VALUE: u32 = 0x1002_0000;
}
