#[doc = "Register `FREEZE` reader"]
pub type R = crate::R<FREEZE_SPEC>;
#[doc = "Register `FREEZE` writer"]
pub type W = crate::W<FREEZE_SPEC>;
#[doc = "Field `REGFREEZE` reader - Register Update Freeze"]
pub type REGFREEZE_R = crate::BitReader;
#[doc = "Field `REGFREEZE` writer - Register Update Freeze"]
pub type REGFREEZE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCDGATE` reader - LCD Gate"]
pub type LCDGATE_R = crate::BitReader;
#[doc = "Field `LCDGATE` writer - LCD Gate"]
pub type LCDGATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Register Update Freeze"]
    #[inline(always)]
    pub fn regfreeze(&self) -> REGFREEZE_R {
        REGFREEZE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Gate"]
    #[inline(always)]
    pub fn lcdgate(&self) -> LCDGATE_R {
        LCDGATE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Update Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn regfreeze(&mut self) -> REGFREEZE_W<FREEZE_SPEC, 0> {
        REGFREEZE_W::new(self)
    }
    #[doc = "Bit 1 - LCD Gate"]
    #[inline(always)]
    #[must_use]
    pub fn lcdgate(&mut self) -> LCDGATE_W<FREEZE_SPEC, 1> {
        LCDGATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freeze::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freeze::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FREEZE_SPEC;
impl crate::RegisterSpec for FREEZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freeze::R`](R) reader structure"]
impl crate::Readable for FREEZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`freeze::W`](W) writer structure"]
impl crate::Writable for FREEZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREEZE to value 0"]
impl crate::Resettable for FREEZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}