#[doc = "Register `INDIRECTREADXFERCTRL` reader"]
pub type R = crate::R<INDIRECTREADXFERCTRL_SPEC>;
#[doc = "Register `INDIRECTREADXFERCTRL` writer"]
pub type W = crate::W<INDIRECTREADXFERCTRL_SPEC>;
#[doc = "Field `START` writer - Start Indirect Read"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CANCEL` writer - Cancel Indirect Read"]
pub type CANCEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDSTATUS` reader - Indirect Read Status"]
pub type RDSTATUS_R = crate::BitReader;
#[doc = "Field `SRAMFULL` reader - SRAM Full"]
pub type SRAMFULL_R = crate::BitReader;
#[doc = "Field `SRAMFULL` writer - SRAM Full"]
pub type SRAMFULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDQUEUED` reader - Two Indirect Read Operations Have Been Queued"]
pub type RDQUEUED_R = crate::BitReader;
#[doc = "Field `INDOPSDONESTATUS` reader - Indirect Completion Status"]
pub type INDOPSDONESTATUS_R = crate::BitReader;
#[doc = "Field `INDOPSDONESTATUS` writer - Indirect Completion Status"]
pub type INDOPSDONESTATUS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NUMINDOPSDONE` reader - Number Indirect Operations Done"]
pub type NUMINDOPSDONE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 2 - Indirect Read Status"]
    #[inline(always)]
    pub fn rdstatus(&self) -> RDSTATUS_R {
        RDSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Full"]
    #[inline(always)]
    pub fn sramfull(&self) -> SRAMFULL_R {
        SRAMFULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Two Indirect Read Operations Have Been Queued"]
    #[inline(always)]
    pub fn rdqueued(&self) -> RDQUEUED_R {
        RDQUEUED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    pub fn indopsdonestatus(&self) -> INDOPSDONESTATUS_R {
        INDOPSDONESTATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Number Indirect Operations Done"]
    #[inline(always)]
    pub fn numindopsdone(&self) -> NUMINDOPSDONE_R {
        NUMINDOPSDONE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start Indirect Read"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<INDIRECTREADXFERCTRL_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Cancel Indirect Read"]
    #[inline(always)]
    #[must_use]
    pub fn cancel(&mut self) -> CANCEL_W<INDIRECTREADXFERCTRL_SPEC, 1> {
        CANCEL_W::new(self)
    }
    #[doc = "Bit 3 - SRAM Full"]
    #[inline(always)]
    #[must_use]
    pub fn sramfull(&mut self) -> SRAMFULL_W<INDIRECTREADXFERCTRL_SPEC, 3> {
        SRAMFULL_W::new(self)
    }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    #[must_use]
    pub fn indopsdonestatus(&mut self) -> INDOPSDONESTATUS_W<INDIRECTREADXFERCTRL_SPEC, 5> {
        INDOPSDONESTATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Indirect Read Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectreadxferctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectreadxferctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDIRECTREADXFERCTRL_SPEC;
impl crate::RegisterSpec for INDIRECTREADXFERCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectreadxferctrl::R`](R) reader structure"]
impl crate::Readable for INDIRECTREADXFERCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`indirectreadxferctrl::W`](W) writer structure"]
impl crate::Writable for INDIRECTREADXFERCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INDIRECTREADXFERCTRL to value 0"]
impl crate::Resettable for INDIRECTREADXFERCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
