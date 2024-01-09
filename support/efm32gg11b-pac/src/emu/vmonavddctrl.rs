#[doc = "Register `VMONAVDDCTRL` reader"]
pub type R = crate::R<VMONAVDDCTRL_SPEC>;
#[doc = "Register `VMONAVDDCTRL` writer"]
pub type W = crate::W<VMONAVDDCTRL_SPEC>;
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RISEWU` reader - Rise Wakeup"]
pub type RISEWU_R = crate::BitReader;
#[doc = "Field `RISEWU` writer - Rise Wakeup"]
pub type RISEWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALLWU` reader - Fall Wakeup"]
pub type FALLWU_R = crate::BitReader;
#[doc = "Field `FALLWU` writer - Fall Wakeup"]
pub type FALLWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALLTHRESFINE` reader - Falling Threshold Fine Adjust"]
pub type FALLTHRESFINE_R = crate::FieldReader;
#[doc = "Field `FALLTHRESFINE` writer - Falling Threshold Fine Adjust"]
pub type FALLTHRESFINE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FALLTHRESCOARSE` reader - Falling Threshold Coarse Adjust"]
pub type FALLTHRESCOARSE_R = crate::FieldReader;
#[doc = "Field `FALLTHRESCOARSE` writer - Falling Threshold Coarse Adjust"]
pub type FALLTHRESCOARSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RISETHRESFINE` reader - Rising Threshold Fine Adjust"]
pub type RISETHRESFINE_R = crate::FieldReader;
#[doc = "Field `RISETHRESFINE` writer - Rising Threshold Fine Adjust"]
pub type RISETHRESFINE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RISETHRESCOARSE` reader - Rising Threshold Coarse Adjust"]
pub type RISETHRESCOARSE_R = crate::FieldReader;
#[doc = "Field `RISETHRESCOARSE` writer - Rising Threshold Coarse Adjust"]
pub type RISETHRESCOARSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    pub fn risewu(&self) -> RISEWU_R {
        RISEWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    pub fn fallwu(&self) -> FALLWU_R {
        FALLWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Falling Threshold Fine Adjust"]
    #[inline(always)]
    pub fn fallthresfine(&self) -> FALLTHRESFINE_R {
        FALLTHRESFINE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Falling Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn fallthrescoarse(&self) -> FALLTHRESCOARSE_R {
        FALLTHRESCOARSE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Rising Threshold Fine Adjust"]
    #[inline(always)]
    pub fn risethresfine(&self) -> RISETHRESFINE_R {
        RISETHRESFINE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Rising Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn risethrescoarse(&self) -> RISETHRESCOARSE_R {
        RISETHRESCOARSE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<VMONAVDDCTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn risewu(&mut self) -> RISEWU_W<VMONAVDDCTRL_SPEC> {
        RISEWU_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fallwu(&mut self) -> FALLWU_W<VMONAVDDCTRL_SPEC> {
        FALLWU_W::new(self, 3)
    }
    #[doc = "Bits 8:11 - Falling Threshold Fine Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn fallthresfine(&mut self) -> FALLTHRESFINE_W<VMONAVDDCTRL_SPEC> {
        FALLTHRESFINE_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Falling Threshold Coarse Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn fallthrescoarse(&mut self) -> FALLTHRESCOARSE_W<VMONAVDDCTRL_SPEC> {
        FALLTHRESCOARSE_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Rising Threshold Fine Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn risethresfine(&mut self) -> RISETHRESFINE_W<VMONAVDDCTRL_SPEC> {
        RISETHRESFINE_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Rising Threshold Coarse Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn risethrescoarse(&mut self) -> RISETHRESCOARSE_W<VMONAVDDCTRL_SPEC> {
        RISETHRESCOARSE_W::new(self, 20)
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
#[doc = "VMON AVDD Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmonavddctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmonavddctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMONAVDDCTRL_SPEC;
impl crate::RegisterSpec for VMONAVDDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmonavddctrl::R`](R) reader structure"]
impl crate::Readable for VMONAVDDCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vmonavddctrl::W`](W) writer structure"]
impl crate::Writable for VMONAVDDCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VMONAVDDCTRL to value 0"]
impl crate::Resettable for VMONAVDDCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
