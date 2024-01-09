#[doc = "Register `SPECADDR4BOTTOM` reader"]
pub type R = crate::R<SPECADDR4BOTTOM_SPEC>;
#[doc = "Register `SPECADDR4BOTTOM` writer"]
pub type W = crate::W<SPECADDR4BOTTOM_SPEC>;
#[doc = "Field `ADDR` reader - Least significant 32 bits of the destination address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Least significant 32 bits of the destination address"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Least significant 32 bits of the destination address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Least significant 32 bits of the destination address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SPECADDR4BOTTOM_SPEC> {
        ADDR_W::new(self, 0)
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
#[doc = "Specific Address 4 Bottom\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr4bottom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr4bottom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPECADDR4BOTTOM_SPEC;
impl crate::RegisterSpec for SPECADDR4BOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`specaddr4bottom::R`](R) reader structure"]
impl crate::Readable for SPECADDR4BOTTOM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`specaddr4bottom::W`](W) writer structure"]
impl crate::Writable for SPECADDR4BOTTOM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPECADDR4BOTTOM to value 0"]
impl crate::Resettable for SPECADDR4BOTTOM_SPEC {
    const RESET_VALUE: u32 = 0;
}
