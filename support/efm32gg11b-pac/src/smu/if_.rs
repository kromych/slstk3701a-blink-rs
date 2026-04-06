#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `PPUPRIV` reader - PPU Privilege Interrupt Flag"]
pub type PpuprivR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PPU Privilege Interrupt Flag"]
    #[inline(always)]
    pub fn ppupriv(&self) -> PpuprivR {
        PpuprivR::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
