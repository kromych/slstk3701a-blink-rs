#[doc = "Register `ETMCR` reader"]
pub type R = crate::R<ETMCR_SPEC>;
#[doc = "Register `ETMCR` writer"]
pub type W = crate::W<ETMCR_SPEC>;
#[doc = "Field `POWERDWN` reader - ETM Control in low power mode"]
pub type POWERDWN_R = crate::BitReader;
#[doc = "Field `POWERDWN` writer - ETM Control in low power mode"]
pub type POWERDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTSIZE` reader - ETM Port Size"]
pub type PORTSIZE_R = crate::FieldReader;
#[doc = "Field `PORTSIZE` writer - ETM Port Size"]
pub type PORTSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STALL` reader - Stall Processor"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - Stall Processor"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRANCHOUTPUT` reader - Branch Output"]
pub type BRANCHOUTPUT_R = crate::BitReader;
#[doc = "Field `BRANCHOUTPUT` writer - Branch Output"]
pub type BRANCHOUTPUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGREQCTRL` reader - Debug Request Control"]
pub type DBGREQCTRL_R = crate::BitReader;
#[doc = "Field `DBGREQCTRL` writer - Debug Request Control"]
pub type DBGREQCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETMPROG` reader - ETM Programming"]
pub type ETMPROG_R = crate::BitReader;
#[doc = "Field `ETMPROG` writer - ETM Programming"]
pub type ETMPROG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETMPORTSEL` reader - ETM Port Selection"]
pub type ETMPORTSEL_R = crate::BitReader;
#[doc = "Field `ETMPORTSEL` writer - ETM Port Selection"]
pub type ETMPORTSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTMODE2` reader - Port Mode\\[2\\]"]
pub type PORTMODE2_R = crate::BitReader;
#[doc = "Field `PORTMODE2` writer - Port Mode\\[2\\]"]
pub type PORTMODE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTMODE` reader - Port Mode Control"]
pub type PORTMODE_R = crate::FieldReader;
#[doc = "Field `PORTMODE` writer - Port Mode Control"]
pub type PORTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPORTSIZE` reader - Port Size\\[3\\]"]
pub type EPORTSIZE_R = crate::FieldReader;
#[doc = "Field `EPORTSIZE` writer - Port Size\\[3\\]"]
pub type EPORTSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSTAMPEN` reader - Time Stamp Enable"]
pub type TSTAMPEN_R = crate::BitReader;
#[doc = "Field `TSTAMPEN` writer - Time Stamp Enable"]
pub type TSTAMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ETM Control in low power mode"]
    #[inline(always)]
    pub fn powerdwn(&self) -> POWERDWN_R {
        POWERDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - ETM Port Size"]
    #[inline(always)]
    pub fn portsize(&self) -> PORTSIZE_R {
        PORTSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn branchoutput(&self) -> BRANCHOUTPUT_R {
        BRANCHOUTPUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgreqctrl(&self) -> DBGREQCTRL_R {
        DBGREQCTRL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn etmprog(&self) -> ETMPROG_R {
        ETMPROG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ETM Port Selection"]
    #[inline(always)]
    pub fn etmportsel(&self) -> ETMPORTSEL_R {
        ETMPORTSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Mode\\[2\\]"]
    #[inline(always)]
    pub fn portmode2(&self) -> PORTMODE2_R {
        PORTMODE2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Port Mode Control"]
    #[inline(always)]
    pub fn portmode(&self) -> PORTMODE_R {
        PORTMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 21:22 - Port Size\\[3\\]"]
    #[inline(always)]
    pub fn eportsize(&self) -> EPORTSIZE_R {
        EPORTSIZE_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 28 - Time Stamp Enable"]
    #[inline(always)]
    pub fn tstampen(&self) -> TSTAMPEN_R {
        TSTAMPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETM Control in low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn powerdwn(&mut self) -> POWERDWN_W<ETMCR_SPEC> {
        POWERDWN_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - ETM Port Size"]
    #[inline(always)]
    #[must_use]
    pub fn portsize(&mut self) -> PORTSIZE_W<ETMCR_SPEC> {
        PORTSIZE_W::new(self, 4)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<ETMCR_SPEC> {
        STALL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    #[must_use]
    pub fn branchoutput(&mut self) -> BRANCHOUTPUT_W<ETMCR_SPEC> {
        BRANCHOUTPUT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    #[must_use]
    pub fn dbgreqctrl(&mut self) -> DBGREQCTRL_W<ETMCR_SPEC> {
        DBGREQCTRL_W::new(self, 9)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    #[must_use]
    pub fn etmprog(&mut self) -> ETMPROG_W<ETMCR_SPEC> {
        ETMPROG_W::new(self, 10)
    }
    #[doc = "Bit 11 - ETM Port Selection"]
    #[inline(always)]
    #[must_use]
    pub fn etmportsel(&mut self) -> ETMPORTSEL_W<ETMCR_SPEC> {
        ETMPORTSEL_W::new(self, 11)
    }
    #[doc = "Bit 13 - Port Mode\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn portmode2(&mut self) -> PORTMODE2_W<ETMCR_SPEC> {
        PORTMODE2_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - Port Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn portmode(&mut self) -> PORTMODE_W<ETMCR_SPEC> {
        PORTMODE_W::new(self, 16)
    }
    #[doc = "Bits 21:22 - Port Size\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn eportsize(&mut self) -> EPORTSIZE_W<ETMCR_SPEC> {
        EPORTSIZE_W::new(self, 21)
    }
    #[doc = "Bit 28 - Time Stamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstampen(&mut self) -> TSTAMPEN_W<ETMCR_SPEC> {
        TSTAMPEN_W::new(self, 28)
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
#[doc = "Main Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMCR_SPEC;
impl crate::RegisterSpec for ETMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcr::R`](R) reader structure"]
impl crate::Readable for ETMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmcr::W`](W) writer structure"]
impl crate::Writable for ETMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMCR to value 0x0411"]
impl crate::Resettable for ETMCR_SPEC {
    const RESET_VALUE: u32 = 0x0411;
}
