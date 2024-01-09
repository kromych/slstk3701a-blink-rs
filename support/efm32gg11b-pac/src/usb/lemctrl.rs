#[doc = "Register `LEMCTRL` reader"]
pub type R = crate::R<LEMCTRL_SPEC>;
#[doc = "Register `LEMCTRL` writer"]
pub type W = crate::W<LEMCTRL_SPEC>;
#[doc = "Field `TIMEBASE` reader - Set the Number of LFC Clk Counts to Form 3ms"]
pub type TIMEBASE_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEBASE` writer - Set the Number of LFC Clk Counts to Form 3ms"]
pub type TIMEBASE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Set the Number of LFC Clk Counts to Form 3ms"]
    #[inline(always)]
    pub fn timebase(&self) -> TIMEBASE_R {
        TIMEBASE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Set the Number of LFC Clk Counts to Form 3ms"]
    #[inline(always)]
    #[must_use]
    pub fn timebase(&mut self) -> TIMEBASE_W<LEMCTRL_SPEC> {
        TIMEBASE_W::new(self, 0)
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
#[doc = "USB LEM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lemctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lemctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEMCTRL_SPEC;
impl crate::RegisterSpec for LEMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lemctrl::R`](R) reader structure"]
impl crate::Readable for LEMCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lemctrl::W`](W) writer structure"]
impl crate::Writable for LEMCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LEMCTRL to value 0x67"]
impl crate::Resettable for LEMCTRL_SPEC {
    const RESET_VALUE: u32 = 0x67;
}
