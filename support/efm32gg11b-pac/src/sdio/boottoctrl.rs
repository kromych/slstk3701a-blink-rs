#[doc = "Register `BOOTTOCTRL` reader"]
pub type R = crate::R<BOOTTOCTRL_SPEC>;
#[doc = "Register `BOOTTOCTRL` writer"]
pub type W = crate::W<BOOTTOCTRL_SPEC>;
#[doc = "Field `BOOTDATTOCNT` reader - Boot Data Timeout Counter Value"]
pub type BOOTDATTOCNT_R = crate::FieldReader<u32>;
#[doc = "Field `BOOTDATTOCNT` writer - Boot Data Timeout Counter Value"]
pub type BOOTDATTOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Boot Data Timeout Counter Value"]
    #[inline(always)]
    pub fn bootdattocnt(&self) -> BOOTDATTOCNT_R {
        BOOTDATTOCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Boot Data Timeout Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn bootdattocnt(&mut self) -> BOOTDATTOCNT_W<BOOTTOCTRL_SPEC> {
        BOOTDATTOCNT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Boot Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boottoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boottoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTTOCTRL_SPEC;
impl crate::RegisterSpec for BOOTTOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boottoctrl::R`](R) reader structure"]
impl crate::Readable for BOOTTOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boottoctrl::W`](W) writer structure"]
impl crate::Writable for BOOTTOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTTOCTRL to value 0"]
impl crate::Resettable for BOOTTOCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
