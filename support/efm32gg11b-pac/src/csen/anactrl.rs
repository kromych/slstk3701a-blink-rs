#[doc = "Register `ANACTRL` reader"]
pub type R = crate::R<ANACTRL_SPEC>;
#[doc = "Register `ANACTRL` writer"]
pub type W = crate::W<ANACTRL_SPEC>;
#[doc = "Field `IREFPROG` reader - Reference Current Control."]
pub type IREFPROG_R = crate::FieldReader;
#[doc = "Field `IREFPROG` writer - Reference Current Control."]
pub type IREFPROG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IDACIREFS` reader - Current DAC and Reference Current Scale"]
pub type IDACIREFS_R = crate::FieldReader;
#[doc = "Field `IDACIREFS` writer - Current DAC and Reference Current Scale"]
pub type IDACIREFS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRSTPROG` reader - Reset Timing"]
pub type TRSTPROG_R = crate::FieldReader;
#[doc = "Field `TRSTPROG` writer - Reset Timing"]
pub type TRSTPROG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:6 - Reference Current Control."]
    #[inline(always)]
    pub fn irefprog(&self) -> IREFPROG_R {
        IREFPROG_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Current DAC and Reference Current Scale"]
    #[inline(always)]
    pub fn idacirefs(&self) -> IDACIREFS_R {
        IDACIREFS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Reset Timing"]
    #[inline(always)]
    pub fn trstprog(&self) -> TRSTPROG_R {
        TRSTPROG_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Reference Current Control."]
    #[inline(always)]
    #[must_use]
    pub fn irefprog(&mut self) -> IREFPROG_W<ANACTRL_SPEC> {
        IREFPROG_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Current DAC and Reference Current Scale"]
    #[inline(always)]
    #[must_use]
    pub fn idacirefs(&mut self) -> IDACIREFS_W<ANACTRL_SPEC> {
        IDACIREFS_W::new(self, 8)
    }
    #[doc = "Bits 20:22 - Reset Timing"]
    #[inline(always)]
    #[must_use]
    pub fn trstprog(&mut self) -> TRSTPROG_W<ANACTRL_SPEC> {
        TRSTPROG_W::new(self, 20)
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
#[doc = "Analog Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anactrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anactrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANACTRL_SPEC;
impl crate::RegisterSpec for ANACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anactrl::R`](R) reader structure"]
impl crate::Readable for ANACTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`anactrl::W`](W) writer structure"]
impl crate::Writable for ANACTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANACTRL to value 0x70"]
impl crate::Resettable for ANACTRL_SPEC {
    const RESET_VALUE: u32 = 0x70;
}
