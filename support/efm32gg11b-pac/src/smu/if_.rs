#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `PPUPRIV` reader - PPU Privilege Interrupt Flag"]
pub type PPUPRIV_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PPU Privilege Interrupt Flag"]
    #[inline(always)]
    pub fn ppupriv(&self) -> PPUPRIV_R {
        PPUPRIV_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: u32 = 0;
}
