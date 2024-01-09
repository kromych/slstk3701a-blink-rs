#[doc = "Register `INDIRECTWRITEXFERCTRL` reader"]
pub type R = crate::R<INDIRECTWRITEXFERCTRL_SPEC>;
#[doc = "Register `INDIRECTWRITEXFERCTRL` writer"]
pub type W = crate::W<INDIRECTWRITEXFERCTRL_SPEC>;
#[doc = "Field `START` writer - Start Indirect Write"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANCEL` writer - Cancel Indirect Write"]
pub type CANCEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRSTATUS` reader - Indirect Write Status"]
pub type WRSTATUS_R = crate::BitReader;
#[doc = "Field `WRQUEUED` reader - Two Indirect Write Operations Have Been Queued"]
pub type WRQUEUED_R = crate::BitReader;
#[doc = "Field `INDOPSDONESTATUS` reader - Indirect Completion Status"]
pub type INDOPSDONESTATUS_R = crate::BitReader;
#[doc = "Field `INDOPSDONESTATUS` writer - Indirect Completion Status"]
pub type INDOPSDONESTATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMINDOPSDONE` reader - Indirect Operations Done"]
pub type NUMINDOPSDONE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 2 - Indirect Write Status"]
    #[inline(always)]
    pub fn wrstatus(&self) -> WRSTATUS_R {
        WRSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Two Indirect Write Operations Have Been Queued"]
    #[inline(always)]
    pub fn wrqueued(&self) -> WRQUEUED_R {
        WRQUEUED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    pub fn indopsdonestatus(&self) -> INDOPSDONESTATUS_R {
        INDOPSDONESTATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Indirect Operations Done"]
    #[inline(always)]
    pub fn numindopsdone(&self) -> NUMINDOPSDONE_R {
        NUMINDOPSDONE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start Indirect Write"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<INDIRECTWRITEXFERCTRL_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Cancel Indirect Write"]
    #[inline(always)]
    #[must_use]
    pub fn cancel(&mut self) -> CANCEL_W<INDIRECTWRITEXFERCTRL_SPEC> {
        CANCEL_W::new(self, 1)
    }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    #[must_use]
    pub fn indopsdonestatus(&mut self) -> INDOPSDONESTATUS_W<INDIRECTWRITEXFERCTRL_SPEC> {
        INDOPSDONESTATUS_W::new(self, 5)
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
#[doc = "Indirect Write Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectwritexferctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectwritexferctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDIRECTWRITEXFERCTRL_SPEC;
impl crate::RegisterSpec for INDIRECTWRITEXFERCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectwritexferctrl::R`](R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`indirectwritexferctrl::W`](W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERCTRL to value 0"]
impl crate::Resettable for INDIRECTWRITEXFERCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
